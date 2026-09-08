as -o multiboot_segment.o src/arch/x86_64/multiboot_segment.asm
as -o boot.o src/arch/x86_64/boot.asm
as -o pre_long_util.o src/arch/x86_64/pre_long_util.asm
as -o pre_long_vga_out_stream.o src/arch/x86_64/pre_long_vga_out_stream.asm
as -o pre_long_core_dump.o src/arch/x86_64/pre_long_core_dump.asm
#as -o kernel_main.o src/arch/x86_64/kernel_main.asm
cargo build --release

exit_code=$?
if [ $exit_code -eq 0 ]; then
	# --oformat binary
	#ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld pre_long_core_dump.o pre_long_vga_out_stream.o pre_long_util.o multiboot_segment.o boot.o target/debug/liblinl_os.a
	ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld pre_long_core_dump.o pre_long_vga_out_stream.o pre_long_util.o multiboot_segment.o boot.o target/target/release/liblinl_os.a --no-relax -no-pie
	#ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld kernel_main.o pre_long_core_dump.o pre_long_vga_out_stream.o pre_long_util.o multiboot_segment.o boot.o
	#ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld multiboot_segment.o

	exit_code=$?
	if [ $exit_code -eq 0 ]; then
		rm boot.o
		rm multiboot_segment.o
		rm pre_long_util.o
		rm pre_long_vga_out_stream.o
		rm pre_long_core_dump.o
		#rm kernel_main.o
		grub-mkrescue -o linl.iso isofiles &>/dev/null
		#grub-mkrescue /usr/lib/grub/i386-pc -o linl.iso isofiles
		qemu-system-x86_64 -D ./qemu-test.log -d cpu,exec,int -no-reboot -drive file=linl.iso,format=raw,media=cdrom -no-reboot -no-shutdown -m 6G -cpu host -monitor stdio -enable-kvm
		#qemu-system-x86_64 -D ./qemu-test.log -d cpu,exec,int -no-reboot -drive file=linl.iso,format=raw,media=cdrom -no-reboot -no-shutdown -m 4G -monitor stdio
		printf '\n'
	fi
fi

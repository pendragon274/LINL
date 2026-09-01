as -o multiboot_segment.o src/arch/x86_64/multiboot_segment.asm
as -o boot.o src/arch/x86_64/boot.asm
as -o pre_long_util.o src/arch/x86_64/pre_long_util.asm
as -o pre_long_vga_out_stream.o src/arch/x86_64/pre_long_vga_out_stream.asm
as -o pre_long_core_dump.o src/arch/x86_64/pre_long_core_dump.asm

exit_code=$?
if [ $exit_code -eq 0 ]; then
	# --oformat binary
	ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld pre_long_core_dump.o pre_long_vga_out_stream.o pre_long_util.o multiboot_segment.o boot.o
	#ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld multiboot_segment.o

	exit_code=$?
	if [ $exit_code -eq 0 ]; then
		rm boot.o
		rm multiboot_segment.o
		rm pre_long_util.o
		rm pre_long_vga_out_stream.o
		rm pre_long_core_dump.o
		grub-mkrescue -o linl.iso isofiles
		#grub-mkrescue /usr/lib/grub/i386-pc -o linl.iso isofiles
		qemu-system-x86_64 -D ./qemu-test.log -d cpu,exec,int -no-reboot -drive file=linl.iso,format=raw,media=cdrom -no-reboot -no-shutdown -m 4G -cpu host -monitor stdio -enable-kvm
	fi
fi

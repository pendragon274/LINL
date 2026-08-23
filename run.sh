as -o multiboot_segment.o src/arch/x86_64/multiboot_segment.asm
as -o boot.o src/arch/x86_64/boot.asm

exit_code=$?
if [ $exit_code -eq 0 ]; then
	# --oformat binary
	ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld multiboot_segment.o boot.o
	#ld -n -o isofiles/boot/kernel.bin -T src/arch/x86_64/linker.ld multiboot_segment.o

	exit_code=$?
	if [ $exit_code -eq 0 ]; then
		rm boot.o
		rm multiboot_segment.o
		grub-mkrescue -o linl.iso isofiles
		#grub-mkrescue /usr/lib/grub/i386-pc -o linl.iso isofiles
		qemu-system-x86_64 -D ./qemu-test.log -d cpu,exec,int -drive file=linl.iso,format=raw,media=cdrom -m 4G -cpu host -enable-kvm
	fi
fi

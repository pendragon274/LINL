as -o multiboot_segment.o multiboot_segment.asm
as -o boot.o boot.asm

exit_code=$?
if [ $exit_code -eq 0 ]; then
	ld -n -o kernel.bin -T linker.ld -e _start multiboot_segment.o boot.o
	
	exit_code=$?
	if [ $exit_code -eq 0 ]; then
		rm boot.o
		rm multiboot_segment.o
		qemu-system-x86_64 kernel.bin
	fi
fi

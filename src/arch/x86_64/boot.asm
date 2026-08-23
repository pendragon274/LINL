.code32
.section .text
.globl _start
_start:
	mov $stack_top, %esp

	call check_multiboot

	#Print OK
	movl $0x2f4b2f4f, (0xb8000)

	hlt

check_multiboot:
	cmp $0x36d76289, %eax
	jne multiboot_fail
	ret
	multiboot_fail:
	mov $'0', %al
	jmp error

error:
	movl $0x4f524f45, (0xb8000)
	movl $0x4f3a4f52, (0xb8004)
	movl $0x4f204f20, (0xb8008)
	movb %al, (0xb800a)
	hlt

.fill 510 - (.-_start), 1, 0
.word 0xaa55

.section .bss
stack_bottom:
	.lcomm buffer, 64
stack_top:

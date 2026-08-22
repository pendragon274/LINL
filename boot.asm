.code16
.section .text
.globl _start
_start:
	mov $0x0e41, %ax
	int $0x10
	hlt

.fill 510 - (.-_start), 1, 0
.word 0xaa55

.code32
.section .multiboot_segment
.globl header_start
header_start:
	.long 0xe85250d6
	.long 0
	.long header_end - header_start
	.long 0x100000000 - (0xe85250d6 + (header_end - header_start))
	.word 0
	.word 0
	.long 8
header_end:

.section .text
.align 16
.globl _start
_start:
	jmp InitLongMode

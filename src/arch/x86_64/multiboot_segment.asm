.code16
.section .multiboot_segment
header_start:
	.long 0xe85250d6
	.long 0
	.long header_end - header_start
	.long 0x100000000 - (0xe85250d6 + (header_end - header_start))
	.word 0
	.word 0
	.long 8
header_end:

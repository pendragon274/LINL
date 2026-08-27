.code32
.section .pre_long_mode_kernel

# WriteBuffer(void) -> void
#.globl WriteBuffer
#WriteBuffer:
#	endbr32
#	
#	xor %eax, %eax					# idx = 0
#	push %eax
#
#	WriteBuffer_loop:
#	mov (%esp), %eax
#	cmp $25, %eax
#	jge WriteBuffer_end_loop
#	
#	mov (%esp), %edi
#	call WriteBufferLine
#
#	mov (%esp), %eax
#	inc %eax
#	mov %eax, (%esp)
#	jmp WriteBuffer_loop
#
#	WriteBuffer_end_loop:
#	pop %eax
#	ret

# FlushBufferVGA(void) -> void
.globl FlushBufferVGA
FlushBufferVGA:
	endbr32

	mov $VGA_Out_Buffer.buffer, %edi
	mov $0xb8000, %esi
	mov $40*25, %edx
	call MemCpy

	ret

# WriteCharVGA(c: char) -> void
.globl WriteCharVGA
WriteCharVGA:
	endbr32

	cmp %dl, '\n'
	jne WriteCharVGA_skip_new_line
	# if c == '\n' here...
	WriteCharVGA_skip_new_line:

	movb (VGA_Out_Buffer.current_color), %dh

	movl $0, %eax
	movb (VGA_Out_Buffer.cursor_x), %al
	movb %al, %cl

	add $VGA_Out_Buffer.bottom_line, %eax
	movw %dx, (%eax)

	cmp $78, %cl
	jge WriteCharVGA_reset_cursor
	add $2, %cl
	movb %cl, (VGA_Out_Buffer.cursor_x)
	jmp WriteCharVGA_end
	WriteCharVGA_reset_cursor:
	movb $0, (VGA_Out_Buffer.cursor_x)
	#call LineShiftBufferVGA
	WriteCharVGA_end:
	ret

# WriteStrVGA (str: char*) -> void
.globl WriteStrVGA
WriteStrVGA:
	endbr32
	
	call StrLen
	
	push %edi
	push %eax
	xor %eax, %eax
	push %eax
	WriteStrVGA_loop:
	mov (%esp), %eax
	mov 4(%esp), %edx
	cmp %eax, %edx
	jle WriteStrVGA_end_loop

	mov 8(%esp), %edi
	add %eax, %edi
	
	xor %edx, %edx
	movb (%edi), %dl
	call WriteCharVGA

	pop %eax
	inc %eax
	push %eax
	jmp WriteStrVGA_loop
	WriteStrVGA_end_loop:

	add $12, %esp
	ret

# SetColor(color: u8) -> void
.globl SetColor
SetColor:
	endbr32

	movb %dl, (VGA_Out_Buffer.current_color)

	ret

# WriteBufferLine(line: int) -> void
#.globl WriteBufferLine
#WriteBufferLine:
#	endbr32
#	push %edi
#
#	mov $2, %edx
#	mov $80, %eax
#	mul %edx
#	
#	mov (%esp), %edx
#	mul %edx
#	
#	mov $VGA_Out_Buffer.buffer, %edi
#	add %edx, %edi
#
#	mov $0xb8000, %esi
#	add %edx, %esi
#
#	mov $40, %edx
#	call MemCpy
#
#	pop %edi
#	ret

.section .data
VGA_Out_Buffer:
	VGA_Out_Buffer.buffer: .space 2 * 80 * 24
	VGA_Out_Buffer.bottom_line: .space 2 * 80
	VGA_Out_Buffer.current_color: .space 1
	VGA_Out_Buffer.cursor_x: .space 1

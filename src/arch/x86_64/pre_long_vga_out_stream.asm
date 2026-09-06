.code32
.section .pre_long_mode_kernel

# FlushBufferVGA(void) -> void
.globl FlushBufferVGA
FlushBufferVGA:
	endbr32

    call ClearVGA

	mov $VGA_Out_Buffer.image_buffer, %edi
	mov $0xb8000, %esi
	mov $40*25, %edx
	call MemCpy

	mov $VGA_Out_Buffer.buffer, %edi
	mov $0xb8000, %esi
	mov $80*25, %edx
	call MemCpyWNonZeroNonSpaceVGA

	ret

# ClearVGA(void) -> void
.globl ClearVGA
ClearVGA:
    endbr32

    mov $0xb8000, %edi
    mov $0, %esi
    mov $40*25, %edx
    call MemFill

    ret

# ClearBufferVGA(void) -> void
.globl ClearBufferVGA
ClearBufferVGA:
	endbr32

	mov $VGA_Out_Buffer.buffer, %edi
	mov $0, %esi
	mov $40*25, %edx
	call MemFill

	ret

# ClearImageBufferVGA(void) -> void
.globl ClearImageBufferVGA
ClearImageBufferVGA:
	endbr32

	mov $VGA_Out_Buffer.image_buffer, %edi
	mov $0, %esi
	mov $40*25, %edx
	call MemFill

	ret

# ClearBottomLineVGA(void) -> void
.globl ClearBottomLineVGA
ClearBottomLineVGA:
	endbr32

	mov $VGA_Out_Buffer.bottom_line, %edi
	mov $0, %esi
	mov $40, %edx
	call MemFill

	ret

# LineShiftBufferVGA(void) -> void
.globl LineShiftBufferVGA
LineShiftBufferVGA:
	endbr32

	mov $80*2, %edi
	add $VGA_Out_Buffer.buffer, %edi
	mov $VGA_Out_Buffer.buffer, %esi
	mov $40*24, %edx
	call MemCpy

	ret

# WriteCharVGA(c: char) -> void
.globl WriteCharVGA
WriteCharVGA:
	endbr32

	and $0x000000ff, %edx
	cmp $'\n', %dl
	je WriteCharVGA_reset_cursor

	movb (VGA_Out_Buffer.current_color), %dh

	movl $0, %eax
	movb (VGA_Out_Buffer.cursor_x), %al
	movb %al, %cl

	add $VGA_Out_Buffer.bottom_line, %eax
	movw %dx, (%eax)

	cmp $158, %cl
	jae WriteCharVGA_reset_cursor
	add $2, %cl
	movb %cl, (VGA_Out_Buffer.cursor_x)
	jmp WriteCharVGA_end
	WriteCharVGA_reset_cursor:
	movb $0, (VGA_Out_Buffer.cursor_x)
	call LineShiftBufferVGA

	call ClearBottomLineVGA
	WriteCharVGA_end:
	ret

# IntToSingleHex(i: u8) -> char
.globl IntToSingleHex
IntToSingleHex:
	endbr32

	cmp $10, %edi
	jge IntToSingleHex_letters

	add $'0', %edi
	mov %edi, %eax

	jmp IntToSingleHex_end
	IntToSingleHex_letters:
	
	sub $10, %edi
	add $'a', %edi
	mov %edi, %eax

	IntToSingleHex_end:

	ret

# WriteIntAsHexVGA(i: int32) -> char[12]
.globl WriteIntAsHexVGA
WriteIntAsHexVGA:
	endbr32
	sub $12, %esp

	mov %esp, %esi

	push %edi

	movb $'0', (%esi)
	movb $'x', 1(%esi)
	
	and $0xf0000000, %edi
	shr $28, %edi
	call IntToSingleHex

	movb %al, 2(%esi)

	mov (%esp), %edi
	and $0x0f000000, %edi
	shr $24, %edi
	call IntToSingleHex

	movb %al, 3(%esi)

	mov (%esp), %edi
	and $0x00f00000, %edi
	shr $20, %edi
	call IntToSingleHex

	movb %al, 4(%esi)

	mov (%esp), %edi
	and $0x000f0000, %edi
	shr $16, %edi
	call IntToSingleHex

	movb %al, 5(%esi)

	mov (%esp), %edi
	and $0x0000f000, %edi
	shr $12, %edi
	call IntToSingleHex

	movb %al, 6(%esi)

	mov (%esp), %edi
	and $0x00000f00, %edi
	shr $8, %edi
	call IntToSingleHex

	movb %al, 7(%esi)

	mov (%esp), %edi
	and $0x000000f0, %edi
	shr $4, %edi
	call IntToSingleHex

	movb %al, 8(%esi)

	mov (%esp), %edi
	and $0x0000000f, %edi
	call IntToSingleHex

	movb %al, 9(%esi)

	movw $0, 10(%esi)

	mov %esi, %edi
	call WriteStrVGA
	
	pop %edi
	add $12, %esp
	ret

# WriteStrVGA(str: char*) -> void
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
	cmp $0, %dl
	je WriteStrVGA_skip_write
	call WriteCharVGA
	WriteStrVGA_skip_write:

	pop %eax
	inc %eax
	push %eax
	jmp WriteStrVGA_loop
	WriteStrVGA_end_loop:

	add $12, %esp
	ret

# WriteImageCharBufferVGA(x: int, y: int, width: int, char_arr: char*, arr_len: int) -> void
.globl WriteImageCharBufferVGA
WriteImageCharBufferVGA:
	endbr32
	push %eax				# x
	push %edi				# y
	push %esi				# width
	push %edx				# char_arr
	push %ecx				# arr_len
	push $0					# idx

	.equ x, 20
	.equ y, 16
	.equ width, 12
	.equ char_arr, 8
	.equ arr_len, 4
	.equ idx, 0

	WriteImageCharBufferVGA_loop:
	mov idx(%esp), %eax
	mov arr_len(%esp), %edx
	cmp %eax, %edx
	jle WriteImageCharBufferVGA_end_loop

	mov char_arr(%esp), %edx
	mov idx(%esp), %eax
	add %edx, %eax
	xor %ecx, %ecx
	movb (%eax), %cl
	push %ecx

	mov idx+4(%esp), %eax
	xor %edx, %edx
	mov width+4(%esp), %ecx
	div %ecx

	mov x+4(%esp), %edi
	add %edi, %edx				# edx = x in buffer
	push %edx

	mov y+8(%esp), %edi
	add %edi, %eax				# eax = y in buffer
	push %eax
	
	xor %edx, %edx
	mov $160, %ecx
	mul %ecx

	mov $VGA_Out_Buffer.image_buffer, %edi
	add %eax, %edi

	pop %eax
	pop %eax				# eax = x in buffer

	xor %edx, %edx
	mov $2, %ecx
	mul %ecx

	add %eax, %edi

	pop %eax

	movb %al, (%edi)

	pop %eax
	inc %eax
	push %eax
	jmp WriteImageCharBufferVGA_loop
	WriteImageCharBufferVGA_end_loop:

	add $24, %esp
	ret

# WriteImageColorBufferVGA(x: int, y: int, width: int, color_arr: byte*, arr_len: int) -> void
.globl WriteImageColorBufferVGA
WriteImageColorBufferVGA:
	endbr32
    push %eax				# x
    push %edi				# y
    push %esi				# width
    push %edx				# char_arr
    push %ecx				# arr_len
    push $0					# idx

    .equ x, 20
    .equ y, 16
    .equ width, 12
    .equ char_arr, 8
    .equ arr_len, 4
    .equ idx, 0

    WriteImageColorBufferVGA_loop:
    mov idx(%esp), %eax
    mov arr_len(%esp), %edx
    cmp %eax, %edx
    jle WriteImageColorBufferVGA_end_loop

    mov char_arr(%esp), %edx
    mov idx(%esp), %eax
    add %edx, %eax
    xor %ecx, %ecx
    movb (%eax), %cl
    push %ecx

    mov idx+4(%esp), %eax
    xor %edx, %edx
    mov width+4(%esp), %ecx
    div %ecx

    mov x+4(%esp), %edi
    add %edi, %edx				# edx = x in buffer
    push %edx

    mov y+8(%esp), %edi
    add %edi, %eax				# eax = y in buffer
    push %eax

    xor %edx, %edx
    mov $160, %ecx
    mul %ecx

    mov $VGA_Out_Buffer.image_buffer, %edi
    add %eax, %edi

    pop %eax
    pop %eax				# eax = x in buffer

    xor %edx, %edx
    mov $2, %ecx
    mul %ecx

    add %eax, %edi

    pop %eax

    movb %al, 1(%edi)

    pop %eax
    inc %eax
    push %eax
    jmp WriteImageColorBufferVGA_loop
    WriteImageColorBufferVGA_end_loop:

    add $24, %esp
    ret

# MemCpyWNonZeroNonSpaceVGA(arr1: int16*, arr2: int16*, w_len: int32) -> void
.globl MemCpyWNonZeroNonSpaceVGA
MemCpyWNonZeroNonSpaceVGA:
    endbr32
    xor %eax, %eax
    push %edi                       # arr1
    push %esi                       # arr2
    push %edx                       # w_len
    push %eax                       # idx

    MemCpyWNonZeroNonSpaceVGA_loop:
    mov 4(%esp), %edx
    mov (%esp), %eax
    cmp %eax, %edx
    jle MemCpyWNonZeroNonSpaceVGA_loop_end

    mov $2, %edx
    mul %edx

    mov 12(%esp), %edi
    add %eax, %edi

    mov 8(%esp), %esi
    add %eax, %esi

    movw (%edi), %ax

    cmp $0, %ax
    je MemCpyWNonZeroNonSpaceVGA_skip

    cmp $0x0f20, %ax
    je MemCpyWNonZeroNonSpaceVGA_skip

    movw %ax, (%esi)
    MemCpyWNonZeroNonSpaceVGA_skip:

    pop %eax
    inc %eax
    push %eax
    jmp MemCpyWNonZeroNonSpaceVGA_loop
    MemCpyWNonZeroNonSpaceVGA_loop_end:

    pop %eax
    pop %edx
    pop %esi
    pop %edi
    ret

# SetColor(color: u8) -> void
.globl SetColor
SetColor:
	endbr32

	movb %dl, (VGA_Out_Buffer.current_color)

	ret

.section .pre_long_data, "aw"
.globl VGA_Out_Buffer
.globl VGA_Out_Buffer.sizeof
VGA_Out_Buffer:
	VGA_Out_Buffer.buffer: .space 2 * 80 * 24
	VGA_Out_Buffer.bottom_line: .space 2 * 80
	VGA_Out_Buffer.image_buffer: .space 2 * 80 * 25
	VGA_Out_Buffer.current_color: .space 1
	VGA_Out_Buffer.cursor_x: .space 1
VGA_Out_Buffer_end:
.set VGA_Out_Buffer.sizeof, (VGA_Out_Buffer_end - VGA_Out_Buffer)

.section .note.GNU-stack,"",@progbits

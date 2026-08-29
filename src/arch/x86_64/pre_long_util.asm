.code32
.section .pre_long_mode_kernel

# WriteCharCOM1(c: char) -> void
.globl WriteCharCOM1
WriteCharCOM1:
	endbr32
	push %edx

	mov $0x0, %dx
	outb %al, %dx
	
	pop %edx
	ret

# StrLen(s: char*) -> int
.globl StrLen
StrLen:
	endbr32

	xor %eax, %eax

	StrLen_loop:
	mov %edi, %ecx
	add %eax, %ecx
	movb (%ecx), %dl

	cmp $0, %dl
	je StrLen_end_loop

	inc %eax
	jmp StrLen_loop
	StrLen_end_loop:
	
	ret

# MemCpy(arr1: int32*, arr2: int32*, len: int) -> void
.globl MemCpy
MemCpy:
	endbr32
	
	xor %eax, %eax
	push %edi
	push %esi
	push %edx
	push %eax
	MemCpy_loop:
	mov 4(%esp), %edx
	mov (%esp), %eax
	cmp %eax, %edx
	jle MemCpy_loop_end

	mov $4, %edx
	mul %edx

	mov 12(%esp), %edi
	add %eax, %edi

	mov 8(%esp), %esi
	add %eax, %esi

	movl (%edi), %eax
	movl %eax, (%esi)
	
	pop %eax
	inc %eax
	push %eax
	jmp MemCpy_loop
	MemCpy_loop_end:

	pop %eax
	pop %edx
	pop %esi
	pop %edi
	ret

# MemCpyNonZero(arr1: int32*, arr2: int32*, len: int) -> void
.globl MemCpyNonZero
MemCpyNonZero:
	endbr32

	xor %eax, %eax
	push %edi
	push %esi
	push %edx
	push %eax
	MemCpyNonZero_loop:
	mov 4(%esp), %edx
	mov (%esp), %eax
	cmp %eax, %edx
	jle MemCpyNonZero_loop_end

	mov $4, %edx
	mul %edx

	mov 12(%esp), %edi
	add %eax, %edi

	mov 8(%esp), %esi
	add %eax, %esi

	movl (%edi), %eax
	
	cmp $0, %eax
	je MemCpyNonZero_skip

	movl %eax, (%esi)
	MemCpyNonZero_skip:

	pop %eax
	inc %eax
	push %eax
	jmp MemCpyNonZero_loop
	MemCpyNonZero_loop_end:

	pop %eax
	pop %edx
	pop %esi
	pop %edi
	ret

# MemFill(arr: int32*, val: int32, len: int32) -> void
.globl MemFill
MemFill:
	endbr32

	push %edi
	push %esi
	push %edx
	xor %eax, %eax
	push %eax
	MemFill_loop:
	mov (%esp), %eax
	mov 4(%esp), %edx
	cmp %eax, %edx
	jle MemFill_end_loop

	mov $4, %ecx
	mul %ecx

	mov 12(%esp), %edi
	add %eax, %edi

	mov 8(%esp), %esi

	movl %esi, (%edi)

	pop %eax
	inc %eax
	push %eax
	jmp MemFill_loop
	MemFill_end_loop:

	pop %eax
	pop %edx
	pop %esi
	pop %edi
	ret


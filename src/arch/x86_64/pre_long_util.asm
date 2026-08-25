.code32
.section .pre_long_mode_kernel

# PrintCharVGA(x: int, y: int, c: char, color: u8) -> void
.globl PrintCharVGA
PrintCharVGA:
	endbr64
	push %ebx

	cmp $0, %edi
	jl PrintCharVGA_end			# if x < 0: jmp PrintCharVGA_end

	cmp $79, %edi
	jg PrintCharVGA_end			# if x > 99: jmp PrintCharVGA_end

	cmp $0, %esi
	jl PrintCharVGA_end			# if y < 0: jmp PrintCharVGA_end

	cmp $24, %esi
	jg PrintCharVGA_end			# if y > 39: jmp PrintCharVGA_end

	shl $8, %cx
	#add %ecx, %edx
	add %dx, %cx
	mov %cx, %bx				# ebx = (ecx << 8) + edx

	mov $80, %eax				
	mul %esi				# eax = y * 80
	add %edi, %eax				# eax = x + eax
	mov $2, %esi
	mul %esi				# eax = 2 * eax

	add $0xb8000, %eax

	movw %bx, (%eax)			# 0xb8000[eax] = low-half ebx
	
	PrintCharVGA_end:
	pop %ebx
	ret

# PrintStrVGA(x: int, y: int, str: char*, color: u8) -> void
.globl PrintStrVGA
PrintStrVGA:
	endbr64
	push %ebx
	push %ecx
	
	mov $0, %eax				# idx = 0

	PrintStrVGA_loop:
	push %eax
	add %edx, %eax
	movb (%eax), %bl			# ebx = edx[idx(= eax)]
	pop %eax				# eax = idx

	cmp $0, %bl
	je PrintStrVGA_end_loop			# if edx[idx] == 0: jmp PrintStrVGA_end_loop

	push %eax
	push %edi
	push %esi
	push %edx
	push %ecx
	add %eax, %edi
	mov %bl, %dl
	call PrintCharVGA
	pop %ecx
	pop %edx
	pop %esi
	pop %edi
	pop %eax

	inc %eax
	jmp PrintStrVGA_loop

	PrintStrVGA_end_loop:
	pop %ecx
	pop %ebx
	ret

# ClearVGA(void) -> void
.globl ClearVGA
ClearVGA:
	endbr64
	
	mov $0xb8000, %eax
	
	ClearVGA_loop:
	cmp $0xb8fa0, %eax
	jge ClearVGA_end_loop
	movl $0x00000000, (%eax)
	add $4, %eax
	jmp ClearVGA_loop
	ClearVGA_end_loop:

	ret

# WriteCharCOM1(c: char) -> void
.globl WriteCharCOM1
WriteCharCOM1:
	endbr64
	push %edx

	mov $0x0, %dx
	outb %al, %dx
	
	pop %edx
	ret

# StrLen(s: char*) -> int
.globl StrLen
StrLen:
	endbr64
	ret

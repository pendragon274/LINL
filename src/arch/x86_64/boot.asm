# Reference for System V ABI:
# Parameters (in order): rdi, rsi, rdx, rcx, r8, r9, stack, return stored in rax
# Preserved registers: rbx, rsp, rbp, r12, r13, r14, r15
# Non-preserved registers: rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11

.code32
.section .pre_long_mode_kernel
.globl InitLongMode
InitLongMode:
	mov $stack_top, %esp			# Initialize stack to pre-long stack space.

	push %eax
	call WriteImage
	pop %eax

	call CheckMultiboot
	call CheckCPUID
	call CheckLongMode

	hlt

WriteImage:
    endbr32

    mov $15, %eax
    mov $8, %edi
    mov $50, %esi
    mov $linl_image_color_arr, %edx
    mov $linl_image_color_arr.sizeof, %ecx
    call WriteImageColorBufferVGA

    mov $15, %eax
    mov $8, %edi
    mov $50, %esi
    mov $linl_image_char_arr, %edx
    mov $linl_image_char_arr.sizeof, %ecx
    call WriteImageCharBufferVGA

    call FlushBufferVGA

    ret

CheckMultiboot:
    endbr32

	push %eax

	mov $0x0f, %edx
	call SetColor

	mov $checking_multiboot_txt, %edi
	call WriteStrVGA

	call FlushBufferVGA

	pop %eax
	cmp $0x36d76289, %eax

	jne DumpAndFail
	call EmitPass
	ret

CheckCPUID:
    endbr32

	mov $0x0f, %edx
	call SetColor

	mov $checking_cpuid_txt, %edi
	call WriteStrVGA

	pushfl
	pop %eax
	mov %eax, %ecx
	xor $(1<<21), %eax
	
	push %eax
	popfl

	pushfl
	pop %eax

	push %ecx
	popfl

	cmp %eax, %ecx
	je DumpAndFail

	call EmitPass
	ret

CheckLongMode:
    endbr32

	mov $0x0f, %edx
	call SetColor

	mov $checking_long_mode_txt, %edi
	call WriteStrVGA

	call FlushBufferVGA

	movl $0x80000000, %eax
	cpuid

	cmpl $0x80000001, %eax
	jb DumpAndFail
	
	movl $0x80000001, %eax
	cpuid

	test $(1<<29), %edx
	jz DumpAndFail

	call EmitPass
	ret

EmitPass:
	endbr32

	mov $'[', %edx
	call WriteCharVGA

	mov $0x2f, %edx
	call SetColor

	mov $pass_txt, %edi
	call WriteStrVGA

	mov $0x0f, %edx
	call SetColor

	mov $']', %edx
	call WriteCharVGA

	call FlushBufferVGA

	ret

FailOut:
	endbr32

	mov $'[', %edx
	call WriteCharVGA

	mov $0x40, %edx
	call SetColor

	mov $fail_txt, %edi
	call WriteStrVGA

	mov $0x0f, %edx
	call SetColor

	mov $']', %edx
	call WriteCharVGA

	mov $'\n', %edx
	call WriteCharVGA

	call FlushBufferVGA

	ret

DumpAndFail:
    endbr32

    call PreserveCore
    push %eax
    call FailOut
    pop %edi
    jmp DumpCore

.section .rodata
checking_multiboot_txt:
	.asciz "Checking multiboot...                                                     "
checking_cpuid_txt:
	.asciz "Checking CPUID...                                                         "
checking_long_mode_txt:
	.asciz "Checking long mode...                                                     "
pass_txt:
	.asciz "PASS"
fail_txt:
	.asciz "FAIL"
linl_image_char_arr:
    .equ c, 0x1e
    .equ z, 0x00
    .byte z, z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z
    .byte z, c, c, z, z, z, z, z,   c, c, c, c, c, c, z,   c, c, z, z, c, c, z,   c, c, z, z, z, z, z,   z, z, z, z, z, z, z,   z, c, c, c, c, z, z,   z, c, c, c, c, z, z
    .byte z, c, c, z, z, z, z, z,   z, z, c, c, z, z, z,   c, c, c, z, c, c, z,   c, c, z, z, z, z, z,   z, z, z, z, z, z, z,   c, c, z, z, c, c, z,   c, c, z, z, z, c, z
    .byte z, c, c, z, z, z, z, z,   z, z, c, c, z, z, z,   c, c, c, c, c, c, z,   c, c, z, z, z, z, z,   z, z, z, z, z, z, z,   c, c, z, z, c, c, z,   z, c, c, c, z, z, z
    .byte z, c, c, z, z, z, z, z,   z, z, c, c, z, z, z,   c, c, c, c, c, c, z,   c, c, z, z, z, z, z,   z, z, z, z, z, z, z,   c, c, z, z, c, c, z,   z, z, c, c, c, z, z
    .byte z, c, c, z, z, z, z, z,   z, z, c, c, z, z, z,   c, c, z, c, c, c, z,   c, c, z, z, z, z, z,   z, z, z, z, z, z, z,   c, c, z, z, c, c, z,   c, z, z, z, c, c, z
    .byte z, c, c, c, c, c, c, z,   c, c, c, c, c, c, z,   c, c, z, z, c, c, z,   c, c, c, c, c, c, z,   z, z, z, z, z, z, z,   z, c, c, c, c, z, z,   z, c, c, c, c, z, z
    .byte z, z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z,   z, z, z, z, z, z, z
linl_image_char_arr_end:
.set linl_image_char_arr.sizeof, linl_image_char_arr_end - linl_image_char_arr
linl_image_color_arr:
    .equ g, 0x3b
    .equ b, 0xfb
    .equ r, 0x70
    .byte r, r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r
	.byte r, g, g, b, b, b, b, b,   g, g, g, g, g, g, b,   g, g, b, b, g, g, b,   g, g, b, b, b, b, b,   b, b, b, b, b, b, b,   b, g, g, g, g, b, b,   b, g, g, g, g, b, r
	.byte r, g, g, b, b, b, b, b,   b, b, g, g, b, b, b,   g, g, g, b, g, g, b,   g, g, b, b, b, b, b,   b, b, b, b, b, b, b,   g, g, b, b, g, g, b,   g, g, b, b, b, g, r
	.byte r, g, g, b, b, b, b, b,   b, b, g, g, b, b, b,   g, g, g, g, g, g, b,   g, g, b, b, b, b, b,   b, b, b, b, b, b, b,   g, g, b, b, g, g, b,   b, g, g, g, b, b, r
	.byte r, g, g, b, b, b, b, b,   b, b, g, g, b, b, b,   g, g, g, g, g, g, b,   g, g, b, b, b, b, b,   b, b, b, b, b, b, b,   g, g, b, b, g, g, b,   b, b, g, g, g, b, r
	.byte r, g, g, b, b, b, b, b,   b, b, g, g, b, b, b,   g, g, b, g, g, g, b,   g, g, b, b, b, b, b,   b, b, b, b, b, b, b,   g, g, b, b, g, g, b,   g, b, b, b, g, g, r
	.byte r, g, g, g, g, g, g, b,   g, g, g, g, g, g, b,   g, g, b, b, g, g, b,   g, g, g, g, g, g, b,   b, b, b, b, b, b, b,   b, g, g, g, g, b, b,   b, g, g, g, g, b, r
	.byte r, r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r,   r, r, r, r, r, r, r
linl_image_color_arr_end:
.set linl_image_color_arr.sizeof, linl_image_color_arr_end - linl_image_color_arr

.section .bss
	.lcomm stack_bottom, 4096
	stack_top:

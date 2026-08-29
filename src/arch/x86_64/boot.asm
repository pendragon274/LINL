# Reference for System V ABI:
# Parameters (in order): rdi, rsi, rdx, rcx, r8, r9, stack, return stored in rax
# Preserved registers: rbx, rsp, rbp, r12, r13, r14, r15
# Non-preserved registers: rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11

.code32
.section .pre_long_mode_kernel
.globl InitLongMode
InitLongMode:
	mov $stack_top, %esp			# Initialize stack to pre-long stack space.

	call CheckMultiboot
	call CheckCPUID
	call CheckLongMode

	mov $30, %eax
	mov $8, %edi
	mov $20, %esi
	mov $linl_image_color_arr, %edx
	mov $linl_image_color_arr.sizeof, %ecx
	call WriteImageColorBufferVGA

	mov $0, %eax
	mov $0, %edi
	mov $20, %esi
	mov $linl_image_char_arr, %edx
	mov $linl_image_char_arr.sizeof, %ecx
	call WriteImageCharBufferVGA

	call FlushBufferVGA

	hlt

CheckMultiboot:
	push %eax

	mov $0x0f, %edx
	call SetColor

	mov $checking_multiboot_txt, %edi
	call WriteStrVGA

	call FlushBufferVGA

	pop %eax
	cmp $0x36d76289, %eax

	jne CheckMultiboot_fail
	call EmitPass
	ret
	CheckMultiboot_fail:
	call FailOut

CheckCPUID:
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
	jne CheckCPUID_success
	call FailOut
	CheckCPUID_success:
	call EmitPass
	ret

CheckLongMode:
	mov $0x0f, %edx
	call SetColor

	mov $checking_long_mode_txt, %edi
	call WriteStrVGA

	call FlushBufferVGA

	movl $0x80000000, %eax
	cpuid

	cmpl $0x80000001, %eax
	jb CheckLongMode_fail
	
	movl $0x80000001, %eax
	cpuid

	test $(1<<29), %edx
	jz CheckLongMode_fail

	call EmitPass
	ret

	CheckLongMode_fail:
	call FailOut

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

	#mov $'\n', %edx
	#call WriteCharVGA

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

	call FlushBufferVGA

	hlt

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
	.ascii "LINL OS"
linl_image_char_arr_end:
.set linl_image_char_arr.sizeof, linl_image_char_arr_end - linl_image_char_arr
linl_image_color_arr:
	.byte 0x2f, 0x2f, 0x2f, 0x2f, 0x2f, 0x2f, 0x2f
linl_image_color_arr_end:
.set linl_image_color_arr.sizeof, linl_image_color_arr_end - linl_image_color_arr

.section .bss
	.lcomm stack_bottom, 4096
	stack_top:

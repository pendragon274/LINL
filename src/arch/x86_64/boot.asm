# Reference for System V ABI:
# Parameters (in order): rdi, rsi, rdx, rcx, r8, r9, stack, return stored in rax
# Preserved registers: rbx, rsp, rbp, r12, r13, r14, r15
# Non-preserved registers: rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11

.code32
.section .pre_long_mode_kernel
.globl InitLongMode
InitLongMode:
	mov $stack_top, %esp			# Initialize stack to pre-long stack space.
	
	push %eax				# Save %eax as it is relevant for check_multiboot

	call ClearVGA

	pop %eax
	call check_multiboot
	call check_cpuid
	call check_long_mode

	mov $73, %edi
	mov $0, %esi
	mov $linl_os_txt, %edx
	mov $0x0f, %ecx
	call PrintStrVGA

	mov $0x2f, %edx
	call SetColor

	mov $test_txt, %edi
	call WriteStrVGA

	mov $'a', %edx
	call WriteCharVGA

	mov $'b', %edx
	call WriteCharVGA

	call FlushBufferVGA

	mov $0, %edi
	mov $0, %esi
	mov $ok_txt, %edx
	mov $0x2f, %ecx
	call PrintStrVGA

	hlt

check_cpuid:
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
	jne cpuid_success
	movb $'1', %al
	cpuid_success:
	ret

check_multiboot:
	cmp $0x36d76289, %eax

	jne multiboot_fail
	ret
	multiboot_fail:
	mov $'0', %al
	jmp error

check_long_mode:
	movl $0x80000000, %eax
	cpuid

	cmpl $0x80000001, %eax
	jb long_mode_fail
	
	movl $0x80000001, %eax
	cpuid

	test $(1<<29), %edx
	jz long_mode_fail
	ret

	long_mode_fail:
	mov $'2', %al
	jmp error

error:
	mov $37, %edi
	mov $12, %esi
	mov $err, %edx
	mov $0xc1, %ecx
	call PrintStrVGA
	hlt

.section .rodata
err:
	.asciz "ERROR"
linl_os_txt:
	.asciz "LINL OS"
test_txt:
	.asciz "This is a test right here."
ok_txt:
	.asciz "Ok"

.section .bss
	.lcomm stack_bottom, 4096
	stack_top:

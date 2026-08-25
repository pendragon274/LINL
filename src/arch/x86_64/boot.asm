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

	mov $0, %edi
	mov $22, %esi
	mov $msg, %edx
	mov $0x2f, %ecx
	call PrintStrVGA

	mov $0, %edi
	mov $0, %esi
	mov $'O', %edx
	mov $0x2f, %ecx
	call PrintCharVGA

	mov $1, %edi
	mov $0, %esi
	mov $'k', %edx
	mov $0x2f, %ecx
	call PrintCharVGA

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
	movl $0x4f524f45, (0xb8000)
	movl $0x4f3a4f52, (0xb8004)
	movl $0x4f204f20, (0xb8008)
	movb %al, (0xb800a)
	hlt

.section .rodata
msg:
	.asciz "This is a message."

.section .bss
	.lcomm stack_bottom, 4096
	stack_top:

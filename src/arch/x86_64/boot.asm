.code32
.section .pre_long_mode_kernel
.globl InitLongMode
InitLongMode:
	#movl $0x2f4b2f4f, (0xb8000)
	mov $stack_top, %esp

	call check_multiboot
	call check_cpuid
	call check_long_mode

	#Print OK
	movl $0x2f4b2f4f, (0xb8000)

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

.section .bss
stack_bottom:
	.lcomm buffer, 64
stack_top:

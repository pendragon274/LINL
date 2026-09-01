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
	call InitPageTables
	call TestIdentityMapping
	call EnablePaging
	call LoadGDT

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

InitPageTables:
    endbr32

    mov $0x0f, %edx
    call SetColor

    mov $init_paging_txt, %edi
    call WriteStrVGA

    call FlushBufferVGA

    mov $p3_table, %eax
    or $0b11, %eax
    mov %eax, (p4_table)

    mov $p2_table, %eax
    or $0b11, %eax
    mov %eax, (p3_table)

    xor %ecx, %ecx
    InitPageTables_loop:
    cmp $512, %ecx
    jge InitPageTables_loop_end

    mov $0x200000, %eax
    xor %edx, %edx
    mul %ecx

    or $0b10000011, %eax

    push %eax

    mov $8, %eax
    mul %ecx

    mov %eax, %edi

    pop %eax
    mov %eax, p2_table(%edi)

    inc %ecx
    jmp InitPageTables_loop
    InitPageTables_loop_end:

    mov $p4_table, %eax
    mov %eax, %cr3

    call EmitPass
    ret

TestIdentityMapping:
    endbr32

    mov $0x0f, %edx
    call SetColor

    mov $test_identity_map_txt, %edi
    call WriteStrVGA

    /*mov $'\n', %edx
    call WriteCharVGA

    mov $TestIdentityMapping, %ebx
    mov %ebx, %edi
    call WriteIntAsHexVGA

    mov $'|', %edi
    call WriteCharVGA

    mov %ebx, %edi
    call VirtualToPhysical

    mov %eax, %edi
    call WriteIntAsHexVGA

    mov $'\n', %edx
    call WriteCharVGA*/
    # Should put a real test in here. Skipping for now.

    call FlushBufferVGA

    call EmitPass
    ret

EnablePaging:
    endbr32

    mov $0x0f, %edx
    call SetColor

    mov $enable_paging_txt, %edi
    call WriteStrVGA

    call FlushBufferVGA

    mov %cr4, %eax
    or $(1<<5), %eax
    mov %eax, %cr4

    mov $0xc0000080, %ecx
    rdmsr
    or $(1<<8), %eax
    wrmsr

    mov %cr0, %eax
    or $(1<<31), %eax
    mov %eax, %cr0

    call EmitPass
    ret

LoadGDT:
    endbr32



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

EmitFail:
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
    call EmitFail
    pop %edi
    jmp DumpCore

.section .rodata
checking_multiboot_txt:
	.asciz "Checking multiboot...                                                     "
checking_cpuid_txt:
	.asciz "Checking CPUID...                                                         "
checking_long_mode_txt:
	.asciz "Checking long mode...                                                     "
init_paging_txt:
    .asciz "Initializing paging...                                                    "
test_identity_map_txt:
    .asciz "Testing identity map...                                                   "
enable_paging_txt:
    .asciz "Enabling paging...                                                        "
load_gdt_txt:
    .asciz "Loading the GDT...                                                        "
pass_txt:
	.asciz "PASS"
fail_txt:
	.asciz "FAIL"
linl_image_char_arr: .set linl_image_char_arr.sizeof, linl_image_char_arr_end - linl_image_char_arr
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
linl_image_color_arr: .set linl_image_color_arr.sizeof, linl_image_color_arr_end - linl_image_color_arr
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

.section .bss
    .lcomm p4_table, 4096
    .lcomm p3_table, 4096
    .lcomm p2_table, 4096
	.lcomm stack_bottom, 4096
	stack_top:

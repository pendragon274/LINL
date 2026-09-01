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

# VirtualToPhysical(addr: i32) -> i32
.globl VirtualToPhysical
VirtualToPhysical:
    endbr32
    push %edi
    push %eax

    mov %cr3, %eax                      # eax = p4_table*

    and $0xfffff000, %eax               # This table must be aligned to 4096 and the last 12 bits contain other information, so this is the real p4_table*.

    cmp $0, %eax
    jz VirtualToPhysical_end            # If p4_table == null, then end and return 0.

    mov %eax, (%esp)                    # 0(%esp) = p4_table

    # Something should go here in long mode for multiple p3_tables. In 32 bit mode, there can only be this one p3 table.

    mov (%eax), %eax                    # %eax = p4_table entry 0, which is the (or a) p3_table.

    and $0xfffff000, %eax               # Table must be aligned to 4096, with these bits not relevant to the address.

    mov %eax, (%esp)                    # 0(esp) = p3_table*

    cmp $0, %eax
    jz VirtualToPhysical_end

    mov 4(%esp), %edi                   # edi = addr

    and $0xc0000000, %edi               # These two bits should provide the index of the p3 table.
    shr $30, %edi                       # edi = p3 table index.

    mov $8, %eax
    mul %edi

    mov (%esp), %edi
    add %edi, %eax                      # eax = address of the nth index of the p3 table.

    mov (%eax), %eax                    # eax = p2 table* for this addr (with the last 12 bits other information).

    and $0xfffff000, %eax               # Hack off the last 12 bits of eax to provide the p2 table address.

    mov %eax, (%esp)                    # 0(esp) = p2_table*

    cmp $0, %eax
    jz VirtualToPhysical_end            # If p2_table* == null

    mov 4(%esp), %edi                   # edi = addr

    and $0x3fe00000, %edi               # Pull the p2 index bits out of the addr.
    shr $21, %edi

    mov $8, %eax
    mul %edi

    mov (%esp), %edi
    add %edi, %eax

    pop %edi
    pop %edi
    ret

    VirtualToPhysical_end:
    pop %eax
    pop %edi
    mov $0, %eax
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

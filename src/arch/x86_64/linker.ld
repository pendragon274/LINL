ENTRY(_start)

KERN_PHYSICAL = 0x00400000;
KERN_VIRTUAL = 0xC0000000;
KERN_SIZE = 0x10000000;

MEMORY
{
    pre_long_mode (ax) : ORIGIN = 0x00200000, LENGTH = 0x00200000
    kernel_physical (ax) : ORIGIN = KERN_PHYSICAL, LENGTH = KERN_SIZE
    kernel_virtual (ax) : ORIGIN = KERN_VIRTUAL, LENGTH = KERN_SIZE
}

PHDRS
{
    headers PT_PHDR FILEHDR PHDRS;
    boot PT_LOAD FILEHDR PHDRS FLAGS(5);
    pre_long_rodata PT_LOAD FLAGS(4);
    pre_long_data PT_LOAD FLAGS(6);
    pre_long_bss PT_LOAD FLAGS(6);
    kernel_text PT_LOAD FLAGS(5);
    kernel_data PT_LOAD FLAGS(6);
    kernel_rodata PT_LOAD FLAGS(4);
    kernel_rust_internals PT_LOAD FLAGS(4);
    kernel_throwaway PT_LOAD FLAGS(4);
    kernel_remainder PT_LOAD FLAGS(5);
    kernel_stack_lid PT_LOAD FLAGS(4);
    kernel_stack PT_LOAD FLAGS(6);
}

PRE_KERN_START = 2M;
BOOT_BLOCK_SIZE = 2M;
KERN_START = 4M;

SECTIONS{
	. = PRE_KERN_START;

	.boot BLOCK(4K) : ALIGN(4K)
	{
		KEEP(*(.multiboot_segment))
		*(.pre_long_mode_kernel)
	} >pre_long_mode :boot

	.pre_long_rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.pre_long_rodata)
	} >pre_long_mode :pre_long_rodata

	.pre_long_data BLOCK(8K) : ALIGN(4K)
	{
		*(.pre_long_data)
	} >pre_long_mode :pre_long_data

	.pre_long_bss BLOCK(12K) : ALIGN(4K)
	{
		*(.pre_long_bss)
	} >pre_long_mode :pre_long_bss

	. = KERN_START;

	.kernel_text : ALIGN(4K)
    {
        *(.kernel.text)
        *(.text)
        *(.text.*)
        *(.ltext*)
    } > kernel_virtual AT> kernel_physical :kernel_text

    .kernel_data : ALIGN(4K)
    {
        *(.data)
        *(.data.*)
        *(.ldata.*)
    } > kernel_virtual AT> kernel_physical :kernel_data

    .kernel_rodata : ALIGN(4K)
    {
        *(.rodata)
        *(.rodata.*)
        *(.lrodata.*)
    } > kernel_virtual AT> kernel_physical :kernel_rodata

    .kernel_remainder : ALIGN(4k)
    {
        *(.got)
        *(.got.plt)
        *(.eh_frame)
        *(.comment)
    } > kernel_virtual AT> kernel_physical :kernel_remainder

    .kernel_stack_lid BLOCK(4K) : ALIGN(4K)
    {
        LONG(0)
    } > kernel_virtual AT> kernel_physical :kernel_stack_lid

    .kernel_stack : ALIGN(4K)
    {
        KERNEL_STACK_BOTTOM = .;
        FILL(0);
        /*. += 256M;*/
        /*. += (KERN_SIZE - (KERNEL_STACK_TOP + KERN_START));*/
        . = ALIGN(KERN_SIZE);
        KERNEL_STACK_TOP = .;
    } > kernel_virtual AT> kernel_physical :kernel_stack
}

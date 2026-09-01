ENTRY(_start)

PHDRS
{
    boot PT_LOAD FLAGS(5);
    kernel PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
    bss PT_LOAD FLAGS(6);
}

SECTIONS{
	. = 2M;

	.boot BLOCK(4K) : ALIGN(4K)
	{
		KEEP(*(.multiboot_segment))
		*(.text)
		*(.pre_long_mode_kernel)
	} :boot

	.kernel BLOCK(4K) : ALIGN(4K)
	{
		*(.kernel)
	} :kernel

	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
	} :rodata

	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
	} :data

	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(.bss)
	} :bss
}

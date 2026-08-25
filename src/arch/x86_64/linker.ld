ENTRY(_start)

SECTIONS{
	. = 2M;

	.boot BLOCK(4K) : ALIGN(4K)
	{
		KEEP(*(.multiboot_segment))
		*(.text)
		*(.pre_long_mode_kernel)
	}

	.kernel BLOCK(4K) : ALIGN(4K)
	{
		*(.kernel)
	}

	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
	}

	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
	}

	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(.bss)
	}
}

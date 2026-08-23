ENTRY(_start)

SECTIONS{
	. = 2M;

	.boot :
	{
		KEEP(*(.multiboot_segment))
		*(.text)
		*(.pre_long_mode_kernel)
	}

	.kernel :
	{
		*(.kernel)
	}

	.rodata :
	{
		*(.rodata)
	}

	.data :
	{
		*(.data)
	}

	.bss :
	{
		*(.bss)
	}
}

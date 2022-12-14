ENTRY(_start)

MEMORY{
	RAM (rwx): ORIGIN = 0x80000000, LENGTH = 1M
}

SECTIONS{
	.text :
	{
	    . = ALIGN(8);
	    PROVIDE(__TEXT_START = .);
		*(.text.init)
		*(.text*)
	    PROVIDE(__TEXT_END = .);
	} > RAM
	.rodata : {
		. = ALIGN(8);
	    PROVIDE(__DATA_START = .);
		*(.rodata*)
	    PROVIDE(__DATA_END = .);
	} > RAM
	.data : {
		*(.data .data*)
		*(.sdata .sdata*)
	} > RAM
	.bss : {
		. = ALIGN(8);
		PROVIDE(__BSS_START = .);
		*(.bss .bss*)
		*(.sbss .sbss*)
		PROVIDE(__BSS_END = .);
	} > RAM

	PROVIDE(__UART_START = 0x10013000);
	PROVIDE(__GPIO_START = 0x10012000);
	PROVIDE(__RAM_START = 0x80000000);
	PROVIDE(__RAM_PROGRAM_START = 0x80000000 + 1M);
}
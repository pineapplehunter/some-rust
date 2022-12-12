ENTRY(_start)

MEMORY{
	ROM (rx) : ORIGIN = 0x20000000, LENGTH = 1M
	RAM (rwx): ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS{
	.text :
	{
	    . = ALIGN(8);
	    PROVIDE(__TEXT_START = .);
		*(.text.init)
		*(.text*)
	    PROVIDE(__TEXT_END = .);
	} > ROM
	.rodata : {
		. = ALIGN(8);
	    PROVIDE(__DATA_START = .);
		*(.rodata*)
	    PROVIDE(__DATA_END = .);
	} > ROM
	.bss : {
		. = ALIGN(8);
		PROVIDE(__BSS_START = .);
	    *(.data)
		*(.data*)
		*(.bss .bss*)
		*(.sbss .sbss*)
		PROVIDE(__BSS_END = .);
	} > RAM;

	PROVIDE(__UART_START = 0x10013000);
	PROVIDE(__GPIO_START = 0x10012000);
	PROVIDE(__RAM_START = 0x80000000);
	ram_main = 0x80000000;
}
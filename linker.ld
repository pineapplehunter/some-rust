ENTRY(_start)

MEMORY{
	RAM (rwx): ORIGIN = 0x80000000 + 1M, LENGTH = 256M
}

SECTIONS{
	.text ALIGN(CONSTANT(MAXPAGESIZE)) :
	{
        PROVIDE(__TEXT_START = .);
		*(.text.init)
		*(.text .text*)
        PROVIDE(__TEXT_END = .);
	} > RAM
	.rodata ALIGN(CONSTANT(MAXPAGESIZE)) : {
        PROVIDE(__DATA_START = .);
		*(.rodata*)
        PROVIDE(__DATA_END = .);
	} > RAM
	.data ALIGN(CONSTANT(MAXPAGESIZE)) : {
		*(.data .data*)
		*(.sdata .sdata*)
	} > RAM
	.bss ALIGN(CONSTANT(MAXPAGESIZE)) : {
		PROVIDE(__BSS_START = .);
		*(.bss .bss*)
		*(.sbss .sbss*)
		PROVIDE(__BSS_END = .);
	} > RAM

	. = ALIGN(CONSTANT(MAXPAGESIZE));
	PROVIDE(__PROGRAM_END = .);
	PROVIDE(__HEAP_END = . + 128k);

	/* PROVIDE(__UART_START = 0x10013000); */
	PROVIDE(__UART_START = 0x10010000);
	PROVIDE(__GPIO_START = 0x10012000);
	PROVIDE(__RAM_START = 0x80000000);
	PROVIDE(__RAM_PROGRAM_START = 0x80000000 + 1M);
}

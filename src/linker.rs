use crate::uart::UARTInner;

#[allow(dead_code)]
extern "C" {
    #[link_name = "__UART_START"]
    pub static mut UART: UARTInner;
    #[link_name = "__GPIO_START"]
    pub static GPIO: u32;
    #[link_name = "__RAM_START"]
    pub static RAM: [u8; 256 * 1024 * 1024];
    #[link_name = "__RAM_PROGRAM_START"]
    pub static RAM_PROGRAM_START: u8;
    #[link_name = "__PROGRAM_END"]
    pub static PROGRAM_END: u8;
    #[link_name = "__BSS_START"]
    pub static BSS_START: u64;
    #[link_name = "__BSS_END"]
    pub static BSS_END: u64;
}

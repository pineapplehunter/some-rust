#![no_std]
#![no_main]
#![feature(linkage)]

mod delay;
mod linker;
mod uart;

use core::panic::PanicInfo;
use core::{arch::global_asm, fmt::Write};
use delay::delay;
use linker::{__RAM_START, __UART_START};
use uart::UART0;
use volatile::Volatile;

static mut LOAD_DONE: bool = false;

#[no_mangle]
pub extern "C" fn loader_main(thread_id: usize) {
    let mut load_done_val = unsafe { Volatile::new(&mut LOAD_DONE) };
    load_done_val.write(false);
    if thread_id == 0 {
        println!("Hello!");
        println!("I am B4Processor!");
        println!("UART = {:16X}", unsafe { __UART_START });
        println!("RAM  = {:16X}", unsafe { __RAM_START });
        load_done_val.write(true);
    } else {
        while !load_done_val.read() {
            delay(100);
        }
    }
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    unsafe {
        if let Some(uart) = &mut UART0 {
            writeln!(uart, "{:?}", info).ok();
        }
    }
    loop {}
}

global_asm!(include_str!("boot.s"));

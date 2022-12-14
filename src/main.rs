#![no_std]
#![no_main]

mod delay;
mod linker;
mod uart;

use core::fmt::Write;
use core::panic::PanicInfo;
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
    writeln!(UART0.lock(), "{:?}", info).ok();
    loop {}
}

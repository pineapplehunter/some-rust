#![no_std]
#![no_main]

use core::{
    cell::UnsafeCell,
    fmt::{self, Write},
    panic::PanicInfo,
};

use volatile::Volatile;

#[repr(C)]
struct UART {
    rx: u32,
    tx: u32,
    stat: u32,
    control: u32,
}

extern "C" {
    static mut __UART_START: UnsafeCell<UART>;
}

impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut uart = Volatile::new(self);
        for c in s.chars() {
            while uart.map(|v| &v.stat).read() & 0b1000 != 0 {
                delay(100);
            }
            let c = if c.is_ascii() { c as u8 } else { b'.' };
            uart.map_mut(|v| &mut v.tx).write(c as u32)
        }
        Ok(())
    }
}

fn delay(mut amount: usize) {
    while amount > 0 {
        amount -= 1;
    }
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        let uart = __UART_START.get_mut();
        writeln!(uart, "Hello World!").unwrap();
    }
}

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {}
}

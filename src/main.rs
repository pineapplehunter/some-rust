#![no_std]
#![no_main]

use core::{
    fmt::{self, Write},
    mem::size_of,
    panic::PanicInfo,
};
use spin::Mutex;

use lazy_static::lazy_static;
use volatile::Volatile;

extern "C" {
    static __UART_START: usize;
    static __RAM_START: usize;
}

#[repr(C)]
struct UARTInner {
    rx: u8,
    _rx_pad: [u8; 3],
    tx: u8,
    _tx_pad: [u8; 3],
    stat: u8,
    _stat_pad: [u8; 3],
    control: u8,
    _control_pad: [u8; 3],
}

lazy_static! {
    static ref UART0: Mutex<UART> = Mutex::new(UART(unsafe { __UART_START }));
}

struct UART(usize);

impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        _print(self, s)
    }
}

#[no_mangle]
#[inline(never)]
fn _print(uart: &mut UART, s: &str) -> fmt::Result {
    let mut uart = Volatile::new(unsafe { &mut *(uart.0 as *mut UARTInner) });
    for c in s.chars() {
        while uart.map(|v| &v.stat).read() & 0b1000 != 0 {
            delay(100);
        }
        let c = if c.is_ascii() { c as u8 } else { b'.' };
        uart.map_mut(|v| &mut v.tx).write(c);
    }
    Ok(())
}

fn delay(mut amount: usize) {
    while amount > 0 {
        amount -= 1;
    }
}

static_assertions::const_assert!(size_of::<UARTInner>() == 16);

static mut LOAD_DONE: bool = false;

#[no_mangle]
pub extern "C" fn loader_main(thread_id: usize) {
    let mut load_done_val = unsafe { Volatile::new(&mut LOAD_DONE) };
    load_done_val.write(false);
    if thread_id == 0 {
        writeln!(UART0.lock(), "Hello!").unwrap();
        writeln!(UART0.lock(), "I am B4Processor!").unwrap();
        writeln!(UART0.lock(), "UART = {:16X}", unsafe { __UART_START }).unwrap();
        writeln!(UART0.lock(), "RAM  = {:16X}", unsafe { __RAM_START }).unwrap();
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

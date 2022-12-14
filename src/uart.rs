use core::fmt::{self, Write};
use core::mem::size_of;
use spin::Mutex;

use crate::delay::delay;
use crate::linker::__UART_START;

use lazy_static::lazy_static;
use volatile::Volatile;

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

static_assertions::const_assert!(size_of::<UARTInner>() == 16);

lazy_static! {
    pub static ref UART0: Mutex<UART> = Mutex::new(UART(unsafe { __UART_START }));
}

pub struct UART(usize);

impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut uart = Volatile::new(unsafe { &mut *(self.0 as *mut UARTInner) });
        for c in s.chars() {
            while uart.map(|v| &v.stat).read() & 0b1000 != 0 {
                delay(100);
            }
            let c = if c.is_ascii() { c as u8 } else { b'.' };
            uart.map_mut(|v| &mut v.tx).write(c);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    UART0.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

use core::fmt::{self, Write};
use core::mem::size_of;

use crate::delay::delay;
use crate::linker::__UART_START;

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

pub static mut UART0: Option<UART> = None;

pub struct UART(usize);

impl UART {
    pub const fn initialize(address: usize) -> Self {
        Self(address)
    }
}

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

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
}

impl Read for UART {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        let mut uart = Volatile::new(unsafe { &mut *(self.0 as *mut UARTInner) });
        let mut i = 0;
        'outer: loop {
            let mut break_count = 3;
            while uart.map(|v| &v.stat).read() & 0b0001 == 0 {
                delay(100);
                break_count -= 1;
                if break_count == 0 {
                    break 'outer;
                }
            }

            let b = uart.map_mut(|v| &mut v.rx).read();
            i += 1;
            buf[i] = b;
            if i == buf.len() {
                break;
            }
        }
        Ok(i)
    }
}

pub fn _print(args: fmt::Arguments) {
    unsafe {
        if UART0.is_none() {
            UART0.replace(UART::initialize(__UART_START));
        }
        if let Some(uart) = &mut UART0 {
            uart.write_fmt(args).unwrap();
        }
    }
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

use core::fmt;
use core::mem::size_of;
use core::ptr::NonNull;
use core::ptr::addr_of_mut;

use crate::linker::UART;

use alloc::boxed::Box;
use alloc::sync::Arc;
use embedded_hal_nb::serial::{self, ErrorKind, ErrorType};
use nb::block;
use spin::{Lazy, Mutex};
use volatile::{VolatilePtr, map_field};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UARTInner {
    tx: u32,
    rx: u32,
    tx_ctrl: u32,
    rx_ctrl: u32,
    ie: u32,
    ip: u32,
    div: u32,
}

impl fmt::Debug for UARTInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UARTInner").finish_non_exhaustive()
    }
}

static_assertions::const_assert_eq!(size_of::<UARTInner>(), 4 * 7);

pub static UART0: Lazy<Mutex<Uart>> =
    Lazy::new(|| unsafe { Mutex::new(Uart::new(NonNull::new_unchecked(addr_of_mut!(UART)))) });

#[derive(Debug)]
#[repr(transparent)]
pub struct Uart(VolatilePtr<'static, UARTInner>);

unsafe impl Sync for Uart {}
unsafe impl Send for Uart {}

impl Uart {
    pub fn new(uart_ref: NonNull<UARTInner>) -> Self {
        Self(unsafe { VolatilePtr::new(uart_ref) })
    }
}

impl ErrorType for Uart {
    type Error = ErrorKind;
}

impl serial::Read<u8> for Uart {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let uart = &self.0;
        let b = map_field!(uart.rx).read();
        let rx_full = 0x8000_0000;
        if b & rx_full != 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok((b & 0xFF) as u8)
        }
    }
}

impl serial::Write<u8> for Uart {
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        block!(self.write_byte(byte))?;
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}

impl Uart
where
    Self: serial::ErrorType,
{
    fn write_byte(&mut self, byte: u8) -> nb::Result<(), <Self as serial::ErrorType>::Error> {
        let uart = &mut self.0;
        let b = map_field!(uart.tx).read();
        let tx_full: u32 = 0x8000_0000;
        if b & tx_full == 0 {
            map_field!(uart.tx).write(byte as u32);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?
        }
        Ok(())
    }

    #[cfg(feature = "unicode")]
    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut bytes = [0; 4];
        let s = c.encode_utf8(&mut bytes);
        for b in s.bytes() {
            block!(self.write_byte(b)).map_err(|_| core::fmt::Error)?;
        }
        Ok(())
    }

    #[cfg(not(feature = "unicode"))]
    fn write_char(&mut self, c: char) -> fmt::Result {
        let c = if c.is_ascii() { c as u8 } else { b'?' };
        block!(self.write_byte(c)).map_err(|_| core::fmt::Error)
    }
}

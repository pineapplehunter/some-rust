use crate::sync::Mutex;

#[cfg(feature = "uart_sifive_u")]
#[path = "uart_sifive_u.rs"]
pub mod ioport;

#[cfg(feature = "b4smt")]
#[path = "b4smt.rs"]
pub mod ioport;

pub mod macros;

pub static MAIN_OUTPUT: Mutex<ioport::IOPort> = Mutex::new(ioport::IOPort);

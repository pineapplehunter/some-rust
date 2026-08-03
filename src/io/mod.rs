use crate::sync::Mutex;

#[cfg(all(feature = "uart_sifive_u", not(feature = "b4smt")))]
#[path = "uart_sifive_u.rs"]
mod uart_sifive_u;

#[cfg(feature = "b4smt")]
#[path = "b4smt.rs"]
mod b4smt;

pub mod macros;

#[cfg(all(feature = "uart_sifive_u", not(feature = "b4smt")))]
pub use uart_sifive_u::IOPort;

#[cfg(feature = "b4smt")]
pub use b4smt::IOPort;

pub static MAIN_OUTPUT: Mutex<IOPort> = Mutex::new(IOPort);

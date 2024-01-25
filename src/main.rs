#![no_std]
#![no_main]

use core::fmt::Write;
use core::mem::size_of;
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, NonNull};
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use embedded_hal_nb::serial::Read;
use linker::UART;
use rust_riscv_benches::delay::delay;
use rust_riscv_benches::{dbg, debug, get_thread_id, info, linker, println, uart};
use uart::{Uart, UART0};

use crate::linker::{GPIO, PROGRAM_END, RAM};

static LOAD_DONE: AtomicBool = AtomicBool::new(false);

fn show_program_info() {
    unsafe {
        dbg!(addr_of!(UART));
        dbg!(addr_of!(GPIO));
        dbg!(addr_of!(RAM));
        dbg!(addr_of!(PROGRAM_END));
    }
}

fn echo() {
    loop {
        let Ok(b) = UART0.lock().read() else {
            continue;
        };
        info!("received!", b as char);
        delay(1000);
    }
}

struct FibonacciIterator(u64, u64);

impl FibonacciIterator {
    pub fn new() -> Self {
        FibonacciIterator(0, 0)
    }
}

impl Iterator for FibonacciIterator {
    type Item = u64;

    #[inline(never)]
    fn next(&mut self) -> Option<Self::Item> {
        Some(match (&mut self.1, &mut self.0) {
            (0, 0) => {
                self.0 = 1;
                0
            }
            (0, 1) => {
                self.0 = 2;
                1
            }
            (0, 2) => {
                self.0 = 1;
                self.1 = 1;
                1
            }
            (b, a) => {
                let t = *a;
                *a = *b;
                *b = b.checked_add(t)?;
                *b
            }
        })
    }
}

#[inline(never)]
fn fibonacci() {
    for (i, v) in FibonacciIterator::new().enumerate() {
        println!("fib({:>3}) = {:>32X}", i, v);
    }
    println!(
        "size of FibonacciIterator = {}",
        size_of::<FibonacciIterator>()
    )
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn main() {
    let thread_id = get_thread_id();
    debug!("start loading!");
    debug!("Hello 世界!");
    debug!(
        "thread id = {}, id*id = {}",
        thread_id,
        thread_id * thread_id
    );
    // let mut output_buf = staticvec::StaticString::<512>::new();
    // writeln!(&mut output_buf, "{:#?}", &UART0).expect("failed to write to output buffer");
    // println!("{}", output_buf);

    let time = riscv::register::cycle::read();
    dbg!(time);
    if thread_id == 0 {
        println!("Hello!");
        println!("I am B4Processor!");
        fibonacci();
        show_program_info();
        LOAD_DONE.store(true, Relaxed);
        echo();
    } else {
        while !LOAD_DONE.load(Relaxed) {
            delay(100000);
            println!("thread {} still waiting...", thread_id);
        }
        println!("init done");
        echo();
    }
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    let _uart = UART0.try_lock();
    let mut uart = Uart::new(unsafe { NonNull::new_unchecked(addr_of_mut!(UART)) });
    writeln!(uart).ok();
    writeln!(uart).ok();
    writeln!(uart, "!!!!! panic at thread {} !!!!!", get_thread_id()).ok();
    writeln!(uart, "{}", info).ok();
    loop {}
}

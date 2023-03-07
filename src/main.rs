#![no_std]
#![no_main]

mod delay;
mod linker;
mod sifive_u_uart;

use core::arch::asm;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use core::{arch::global_asm, fmt::Write};
use delay::delay;
use embedded_hal::serial::Read;
use linker::UART;
use sifive_u_uart::{Uart, UART0};
use spin::Lazy;

use crate::linker::{GPIO, PROGRAM_END, RAM};

static LOAD_DONE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn show_program_info() {
    unsafe {
        dbg!(&UART as *const _);
        dbg!(&GPIO as *const _);
        dbg!(&RAM as *const _);
        dbg!(&PROGRAM_END as *const _);
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

/// get thread id assuming `mhartid` is stored in `tp`
#[inline(always)]
pub fn get_thread_id() -> usize {
    let thread_id: usize;
    unsafe { asm!("mv {tp}, tp", tp = out(reg) thread_id) };
    thread_id
}

#[no_mangle]
pub extern "C" fn loader_main() {
    let thread_id = get_thread_id();
    debug!("start loading!");
    debug!("Hello 世界!");
    let mut output_buf = staticvec::StaticString::<128>::new();
    writeln!(&mut output_buf, "{:#?}", &UART0).expect("failed to write to output buffer");
    println!("{}", output_buf);

    let time = riscv::register::cycle::read();
    dbg!(time);
    if thread_id == 0 {
        println!("Hello!");
        println!("I am B4Processor!");
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
    let mut uart = Uart::new(unsafe { &mut linker::UART });
    writeln!(uart).ok();
    writeln!(uart).ok();
    writeln!(uart, "!!!!! panic at thread {} !!!!!", get_thread_id()).ok();
    writeln!(uart, "{}", info).ok();
    loop {}
}

global_asm!(include_str!("boot.s"));

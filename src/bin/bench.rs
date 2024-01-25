#![no_std]
#![no_main]

use core::{
    arch::asm,
    fmt,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

static COUNT: AtomicUsize = AtomicUsize::new(0);

#[inline(never)]
#[no_mangle]
pub extern "C" fn fibonacci(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n {
        let t = b;
        b += a;
        a = t;
    }
    b
}

static mut RESULTS: [usize; 50] = [0; 50];
static START: AtomicBool = AtomicBool::new(false);
static THREADS_DONE: AtomicUsize = AtomicUsize::new(0);

/// get thread id assuming `mhartid` is stored in `tp`
#[inline(always)]
pub fn get_thread_id() -> usize {
    let thread_id: usize;
    unsafe { asm!("mv {tp}, tp", tp = out(reg) thread_id) };
    thread_id
}

#[no_mangle]
#[inline(never)]
extern "C" fn main(thread_id: usize) {
    use fmt::Write;
    let mut start_cycle: usize = 0;
    let mut end_cycle: usize;

    if thread_id == 0 {
        writeln!(IOPort, "OK!").unwrap();
        START.swap(true, Ordering::Relaxed);
        unsafe { asm!("rdcycle {cycle}", cycle = out(reg) start_cycle) };
    }

    while !START.load(Ordering::Relaxed) {
        riscv::asm::nop();
    }

    loop {
        let a = COUNT.fetch_add(1, Ordering::Relaxed);
        if a < unsafe { RESULTS.len() } {
            unsafe {
                RESULTS[a] = fibonacci(a);
            }
        } else {
            THREADS_DONE.fetch_add(1, Ordering::Relaxed);
            break;
        }
    }

    if thread_id == 0 {
        while get_thread_count() != THREADS_DONE.load(Ordering::Relaxed) {}
        unsafe { asm!("rdcycle {cycle}", cycle = out(reg) end_cycle) };

        writeln!(IOPort, "took {} cycles", end_cycle - start_cycle).unwrap();

        for i in unsafe { RESULTS } {
            writeln!(IOPort, "{}", i).unwrap();
        }
    }
    loop {
        riscv::asm::nop();
    }
}

struct IOPort;

impl fmt::Write for IOPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if c.is_ascii() {
            unsafe { (0x1000_0000 as *mut u32).write_volatile(c as u32) }
        }
        Ok(())
    }
}

use core::panic::PanicInfo;

use rust_riscv_benches::get_thread_count;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {
        // unsafe { riscv::asm::wfi() };
    }
}

#![no_std]
#![no_main]

extern crate alloc;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    ptr::addr_of,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use crate::heap::CustomLockedHeap;
use crate::{
    io::{ioport::IOPort, MAIN_OUTPUT},
    linker::{HEAP_END, PROGRAM_END},
};

pub mod delay;
pub mod heap;
pub mod io;
pub mod linker;
pub mod metrics;
pub mod pxet;
pub mod sync;
pub mod thread;

#[cfg(feature = "critical_section_mt")]
mod critical_section;

/// get thread id assuming `mhartid` is stored in `tp`
#[inline(always)]
pub fn get_thread_id() -> usize {
    let thread_id: usize;
    unsafe { asm!("mv {tp}, tp", tp = out(reg) thread_id) };
    thread_id
}

pub static THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static RT_INIT_DONE: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub fn get_thread_count() -> usize {
    debug_assert!(RT_INIT_DONE.load(Ordering::Relaxed));
    THREAD_COUNT.load(Ordering::Relaxed)
}

extern "C" {
    fn main(thread_id: usize);
}

#[inline(never)]
#[no_mangle]
unsafe fn clear_bss() {
    // ASSUMES BSS IS 64bit ALIGNED !!!
    let mut start = addr_of!(linker::BSS_START).cast_mut();
    let end = addr_of!(linker::BSS_END);

    while (start as usize) < (end as usize) {
        start.write_volatile(0);
        start = start.offset(1);
    }
}

#[global_allocator]
static HEAP: CustomLockedHeap = CustomLockedHeap::empty();

/// # Safety
/// this will only be called from asm
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn init_rt(thread_id: usize) -> ! {
    // DON'T ASSUME BSS IS 0 AT THIS POINT!!!!
    RT_INIT_DONE.store(false, Ordering::SeqCst);
    if thread_id == 0 {
        unsafe { clear_bss() };

        let thread_count: usize;
        asm!("csrr {}, 0xCC0", out(reg) thread_count);
        THREAD_COUNT.store(thread_count, Ordering::Relaxed);

        let heap_size = addr_of!(HEAP_END) as usize - addr_of!(PROGRAM_END) as usize;
        let heap_addr = addr_of!(PROGRAM_END) as *mut u8;
        unsafe { HEAP.lock().init(heap_addr, heap_size) }

        RT_INIT_DONE.swap(true, Ordering::Relaxed);
    } else {
        while RT_INIT_DONE.load(Ordering::SeqCst) {
            riscv::asm::nop()
        }
    }
    // JUMP TO MAIN!!!
    main(thread_id);

    loop {
        riscv::asm::nop();
    }
}

global_asm!(include_str!("boot.S"));

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TrapFrame {
    pub regs: [usize; 32],   // 0 - 255
    pub fregs: [usize; 32],  // 256 - 511
    pub satp: usize,         // 512 - 519
    pub trap_stack: *mut u8, // 520
    pub hartid: usize,       // 528
}

#[no_mangle]
#[inline(never)]
extern "C" fn m_trap(frame: TrapFrame) {
    println!("TRAP!!!!");
    println!("{:#?}", frame);
    loop {
        riscv::asm::nop();
    }
}

#[inline(never)]
#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    MAIN_OUTPUT.try_lock();
    let mut out = IOPort;

    write!(out, "\n\n\n").ok();
    writeln!(out, "NG: panic on thread {}", get_thread_id()).ok();
    writeln!(out, "{}", info).ok();
    loop {
        riscv::asm::nop();
    }
}

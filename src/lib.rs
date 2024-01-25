#![no_std]

use core::{
    arch::{asm, global_asm},
    ptr::{addr_of, addr_of_mut},
};

pub mod delay;
pub mod linker;
#[cfg(feature = "uart_sifive_u")]
#[path = "uart_sifive_u.rs"]
pub mod uart;

/// get thread id assuming `mhartid` is stored in `tp`
#[inline(always)]
pub fn get_thread_id() -> usize {
    let thread_id: usize;
    unsafe { asm!("mv {tp}, tp", tp = out(reg) thread_id) };
    thread_id
}

pub static mut THREAD_COUNT: usize = 0;
pub static mut RT_INIT_DONE: bool = false;

#[inline(always)]
pub fn get_thread_count() -> usize {
    unsafe {
        debug_assert!(addr_of!(RT_INIT_DONE).read_volatile());
        THREAD_COUNT
    }
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

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn init_rt(thread_id: usize) -> ! {
    // DON'T ASSUME BSS IS 0 AT THIS POINT!!!!
    addr_of_mut!(RT_INIT_DONE).write(false);
    if thread_id == 0 {
        unsafe { clear_bss() };

        let thread_count: usize;
        asm!("csrr {}, 0xCC0", out(reg) thread_count);
        addr_of_mut!(THREAD_COUNT).write_volatile(thread_count);
        addr_of_mut!(RT_INIT_DONE).write_volatile(true);
    } else {
        while !addr_of!(RT_INIT_DONE).read_volatile() {
            delay::delay(10);
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
    loop {}
}

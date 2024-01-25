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

#[no_mangle]
#[inline(never)]
pub extern "C" fn init_rt(thread_id: usize) -> ! {
    if thread_id == 0 {
        let thread_count: usize;
        unsafe {
            asm!("csrr {}, 0xCC0", out(reg) thread_count);
            addr_of_mut!(THREAD_COUNT).write_volatile(thread_count);
            addr_of_mut!(RT_INIT_DONE).write_volatile(true);
        };
    } else {
        unsafe {
            while !addr_of!(RT_INIT_DONE).read_volatile() {
                for _ in 0..50 {
                    asm!("nop");
                }
            }
        }
    }
    unsafe { main(thread_id) };
    loop {
        riscv::asm::nop();
    }
}

global_asm!(include_str!("boot.S"));

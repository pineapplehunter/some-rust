#![no_main]
#![no_std]

use core::{arch::asm, ptr::addr_of};

use rust_riscv_benches::println;

fn inner_product_u16(a: &mut [i16], b: &[i16]) -> isize {
    assert_eq!(a.len(), b.len());
    for (e1, e2) in a.iter_mut().zip(b) {
        *e1 *= e2;
    }
    0
}

#[allow(dead_code)]
fn inner_product_u8(a: &mut [i8], b: &[i8]) {
    assert_eq!(a.len(), b.len());
    for (e1, e2) in a.iter_mut().zip(b) {
        *e1 *= e2;
    }
}

fn khm16(a: usize, b: usize) -> usize {
    let mut out: usize;
    unsafe {
        asm!(".insn r 0b1110111, 0, 0b1000011, {out}, {in1}, {in2}", out = out(reg)out, in1 = in(reg)a, in2= in(reg)b)
    };
    out
}

#[no_mangle]
#[inline(never)]
unsafe fn inner_product_u16_pext(a: &mut [i16], b: &[i16]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(addr_of!(a) as usize % 8, addr_of!(b) as usize % 8);

    let (a_prefix, a_chunks, a_postfix) = a.align_to_mut::<usize>();
    let (b_prefix, b_chunks, b_postfix) = b.align_to::<usize>();

    if a_prefix.is_empty() {
        let mut a_prefix_usize = *a_prefix.get_unchecked(0) as u16 as usize;
        let mut b_prefix_usize = *b_prefix.get_unchecked(0) as u16 as usize;

        if a_prefix.len() > 1 {
            a_prefix_usize |= (*a_prefix.get_unchecked(1) as u16 as usize) << 16;
            b_prefix_usize |= (*b_prefix.get_unchecked(1) as u16 as usize) << 16;

            if a_prefix.len() > 2 {
                a_prefix_usize |= (*a_prefix.get_unchecked(2) as u16 as usize) << 32;
                b_prefix_usize |= (*b_prefix.get_unchecked(2) as u16 as usize) << 32;

                if a_prefix.len() > 3 {
                    a_prefix_usize |= (*a_prefix.get_unchecked(3) as u16 as usize) << 48;
                    b_prefix_usize |= (*b_prefix.get_unchecked(3) as u16 as usize) << 48;
                }
            }
        }
        let result = khm16(a_prefix_usize, b_prefix_usize);
        for i in 0..a_prefix.len() {
            *a_prefix.get_unchecked_mut(i) = (result >> (i * 16)) as i16;
        }
    }

    for (e1, e2) in a_chunks.iter_mut().zip(b_chunks) {
        *e1 = khm16(*e1, *e2);
    }

    if a_postfix.is_empty() {
        let mut a_postfix_usize = *a_postfix.get_unchecked(0) as u16 as usize;
        let mut b_postfix_usize = *b_postfix.get_unchecked(0) as u16 as usize;

        if a_prefix.len() > 1 {
            a_postfix_usize |= (*a_postfix.get_unchecked(1) as u16 as usize) << 16;
            b_postfix_usize |= (*b_postfix.get_unchecked(1) as u16 as usize) << 16;

            if a_prefix.len() > 2 {
                a_postfix_usize |= (*a_postfix.get_unchecked(2) as u16 as usize) << 32;
                b_postfix_usize |= (*b_postfix.get_unchecked(2) as u16 as usize) << 32;

                if a_prefix.len() > 3 {
                    a_postfix_usize |= (*a_postfix.get_unchecked(3) as u16 as usize) << 48;
                    b_postfix_usize |= (*b_postfix.get_unchecked(3) as u16 as usize) << 48;
                }
            }
        }
        let result = khm16(a_postfix_usize, b_postfix_usize);
        for i in 0..a_postfix.len() {
            *a_postfix.get_unchecked_mut(i) = (result >> (i * 16)) as i16;
        }
    }
}

#[inline(never)]
#[no_mangle]
fn main(thread_id: usize) {
    if thread_id != 0 {
        return;
    }
    println!("Hello World");
    let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let b = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    // unsafe { inner_product_u16_pext(&mut a, &b) };
    inner_product_u16(&mut a, &b);

    println!("{:?}", a);
}

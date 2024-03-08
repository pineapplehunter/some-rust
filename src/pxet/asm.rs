use core::arch::asm;

#[inline(always)]
pub fn khm16(a: usize, b: usize) -> usize {
    let mut out: usize;
    unsafe {
        asm!(".insn r 0b1110111, 0, 0b1000011, {out}, {in1}, {in2}", out = out(reg)out, in1 = in(reg)a, in2= in(reg)b)
    };
    out
}

#[inline(always)]
pub fn khm8(a: usize, b: usize) -> usize {
    let mut out: usize;
    unsafe {
        asm!(".insn r 0b1110111, 0, 0b1000111, {out}, {in1}, {in2}", out = out(reg)out, in1 = in(reg)a, in2= in(reg)b)
    };
    out
}

#[inline(always)]
pub fn smul16(a: usize, b: usize) -> usize {
    let mut out: usize;
    unsafe {
        asm!(".insn r 0b1110111, 0, 0b1010000, {out}, {in1}, {in2}", out = out(reg)out, in1 = in(reg)a, in2= in(reg)b)
    };
    out
}

#[inline(always)]
pub fn smul8(a: usize, b: usize) -> usize {
    let mut out: usize;
    unsafe {
        asm!(".insn r 0b1110111, 0b000, 0b1010100, {out}, {in1}, {in2}", out = out(reg)out, in1 = in(reg)a, in2= in(reg)b)
    };
    out
}

#[inline(always)]
pub fn smalbb(mut acc: usize, a: usize, b: usize) -> usize {
    unsafe {
        asm!(".insn r 0b1110111, 0b001, 0b1000100, {out}, {in1}, {in2}", out = inout(reg)acc, in1 = in(reg)a, in2= in(reg)b)
    };
    acc
}

#[inline(always)]
pub fn smaltt(mut acc: usize, a: usize, b: usize) -> usize {
    unsafe {
        asm!(".insn r 0b1110111, 0b001, 0b1010100, {out}, {in1}, {in2}", out = inout(reg)acc, in1 = in(reg)a, in2= in(reg)b)
    };
    acc
}

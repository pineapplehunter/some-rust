unsafe extern "C" {
    fn __delay(amount: usize);
}
#[inline(always)]
pub fn delay(amount: usize) {
    unsafe { __delay(amount) };
}

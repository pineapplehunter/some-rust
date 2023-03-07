#[inline(never)]
pub fn delay(mut amount: usize) {
    while amount > 0 {
        core::hint::black_box(());
        amount -= 1;
    }
}

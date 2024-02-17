pub fn delay(mut amount: usize) {
    while amount > 0 {
        amount = core::hint::black_box(amount - 1);
    }
}

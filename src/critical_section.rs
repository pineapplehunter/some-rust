use core::sync::atomic::{AtomicBool, Ordering};

use critical_section::{set_impl, Impl, RawRestoreState};

use crate::delay::delay;

static CRITICAL_SECTION_LOCK: AtomicBool = AtomicBool::new(false);

struct MultiHartCriticalSection;
set_impl!(MultiHartCriticalSection);

unsafe impl Impl for MultiHartCriticalSection {
    #[inline(never)]
    unsafe fn acquire() -> RawRestoreState {
        while CRITICAL_SECTION_LOCK.swap(true, Ordering::Relaxed) {
            delay(10);
        }
    }

    #[inline(never)]
    unsafe fn release(_: RawRestoreState) {
        let state = CRITICAL_SECTION_LOCK.swap(false, Ordering::Relaxed);
        assert!(state, "critical section was broken");
    }
}

use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

pub struct Mutex<T> {
    lock: AtomicU32,
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}

pub struct UnsafeSyncCell<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for UnsafeSyncCell<T> {}

pub struct MutexGuard<'a, T> {
    lock: &'a AtomicU32,
    data: &'a mut T,
}

impl<T> Mutex<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            lock: AtomicU32::new(0),
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.lock.fetch_or(0, Ordering::SeqCst) != 0
    }

    // #[inline(never)]
    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
        }
    }

    // #[inline(never)]
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        if self.lock.swap(1, Ordering::SeqCst) == 0 {
            Some(MutexGuard {
                lock: &self.lock,
                data: unsafe { &mut *self.inner.get() },
            })
        } else {
            None
        }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        let state = self.lock.swap(0, Ordering::SeqCst);
        assert_eq!(state, 1, "mutex is broken");
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<T> UnsafeSyncCell<T> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: UnsafeCell::new(data),
        }
    }

    /// # Safety
    /// unsafe
    pub unsafe fn get_mut(&self) -> *mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.inner.get() }
    }
}

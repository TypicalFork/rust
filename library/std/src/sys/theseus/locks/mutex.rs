use crate::cell::Cell;

/// "Don't use spinlocks, like, you almost never want a spinlock, you almost never want to implement your own mutex. There's a great article by matklad on spinlocks considered
/// harmful; i'll post it it in chat here. It's a great article - read it and don't implement your own spin locks and probably don't use a spin lock in the first place. But,
/// we're gonna do it." - [https://youtu.be/rMGWeSjctlY?t=1390]
pub struct Mutex {
    inner: libtheseus::sync::Mutex<bool>,
}

pub type MovableMutex = Mutex;

// TODO
// unsafe impl Send for Mutex {}
// unsafe impl Sync for Mutex {}

impl Mutex {
    #[inline]
    pub const fn new() -> Mutex {
        Mutex { inner: libtheseus::Mutex::new(false) }
    }

    #[inline]
    pub unsafe fn init(&mut self) {}

    #[inline]
    pub unsafe fn lock(&self) {
        loop {
            let mut guard = self.inner.lock();
            if !guard.locked {
                guard.locked = true;
                return;
            } else {
                drop(guard);
                // TODO: Yield
            }
        }
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        let mut guard = self.inner.lock();
        guard.locked = false;
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        let mut guard = self.inner.lock();
        if !guard.locked {
            guard.locked = true;
        }
        guard.locked
    }
}

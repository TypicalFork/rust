pub struct RwLock {}

pub type MovableRwLock = RwLock;

impl RwLock {
    #[inline]
    pub const fn new() -> RwLock {
        todo!();
    }

    #[inline]
    pub unsafe fn read(&self) {
        todo!();
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        todo!();
    }

    #[inline]
    pub unsafe fn write(&self) {
        todo!();
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        todo!();
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        todo!();
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        todo!();
    }
}

use crate::sys::locks::Mutex;
use crate::time::Duration;

use libtheseus::sync::wait_queue::WaitQueue;

pub struct Condvar {
    queue: WaitQueue,
}

pub type MovableCondvar = Condvar;

impl Condvar {
    #[inline]
    pub const fn new() -> Condvar {
        Condvar {
            queue: WaitQueue::new(),
        }
    }

    #[inline]
    pub unsafe fn notify_one(&self) {
        self.queue.notify_one();
    }

    #[inline]
    pub unsafe fn notify_all(&self) {
        self.queue.notify_all();
    }

    pub unsafe fn wait(&self, _mutex: &Mutex) {
        panic!("condvar wait not supported")
    }

    pub unsafe fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        panic!("condvar wait not supported");
    }
}

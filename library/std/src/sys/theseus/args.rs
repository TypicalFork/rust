use crate::ffi::OsString;
use crate::fmt;

pub struct Args {
    iter: vec::IntoIter<OsString>,
}

pub fn args() -> Args {
    Args { iter: imp::clone().into_iter() }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        self.iter.next_back()
    }
}

pub(crate) mod imp {
    use super::Args;
    use crate::{
        ffi::{CStr, OsString},
        os::unix::ffi::OsStringExt,
        ptr,
        sys_common::mutex::StaticMutex,
    };

    static mut ARGC: isize = 0;
    static mut ARGV: *const *const u8 = ptr::null();
    static LOCK: StaticMutex = StaticMutex::new();

    pub(crate) unsafe fn init(argc: isize, argv: *const *const u8) {
        let _guard = LOCK.lock();
        ARGC = argc;
        ARGV = argv;
    }

    pub(crate) unsafe fn cleanup() {
        let _guard = LOCK.lock();
        ARGC = 0;
        ARGV = ptr::null();
    }

    fn clone() -> Vec<OsString> {
        unsafe {
            let _guard = LOCK.lock();
            (0..ARGC)
                .map(|i| {
                    let cstr = CStr::from_ptr(*ARGV.offset(i) as *const i8);
                    OsStringExt::from_vec(cstr.to_bytes().to_vec())
                })
                .collect()
        }
    }
}

use crate::io as std_io;

pub mod memchr {
    pub use core::slice::memchr::{memchr, memrchr};
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    super::args::imp::init(argc, argv);
}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {
    super::args::imp::cleanup();
}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::const_io_error!(
        std_io::ErrorKind::Unsupported,
        "operation not supported on Theseus",
    )
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Uncategorized
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

pub fn hashmap_random_keys() -> (u64, u64) {
    // FIXME
    (1, 2)
}

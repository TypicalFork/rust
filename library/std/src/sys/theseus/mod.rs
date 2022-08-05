#![deny(unsafe_op_in_unsafe_fn)]

pub use libtheseus as _;

/// This function exists to satisfy the linker.
///
/// `_start`, and thus this function are never called by Theseus.
// TODO: Don't generate _start / don't call this function for Theseus.
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn __libc_start_main(
    _main: usize,
    _argc: i32,
    _argv: usize,
    _init_dummy: usize,
    _fini_dummy: usize,
    _ldso_dummy: usize,
) -> i32 {
    // Calling this function is an error.
    1
}

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod env;
pub mod fs;
pub mod io;
pub mod locks;
pub mod net;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
pub mod process;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
pub mod thread_local_dtor;
pub mod thread_local_key;
pub mod time;

mod common;
pub use common::*;

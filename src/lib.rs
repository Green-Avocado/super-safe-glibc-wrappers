#![feature(iter_advance_by)]
#![feature(panic_always_abort)]
#![feature(local_key_cell_methods)]

pub mod preload_hooks;
mod utils;

use std::sync::atomic::AtomicBool;

const LIBC_PATH: &str = "/usr/lib/libc.so.6";
static MAIN_STARTED: AtomicBool = AtomicBool::new(false);

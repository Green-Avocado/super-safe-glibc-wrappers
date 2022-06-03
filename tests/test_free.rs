#[cfg(not(disable_heap_hooks))]
mod tests {
    use libc::c_void;
    use somewhat_safe_glibc_wrappers::preload_hooks::heap::free;
    use std::panic;

    #[test]
    fn test_zero() {
        unsafe { free(0 as *mut c_void) };
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        _ = panic::take_hook();
        unsafe { free(1 as *mut c_void) };
    }
}

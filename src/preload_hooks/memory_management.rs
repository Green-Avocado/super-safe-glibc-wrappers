use crate::{utils::get_ptr_info, MAIN_STARTED};
use libc::c_void;
use std::{cell::Cell, panic, sync::atomic::Ordering};

thread_local! {
    static FREE_RECURSION_GUARD: Cell<bool> = Cell::new(true);
}

/*
    Hooks free
    - FREE_RECURSION_GUARD prevents recursive calls to free
    - does nothing if __libc_start_main has not been called
    - performs checks without freeing anything if __libc_start_main has been called
*/
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    if ptr as usize == 0 {
        return;
    }

    if !FREE_RECURSION_GUARD.get() {
        return;
    }

    FREE_RECURSION_GUARD.set(false);

    if MAIN_STARTED.load(Ordering::SeqCst) {
        let page_info = get_ptr_info(ptr).expect("freeing invalid pointer");

        if !(page_info.read && page_info.write && !page_info.execute) {
            panic!("freeing invalid permissions");
        }

        if page_info.file == Some("[stack]".to_string()) {
            panic!("freeing in stack");
        }
    }

    FREE_RECURSION_GUARD.set(true);
}
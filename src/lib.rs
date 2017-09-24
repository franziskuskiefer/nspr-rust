#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// Testing NSPR.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logging() {
        unsafe {
            PR_Init(
                PRThreadType::PR_USER_THREAD,
                PRThreadPriority::PR_PRIORITY_NORMAL,
                0,
            );
            // PR_STDIO_INIT(); // XXX: not generated! only needed on Windows.
            let msg = CString::new("creating new thread...\n").unwrap().as_ptr() as *const i8;
            PR_LogPrint(msg);
        }
    }
}

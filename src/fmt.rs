use libc::{c_char, size_t};
use std::ffi::CString;


extern {
    pub fn php_printf(format: *const c_char , ...) -> size_t;
}

#[macro_export]
macro_rules! printf {
    ($format:expr) => {
        let c_format = CString::new($format).unwrap();
        unsafe {
            php::fmt::php_printf(c_format.as_ptr() as *const c_char);
        }
    };
    ($format:expr, $($arg:expr), *) => {
        let c_format = CString::new($format).unwrap();
        unsafe {
            php::fmt::php_printf(c_format.as_ptr() as *const c_char, $($arg), *);
        }
    };
}

use libc::*;
use std::ffi::CString;

extern {
    pub fn php_info_print_table_start();
    pub fn php_info_print_table_row(num_cols: c_int, ...) -> c_void;
    pub fn php_info_print_table_end();
}

pub fn print_table_row(values: &[&str]) {
    let nargs = values.len() as i32;

    if nargs != 2 {
        unimplemented!();
    }

    let v1 = CString::new(values[0]).unwrap();
    let v2 = CString::new(values[1]).unwrap();

    unsafe {
        php_info_print_table_row(nargs, v1.as_ptr(), v2.as_ptr());
    };
}

pub fn print_table_start() {
    unsafe { php_info_print_table_start() };
}

pub fn print_table_end() {
    unsafe { php_info_print_table_end() }
}


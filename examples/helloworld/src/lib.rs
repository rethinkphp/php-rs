#![allow(unused_variables)]
#![feature(link_args)]

extern crate libc;
extern crate php;

use libc::*;
use php::*;
use php::info::*;

#[link_args = "-Wl,-undefined,dynamic_lookup"]
extern {

}

macro_rules! c_str {
    ($s:expr) => { {
        concat!($s, "\0").as_ptr() as *const c_char
    } }
}

#[no_mangle]
pub extern fn php_module_startup(type_: c_int, module_number: c_int) -> c_int {
   0 
}

#[no_mangle]
pub extern fn php_module_shutdown(type_: c_int, module_number: c_int) -> c_int {
   0 
}

#[no_mangle]
pub extern fn php_module_info() {
    print_table_start();
    unsafe {
        php_info_print_table_row(2, c_str!("A demo PHP extension written in Rust"), c_str!("enabled"));
    }
    print_table_end();
}


#[no_mangle]
pub extern fn get_module() -> *mut zend::Module {

    let mut entry = Box::new(zend::Module::new(
        c_str!("demo"),
        c_str!("0.1.0-dev"),
    ));

    entry.set_info_func(php_module_info);

    Box::into_raw(entry)
}

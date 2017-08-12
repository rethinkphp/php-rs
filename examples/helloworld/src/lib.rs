#![allow(unused_variables)]
#![feature(link_args)]

extern crate libc;
extern crate php;

use libc::*;
use php::*;
use zend::*;
use php::info::*;

#[link_args = "-Wl,-undefined,dynamic_lookup"]
extern {
    pub fn php_printf(format: *const c_char , ...) -> size_t;
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
    print_table_row(&["A demo PHP extension written in Rust", "enabled"]);
    print_table_end();
}

#[no_mangle]
pub extern fn helloworld(data: &ExecuteData, retval: &Value) {
    unsafe {
        php_printf(c_str!("Hello world, Rust!"))
    };
}

#[no_mangle]
pub extern fn get_module() -> *mut zend::Module {

    let mut entry = Box::new(zend::Module::new(
        c_str!("demo"),
        c_str!("0.1.0-dev"),
    ));

    entry.set_info_func(php_module_info);

    let args = Box::new([
        ArgInfo::new(1 as *const c_char, 0, 0, 0),
        ArgInfo::new(c_str!("name"), 0, 0, 0),
    ]);

    let funcs = Box::new([
        Function::new(c_str!("helloworld"), helloworld),
        Function::new_with_args(c_str!("helloworld2"), helloworld, args),
        Function::end(),
    ]);

    entry.set_functions(funcs);

    Box::into_raw(entry)
}

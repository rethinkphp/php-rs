use libc::*;

#[link_args = "-Wl,-undefined,dynamic_lookup"]
extern {
    pub fn php_info_print_table_row(num_cols: c_int, ...) -> c_void;
    pub fn php_info_print_table_row_ex(num_cols: c_int, a: *const c_char, ...);
    pub fn php_info_print_table_start();
    pub fn php_info_print_table_end();
    pub fn php_info_print_hr();
}

pub fn print_table_start() {
    unsafe { php_info_print_table_start() }
}

pub fn print_table_end() {
    unsafe { php_info_print_table_end() }
}


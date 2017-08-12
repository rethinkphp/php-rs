use std;
use std::mem;
use libc::*;

type StartupFunc = extern fn (type_: c_int, module_number: c_int) -> c_int;
type ShutdownFunc = extern fn (type_: c_int, module_number: c_int) -> c_int;
type InfoFunc = extern fn () ;
type GlobalsCtorFunc = extern fn (global: *const c_void) -> c_void;
type GlobalsDtorFunc = extern fn (global: *const c_void) -> c_void;
type PostDeactivateFunc = extern fn () -> c_int;


macro_rules! c_str {
    ($s:expr) => { {
        concat!($s, "\0").as_ptr() as *const c_char
    } }
}

pub struct ModuleDep {}
pub struct Function {}
pub struct INI {}

#[repr(C)]
pub struct Module {
    size: c_ushort,
    zend_api: c_uint,
    zend_debug: c_uchar,
    zts: c_uchar,
    ini_entry: *const INI,
    deps: *const ModuleDep,
    name: *const c_char,
    functions: *const Function,
    module_startup_func: Option<StartupFunc>,
    module_shutdown_func: Option<ShutdownFunc>,
    request_startup_func: Option<StartupFunc>,
    request_shutdown_func: Option<ShutdownFunc>,
    info_func: Option<InfoFunc>,
    version: *const c_char,
    globals_size: size_t,
    globals_ptr: *const c_void,
    globals_ctor: Option<GlobalsCtorFunc>,
    globals_dtor: Option<GlobalsDtorFunc>,
    post_deactivate_func: Option<PostDeactivateFunc>,
    module_started: c_int,
    type_: c_uchar,
    handle: *const c_void,
    module_number: c_int,
    build_id: *const c_char,
}

impl Module {
    pub fn new(name: *const c_char, version: *const c_char) -> Module {
        Module {
            size: mem::size_of::<Module>() as u16,
            zend_api: 20151012,
            zend_debug: 0,
            zts: 0,
            ini_entry: std::ptr::null(),
            deps: std::ptr::null(),
            name: name,
            functions: std::ptr::null(),
            module_startup_func: None,
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: None,
            version: version,
            globals_size: 0,
            globals_ptr: std::ptr::null(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: std::ptr::null(),
            module_number: 0,
            build_id: c_str!("API20151012,NTS"),
        }
    }

    pub fn set_startup_func(&mut self, func: StartupFunc) {
        self.module_startup_func = Some(func);
    }

    pub fn set_shutdown_func(&mut self, func: ShutdownFunc) {
        self.module_shutdown_func = Some(func);
    }

    pub fn set_info_func(&mut self, func: InfoFunc) {
        self.info_func = Some(func);
    }
}

unsafe impl Sync for Module {}

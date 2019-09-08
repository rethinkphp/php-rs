use libc::*;

// Zend Types and Zval
//https://github.com/php/php-src/blob/d0754b86b1cb4774c4af64498641ddaaab745418/Zend/zend_types.h#L176-L233
#[repr(C)]
pub union ZendValue {
    pub long_value: c_long,
    pub double_value: c_double,
    pub string: *mut ZendString,
}

#[repr(C)]
pub union U1 {
    pub type_info: libc::uint32_t,
}

#[repr(C)]
pub union U2 {
    pub next: libc::uint32_t,
    pub num_args: libc::uint32_t,
}

#[repr(C)]
pub struct ZendRefCounted {
    pub ref_count: libc::uint32_t,
    pub type_info: libc::uint32_t,
}

#[repr(C)]
pub struct ZendString {
    pub gc: ZendRefCounted,
    pub hash: libc::uint32_t,
    pub len: libc::size_t,
    pub value: *mut libc::c_char,
}

#[repr(C)]
pub struct Zval {
    pub value: ZendValue,
    pub u1: U1,
    pub u2: U2,
}


#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(link_args)]
#![feature(c_variadic)]

extern crate libc;

#[macro_use]
pub mod macros;

pub mod zend;
pub mod info;
pub mod fmt;

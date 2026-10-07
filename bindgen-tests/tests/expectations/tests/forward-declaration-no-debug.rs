#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct NoDebug {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct WithDebug {
    _unused: [u8; 0],
}

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    /// A variable. (processed)
    ///Parsed: A variable.
    pub static mut variable: ::std::os::raw::c_int;
}
unsafe extern "C" {
    /// A function. (processed)
    ///Parsed: A function.
    pub fn function();
}
/// A type alias. (processed)
///Parsed: A type alias.
pub type Alias = ::std::os::raw::c_int;
/// A struct. (processed)
///Parsed: A struct.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Struct {
    /// A field. (processed)
    pub field: ::std::os::raw::c_int,
}
/// A variant. (processed)
///Parsed: A variant.
pub const Enum_Variant: Enum = 0;
/// An enum. (processed)
///Parsed: An enum.
pub type Enum = ::std::os::raw::c_uint;
/// A forward declared struct alias. (processed)
///Parsed: A forward declared struct alias.
#[repr(C)]
pub struct Forward {
    _unused: [u8; 0],
}

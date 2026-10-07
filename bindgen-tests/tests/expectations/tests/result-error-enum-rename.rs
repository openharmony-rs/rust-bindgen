#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type MyResult = Result<(), MyErrorCode>;
impl MyErrorCode {
    pub const MyResultErr1: MyErrorCode = MyErrorCode(const {
        core::num::NonZero::new(1).unwrap()
    });
    pub const MyResultErr2: MyErrorCode = MyErrorCode(const {
        core::num::NonZero::new(2).unwrap()
    });
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MyErrorCode(pub core::num::NonZero<::std::os::raw::c_uint>);
unsafe extern "C" {
    pub fn do_something() -> MyResult;
}

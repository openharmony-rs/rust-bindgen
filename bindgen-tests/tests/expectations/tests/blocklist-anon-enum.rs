#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WithEmptyAnonEnum {
    pub e: WithEmptyAnonEnum__bindgen_ty_1,
}
pub type WithEmptyAnonEnum__bindgen_ty_1 = ::std::os::raw::c_uint;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithEmptyAnonEnum"][::std::mem::size_of::<WithEmptyAnonEnum>() - 4usize];
    [
        "Alignment of WithEmptyAnonEnum",
    ][::std::mem::align_of::<WithEmptyAnonEnum>() - 4usize];
    [
        "Offset of field: WithEmptyAnonEnum::e",
    ][::std::mem::offset_of!(WithEmptyAnonEnum, e) - 0usize];
};
impl Default for WithEmptyAnonEnum {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

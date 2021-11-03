#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCS_CALLBACK(pub isize);
impl ::std::default::Default for HCS_CALLBACK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HCS_CALLBACK {}
unsafe impl ::windows::runtime::Abi for HCS_CALLBACK {
    type Abi = Self;
}

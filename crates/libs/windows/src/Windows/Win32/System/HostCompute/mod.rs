#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_CALLBACK(pub isize);
impl HCS_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HCS_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_CALLBACK {}
impl ::core::fmt::Debug for HCS_CALLBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_CALLBACK").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HCS_CALLBACK {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_API_PTR(pub usize);
impl JET_API_PTR {
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
impl ::core::default::Default for JET_API_PTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_API_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_API_PTR {}
impl ::core::fmt::Debug for JET_API_PTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_API_PTR").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_API_PTR {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_HANDLE(pub usize);
impl JET_HANDLE {
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
impl ::core::default::Default for JET_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_HANDLE {}
impl ::core::fmt::Debug for JET_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_HANDLE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_INSTANCE(pub usize);
impl JET_INSTANCE {
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
impl ::core::default::Default for JET_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_INSTANCE {}
impl ::core::fmt::Debug for JET_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_INSTANCE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_INSTANCE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_SESID(pub usize);
impl JET_SESID {
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
impl ::core::default::Default for JET_SESID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_SESID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_SESID {}
impl ::core::fmt::Debug for JET_SESID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_SESID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_SESID {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_TABLEID(pub usize);
impl JET_TABLEID {
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
impl ::core::default::Default for JET_TABLEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_TABLEID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_TABLEID {}
impl ::core::fmt::Debug for JET_TABLEID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_TABLEID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_TABLEID {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

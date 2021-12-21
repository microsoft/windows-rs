#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Shell_Common', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMDLG_FILTERSPEC {
    pub pszName: super::super::super::Foundation::PWSTR,
    pub pszSpec: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMDLG_FILTERSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMDLG_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMDLG_FILTERSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMDLG_FILTERSPEC").field("pszName", &self.pszName).field("pszSpec", &self.pszSpec).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMDLG_FILTERSPEC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMDLG_FILTERSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMDLG_FILTERSPEC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMDLG_FILTERSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub type DEVICE_SCALE_FACTOR = i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = 0i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = 100i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = 120i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = 125i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = 140i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = 150i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = 160i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = 175i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = 180i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = 200i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = 225i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = 250i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = 300i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = 350i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = 400i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = 450i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = 500i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
#[repr(transparent)]
pub struct IObjectArray(::windows::core::IUnknown);
impl IObjectArray {
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn GetAt<T: ::windows::core::Interface>(&self, uiindex: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IObjectArray> for ::windows::core::IUnknown {
    fn from(value: IObjectArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectArray> for ::windows::core::IUnknown {
    fn from(value: &IObjectArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IObjectArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IObjectArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectArray {}
impl ::core::fmt::Debug for IObjectArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectArray").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectArray {
    type Vtable = IObjectArrayVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ca9dcd_5622_4bba_a805_5e9f541bd8c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectArrayVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
#[repr(transparent)]
pub struct IObjectCollection(::windows::core::IUnknown);
impl IObjectCollection {
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn GetAt<T: ::windows::core::Interface>(&self, uiindex: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn AddObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn AddFromArray<'a, Param0: ::windows::core::IntoParam<'a, IObjectArray>>(&self, poasource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), poasource.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn RemoveObjectAt(&self, uiindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IObjectCollection> for IObjectArray {
    fn from(value: IObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectCollection> for IObjectArray {
    fn from(value: &IObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IObjectArray> for IObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IObjectArray> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IObjectArray> for &IObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IObjectArray> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IObjectCollection> for ::windows::core::IUnknown {
    fn from(value: IObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectCollection> for ::windows::core::IUnknown {
    fn from(value: &IObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectCollection {}
impl ::core::fmt::Debug for IObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectCollection {
    type Vtable = IObjectCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5632b1a4_e38a_400a_928a_d4cd63230295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ::core::marker::Copy for ITEMIDLIST {}
impl ::core::clone::Clone for ITEMIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ITEMIDLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ITEMIDLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ITEMIDLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for ITEMIDLIST {}
impl ::core::default::Default for ITEMIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub type PERCEIVED = i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = -3i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = -3i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = -2i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = -1i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = 0i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = 1i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = 2i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = 3i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = 4i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = 5i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = 6i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = 7i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = 8i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = 9i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = 10i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVED_TYPE_LAST: PERCEIVED = 10i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub type SHCOLSTATE = i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = 0i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = 1i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = 2i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = 3i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = 15i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = 16i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_SLOW: SHCOLSTATE = 32i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = 64i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = 128i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = 256i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = 512i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = 1024i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = 2048i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = 65536i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = 131072i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = 262144i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = 4096i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = 8192i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = 16384i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = 61440i32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Shell_Common', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SHELLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SHELLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SHELLDETAILS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SHELLDETAILS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHELLDETAILS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SHELLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl ::core::marker::Copy for SHITEMID {}
impl ::core::clone::Clone for SHITEMID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SHITEMID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHITEMID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHITEMID>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHITEMID {}
impl ::core::default::Default for SHITEMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Shell_Common', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRRET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRRET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STRRET {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STRRET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRRET>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STRRET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STRRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Shell_Common', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union STRRET_0 {
    pub pOleStr: super::super::super::Foundation::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRRET_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRRET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STRRET_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STRRET_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRRET_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STRRET_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STRRET_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub type STRRET_TYPE = i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const STRRET_WSTR: STRRET_TYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const STRRET_OFFSET: STRRET_TYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Shell_Common'*"]
pub const STRRET_CSTR: STRRET_TYPE = 2i32;

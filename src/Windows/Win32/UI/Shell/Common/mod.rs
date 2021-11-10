#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Shell_Common`, `Win32_Foundation`*"]
pub struct COMDLG_FILTERSPEC {
    pub pszName: super::super::super::Foundation::PWSTR,
    pub pszSpec: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl COMDLG_FILTERSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMDLG_FILTERSPEC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMDLG_FILTERSPEC").field("pszName", &self.pszName).field("pszSpec", &self.pszSpec).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMDLG_FILTERSPEC {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszSpec == other.pszSpec
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMDLG_FILTERSPEC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMDLG_FILTERSPEC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICE_SCALE_FACTOR(pub i32);
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(0i32);
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(100i32);
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(120i32);
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(125i32);
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(140i32);
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(150i32);
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(160i32);
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(175i32);
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(180i32);
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(200i32);
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(225i32);
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(250i32);
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(300i32);
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(350i32);
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(400i32);
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(450i32);
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(500i32);
impl ::core::convert::From<i32> for DEVICE_SCALE_FACTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEVICE_SCALE_FACTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectArray(pub ::windows::runtime::IUnknown);
impl IObjectArray {
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn GetAt<T: ::windows::runtime::Interface>(&self, uiindex: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IObjectArray {
    type Vtable = IObjectArray_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92ca9dcd_5622_4bba_a805_5e9f541bd8c9);
}
impl ::core::convert::From<IObjectArray> for ::windows::runtime::IUnknown {
    fn from(value: IObjectArray) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectArray> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectArray) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectArray_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcobjects: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectCollection(pub ::windows::runtime::IUnknown);
impl IObjectCollection {
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn GetAt<T: ::windows::runtime::Interface>(&self, uiindex: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn AddObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn AddFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, IObjectArray>>(&self, poasource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), poasource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn RemoveObjectAt(&self, uiindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Shell_Common`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectCollection {
    type Vtable = IObjectCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5632b1a4_e38a_400a_928a_d4cd63230295);
}
impl ::core::convert::From<IObjectCollection> for ::windows::runtime::IUnknown {
    fn from(value: IObjectCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IObjectArray> for IObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObjectArray> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObjectArray> for &IObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObjectArray> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcobjects: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poasource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ITEMIDLIST {}
impl ::core::default::Default for ITEMIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ITEMIDLIST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ITEMIDLIST {}
unsafe impl ::windows::runtime::Abi for ITEMIDLIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PERCEIVED(pub i32);
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = PERCEIVED(-3i32);
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = PERCEIVED(-3i32);
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = PERCEIVED(-2i32);
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = PERCEIVED(-1i32);
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = PERCEIVED(0i32);
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = PERCEIVED(1i32);
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = PERCEIVED(2i32);
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = PERCEIVED(3i32);
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = PERCEIVED(4i32);
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = PERCEIVED(5i32);
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = PERCEIVED(6i32);
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = PERCEIVED(7i32);
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = PERCEIVED(8i32);
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = PERCEIVED(9i32);
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = PERCEIVED(10i32);
pub const PERCEIVED_TYPE_LAST: PERCEIVED = PERCEIVED(10i32);
impl ::core::convert::From<i32> for PERCEIVED {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PERCEIVED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHCOLSTATE(pub i32);
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = SHCOLSTATE(0i32);
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = SHCOLSTATE(1i32);
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = SHCOLSTATE(2i32);
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = SHCOLSTATE(3i32);
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = SHCOLSTATE(15i32);
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = SHCOLSTATE(16i32);
pub const SHCOLSTATE_SLOW: SHCOLSTATE = SHCOLSTATE(32i32);
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = SHCOLSTATE(64i32);
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = SHCOLSTATE(128i32);
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = SHCOLSTATE(256i32);
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = SHCOLSTATE(512i32);
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = SHCOLSTATE(1024i32);
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = SHCOLSTATE(2048i32);
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = SHCOLSTATE(65536i32);
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = SHCOLSTATE(131072i32);
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = SHCOLSTATE(262144i32);
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = SHCOLSTATE(4096i32);
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = SHCOLSTATE(8192i32);
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = SHCOLSTATE(16384i32);
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = SHCOLSTATE(61440i32);
impl ::core::convert::From<i32> for SHCOLSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHCOLSTATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Shell_Common`, `Win32_Foundation`*"]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
#[cfg(feature = "Win32_Foundation")]
impl SHELLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SHELLDETAILS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SHELLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SHELLDETAILS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl SHITEMID {}
impl ::core::default::Default for SHITEMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHITEMID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SHITEMID {}
unsafe impl ::windows::runtime::Abi for SHITEMID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Shell_Common`, `Win32_Foundation`*"]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
#[cfg(feature = "Win32_Foundation")]
impl STRRET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STRRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STRRET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STRRET {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STRRET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union STRRET_0 {
    pub pOleStr: super::super::super::Foundation::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl STRRET_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STRRET_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STRRET_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STRRET_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STRRET_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct STRRET_TYPE(pub i32);
pub const STRRET_WSTR: STRRET_TYPE = STRRET_TYPE(0i32);
pub const STRRET_OFFSET: STRRET_TYPE = STRRET_TYPE(1i32);
pub const STRRET_CSTR: STRRET_TYPE = STRRET_TYPE(2i32);
impl ::core::convert::From<i32> for STRRET_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STRRET_TYPE {
    type Abi = Self;
}

#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
#[repr(transparent)]
pub struct IObjectArray(::windows::core::IUnknown);
impl IObjectArray {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, uiindex: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), uiindex, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IObjectArray, ::windows::core::IUnknown);
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
    type Vtable = IObjectArray_Vtbl;
}
impl ::core::clone::Clone for IObjectArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IObjectArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ca9dcd_5622_4bba_a805_5e9f541bd8c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectArray_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
#[repr(transparent)]
pub struct IObjectCollection(::windows::core::IUnknown);
impl IObjectCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, uiindex: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetAt)(::windows::core::Interface::as_raw(self), uiindex, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddObject<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).AddObject)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn AddFromArray<P0>(&self, poasource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IObjectArray>,
    {
        (::windows::core::Interface::vtable(self).AddFromArray)(::windows::core::Interface::as_raw(self), poasource.into_param().abi()).ok()
    }
    pub unsafe fn RemoveObjectAt(&self, uiindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveObjectAt)(::windows::core::Interface::as_raw(self), uiindex).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IObjectCollection, ::windows::core::IUnknown, IObjectArray);
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
    type Vtable = IObjectCollection_Vtbl;
}
impl ::core::clone::Clone for IObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IObjectCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5632b1a4_e38a_400a_928a_d4cd63230295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectCollection_Vtbl {
    pub base__: IObjectArray_Vtbl,
    pub AddObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poasource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveObjectAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICE_SCALE_FACTOR(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(100i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(120i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(125i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(140i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(150i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(160i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(175i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(180i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(200i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(225i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(250i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(300i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(350i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(400i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(450i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(500i32);
impl ::core::marker::Copy for DEVICE_SCALE_FACTOR {}
impl ::core::clone::Clone for DEVICE_SCALE_FACTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_SCALE_FACTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEVICE_SCALE_FACTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEVICE_SCALE_FACTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_SCALE_FACTOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PERCEIVED(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = PERCEIVED(-3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = PERCEIVED(-3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = PERCEIVED(-2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = PERCEIVED(-1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = PERCEIVED(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = PERCEIVED(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = PERCEIVED(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = PERCEIVED(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = PERCEIVED(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = PERCEIVED(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = PERCEIVED(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = PERCEIVED(7i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = PERCEIVED(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = PERCEIVED(9i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = PERCEIVED(10i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_LAST: PERCEIVED = PERCEIVED(10i32);
impl ::core::marker::Copy for PERCEIVED {}
impl ::core::clone::Clone for PERCEIVED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PERCEIVED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PERCEIVED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PERCEIVED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERCEIVED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHCOLSTATE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = SHCOLSTATE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = SHCOLSTATE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = SHCOLSTATE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = SHCOLSTATE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = SHCOLSTATE(15i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = SHCOLSTATE(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_SLOW: SHCOLSTATE = SHCOLSTATE(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = SHCOLSTATE(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = SHCOLSTATE(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = SHCOLSTATE(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = SHCOLSTATE(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = SHCOLSTATE(1024i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = SHCOLSTATE(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = SHCOLSTATE(65536i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = SHCOLSTATE(131072i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = SHCOLSTATE(262144i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = SHCOLSTATE(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = SHCOLSTATE(8192i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = SHCOLSTATE(16384i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = SHCOLSTATE(61440i32);
impl ::core::marker::Copy for SHCOLSTATE {}
impl ::core::clone::Clone for SHCOLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHCOLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SHCOLSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SHCOLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHCOLSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STRRET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const STRRET_WSTR: STRRET_TYPE = STRRET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const STRRET_OFFSET: STRRET_TYPE = STRRET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const STRRET_CSTR: STRRET_TYPE = STRRET_TYPE(2i32);
impl ::core::marker::Copy for STRRET_TYPE {}
impl ::core::clone::Clone for STRRET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STRRET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STRRET_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STRRET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRRET_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct COMDLG_FILTERSPEC {
    pub pszName: ::windows::core::PCWSTR,
    pub pszSpec: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for COMDLG_FILTERSPEC {}
impl ::core::clone::Clone for COMDLG_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMDLG_FILTERSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMDLG_FILTERSPEC").field("pszName", &self.pszName).field("pszSpec", &self.pszSpec).finish()
    }
}
impl ::windows::core::TypeKind for COMDLG_FILTERSPEC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COMDLG_FILTERSPEC {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszSpec == other.pszSpec
    }
}
impl ::core::cmp::Eq for COMDLG_FILTERSPEC {}
impl ::core::default::Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ::core::marker::Copy for ITEMIDLIST {}
impl ::core::clone::Clone for ITEMIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ITEMIDLIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ITEMIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
impl ::core::marker::Copy for SHELLDETAILS {}
impl ::core::clone::Clone for SHELLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SHELLDETAILS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
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
impl ::windows::core::TypeKind for SHITEMID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SHITEMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
impl ::core::marker::Copy for STRRET {}
impl ::core::clone::Clone for STRRET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for STRRET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for STRRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub union STRRET_0 {
    pub pOleStr: ::windows::core::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
impl ::core::marker::Copy for STRRET_0 {}
impl ::core::clone::Clone for STRRET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for STRRET_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for STRRET_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

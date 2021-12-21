#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const AUTO_WIDTH: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct AppEvents(::windows::core::IUnknown);
impl AppEvents {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<AppEvents> for super::Com::IDispatch {
    fn from(value: AppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&AppEvents> for super::Com::IDispatch {
    fn from(value: &AppEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppEvents> for ::windows::core::IUnknown {
    fn from(value: AppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppEvents> for ::windows::core::IUnknown {
    fn from(value: &AppEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AppEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppEvents {}
impl ::core::fmt::Debug for AppEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AppEvents {
    type Vtable = AppEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc7a4252_78ac_4532_8c5a_563cfe138863);
}
#[repr(C)]
#[doc(hidden)]
pub struct AppEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
pub const AppEventsDHTMLConnector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xade6444b_c91f_4e37_92a4_5bb430a33340);
pub const Application: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b2791a_b1ae_4c90_9b8e_e860ba07f889);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type CCM_COMMANDID_MASK_CONSTANTS = u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS = 4294901760u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type CCM_INSERTIONALLOWED = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = 8i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type CCM_INSERTIONPOINTID = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: CCM_INSERTIONPOINTID = -65536i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_SHARED: CCM_INSERTIONPOINTID = -2147483648i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: CCM_INSERTIONPOINTID = 1073741824i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: CCM_INSERTIONPOINTID = 536870912i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: CCM_INSERTIONPOINTID = 268435456i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: CCM_INSERTIONPOINTID = 268369920i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: CCM_INSERTIONPOINTID = 31i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: CCM_INSERTIONPOINTID = -1610612736i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: CCM_INSERTIONPOINTID = -1610612735i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: CCM_INSERTIONPOINTID = -1610612734i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: CCM_INSERTIONPOINTID = -1610612733i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: CCM_INSERTIONPOINTID = -1610612732i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: CCM_INSERTIONPOINTID = -1879048191i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: CCM_INSERTIONPOINTID = -1879048190i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_INSERTIONPOINTID_ROOT_MENU: CCM_INSERTIONPOINTID = -2147483648i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type CCM_SPECIAL = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_SPECIAL_SEPARATOR: CCM_SPECIAL = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_SPECIAL_SUBMENU: CCM_SPECIAL = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_SPECIAL_DEFAULT_ITEM: CCM_SPECIAL = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_SPECIAL_INSERTION_POINT: CCM_SPECIAL = 8i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCM_SPECIAL_TESTONLY: CCM_SPECIAL = 16i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM {
    pub strName: super::super::Foundation::PWSTR,
    pub strStatusBarText: super::super::Foundation::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONTEXTMENUITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONTEXTMENUITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONTEXTMENUITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTEXTMENUITEM").field("strName", &self.strName).field("strStatusBarText", &self.strStatusBarText).field("lCommandID", &self.lCommandID).field("lInsertionPointID", &self.lInsertionPointID).field("fFlags", &self.fFlags).field("fSpecialFlags", &self.fSpecialFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONTEXTMENUITEM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONTEXTMENUITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTEXTMENUITEM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONTEXTMENUITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONTEXTMENUITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM2 {
    pub strName: super::super::Foundation::PWSTR,
    pub strStatusBarText: super::super::Foundation::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONTEXTMENUITEM2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONTEXTMENUITEM2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONTEXTMENUITEM2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTEXTMENUITEM2").field("strName", &self.strName).field("strStatusBarText", &self.strStatusBarText).field("lCommandID", &self.lCommandID).field("lInsertionPointID", &self.lInsertionPointID).field("fFlags", &self.fFlags).field("fSpecialFlags", &self.fSpecialFlags).field("strLanguageIndependentName", &self.strLanguageIndependentName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONTEXTMENUITEM2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONTEXTMENUITEM2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTEXTMENUITEM2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONTEXTMENUITEM2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONTEXTMENUITEM2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Column(::windows::core::IUnknown);
impl Column {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(width)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DisplayPosition(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetDisplayPosition(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hidden(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHidden<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hidden: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), hidden.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetAsSortColumn(&self, sortorder: _ColumnSortOrder) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(sortorder)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSortColumn(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Column> for super::Com::IDispatch {
    fn from(value: Column) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Column> for super::Com::IDispatch {
    fn from(value: &Column) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Column {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Column {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Column> for ::windows::core::IUnknown {
    fn from(value: Column) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Column> for ::windows::core::IUnknown {
    fn from(value: &Column) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Column {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Column {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Column {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Column {}
impl ::core::fmt::Debug for Column {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Column").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Column {
    type Vtable = ColumnVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1c5f63_2b16_4d06_9ab3_f45350b940ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ColumnVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Columns(::windows::core::IUnknown);
impl Columns {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<Column> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<Column>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Columns> for super::Com::IDispatch {
    fn from(value: Columns) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Columns> for super::Com::IDispatch {
    fn from(value: &Columns) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Columns {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Columns {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Columns> for ::windows::core::IUnknown {
    fn from(value: Columns) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Columns> for ::windows::core::IUnknown {
    fn from(value: &Columns) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Columns {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Columns {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Columns {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Columns {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Columns {}
impl ::core::fmt::Debug for Columns {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Columns").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Columns {
    type Vtable = ColumnsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x383d4d97_fc44_478b_b139_6323dc48611c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ColumnsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, column: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
pub const ConsolePower: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0285374_dff1_11d3_b433_00c04f8ecd78);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ContextMenu(::windows::core::IUnknown);
impl ContextMenu {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, indexorpath: Param0) -> ::windows::core::Result<MenuItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), indexorpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<MenuItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextMenu> for super::Com::IDispatch {
    fn from(value: ContextMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextMenu> for super::Com::IDispatch {
    fn from(value: &ContextMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ContextMenu {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ContextMenu {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContextMenu> for ::windows::core::IUnknown {
    fn from(value: ContextMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContextMenu> for ::windows::core::IUnknown {
    fn from(value: &ContextMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContextMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContextMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ContextMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContextMenu {}
impl ::core::fmt::Debug for ContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ContextMenu {
    type Vtable = ContextMenuVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdab39ce0_25e6_4e07_8362_ba9c95706545);
}
#[repr(C)]
#[doc(hidden)]
pub struct ContextMenuVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexorpath: ::core::mem::ManuallyDrop<super::Com::VARIANT>, menuitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type DATA_OBJECT_TYPES = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCT_SCOPE: DATA_OBJECT_TYPES = 32768i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCT_RESULT: DATA_OBJECT_TYPES = 32769i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCT_SNAPIN_MANAGER: DATA_OBJECT_TYPES = 32770i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CCT_UNINITIALIZED: DATA_OBJECT_TYPES = 65535i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Document(::windows::core::IUnknown);
impl Document {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, savechanges: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), savechanges.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Views(&self) -> ::windows::core::Result<Views> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Views>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SnapIns(&self) -> ::windows::core::Result<SnapIns> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<SnapIns>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ActiveView(&self) -> ::windows::core::Result<View> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<View>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Location(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSaved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Mode(&self) -> ::windows::core::Result<_DocumentMode> {
        let mut result__: _DocumentMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<_DocumentMode>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetMode(&self, mode: _DocumentMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn RootNode(&self) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ScopeNamespace(&self) -> ::windows::core::Result<ScopeNamespace> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ScopeNamespace>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn CreateProperties(&self) -> ::windows::core::Result<Properties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Properties>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Application(&self) -> ::windows::core::Result<_Application> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<_Application>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Document> for super::Com::IDispatch {
    fn from(value: Document) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Document> for super::Com::IDispatch {
    fn from(value: &Document) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Document {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Document {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Document> for ::windows::core::IUnknown {
    fn from(value: Document) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Document> for ::windows::core::IUnknown {
    fn from(value: &Document) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Document {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Document {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Document {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Document {}
impl ::core::fmt::Debug for Document {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Document").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Document {
    type Vtable = DocumentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x225120d6_1e0f_40a3_93fe_1079e6a8017b);
}
#[repr(C)]
#[doc(hidden)]
pub struct DocumentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, views: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapins: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Extension(::windows::core::IUnknown);
impl Extension {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Vendor(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Version(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Extensions(&self) -> ::windows::core::Result<Extensions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Extensions>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SnapinCLSID(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAllExtensions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, enable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), enable.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, enable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), enable.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Extension> for super::Com::IDispatch {
    fn from(value: Extension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Extension> for super::Com::IDispatch {
    fn from(value: &Extension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Extension {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Extension {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Extension> for ::windows::core::IUnknown {
    fn from(value: Extension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Extension> for ::windows::core::IUnknown {
    fn from(value: &Extension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Extension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Extension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Extension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Extension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Extension {}
impl ::core::fmt::Debug for Extension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Extension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Extension {
    type Vtable = ExtensionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4d6ca6_912f_409b_a26e_7fd234aef542);
}
#[repr(C)]
#[doc(hidden)]
pub struct ExtensionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Extensions(::windows::core::IUnknown);
impl Extensions {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<Extension> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<Extension>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Extensions> for super::Com::IDispatch {
    fn from(value: Extensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Extensions> for super::Com::IDispatch {
    fn from(value: &Extensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Extensions {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Extensions {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Extensions> for ::windows::core::IUnknown {
    fn from(value: Extensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Extensions> for ::windows::core::IUnknown {
    fn from(value: &Extensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Extensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Extensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Extensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Extensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Extensions {}
impl ::core::fmt::Debug for Extensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Extensions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Extensions {
    type Vtable = ExtensionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82dbea43_8ca4_44bc_a2ca_d18741059ec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ExtensionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, extension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Frame(::windows::core::IUnknown);
impl Frame {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Maximize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Minimize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Top(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Bottom(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetBottom(&self, bottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Left(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Right(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetRight(&self, right: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Frame> for super::Com::IDispatch {
    fn from(value: Frame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Frame> for super::Com::IDispatch {
    fn from(value: &Frame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Frame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Frame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Frame> for ::windows::core::IUnknown {
    fn from(value: Frame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Frame> for ::windows::core::IUnknown {
    fn from(value: &Frame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Frame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Frame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Frame {}
impl ::core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Frame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Frame {
    type Vtable = FrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5e2d970_5bb3_4306_8804_b0968a31c8e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct FrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const HDI_HIDDEN: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const HIDE_COLUMN: i32 = -4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IColumnData(::windows::core::IUnknown);
impl IColumnData {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetColumnConfigData(&self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolid), ::core::mem::transmute(pcolsetdata)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetColumnConfigData(&self, pcolid: *const SColumnSetID) -> ::windows::core::Result<*mut MMC_COLUMN_SET_DATA> {
        let mut result__: *mut MMC_COLUMN_SET_DATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolid), ::core::mem::transmute(&mut result__)).from_abi::<*mut MMC_COLUMN_SET_DATA>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetColumnSortData(&self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolid), ::core::mem::transmute(pcolsortdata)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetColumnSortData(&self, pcolid: *const SColumnSetID) -> ::windows::core::Result<*mut MMC_SORT_SET_DATA> {
        let mut result__: *mut MMC_SORT_SET_DATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolid), ::core::mem::transmute(&mut result__)).from_abi::<*mut MMC_SORT_SET_DATA>(result__)
    }
}
impl ::core::convert::From<IColumnData> for ::windows::core::IUnknown {
    fn from(value: IColumnData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IColumnData> for ::windows::core::IUnknown {
    fn from(value: &IColumnData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IColumnData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IColumnData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IColumnData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IColumnData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnData {}
impl ::core::fmt::Debug for IColumnData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IColumnData {
    type Vtable = IColumnDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x547c1354_024d_11d3_a707_00c04f8ef4cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IComponent(::windows::core::IUnknown);
impl IComponent {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IConsole>>(&self, lpconsole: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpconsole.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, event: MMC_NOTIFY_TYPE, arg: Param2, param3: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), ::core::mem::transmute(event), arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Destroy(&self, cookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(ppviewtype), ::core::mem::transmute(pviewoptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(presultdataitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpdataobjecta: Param0, lpdataobjectb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComponent> for ::windows::core::IUnknown {
    fn from(value: IComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponent> for ::windows::core::IUnknown {
    fn from(value: &IComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponent {}
impl ::core::fmt::Debug for IComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponent {
    type Vtable = IComponentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb2_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpconsole: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IComponent2(::windows::core::IUnknown);
impl IComponent2 {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IConsole>>(&self, lpconsole: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpconsole.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, event: MMC_NOTIFY_TYPE, arg: Param2, param3: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), ::core::mem::transmute(event), arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Destroy(&self, cookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(ppviewtype), ::core::mem::transmute(pviewoptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(presultdataitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpdataobjecta: Param0, lpdataobjectb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultViewType2(&self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(presultviewtype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreResultView(&self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(presultviewtype)).ok()
    }
}
impl ::core::convert::From<IComponent2> for IComponent {
    fn from(value: IComponent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponent2> for IComponent {
    fn from(value: &IComponent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IComponent> for IComponent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IComponent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IComponent> for &IComponent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IComponent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IComponent2> for ::windows::core::IUnknown {
    fn from(value: IComponent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponent2> for ::windows::core::IUnknown {
    fn from(value: &IComponent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IComponent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponent2 {}
impl ::core::fmt::Debug for IComponent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponent2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponent2 {
    type Vtable = IComponent2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79a2d615_4a10_4ed4_8c65_8633f9335095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponent2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpconsole: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IComponentData(::windows::core::IUnknown);
impl IComponentData {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punknown: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn CreateComponent(&self) -> ::windows::core::Result<IComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, event: MMC_NOTIFY_TYPE, arg: Param2, param3: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), ::core::mem::transmute(event), arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pscopedataitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpdataobjecta: Param0, lpdataobjectb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComponentData> for ::windows::core::IUnknown {
    fn from(value: IComponentData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentData> for ::windows::core::IUnknown {
    fn from(value: &IComponentData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IComponentData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponentData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponentData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentData {}
impl ::core::fmt::Debug for IComponentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponentData {
    type Vtable = IComponentDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x955ab28a_5218_11d0_a985_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IComponentData2(::windows::core::IUnknown);
impl IComponentData2 {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punknown: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn CreateComponent(&self) -> ::windows::core::Result<IComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, event: MMC_NOTIFY_TYPE, arg: Param2, param3: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), ::core::mem::transmute(event), arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pscopedataitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpdataobjecta: Param0, lpdataobjectb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
}
impl ::core::convert::From<IComponentData2> for IComponentData {
    fn from(value: IComponentData2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentData2> for IComponentData {
    fn from(value: &IComponentData2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IComponentData> for IComponentData2 {
    fn into_param(self) -> ::windows::core::Param<'a, IComponentData> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IComponentData> for &IComponentData2 {
    fn into_param(self) -> ::windows::core::Param<'a, IComponentData> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IComponentData2> for ::windows::core::IUnknown {
    fn from(value: IComponentData2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentData2> for ::windows::core::IUnknown {
    fn from(value: &IComponentData2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentData2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IComponentData2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponentData2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponentData2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentData2 {}
impl ::core::fmt::Debug for IComponentData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentData2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponentData2 {
    type Vtable = IComponentData2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcca0f2d2_82de_41b5_bf47_3b2076273d5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentData2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsole(::windows::core::IUnknown);
impl IConsole {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetHeader<'a, Param0: ::windows::core::IntoParam<'a, IHeaderCtrl>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetToolbar<'a, Param0: ::windows::core::IntoParam<'a, IToolbar>>(&self, ptoolbar: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ptoolbar.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryResultView(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IImageList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryResultImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IImageList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, data: Param1, hint: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), data.into_param().abi(), ::core::mem::transmute(hint)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBox<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpsztext: Param0, lpsztitle: Param1, fustyle: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpsztext.into_param().abi(), lpsztitle.into_param().abi(), ::core::mem::transmute(fustyle), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::core::Result<IConsoleVerb> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IConsoleVerb>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem), ::core::mem::transmute(loptions)).ok()
    }
}
impl ::core::convert::From<IConsole> for ::windows::core::IUnknown {
    fn from(value: IConsole) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsole> for ::windows::core::IUnknown {
    fn from(value: &IConsole) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsole {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsole {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsole {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsole {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsole {}
impl ::core::fmt::Debug for IConsole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsole").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsole {
    type Vtable = IConsoleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb1_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsole2(::windows::core::IUnknown);
impl IConsole2 {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetHeader<'a, Param0: ::windows::core::IntoParam<'a, IHeaderCtrl>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetToolbar<'a, Param0: ::windows::core::IntoParam<'a, IToolbar>>(&self, ptoolbar: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ptoolbar.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryResultView(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IImageList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryResultImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IImageList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, data: Param1, hint: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), data.into_param().abi(), ::core::mem::transmute(hint)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBox<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpsztext: Param0, lpsztitle: Param1, fustyle: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpsztext.into_param().abi(), lpsztitle.into_param().abi(), ::core::mem::transmute(fustyle), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::core::Result<IConsoleVerb> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IConsoleVerb>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem), ::core::mem::transmute(loptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Expand<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hitem: isize, bexpand: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem), bexpand.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn IsTaskpadViewPreferred(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstatustext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pszstatustext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IConsole2> for IConsole {
    fn from(value: IConsole2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsole2> for IConsole {
    fn from(value: &IConsole2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsole> for IConsole2 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsole> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsole> for &IConsole2 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsole> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IConsole2> for ::windows::core::IUnknown {
    fn from(value: IConsole2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsole2> for ::windows::core::IUnknown {
    fn from(value: &IConsole2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsole2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsole2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsole2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsole2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsole2 {}
impl ::core::fmt::Debug for IConsole2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsole2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsole2 {
    type Vtable = IConsole2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x103d842a_aa63_11d1_a7e1_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsole3(::windows::core::IUnknown);
impl IConsole3 {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetHeader<'a, Param0: ::windows::core::IntoParam<'a, IHeaderCtrl>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetToolbar<'a, Param0: ::windows::core::IntoParam<'a, IToolbar>>(&self, ptoolbar: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ptoolbar.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryResultView(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IImageList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryResultImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IImageList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lpdataobject: Param0, data: Param1, hint: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi(), data.into_param().abi(), ::core::mem::transmute(hint)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBox<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpsztext: Param0, lpsztitle: Param1, fustyle: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpsztext.into_param().abi(), lpsztitle.into_param().abi(), ::core::mem::transmute(fustyle), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::core::Result<IConsoleVerb> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IConsoleVerb>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem), ::core::mem::transmute(loptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Expand<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hitem: isize, bexpand: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem), bexpand.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn IsTaskpadViewPreferred(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstatustext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pszstatustext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn RenameScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(hscopeitem)).ok()
    }
}
impl ::core::convert::From<IConsole3> for IConsole2 {
    fn from(value: IConsole3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsole3> for IConsole2 {
    fn from(value: &IConsole3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsole2> for IConsole3 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsole2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsole2> for &IConsole3 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsole2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IConsole3> for IConsole {
    fn from(value: IConsole3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsole3> for IConsole {
    fn from(value: &IConsole3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsole> for IConsole3 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsole> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsole> for &IConsole3 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsole> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IConsole3> for ::windows::core::IUnknown {
    fn from(value: IConsole3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsole3> for ::windows::core::IUnknown {
    fn from(value: &IConsole3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsole3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsole3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsole3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsole3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsole3 {}
impl ::core::fmt::Debug for IConsole3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsole3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsole3 {
    type Vtable = IConsole3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f85efdb_d0e1_498c_8d4a_d010dfdd404f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsoleNameSpace(::windows::core::IUnknown);
impl IConsoleNameSpace {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem), ::core::mem::transmute(fdeletethis)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), ::core::mem::transmute(pitemchild), ::core::mem::transmute(pcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), ::core::mem::transmute(pitemnext), ::core::mem::transmute(pcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), ::core::mem::transmute(pitemparent), ::core::mem::transmute(pcookie)).ok()
    }
}
impl ::core::convert::From<IConsoleNameSpace> for ::windows::core::IUnknown {
    fn from(value: IConsoleNameSpace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsoleNameSpace> for ::windows::core::IUnknown {
    fn from(value: &IConsoleNameSpace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsoleNameSpace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsoleNameSpace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsoleNameSpace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsoleNameSpace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsoleNameSpace {}
impl ::core::fmt::Debug for IConsoleNameSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsoleNameSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsoleNameSpace {
    type Vtable = IConsoleNameSpaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbedeb620_f24d_11cf_8afc_00aa003ca9f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsoleNameSpace2(::windows::core::IUnknown);
impl IConsoleNameSpace2 {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem), ::core::mem::transmute(fdeletethis)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), ::core::mem::transmute(pitemchild), ::core::mem::transmute(pcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), ::core::mem::transmute(pitemnext), ::core::mem::transmute(pcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), ::core::mem::transmute(pitemparent), ::core::mem::transmute(pcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Expand(&self, hitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn AddExtension(&self, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem), ::core::mem::transmute(lpclsid)).ok()
    }
}
impl ::core::convert::From<IConsoleNameSpace2> for IConsoleNameSpace {
    fn from(value: IConsoleNameSpace2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsoleNameSpace2> for IConsoleNameSpace {
    fn from(value: &IConsoleNameSpace2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsoleNameSpace> for IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsoleNameSpace> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IConsoleNameSpace> for &IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::core::Param<'a, IConsoleNameSpace> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IConsoleNameSpace2> for ::windows::core::IUnknown {
    fn from(value: IConsoleNameSpace2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsoleNameSpace2> for ::windows::core::IUnknown {
    fn from(value: &IConsoleNameSpace2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsoleNameSpace2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsoleNameSpace2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsoleNameSpace2 {}
impl ::core::fmt::Debug for IConsoleNameSpace2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsoleNameSpace2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsoleNameSpace2 {
    type Vtable = IConsoleNameSpace2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x255f18cc_65db_11d1_a7dc_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpace2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsolePower(::windows::core::IUnknown);
impl IConsolePower {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetExecutionState(&self, dwadd: u32, dwremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwadd), ::core::mem::transmute(dwremove)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ResetIdleTimer(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IConsolePower> for ::windows::core::IUnknown {
    fn from(value: IConsolePower) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsolePower> for ::windows::core::IUnknown {
    fn from(value: &IConsolePower) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsolePower {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsolePower {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsolePower {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsolePower {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsolePower {}
impl ::core::fmt::Debug for IConsolePower {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsolePower").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsolePower {
    type Vtable = IConsolePowerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cfbdd0e_62ca_49ce_a3af_dbb2de61b068);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsolePowerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsolePowerSink(::windows::core::IUnknown);
impl IConsolePowerSink {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPowerBroadcast<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, nevent: u32, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::LRESULT> {
        let mut result__: super::super::Foundation::LRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(nevent), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LRESULT>(result__)
    }
}
impl ::core::convert::From<IConsolePowerSink> for ::windows::core::IUnknown {
    fn from(value: IConsolePowerSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsolePowerSink> for ::windows::core::IUnknown {
    fn from(value: &IConsolePowerSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsolePowerSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsolePowerSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsolePowerSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsolePowerSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsolePowerSink {}
impl ::core::fmt::Debug for IConsolePowerSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsolePowerSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsolePowerSink {
    type Vtable = IConsolePowerSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3333759f_fe4f_4975_b143_fec0a5dd6d65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsolePowerSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IConsoleVerb(::windows::core::IUnknown);
impl IConsoleVerb {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecmdid), ::core::mem::transmute(nstate), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVerbState<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecmdid), ::core::mem::transmute(nstate), bstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetDefaultVerb(&self, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecmdid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetDefaultVerb(&self) -> ::windows::core::Result<MMC_CONSOLE_VERB> {
        let mut result__: MMC_CONSOLE_VERB = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MMC_CONSOLE_VERB>(result__)
    }
}
impl ::core::convert::From<IConsoleVerb> for ::windows::core::IUnknown {
    fn from(value: IConsoleVerb) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConsoleVerb> for ::windows::core::IUnknown {
    fn from(value: &IConsoleVerb) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConsoleVerb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConsoleVerb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConsoleVerb {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConsoleVerb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsoleVerb {}
impl ::core::fmt::Debug for IConsoleVerb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsoleVerb").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsoleVerb {
    type Vtable = IConsoleVerbVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49f7a60_74af_11d0_a286_00c04fd8fe93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleVerbVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IContextMenuCallback(::windows::core::IUnknown);
impl IContextMenuCallback {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pitem)).ok()
    }
}
impl ::core::convert::From<IContextMenuCallback> for ::windows::core::IUnknown {
    fn from(value: IContextMenuCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextMenuCallback> for ::windows::core::IUnknown {
    fn from(value: &IContextMenuCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContextMenuCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContextMenuCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextMenuCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextMenuCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuCallback {}
impl ::core::fmt::Debug for IContextMenuCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextMenuCallback {
    type Vtable = IContextMenuCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb7_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IContextMenuCallback2(::windows::core::IUnknown);
impl IContextMenuCallback2 {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pitem)).ok()
    }
}
impl ::core::convert::From<IContextMenuCallback2> for ::windows::core::IUnknown {
    fn from(value: IContextMenuCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextMenuCallback2> for ::windows::core::IUnknown {
    fn from(value: &IContextMenuCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContextMenuCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContextMenuCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextMenuCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextMenuCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuCallback2 {}
impl ::core::fmt::Debug for IContextMenuCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextMenuCallback2 {
    type Vtable = IContextMenuCallback2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe178bc0e_2ed0_4b5e_8097_42c9087e8b33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuCallback2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IContextMenuProvider(::windows::core::IUnknown);
impl IContextMenuProvider {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pitem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn EmptyMenuList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPrimaryExtensionItems<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, piextension: Param0, pidataobject: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), piextension.into_param().abi(), pidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddThirdPartyExtensionItems<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, pidataobject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowContextMenu<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, xpos: i32, ypos: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(xpos), ::core::mem::transmute(ypos), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IContextMenuProvider> for IContextMenuCallback {
    fn from(value: IContextMenuProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextMenuProvider> for IContextMenuCallback {
    fn from(value: &IContextMenuProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContextMenuCallback> for IContextMenuProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IContextMenuCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContextMenuCallback> for &IContextMenuProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IContextMenuCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContextMenuProvider> for ::windows::core::IUnknown {
    fn from(value: IContextMenuProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextMenuProvider> for ::windows::core::IUnknown {
    fn from(value: &IContextMenuProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContextMenuProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContextMenuProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextMenuProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextMenuProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuProvider {}
impl ::core::fmt::Debug for IContextMenuProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextMenuProvider {
    type Vtable = IContextMenuProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb6_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IControlbar(::windows::core::IUnknown);
impl IControlbar {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Create<'a, Param1: ::windows::core::IntoParam<'a, IExtendControlbar>>(&self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: Param1) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntype), pextendcontrolbar.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Attach<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, ntype: MMC_CONTROL_TYPE, lpunknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntype), lpunknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Detach<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, lpunknown: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lpunknown.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IControlbar> for ::windows::core::IUnknown {
    fn from(value: IControlbar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlbar> for ::windows::core::IUnknown {
    fn from(value: &IControlbar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlbar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IControlbar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IControlbar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IControlbar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlbar {}
impl ::core::fmt::Debug for IControlbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlbar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IControlbar {
    type Vtable = IControlbarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69fb811e_6c1c_11d0_a2cb_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlbarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::windows::core::RawPtr, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IDisplayHelp(::windows::core::IUnknown);
impl IDisplayHelp {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowTopic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszhelptopic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszhelptopic.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDisplayHelp> for ::windows::core::IUnknown {
    fn from(value: IDisplayHelp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDisplayHelp> for ::windows::core::IUnknown {
    fn from(value: &IDisplayHelp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDisplayHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDisplayHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDisplayHelp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayHelp {}
impl ::core::fmt::Debug for IDisplayHelp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayHelp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDisplayHelp {
    type Vtable = IDisplayHelpVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc593830_b926_11d1_8063_0000f875a9ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayHelpVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelptopic: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IEnumTASK(::windows::core::IUnknown);
impl IEnumTASK {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTASK> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTASK>(result__)
    }
}
impl ::core::convert::From<IEnumTASK> for ::windows::core::IUnknown {
    fn from(value: IEnumTASK) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTASK> for ::windows::core::IUnknown {
    fn from(value: &IEnumTASK) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTASK {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTASK {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTASK {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTASK {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTASK {}
impl ::core::fmt::Debug for IEnumTASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTASK").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTASK {
    type Vtable = IEnumTASKVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338698b1_5a02_11d1_9fec_00600832db4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTASKVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IExtendContextMenu(::windows::core::IUnknown);
impl IExtendContextMenu {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddMenuItems<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, IContextMenuCallback>>(&self, pidataobject: Param0, picallback: Param1, pinsertionallowed: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pidataobject.into_param().abi(), picallback.into_param().abi(), ::core::mem::transmute(pinsertionallowed)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Command<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lcommandid: i32, pidataobject: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcommandid), pidataobject.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IExtendContextMenu> for ::windows::core::IUnknown {
    fn from(value: IExtendContextMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendContextMenu> for ::windows::core::IUnknown {
    fn from(value: &IExtendContextMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExtendContextMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExtendContextMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExtendContextMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExtendContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendContextMenu {}
impl ::core::fmt::Debug for IExtendContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendContextMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendContextMenu {
    type Vtable = IExtendContextMenuVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f3b7a4f_cfac_11cf_b8e3_00c04fd8d5b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendContextMenuVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr, picallback: ::windows::core::RawPtr, pinsertionallowed: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IExtendControlbar(::windows::core::IUnknown);
impl IExtendControlbar {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetControlbar<'a, Param0: ::windows::core::IntoParam<'a, IControlbar>>(&self, pcontrolbar: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcontrolbar.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ControlbarNotify<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, event: MMC_NOTIFY_TYPE, arg: Param1, param2: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), arg.into_param().abi(), param2.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IExtendControlbar> for ::windows::core::IUnknown {
    fn from(value: IExtendControlbar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendControlbar> for ::windows::core::IUnknown {
    fn from(value: &IExtendControlbar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExtendControlbar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExtendControlbar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExtendControlbar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExtendControlbar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendControlbar {}
impl ::core::fmt::Debug for IExtendControlbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendControlbar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendControlbar {
    type Vtable = IExtendControlbarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49506520_6f40_11d0_a98b_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendControlbarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontrolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IExtendPropertySheet(::windows::core::IUnknown);
impl IExtendPropertySheet {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<'a, Param0: ::windows::core::IntoParam<'a, IPropertySheetCallback>, Param2: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpprovider: Param0, handle: isize, lpidataobject: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpprovider.into_param().abi(), ::core::mem::transmute(handle), lpidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpdataobject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IExtendPropertySheet> for ::windows::core::IUnknown {
    fn from(value: IExtendPropertySheet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendPropertySheet> for ::windows::core::IUnknown {
    fn from(value: &IExtendPropertySheet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExtendPropertySheet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExtendPropertySheet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExtendPropertySheet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExtendPropertySheet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendPropertySheet {}
impl ::core::fmt::Debug for IExtendPropertySheet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendPropertySheet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendPropertySheet {
    type Vtable = IExtendPropertySheetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85de64dc_ef21_11cf_a285_00c04fd8dbe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpprovider: ::windows::core::RawPtr, handle: isize, lpidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IExtendPropertySheet2(::windows::core::IUnknown);
impl IExtendPropertySheet2 {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<'a, Param0: ::windows::core::IntoParam<'a, IPropertySheetCallback>, Param2: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpprovider: Param0, handle: isize, lpidataobject: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpprovider.into_param().abi(), ::core::mem::transmute(handle), lpidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpdataobject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), lpdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetWatermarks<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, lpidataobject: Param0, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lpidataobject.into_param().abi(), ::core::mem::transmute(lphwatermark), ::core::mem::transmute(lphheader), ::core::mem::transmute(lphpalette), ::core::mem::transmute(bstretch)).ok()
    }
}
impl ::core::convert::From<IExtendPropertySheet2> for IExtendPropertySheet {
    fn from(value: IExtendPropertySheet2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendPropertySheet2> for IExtendPropertySheet {
    fn from(value: &IExtendPropertySheet2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IExtendPropertySheet> for IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::core::Param<'a, IExtendPropertySheet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IExtendPropertySheet> for &IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::core::Param<'a, IExtendPropertySheet> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IExtendPropertySheet2> for ::windows::core::IUnknown {
    fn from(value: IExtendPropertySheet2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendPropertySheet2> for ::windows::core::IUnknown {
    fn from(value: &IExtendPropertySheet2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExtendPropertySheet2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExtendPropertySheet2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendPropertySheet2 {}
impl ::core::fmt::Debug for IExtendPropertySheet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendPropertySheet2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendPropertySheet2 {
    type Vtable = IExtendPropertySheet2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7a87232_4a51_11d1_a7ea_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheet2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpprovider: ::windows::core::RawPtr, handle: isize, lpidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpidataobject: ::windows::core::RawPtr, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IExtendTaskPad(::windows::core::IUnknown);
impl IExtendTaskPad {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TaskNotify<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, pdo: Param0, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdo.into_param().abi(), ::core::mem::transmute(arg), ::core::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumTasks<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pdo: Param0, sztaskgroup: Param1) -> ::windows::core::Result<IEnumTASK> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pdo.into_param().abi(), sztaskgroup.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTASK>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszgroup: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszgroup.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescriptiveText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszgroup: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszgroup.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszgroup: Param0) -> ::windows::core::Result<MMC_TASK_DISPLAY_OBJECT> {
        let mut result__: MMC_TASK_DISPLAY_OBJECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszgroup.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<MMC_TASK_DISPLAY_OBJECT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetListPadInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszgroup: Param0) -> ::windows::core::Result<MMC_LISTPAD_INFO> {
        let mut result__: MMC_LISTPAD_INFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszgroup.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<MMC_LISTPAD_INFO>(result__)
    }
}
impl ::core::convert::From<IExtendTaskPad> for ::windows::core::IUnknown {
    fn from(value: IExtendTaskPad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendTaskPad> for ::windows::core::IUnknown {
    fn from(value: &IExtendTaskPad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExtendTaskPad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExtendTaskPad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExtendTaskPad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExtendTaskPad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendTaskPad {}
impl ::core::fmt::Debug for IExtendTaskPad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendTaskPad").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendTaskPad {
    type Vtable = IExtendTaskPadVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dee6511_554d_11d1_9fea_00600832db4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendTaskPadVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, sztaskgroup: super::super::Foundation::PWSTR, ppenumtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, psztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, pszdescriptivetext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IExtendView(::windows::core::IUnknown);
impl IExtendView {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetViews<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, IViewExtensionCallback>>(&self, pdataobject: Param0, pviewextensioncallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), pviewextensioncallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IExtendView> for ::windows::core::IUnknown {
    fn from(value: IExtendView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExtendView> for ::windows::core::IUnknown {
    fn from(value: &IExtendView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExtendView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExtendView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExtendView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExtendView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendView {}
impl ::core::fmt::Debug for IExtendView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendView {
    type Vtable = IExtendViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89995cee_d2ed_4c0e_ae5e_df7e76f3fa53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pviewextensioncallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IHeaderCtrl(::windows::core::IUnknown);
impl IHeaderCtrl {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertColumn<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncol: i32, title: Param1, nformat: i32, nwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), title.into_param().abi(), ::core::mem::transmute(nformat), ::core::mem::transmute(nwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumnText<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncol: i32, title: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), title.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumnText(&self, ncol: i32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), ::core::mem::transmute(nwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IHeaderCtrl> for ::windows::core::IUnknown {
    fn from(value: IHeaderCtrl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHeaderCtrl> for ::windows::core::IUnknown {
    fn from(value: &IHeaderCtrl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHeaderCtrl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IHeaderCtrl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHeaderCtrl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHeaderCtrl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHeaderCtrl {}
impl ::core::fmt::Debug for IHeaderCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHeaderCtrl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHeaderCtrl {
    type Vtable = IHeaderCtrlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb3_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR, nformat: i32, nwidth: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IHeaderCtrl2(::windows::core::IUnknown);
impl IHeaderCtrl2 {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertColumn<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncol: i32, title: Param1, nformat: i32, nwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), title.into_param().abi(), ::core::mem::transmute(nformat), ::core::mem::transmute(nwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumnText<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncol: i32, title: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), title.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumnText(&self, ncol: i32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), ::core::mem::transmute(nwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncol), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetChangeTimeOut(&self, utimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(utimeout)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumnFilter(&self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncolumn), ::core::mem::transmute(dwtype), ::core::mem::transmute(pfilterdata)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumnFilter(&self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncolumn), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pfilterdata)).ok()
    }
}
impl ::core::convert::From<IHeaderCtrl2> for IHeaderCtrl {
    fn from(value: IHeaderCtrl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHeaderCtrl2> for IHeaderCtrl {
    fn from(value: &IHeaderCtrl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHeaderCtrl> for IHeaderCtrl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IHeaderCtrl> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHeaderCtrl> for &IHeaderCtrl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IHeaderCtrl> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHeaderCtrl2> for ::windows::core::IUnknown {
    fn from(value: IHeaderCtrl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHeaderCtrl2> for ::windows::core::IUnknown {
    fn from(value: &IHeaderCtrl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHeaderCtrl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IHeaderCtrl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHeaderCtrl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHeaderCtrl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHeaderCtrl2 {}
impl ::core::fmt::Debug for IHeaderCtrl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHeaderCtrl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHeaderCtrl2 {
    type Vtable = IHeaderCtrl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9757abb8_1b32_11d1_a7ce_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR, nformat: i32, nwidth: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IImageList(::windows::core::IUnknown);
impl IImageList {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ImageListSetIcon(&self, picon: *const isize, nloc: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(picon), ::core::mem::transmute(nloc)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ImageListSetStrip(&self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbmapsm), ::core::mem::transmute(pbmaplg), ::core::mem::transmute(nstartloc), ::core::mem::transmute(cmask)).ok()
    }
}
impl ::core::convert::From<IImageList> for ::windows::core::IUnknown {
    fn from(value: IImageList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageList> for ::windows::core::IUnknown {
    fn from(value: &IImageList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IImageList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IImageList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageList {}
impl ::core::fmt::Debug for IImageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImageList {
    type Vtable = IImageListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb8_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IMMCVersionInfo(::windows::core::IUnknown);
impl IMMCVersionInfo {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetMMCVersion(&self, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pversionmajor), ::core::mem::transmute(pversionminor)).ok()
    }
}
impl ::core::convert::From<IMMCVersionInfo> for ::windows::core::IUnknown {
    fn from(value: IMMCVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMCVersionInfo> for ::windows::core::IUnknown {
    fn from(value: &IMMCVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMCVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMMCVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMCVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMCVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMCVersionInfo {}
impl ::core::fmt::Debug for IMMCVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMCVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMCVersionInfo {
    type Vtable = IMMCVersionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8d2c5fe_cdcb_4b9d_bde5_a27343ff54bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMCVersionInfoVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IMenuButton(::windows::core::IUnknown);
impl IMenuButton {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddButton<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, idcommand: i32, lpbuttontext: Param1, lptooltiptext: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(idcommand), lpbuttontext.into_param().abi(), lptooltiptext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButton<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, idcommand: i32, lpbuttontext: Param1, lptooltiptext: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(idcommand), lpbuttontext.into_param().abi(), lptooltiptext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtonState<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(idcommand), ::core::mem::transmute(nstate), bstate.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMenuButton> for ::windows::core::IUnknown {
    fn from(value: IMenuButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMenuButton> for ::windows::core::IUnknown {
    fn from(value: &IMenuButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMenuButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMenuButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMenuButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMenuButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMenuButton {}
impl ::core::fmt::Debug for IMenuButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMenuButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMenuButton {
    type Vtable = IMenuButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x951ed750_d080_11d0_b197_000000000000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuButtonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IMessageView(::windows::core::IUnknown);
impl IMessageView {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitleText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztitletext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psztitletext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBodyText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbodytext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszbodytext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetIcon(&self, id: IconIdentifier) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IMessageView> for ::windows::core::IUnknown {
    fn from(value: IMessageView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessageView> for ::windows::core::IUnknown {
    fn from(value: &IMessageView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMessageView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMessageView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMessageView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMessageView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageView {}
impl ::core::fmt::Debug for IMessageView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMessageView {
    type Vtable = IMessageViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80f94174_fccc_11d2_b991_00c04f8ecd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitletext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbodytext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct INodeProperties(::windows::core::IUnknown);
impl INodeProperties {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pdataobject: Param0, szpropertyname: Param1) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), szpropertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
}
impl ::core::convert::From<INodeProperties> for ::windows::core::IUnknown {
    fn from(value: INodeProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INodeProperties> for ::windows::core::IUnknown {
    fn from(value: &INodeProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INodeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INodeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INodeProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INodeProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INodeProperties {}
impl ::core::fmt::Debug for INodeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INodeProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INodeProperties {
    type Vtable = INodePropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15bc4d24_a522_4406_aa55_0749537a6865);
}
#[repr(C)]
#[doc(hidden)]
pub struct INodePropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, szpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproperty: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IPropertySheetCallback(::windows::core::IUnknown);
impl IPropertySheetCallback {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_UI_Controls'*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Controls::HPROPSHEETPAGE>>(&self, hpage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hpage.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_UI_Controls'*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn RemovePage<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Controls::HPROPSHEETPAGE>>(&self, hpage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hpage.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPropertySheetCallback> for ::windows::core::IUnknown {
    fn from(value: IPropertySheetCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertySheetCallback> for ::windows::core::IUnknown {
    fn from(value: &IPropertySheetCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertySheetCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPropertySheetCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertySheetCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertySheetCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySheetCallback {}
impl ::core::fmt::Debug for IPropertySheetCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySheetCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySheetCallback {
    type Vtable = IPropertySheetCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85de64dd_ef21_11cf_a285_00c04fd8dbe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySheetCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_UI_Controls")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))] usize,
    #[cfg(feature = "Win32_UI_Controls")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IPropertySheetProvider(::windows::core::IUnknown);
impl IPropertySheetProvider {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePropertySheet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, title: Param0, r#type: u8, cookie: isize, pidataobjectm: Param3, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), title.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(cookie), pidataobjectm.into_param().abi(), ::core::mem::transmute(dwoptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindPropertySheet<'a, Param1: ::windows::core::IntoParam<'a, IComponent>, Param2: ::windows::core::IntoParam<'a, super::Com::IDataObject>>(&self, hitem: isize, lpcomponent: Param1, lpdataobject: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hitem), lpcomponent.into_param().abi(), lpdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPrimaryPages<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpunknown: Param0, bcreatehandle: Param1, hnotifywindow: Param2, bscopepane: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lpunknown.into_param().abi(), bcreatehandle.into_param().abi(), hnotifywindow.into_param().abi(), bscopepane.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn AddExtensionPages(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Show(&self, window: isize, page: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(window), ::core::mem::transmute(page)).ok()
    }
}
impl ::core::convert::From<IPropertySheetProvider> for ::windows::core::IUnknown {
    fn from(value: IPropertySheetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertySheetProvider> for ::windows::core::IUnknown {
    fn from(value: &IPropertySheetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertySheetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPropertySheetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertySheetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertySheetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySheetProvider {}
impl ::core::fmt::Debug for IPropertySheetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySheetProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySheetProvider {
    type Vtable = IPropertySheetProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85de64de_ef21_11cf_a285_00c04fd8dbe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySheetProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR, r#type: u8, cookie: isize, pidataobjectm: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: ::windows::core::RawPtr, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IRequiredExtensions(::windows::core::IUnknown);
impl IRequiredExtensions {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn EnableAllExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetFirstExtension(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetNextExtension(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IRequiredExtensions> for ::windows::core::IUnknown {
    fn from(value: IRequiredExtensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRequiredExtensions> for ::windows::core::IUnknown {
    fn from(value: &IRequiredExtensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRequiredExtensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRequiredExtensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRequiredExtensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRequiredExtensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRequiredExtensions {}
impl ::core::fmt::Debug for IRequiredExtensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRequiredExtensions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRequiredExtensions {
    type Vtable = IRequiredExtensionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72782d7a_a4a0_11d1_af0f_00c04fb6dd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequiredExtensionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IResultData(::windows::core::IUnknown);
impl IResultData {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itemid), ::core::mem::transmute(ncol)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByLParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lparam: Param0) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteAllRsltItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(itemid), ::core::mem::transmute(uadd), ::core::mem::transmute(uremove)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(add), ::core::mem::transmute(remove)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lviewmode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetViewMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn UpdateItem(&self, itemid: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(itemid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sort<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncolumn), ::core::mem::transmute(dwsortoptions), luserparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescBarText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, desctext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), desctext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(nitemcount), ::core::mem::transmute(dwoptions)).ok()
    }
}
impl ::core::convert::From<IResultData> for ::windows::core::IUnknown {
    fn from(value: IResultData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResultData> for ::windows::core::IUnknown {
    fn from(value: &IResultData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResultData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IResultData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResultData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResultData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultData {}
impl ::core::fmt::Debug for IResultData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultData {
    type Vtable = IResultDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31da5fa0_e0eb_11cf_9f21_00aa003ca9f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desctext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IResultData2(::windows::core::IUnknown);
impl IResultData2 {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itemid), ::core::mem::transmute(ncol)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByLParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lparam: Param0) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteAllRsltItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(item)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(itemid), ::core::mem::transmute(uadd), ::core::mem::transmute(uremove)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(add), ::core::mem::transmute(remove)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lviewmode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetViewMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn UpdateItem(&self, itemid: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(itemid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sort<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncolumn), ::core::mem::transmute(dwsortoptions), luserparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescBarText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, desctext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), desctext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(nitemcount), ::core::mem::transmute(dwoptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn RenameResultItem(&self, itemid: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(itemid)).ok()
    }
}
impl ::core::convert::From<IResultData2> for IResultData {
    fn from(value: IResultData2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResultData2> for IResultData {
    fn from(value: &IResultData2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResultData> for IResultData2 {
    fn into_param(self) -> ::windows::core::Param<'a, IResultData> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResultData> for &IResultData2 {
    fn into_param(self) -> ::windows::core::Param<'a, IResultData> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResultData2> for ::windows::core::IUnknown {
    fn from(value: IResultData2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResultData2> for ::windows::core::IUnknown {
    fn from(value: &IResultData2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResultData2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IResultData2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResultData2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResultData2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultData2 {}
impl ::core::fmt::Debug for IResultData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultData2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultData2 {
    type Vtable = IResultData2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f36e0eb_a7f1_4a81_be5a_9247f7de4b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultData2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desctext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IResultDataCompare(::windows::core::IUnknown);
impl IResultDataCompare {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, luserparam: Param0, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), luserparam.into_param().abi(), ::core::mem::transmute(cookiea), ::core::mem::transmute(cookieb), ::core::mem::transmute(pnresult)).ok()
    }
}
impl ::core::convert::From<IResultDataCompare> for ::windows::core::IUnknown {
    fn from(value: IResultDataCompare) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResultDataCompare> for ::windows::core::IUnknown {
    fn from(value: &IResultDataCompare) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResultDataCompare {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IResultDataCompare {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResultDataCompare {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResultDataCompare {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultDataCompare {}
impl ::core::fmt::Debug for IResultDataCompare {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultDataCompare").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultDataCompare {
    type Vtable = IResultDataCompareVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8315a52_7a1a_11d0_a2d2_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataCompareVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IResultDataCompareEx(::windows::core::IUnknown);
impl IResultDataCompareEx {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare(&self, prdc: *const RDCOMPARE) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(prdc), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IResultDataCompareEx> for ::windows::core::IUnknown {
    fn from(value: IResultDataCompareEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResultDataCompareEx> for ::windows::core::IUnknown {
    fn from(value: &IResultDataCompareEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResultDataCompareEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IResultDataCompareEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResultDataCompareEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResultDataCompareEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultDataCompareEx {}
impl ::core::fmt::Debug for IResultDataCompareEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultDataCompareEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultDataCompareEx {
    type Vtable = IResultDataCompareExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96933476_0251_11d3_aeb0_00c04f8ecd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataCompareExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IResultOwnerData(::windows::core::IUnknown);
impl IResultOwnerData {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItem(&self, pfindinfo: *const RESULTFINDINFO) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfindinfo), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn CacheHint(&self, nstartindex: i32, nendindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nstartindex), ::core::mem::transmute(nendindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SortItems<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncolumn), ::core::mem::transmute(dwsortoptions), luserparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IResultOwnerData> for ::windows::core::IUnknown {
    fn from(value: IResultOwnerData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResultOwnerData> for ::windows::core::IUnknown {
    fn from(value: &IResultOwnerData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResultOwnerData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IResultOwnerData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResultOwnerData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResultOwnerData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultOwnerData {}
impl ::core::fmt::Debug for IResultOwnerData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultOwnerData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultOwnerData {
    type Vtable = IResultOwnerDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cb396d8_ea83_11d0_aef1_00c04fb6dd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultOwnerDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ISnapinAbout(::windows::core::IUnknown);
impl ISnapinAbout {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapinDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapinVersion(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetSnapinImage(&self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__: super::super::UI::WindowsAndMessaging::HICON = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::WindowsAndMessaging::HICON>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetStaticFolderImage(&self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hsmallimage), ::core::mem::transmute(hsmallimageopen), ::core::mem::transmute(hlargeimage), ::core::mem::transmute(cmask)).ok()
    }
}
impl ::core::convert::From<ISnapinAbout> for ::windows::core::IUnknown {
    fn from(value: ISnapinAbout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISnapinAbout> for ::windows::core::IUnknown {
    fn from(value: &ISnapinAbout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISnapinAbout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISnapinAbout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISnapinAbout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISnapinAbout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinAbout {}
impl ::core::fmt::Debug for ISnapinAbout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinAbout").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinAbout {
    type Vtable = ISnapinAboutVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1245208c_a151_11d0_a7d7_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinAboutVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpversion: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ISnapinHelp(::windows::core::IUnknown);
impl ISnapinHelp {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpTopic(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<ISnapinHelp> for ::windows::core::IUnknown {
    fn from(value: ISnapinHelp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISnapinHelp> for ::windows::core::IUnknown {
    fn from(value: &ISnapinHelp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISnapinHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISnapinHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISnapinHelp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISnapinHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinHelp {}
impl ::core::fmt::Debug for ISnapinHelp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinHelp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinHelp {
    type Vtable = ISnapinHelpVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b15ace_df59_11d0_a7dd_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelpVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ISnapinHelp2(::windows::core::IUnknown);
impl ISnapinHelp2 {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpTopic(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkedTopics(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<ISnapinHelp2> for ISnapinHelp {
    fn from(value: ISnapinHelp2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISnapinHelp2> for ISnapinHelp {
    fn from(value: &ISnapinHelp2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISnapinHelp> for ISnapinHelp2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISnapinHelp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISnapinHelp> for &ISnapinHelp2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISnapinHelp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISnapinHelp2> for ::windows::core::IUnknown {
    fn from(value: ISnapinHelp2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISnapinHelp2> for ::windows::core::IUnknown {
    fn from(value: &ISnapinHelp2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISnapinHelp2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISnapinHelp2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISnapinHelp2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISnapinHelp2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinHelp2 {}
impl ::core::fmt::Debug for ISnapinHelp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinHelp2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinHelp2 {
    type Vtable = ISnapinHelp2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4861a010_20f9_11d2_a510_00c04fb6dd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelp2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ISnapinProperties(::windows::core::IUnknown);
impl ISnapinProperties {
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, Properties>>(&self, pproperties: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pproperties.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn QueryPropertyNames<'a, Param0: ::windows::core::IntoParam<'a, ISnapinPropertiesCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PropertiesChanged(&self, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cproperties), ::core::mem::transmute(pproperties)).ok()
    }
}
impl ::core::convert::From<ISnapinProperties> for ::windows::core::IUnknown {
    fn from(value: ISnapinProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISnapinProperties> for ::windows::core::IUnknown {
    fn from(value: &ISnapinProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISnapinProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISnapinProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISnapinProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISnapinProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinProperties {}
impl ::core::fmt::Debug for ISnapinProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinProperties {
    type Vtable = ISnapinPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7889da9_4a02_4837_bf89_1a6f2a021010);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinPropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ISnapinPropertiesCallback(::windows::core::IUnknown);
impl ISnapinPropertiesCallback {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPropertyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<ISnapinPropertiesCallback> for ::windows::core::IUnknown {
    fn from(value: ISnapinPropertiesCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISnapinPropertiesCallback> for ::windows::core::IUnknown {
    fn from(value: &ISnapinPropertiesCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISnapinPropertiesCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISnapinPropertiesCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISnapinPropertiesCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISnapinPropertiesCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinPropertiesCallback {}
impl ::core::fmt::Debug for ISnapinPropertiesCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinPropertiesCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinPropertiesCallback {
    type Vtable = ISnapinPropertiesCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa50fa2e5_7e61_45eb_a8d4_9a07b3e851a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinPropertiesCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IStringTable(::windows::core::IUnknown);
impl IStringTable {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszadd: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszadd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self, stringid: u32, cchbuffer: u32, lpbuffer: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(stringid), ::core::mem::transmute(cchbuffer), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(pcchout)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetStringLength(&self, stringid: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(stringid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteString(&self, stringid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(stringid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteAllStrings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfind: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszfind.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enumerate(&self) -> ::windows::core::Result<super::Com::IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IEnumString>(result__)
    }
}
impl ::core::convert::From<IStringTable> for ::windows::core::IUnknown {
    fn from(value: IStringTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStringTable> for ::windows::core::IUnknown {
    fn from(value: &IStringTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStringTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStringTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStringTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStringTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStringTable {}
impl ::core::fmt::Debug for IStringTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStringTable {
    type Vtable = IStringTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde40b7a4_0f65_11d2_8e25_00c04f8ecd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszadd: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfind: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IToolbar(::windows::core::IUnknown);
impl IToolbar {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, nimages: i32, hbmp: Param1, cxsize: i32, cysize: i32, crmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(nimages), hbmp.into_param().abi(), ::core::mem::transmute(cxsize), ::core::mem::transmute(cysize), ::core::mem::transmute(crmask)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddButtons(&self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nbuttons), ::core::mem::transmute(lpbuttons)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertButton(&self, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(lpbutton)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteButton(&self, nindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(idcommand), ::core::mem::transmute(nstate), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtonState<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(idcommand), ::core::mem::transmute(nstate), bstate.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IToolbar> for ::windows::core::IUnknown {
    fn from(value: IToolbar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToolbar> for ::windows::core::IUnknown {
    fn from(value: &IToolbar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IToolbar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IToolbar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IToolbar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IToolbar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToolbar {}
impl ::core::fmt::Debug for IToolbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToolbar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IToolbar {
    type Vtable = IToolbarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb9_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolbarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct IViewExtensionCallback(::windows::core::IUnknown);
impl IViewExtensionCallback {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddView(&self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pextviewdata)).ok()
    }
}
impl ::core::convert::From<IViewExtensionCallback> for ::windows::core::IUnknown {
    fn from(value: IViewExtensionCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewExtensionCallback> for ::windows::core::IUnknown {
    fn from(value: &IViewExtensionCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IViewExtensionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IViewExtensionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IViewExtensionCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewExtensionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewExtensionCallback {}
impl ::core::fmt::Debug for IViewExtensionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewExtensionCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IViewExtensionCallback {
    type Vtable = IViewExtensionCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34dd928a_7599_41e5_9f5e_d6bc3062c2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewExtensionCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type IconIdentifier = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_None: IconIdentifier = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_Error: IconIdentifier = 32513i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_Question: IconIdentifier = 32514i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_Warning: IconIdentifier = 32515i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_Information: IconIdentifier = 32516i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_First: IconIdentifier = 32513i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const Icon_Last: IconIdentifier = 32516i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for MENUBUTTONDATA {}
impl ::core::clone::Clone for MENUBUTTONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUBUTTONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUBUTTONDATA").field("idCommand", &self.idCommand).field("x", &self.x).field("y", &self.y).finish()
    }
}
unsafe impl ::windows::core::Abi for MENUBUTTONDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MENUBUTTONDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUBUTTONDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MENUBUTTONDATA {}
impl ::core::default::Default for MENUBUTTONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: super::super::Foundation::PWSTR,
    pub lpTooltipText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMCBUTTON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMCBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMCBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMCBUTTON").field("nBitmap", &self.nBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsType", &self.fsType).field("lpButtonText", &self.lpButtonText).field("lpTooltipText", &self.lpTooltipText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMCBUTTON {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMCBUTTON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMCBUTTON>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMCBUTTON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMCBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_AUTO: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_NOICON: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_NOPARAM: i32 = -2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_NOPTR: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
pub const MMCVersionInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6fedb1d_cf21_4bd9_af3b_c5468e9c6684);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_ACTION_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_BUTTON_STATE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ENABLED: MMC_BUTTON_STATE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const CHECKED: MMC_BUTTON_STATE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const HIDDEN: MMC_BUTTON_STATE = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const INDETERMINATE: MMC_BUTTON_STATE = 8i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const BUTTONPRESSED: MMC_BUTTON_STATE = 16i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_COLUMN_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_COLUMN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_COLUMN_DATA").field("nColIndex", &self.nColIndex).field("dwFlags", &self.dwFlags).field("nWidth", &self.nWidth).field("ulReserved", &self.ulReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for MMC_COLUMN_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MMC_COLUMN_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_COLUMN_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MMC_COLUMN_DATA {}
impl ::core::default::Default for MMC_COLUMN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl ::core::marker::Copy for MMC_COLUMN_SET_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_COLUMN_SET_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_COLUMN_SET_DATA").field("cbSize", &self.cbSize).field("nNumCols", &self.nNumCols).field("pColData", &self.pColData).finish()
    }
}
unsafe impl ::windows::core::Abi for MMC_COLUMN_SET_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MMC_COLUMN_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_COLUMN_SET_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MMC_COLUMN_SET_DATA {}
impl ::core::default::Default for MMC_COLUMN_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_CONSOLE_VERB = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_NONE: MMC_CONSOLE_VERB = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_OPEN: MMC_CONSOLE_VERB = 32768i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_COPY: MMC_CONSOLE_VERB = 32769i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_PASTE: MMC_CONSOLE_VERB = 32770i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_DELETE: MMC_CONSOLE_VERB = 32771i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_PROPERTIES: MMC_CONSOLE_VERB = 32772i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_RENAME: MMC_CONSOLE_VERB = 32773i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_REFRESH: MMC_CONSOLE_VERB = 32774i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_PRINT: MMC_CONSOLE_VERB = 32775i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_CUT: MMC_CONSOLE_VERB = 32776i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_MAX: MMC_CONSOLE_VERB = 32777i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_FIRST: MMC_CONSOLE_VERB = 32768i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VERB_LAST: MMC_CONSOLE_VERB = 32776i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_CONTROL_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const TOOLBAR: MMC_CONTROL_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MENUBUTTON: MMC_CONTROL_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const COMBOBOXBAR: MMC_CONTROL_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: super::super::Foundation::BOOL,
    pub bExpanding: super::super::Foundation::BOOL,
    pub hItem: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXPANDSYNC_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_EXPANDSYNC_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_EXPANDSYNC_STRUCT").field("bHandled", &self.bHandled).field("bExpanding", &self.bExpanding).field("hItem", &self.hItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_EXPANDSYNC_STRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_EXPANDSYNC_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_EXPANDSYNC_STRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_EXPANDSYNC_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: ::windows::core::GUID,
    pub pszURL: super::super::Foundation::PWSTR,
    pub pszViewTitle: super::super::Foundation::PWSTR,
    pub pszTooltipText: super::super::Foundation::PWSTR,
    pub bReplacesDefaultView: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXT_VIEW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_EXT_VIEW_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_EXT_VIEW_DATA").field("viewID", &self.viewID).field("pszURL", &self.pszURL).field("pszViewTitle", &self.pszViewTitle).field("pszTooltipText", &self.pszTooltipText).field("bReplacesDefaultView", &self.bReplacesDefaultView).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_EXT_VIEW_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_EXT_VIEW_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_EXT_VIEW_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_EXT_VIEW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_FILTERDATA {
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_FILTERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_FILTERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_FILTERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_FILTERDATA").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("lValue", &self.lValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_FILTERDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_FILTERDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_FILTERDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_FILTERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_FILTERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_FILTER_CHANGE_CODE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_FILTER_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = 32768i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_IMAGECALLBACK: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: super::super::Foundation::PWSTR,
    pub szButtonText: super::super::Foundation::PWSTR,
    pub nCommandID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_LISTPAD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_LISTPAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_LISTPAD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_LISTPAD_INFO").field("szTitle", &self.szTitle).field("szButtonText", &self.szButtonText).field("nCommandID", &self.nCommandID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_LISTPAD_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_LISTPAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_LISTPAD_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_LISTPAD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_LISTPAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_MENU_COMMAND_IDS = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCC_STANDARD_VIEW_SELECT: MMC_MENU_COMMAND_IDS = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_NOTIFY_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_ACTIVATE: MMC_NOTIFY_TYPE = 32769i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_ADD_IMAGES: MMC_NOTIFY_TYPE = 32770i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_BTN_CLICK: MMC_NOTIFY_TYPE = 32771i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_CLICK: MMC_NOTIFY_TYPE = 32772i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_COLUMN_CLICK: MMC_NOTIFY_TYPE = 32773i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_CONTEXTMENU: MMC_NOTIFY_TYPE = 32774i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_CUTORMOVE: MMC_NOTIFY_TYPE = 32775i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_DBLCLICK: MMC_NOTIFY_TYPE = 32776i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_DELETE: MMC_NOTIFY_TYPE = 32777i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_DESELECT_ALL: MMC_NOTIFY_TYPE = 32778i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_EXPAND: MMC_NOTIFY_TYPE = 32779i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_HELP: MMC_NOTIFY_TYPE = 32780i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_MENU_BTNCLICK: MMC_NOTIFY_TYPE = 32781i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_MINIMIZED: MMC_NOTIFY_TYPE = 32782i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_PASTE: MMC_NOTIFY_TYPE = 32783i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_PROPERTY_CHANGE: MMC_NOTIFY_TYPE = 32784i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_QUERY_PASTE: MMC_NOTIFY_TYPE = 32785i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_REFRESH: MMC_NOTIFY_TYPE = 32786i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_REMOVE_CHILDREN: MMC_NOTIFY_TYPE = 32787i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_RENAME: MMC_NOTIFY_TYPE = 32788i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_SELECT: MMC_NOTIFY_TYPE = 32789i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_SHOW: MMC_NOTIFY_TYPE = 32790i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_VIEW_CHANGE: MMC_NOTIFY_TYPE = 32791i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_SNAPINHELP: MMC_NOTIFY_TYPE = 32792i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_CONTEXTHELP: MMC_NOTIFY_TYPE = 32793i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_INITOCX: MMC_NOTIFY_TYPE = 32794i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_FILTER_CHANGE: MMC_NOTIFY_TYPE = 32795i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_FILTERBTN_CLICK: MMC_NOTIFY_TYPE = 32796i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_RESTORE_VIEW: MMC_NOTIFY_TYPE = 32797i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_PRINT: MMC_NOTIFY_TYPE = 32798i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_PRELOAD: MMC_NOTIFY_TYPE = 32799i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_LISTPAD: MMC_NOTIFY_TYPE = 32800i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_EXPANDSYNC: MMC_NOTIFY_TYPE = 32801i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_COLUMNS_CHANGED: MMC_NOTIFY_TYPE = 32802i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMCN_CANPASTE_OUTOFPROC: MMC_NOTIFY_TYPE = 32803i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_PROPERTY_ACTION = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROPACT_DELETING: MMC_PROPERTY_ACTION = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROPACT_CHANGING: MMC_PROPERTY_ACTION = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROPACT_INITIALIZED: MMC_PROPERTY_ACTION = 3i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROP_PERSIST: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PSO_HASHELP: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: super::super::Foundation::PWSTR,
    pub lViewOptions: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_RESTORE_VIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_RESTORE_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_RESTORE_VIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_RESTORE_VIEW").field("dwSize", &self.dwSize).field("cookie", &self.cookie).field("pViewType", &self.pViewType).field("lViewOptions", &self.lViewOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_RESTORE_VIEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_RESTORE_VIEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_RESTORE_VIEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_RESTORE_VIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_RESTORE_VIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_RESULT_VIEW_STYLE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = 8i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_SCOPE_ITEM_STATE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: super::super::Foundation::PWSTR,
    pub varValue: super::Com::VARIANT,
    pub eAction: MMC_PROPERTY_ACTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        Self { pszPropName: self.pszPropName, varValue: self.varValue.clone(), eAction: self.eAction }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for MMC_SNAPIN_PROPERTY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for MMC_SNAPIN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.pszPropName == other.pszPropName && self.varValue == other.varValue && self.eAction == other.eAction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for MMC_SNAPIN_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for MMC_SNAPIN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_SORT_DATA {}
impl ::core::clone::Clone for MMC_SORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_SORT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_SORT_DATA").field("nColIndex", &self.nColIndex).field("dwSortOptions", &self.dwSortOptions).field("ulReserved", &self.ulReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for MMC_SORT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MMC_SORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_SORT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MMC_SORT_DATA {}
impl ::core::default::Default for MMC_SORT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl ::core::marker::Copy for MMC_SORT_SET_DATA {}
impl ::core::clone::Clone for MMC_SORT_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_SORT_SET_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_SORT_SET_DATA").field("cbSize", &self.cbSize).field("nNumItems", &self.nNumItems).field("pSortData", &self.pSortData).finish()
    }
}
unsafe impl ::windows::core::Abi for MMC_SORT_SET_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MMC_SORT_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_SORT_SET_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MMC_SORT_SET_DATA {}
impl ::core::default::Default for MMC_SORT_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: super::super::Foundation::PWSTR,
    pub szHelpString: super::super::Foundation::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_TASK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_TASK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_TASK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_TASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_TASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MMC_TASK_0 {
    pub nCommandID: isize,
    pub szActionURL: super::super::Foundation::PWSTR,
    pub szScript: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_TASK_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_TASK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_TASK_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_TASK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_TASK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: super::super::Foundation::PWSTR,
    pub szMouseOffBitmap: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_BITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_TASK_DISPLAY_BITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_TASK_DISPLAY_BITMAP").field("szMouseOverBitmap", &self.szMouseOverBitmap).field("szMouseOffBitmap", &self.szMouseOffBitmap).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_TASK_DISPLAY_BITMAP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_TASK_DISPLAY_BITMAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_TASK_DISPLAY_BITMAP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_TASK_DISPLAY_BITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_TASK_DISPLAY_BITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_TASK_DISPLAY_OBJECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_TASK_DISPLAY_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_TASK_DISPLAY_OBJECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_TASK_DISPLAY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_TASK_DISPLAY_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MMC_TASK_DISPLAY_OBJECT_0 {
    pub uBitmap: MMC_TASK_DISPLAY_BITMAP,
    pub uSymbol: MMC_TASK_DISPLAY_SYMBOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_TASK_DISPLAY_OBJECT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_TASK_DISPLAY_OBJECT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_TASK_DISPLAY_OBJECT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_TASK_DISPLAY_OBJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_TASK_DISPLAY_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: super::super::Foundation::PWSTR,
    pub szURLtoEOT: super::super::Foundation::PWSTR,
    pub szSymbolString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_SYMBOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_TASK_DISPLAY_SYMBOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_TASK_DISPLAY_SYMBOL").field("szFontFamilyName", &self.szFontFamilyName).field("szURLtoEOT", &self.szURLtoEOT).field("szSymbolString", &self.szSymbolString).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMC_TASK_DISPLAY_SYMBOL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_TASK_DISPLAY_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_TASK_DISPLAY_SYMBOL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_TASK_DISPLAY_SYMBOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_TASK_DISPLAY_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_TASK_DISPLAY_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_TASK_DISPLAY_UNINITIALIZED: MMC_TASK_DISPLAY_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: MMC_TASK_DISPLAY_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: MMC_TASK_DISPLAY_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: MMC_TASK_DISPLAY_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: MMC_TASK_DISPLAY_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VER: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type MMC_VIEW_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl ::core::marker::Copy for MMC_VISIBLE_COLUMNS {}
impl ::core::clone::Clone for MMC_VISIBLE_COLUMNS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_VISIBLE_COLUMNS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_VISIBLE_COLUMNS").field("nVisibleColumns", &self.nVisibleColumns).field("rgVisibleCols", &self.rgVisibleCols).finish()
    }
}
unsafe impl ::windows::core::Abi for MMC_VISIBLE_COLUMNS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MMC_VISIBLE_COLUMNS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMC_VISIBLE_COLUMNS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MMC_VISIBLE_COLUMNS {}
impl ::core::default::Default for MMC_VISIBLE_COLUMNS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct MenuItem(::windows::core::IUnknown);
impl MenuItem {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn LanguageIndependentName(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Path(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn LanguageIndependentPath(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Execute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<MenuItem> for super::Com::IDispatch {
    fn from(value: MenuItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&MenuItem> for super::Com::IDispatch {
    fn from(value: &MenuItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for MenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &MenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MenuItem> for ::windows::core::IUnknown {
    fn from(value: MenuItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MenuItem> for ::windows::core::IUnknown {
    fn from(value: &MenuItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for MenuItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuItem {}
impl ::core::fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for MenuItem {
    type Vtable = MenuItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0178fad1_b361_4b27_96ad_67c57ebf2e1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageindependentname: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageindependentpath: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Node(::windows::core::IUnknown);
impl Node {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Property<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Bookmark(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScopeNode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Nodetype(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Node> for super::Com::IDispatch {
    fn from(value: Node) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Node> for super::Com::IDispatch {
    fn from(value: &Node) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Node {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Node {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Node> for ::windows::core::IUnknown {
    fn from(value: Node) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Node> for ::windows::core::IUnknown {
    fn from(value: &Node) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Node {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Node {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Node {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Node {}
impl ::core::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Node").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Node {
    type Vtable = NodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf81ed800_7839_4447_945d_8e15da59ca55);
}
#[repr(C)]
#[doc(hidden)]
pub struct NodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bookmark: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodetype: *mut *mut u16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Nodes(::windows::core::IUnknown);
impl Nodes {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Nodes> for super::Com::IDispatch {
    fn from(value: Nodes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Nodes> for super::Com::IDispatch {
    fn from(value: &Nodes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Nodes {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Nodes {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Nodes> for ::windows::core::IUnknown {
    fn from(value: Nodes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Nodes> for ::windows::core::IUnknown {
    fn from(value: &Nodes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Nodes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Nodes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Nodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Nodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Nodes {}
impl ::core::fmt::Debug for Nodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Nodes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Nodes {
    type Vtable = NodesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x313b01df_b22f_4d42_b1b8_483cdcf51d35);
}
#[repr(C)]
#[doc(hidden)]
pub struct NodesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Properties(::windows::core::IUnknown);
impl Properties {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<Property> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<Property>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Properties> for super::Com::IDispatch {
    fn from(value: Properties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Properties> for super::Com::IDispatch {
    fn from(value: &Properties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Properties {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Properties {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Properties> for ::windows::core::IUnknown {
    fn from(value: Properties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Properties> for ::windows::core::IUnknown {
    fn from(value: &Properties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Properties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Properties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Properties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Properties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Properties {}
impl ::core::fmt::Debug for Properties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Properties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Properties {
    type Vtable = PropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2886abc2_a425_42b2_91c6_e25c0e04581c);
}
#[repr(C)]
#[doc(hidden)]
pub struct PropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Property(::windows::core::IUnknown);
impl Property {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Property> for super::Com::IDispatch {
    fn from(value: Property) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Property> for super::Com::IDispatch {
    fn from(value: &Property) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Property {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Property {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Property> for ::windows::core::IUnknown {
    fn from(value: Property) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Property> for ::windows::core::IUnknown {
    fn from(value: &Property) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Property {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Property {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Property {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Property {}
impl ::core::fmt::Debug for Property {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Property").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Property {
    type Vtable = PropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4600c3a5_e301_41d8_b6d0_ef2e4212e0ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct PropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDCOMPARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RDCOMPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDCOMPARE").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("nColumn", &self.nColumn).field("lUserParam", &self.lUserParam).field("prdch1", &self.prdch1).field("prdch2", &self.prdch2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RDCOMPARE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RDCOMPARE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RDCOMPARE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RDCOMPARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDITEMHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RDITEMHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDITEMHDR").field("dwFlags", &self.dwFlags).field("cookie", &self.cookie).field("lpReserved", &self.lpReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RDITEMHDR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RDITEMHDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RDITEMHDR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RDITEMHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDI_INDENT: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDI_INDEX: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDI_PARAM: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDI_STATE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RDI_STR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: super::super::Foundation::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: super::super::Foundation::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULTDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESULTDATAITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULTDATAITEM").field("mask", &self.mask).field("bScopeItem", &self.bScopeItem).field("itemID", &self.itemID).field("nIndex", &self.nIndex).field("nCol", &self.nCol).field("str", &self.str).field("nImage", &self.nImage).field("nState", &self.nState).field("lParam", &self.lParam).field("iIndent", &self.iIndent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESULTDATAITEM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESULTDATAITEM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTFINDINFO {
    pub psz: super::super::Foundation::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULTFINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULTFINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESULTFINDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULTFINDINFO").field("psz", &self.psz).field("nStart", &self.nStart).field("dwOptions", &self.dwOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESULTFINDINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULTFINDINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESULTFINDINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULTFINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULTFINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO {
    pub pstrPersistableViewDescription: super::super::Foundation::PWSTR,
    pub eViewType: MMC_VIEW_TYPE,
    pub dwMiscOptions: u32,
    pub Anonymous: RESULT_VIEW_TYPE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO {
    fn clone(&self) -> Self {
        Self {
            pstrPersistableViewDescription: self.pstrPersistableViewDescription,
            eViewType: self.eViewType,
            dwMiscOptions: self.dwMiscOptions,
            Anonymous: self.Anonymous.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESULT_VIEW_TYPE_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULT_VIEW_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pstrPersistableViewDescription == other.pstrPersistableViewDescription && self.eViewType == other.eViewType && self.dwMiscOptions == other.dwMiscOptions && self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULT_VIEW_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union RESULT_VIEW_TYPE_INFO_0 {
    pub dwListOptions: u32,
    pub Anonymous1: RESULT_VIEW_TYPE_INFO_0_0,
    pub Anonymous2: ::core::mem::ManuallyDrop<RESULT_VIEW_TYPE_INFO_0_1>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESULT_VIEW_TYPE_INFO_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESULT_VIEW_TYPE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULT_VIEW_TYPE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESULT_VIEW_TYPE_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULT_VIEW_TYPE_INFO_0_0").field("dwHTMLOptions", &self.dwHTMLOptions).field("pstrURL", &self.pstrURL).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESULT_VIEW_TYPE_INFO_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESULT_VIEW_TYPE_INFO_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULT_VIEW_TYPE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: ::core::option::Option<::windows::core::IUnknown>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_1 {
    fn clone(&self) -> Self {
        Self { dwOCXOptions: self.dwOCXOptions, pUnkControl: self.pUnkControl.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESULT_VIEW_TYPE_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULT_VIEW_TYPE_INFO_0_1").field("dwOCXOptions", &self.dwOCXOptions).field("pUnkControl", &self.pUnkControl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESULT_VIEW_TYPE_INFO_0_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwOCXOptions == other.dwOCXOptions && self.pUnkControl == other.pUnkControl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULT_VIEW_TYPE_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RFI_PARTIAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RFI_WRAP: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RSI_DESCENDING: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RSI_NOSORTICON: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCOPEDATAITEM {
    pub mask: u32,
    pub displayname: super::super::Foundation::PWSTR,
    pub nImage: i32,
    pub nOpenImage: i32,
    pub nState: u32,
    pub cChildren: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub relativeID: isize,
    pub ID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCOPEDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCOPEDATAITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPEDATAITEM").field("mask", &self.mask).field("displayname", &self.displayname).field("nImage", &self.nImage).field("nOpenImage", &self.nOpenImage).field("nState", &self.nState).field("cChildren", &self.cChildren).field("lParam", &self.lParam).field("relativeID", &self.relativeID).field("ID", &self.ID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCOPEDATAITEM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCOPEDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCOPEDATAITEM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCOPEDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SColumnSetID {}
impl ::core::clone::Clone for SColumnSetID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SColumnSetID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SColumnSetID").field("dwFlags", &self.dwFlags).field("cBytes", &self.cBytes).field("id", &self.id).finish()
    }
}
unsafe impl ::windows::core::Abi for SColumnSetID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SColumnSetID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SColumnSetID>()) == 0 }
    }
}
impl ::core::cmp::Eq for SColumnSetID {}
impl ::core::default::Default for SColumnSetID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_CHILDREN: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_FIRST: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_NEXT: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_OPENIMAGE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_PARAM: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_PARENT: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_PREVIOUS: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_STATE: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SDI_STR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [::core::option::Option<super::Com::IDataObject>; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SMMCDataObjects {
    fn clone(&self) -> Self {
        Self { count: self.count, lpDataObject: self.lpDataObject.clone() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SMMCDataObjects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMMCDataObjects").field("count", &self.count).field("lpDataObject", &self.lpDataObject).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for SMMCDataObjects {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SMMCDataObjects {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.lpDataObject == other.lpDataObject
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SMMCDataObjects {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SMMCDataObjects {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for SMMCObjectTypes {}
impl ::core::clone::Clone for SMMCObjectTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMMCObjectTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMMCObjectTypes").field("count", &self.count).field("guid", &self.guid).finish()
    }
}
unsafe impl ::windows::core::Abi for SMMCObjectTypes {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMMCObjectTypes {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMMCObjectTypes>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMMCObjectTypes {}
impl ::core::default::Default for SMMCObjectTypes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID {}
impl ::core::clone::Clone for SNodeID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SNodeID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNodeID").field("cBytes", &self.cBytes).field("id", &self.id).finish()
    }
}
unsafe impl ::windows::core::Abi for SNodeID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SNodeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SNodeID>()) == 0 }
    }
}
impl ::core::cmp::Eq for SNodeID {}
impl ::core::default::Default for SNodeID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID2 {}
impl ::core::clone::Clone for SNodeID2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SNodeID2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNodeID2").field("dwFlags", &self.dwFlags).field("cBytes", &self.cBytes).field("id", &self.id).finish()
    }
}
unsafe impl ::windows::core::Abi for SNodeID2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SNodeID2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SNodeID2>()) == 0 }
    }
}
impl ::core::cmp::Eq for SNodeID2 {}
impl ::core::default::Default for SNodeID2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct ScopeNamespace(::windows::core::IUnknown);
impl ScopeNamespace {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetParent<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), node.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetChild<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), node.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetNext<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), node.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn GetRoot(&self) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Expand<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), node.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ScopeNamespace> for super::Com::IDispatch {
    fn from(value: ScopeNamespace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ScopeNamespace> for super::Com::IDispatch {
    fn from(value: &ScopeNamespace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ScopeNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ScopeNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScopeNamespace> for ::windows::core::IUnknown {
    fn from(value: ScopeNamespace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScopeNamespace> for ::windows::core::IUnknown {
    fn from(value: &ScopeNamespace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScopeNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ScopeNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ScopeNamespace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScopeNamespace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScopeNamespace {}
impl ::core::fmt::Debug for ScopeNamespace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScopeNamespace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ScopeNamespace {
    type Vtable = ScopeNamespaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebbb48dc_1a3b_4d86_b786_c21b28389012);
}
#[repr(C)]
#[doc(hidden)]
pub struct ScopeNamespaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct SnapIn(::windows::core::IUnknown);
impl SnapIn {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Vendor(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Version(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Extensions(&self) -> ::windows::core::Result<Extensions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Extensions>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SnapinCLSID(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<Properties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Properties>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAllExtensions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, enable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), enable.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SnapIn> for super::Com::IDispatch {
    fn from(value: SnapIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SnapIn> for super::Com::IDispatch {
    fn from(value: &SnapIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for SnapIn {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &SnapIn {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SnapIn> for ::windows::core::IUnknown {
    fn from(value: SnapIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SnapIn> for ::windows::core::IUnknown {
    fn from(value: &SnapIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SnapIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SnapIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for SnapIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SnapIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SnapIn {}
impl ::core::fmt::Debug for SnapIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapIn").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for SnapIn {
    type Vtable = SnapInVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3be910f6_3459_49c6_a1bb_41e6be9df3ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct SnapInVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct SnapIns(::windows::core::IUnknown);
impl SnapIns {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<SnapIn> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<SnapIn>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, snapinnameorclsid: Param0, parentsnapin: Param1, properties: Param2) -> ::windows::core::Result<SnapIn> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), snapinnameorclsid.into_param().abi(), parentsnapin.into_param().abi(), properties.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<SnapIn>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, SnapIn>>(&self, snapin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), snapin.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SnapIns> for super::Com::IDispatch {
    fn from(value: SnapIns) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SnapIns> for super::Com::IDispatch {
    fn from(value: &SnapIns) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for SnapIns {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &SnapIns {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SnapIns> for ::windows::core::IUnknown {
    fn from(value: SnapIns) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SnapIns> for ::windows::core::IUnknown {
    fn from(value: &SnapIns) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SnapIns {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SnapIns {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for SnapIns {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SnapIns {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SnapIns {}
impl ::core::fmt::Debug for SnapIns {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapIns").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for SnapIns {
    type Vtable = SnapInsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef3de1d_b12a_49d1_92c5_0b00798768f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct SnapInsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parentsnapin: ::core::mem::ManuallyDrop<super::Com::VARIANT>, properties: ::core::mem::ManuallyDrop<super::Com::VARIANT>, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct View(::windows::core::IUnknown);
impl View {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ActiveScopeNode(&self) -> ::windows::core::Result<Node> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Node>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetActiveScopeNode<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), node.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Selection(&self) -> ::windows::core::Result<Nodes> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Nodes>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ListItems(&self) -> ::windows::core::Result<Nodes> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Nodes>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SnapinScopeObject<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, scopenode: Param0) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), scopenode.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SnapinSelectionObject(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Is<'a, Param0: ::windows::core::IntoParam<'a, View>>(&self, view: Param0) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), view.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Document(&self) -> ::windows::core::Result<Document> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Document>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SelectAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Select<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), node.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Deselect<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), node.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSelected<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), node.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DisplayScopeNodePropertySheet<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, scopenode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), scopenode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DisplaySelectionPropertySheet(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyScopeNode<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, scopenode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), scopenode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn CopySelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScopeNode<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, scopenode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), scopenode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn DeleteSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RenameScopeNode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, newname: Param0, scopenode: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), newname.into_param().abi(), scopenode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameSelectedItem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), newname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ScopeNodeContextMenu<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, scopenode: Param0) -> ::windows::core::Result<ContextMenu> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), scopenode.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ContextMenu>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SelectionContextMenu(&self) -> ::windows::core::Result<ContextMenu> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ContextMenu>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RefreshScopeNode<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, scopenode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), scopenode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn RefreshSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteSelectionMenuItem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, menuitempath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), menuitempath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExecuteScopeNodeMenuItem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, menuitempath: Param0, scopenode: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), menuitempath.into_param().abi(), scopenode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteShellCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, command: Param0, directory: Param1, parameters: Param2, windowstate: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), command.into_param().abi(), directory.into_param().abi(), parameters.into_param().abi(), windowstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Frame(&self) -> ::windows::core::Result<Frame> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Frame>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScopeTreeVisible(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScopeTreeVisible<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, visible: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), visible.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Back(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Forward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusBarText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, statusbartext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), statusbartext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Memento(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ViewMemento<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, memento: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), memento.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Columns(&self) -> ::windows::core::Result<Columns> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Columns>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn CellContents<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0, column: i32) -> ::windows::core::Result<*mut u16> {
        let mut result__: *mut u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), node.into_param().abi(), ::core::mem::transmute(column), ::core::mem::transmute(&mut result__)).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, file: Param0, exportoptions: _ExportListOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), file.into_param().abi(), ::core::mem::transmute(exportoptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ListViewMode(&self) -> ::windows::core::Result<_ListViewMode> {
        let mut result__: _ListViewMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<_ListViewMode>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn SetListViewMode(&self, mode: _ListViewMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ControlObject(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<View> for super::Com::IDispatch {
    fn from(value: View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&View> for super::Com::IDispatch {
    fn from(value: &View) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for View {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &View {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<View> for ::windows::core::IUnknown {
    fn from(value: View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&View> for ::windows::core::IUnknown {
    fn from(value: &View) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for View {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &View {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for View {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for View {}
impl ::core::fmt::Debug for View {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("View").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for View {
    type Vtable = ViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6efc2da2_b38c_457e_9abb_ed2d189b8c38);
}
#[repr(C)]
#[doc(hidden)]
pub struct ViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, scopenodeobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, thesame: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, isselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, directory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memento: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memento: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, columns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, column: i32, cellcontents: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exportoptions: _ExportListOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, control: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct Views(::windows::core::IUnknown);
impl Views {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<View> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<View>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, Node>>(&self, node: Param0, viewoptions: _ViewOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), node.into_param().abi(), ::core::mem::transmute(viewoptions)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<Views> for super::Com::IDispatch {
    fn from(value: Views) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&Views> for super::Com::IDispatch {
    fn from(value: &Views) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for Views {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &Views {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Views> for ::windows::core::IUnknown {
    fn from(value: Views) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Views> for ::windows::core::IUnknown {
    fn from(value: &Views) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Views {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Views {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for Views {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Views {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Views {}
impl ::core::fmt::Debug for Views {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Views").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Views {
    type Vtable = ViewsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b8c29d_a1ff_4d72_aab0_e381e9b9338d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ViewsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, viewoptions: _ViewOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct _AppEvents(::windows::core::IUnknown);
impl _AppEvents {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnQuit<'a, Param0: ::windows::core::IntoParam<'a, _Application>>(&self, application: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDocumentOpen<'a, Param0: ::windows::core::IntoParam<'a, Document>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, document: Param0, new: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), document.into_param().abi(), new.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnDocumentClose<'a, Param0: ::windows::core::IntoParam<'a, Document>>(&self, document: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), document.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnSnapInAdded<'a, Param0: ::windows::core::IntoParam<'a, Document>, Param1: ::windows::core::IntoParam<'a, SnapIn>>(&self, document: Param0, snapin: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), document.into_param().abi(), snapin.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnSnapInRemoved<'a, Param0: ::windows::core::IntoParam<'a, Document>, Param1: ::windows::core::IntoParam<'a, SnapIn>>(&self, document: Param0, snapin: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), document.into_param().abi(), snapin.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnNewView<'a, Param0: ::windows::core::IntoParam<'a, View>>(&self, view: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), view.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnViewClose<'a, Param0: ::windows::core::IntoParam<'a, View>>(&self, view: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), view.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnViewChange<'a, Param0: ::windows::core::IntoParam<'a, View>, Param1: ::windows::core::IntoParam<'a, Node>>(&self, view: Param0, newownernode: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), view.into_param().abi(), newownernode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnSelectionChange<'a, Param0: ::windows::core::IntoParam<'a, View>, Param1: ::windows::core::IntoParam<'a, Nodes>>(&self, view: Param0, newnodes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), view.into_param().abi(), newnodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnContextMenuExecuted<'a, Param0: ::windows::core::IntoParam<'a, MenuItem>>(&self, menuitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), menuitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnToolbarButtonClicked(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn OnListUpdated<'a, Param0: ::windows::core::IntoParam<'a, View>>(&self, view: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), view.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_AppEvents> for super::Com::IDispatch {
    fn from(value: _AppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_AppEvents> for super::Com::IDispatch {
    fn from(value: &_AppEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for _AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &_AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<_AppEvents> for ::windows::core::IUnknown {
    fn from(value: _AppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_AppEvents> for ::windows::core::IUnknown {
    fn from(value: &_AppEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_AppEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _AppEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _AppEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _AppEvents {}
impl ::core::fmt::Debug for _AppEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AppEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _AppEvents {
    type Vtable = _AppEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde46cbdd_53f5_4635_af54_4fe71e923d3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct _AppEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, new: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newownernode: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newnodes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct _Application(::windows::core::IUnknown);
impl _Application {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Help(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Quit(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Document(&self) -> ::windows::core::Result<Document> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Document>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Frame(&self) -> ::windows::core::Result<Frame> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Frame>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Show(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Hide(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserControl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserControl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, usercontrol: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), usercontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn VersionMajor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn VersionMinor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_Application> for super::Com::IDispatch {
    fn from(value: _Application) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_Application> for super::Com::IDispatch {
    fn from(value: &_Application) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for _Application {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &_Application {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<_Application> for ::windows::core::IUnknown {
    fn from(value: _Application) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_Application> for ::windows::core::IUnknown {
    fn from(value: &_Application) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _Application {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_Application {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _Application {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _Application {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _Application {}
impl ::core::fmt::Debug for _Application {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_Application").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _Application {
    type Vtable = _ApplicationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3afb9cc_b653_4741_86ab_f0470ec1384c);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ApplicationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type _ColumnSortOrder = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SortOrder_Ascending: _ColumnSortOrder = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const SortOrder_Descending: _ColumnSortOrder = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type _DocumentMode = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const DocumentMode_Author: _DocumentMode = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const DocumentMode_User: _DocumentMode = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const DocumentMode_User_MDI: _DocumentMode = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const DocumentMode_User_SDI: _DocumentMode = 3i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
#[repr(transparent)]
pub struct _EventConnector(::windows::core::IUnknown);
impl _EventConnector {
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn ConnectTo<'a, Param0: ::windows::core::IntoParam<'a, _Application>>(&self, application: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Mmc'*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_EventConnector> for super::Com::IDispatch {
    fn from(value: _EventConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_EventConnector> for super::Com::IDispatch {
    fn from(value: &_EventConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for _EventConnector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &_EventConnector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<_EventConnector> for ::windows::core::IUnknown {
    fn from(value: _EventConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_EventConnector> for ::windows::core::IUnknown {
    fn from(value: &_EventConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _EventConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_EventConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _EventConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _EventConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _EventConnector {}
impl ::core::fmt::Debug for _EventConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_EventConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _EventConnector {
    type Vtable = _EventConnectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0bccd30_de44_4528_8403_a05a6a1cc8ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct _EventConnectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type _ExportListOptions = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ExportListOptions_Default: _ExportListOptions = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ExportListOptions_Unicode: _ExportListOptions = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ExportListOptions_TabDelimited: _ExportListOptions = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type _ListViewMode = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ListMode_Small_Icons: _ListViewMode = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ListMode_Large_Icons: _ListViewMode = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ListMode_List: _ListViewMode = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ListMode_Detail: _ListViewMode = 3i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ListMode_Filtered: _ListViewMode = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub type _ViewOptions = i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ViewOption_Default: _ViewOptions = 0i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ViewOption_ScopeTreeHidden: _ViewOptions = 1i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ViewOption_NoToolBars: _ViewOptions = 2i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ViewOption_NotPersistable: _ViewOptions = 4i32;
#[doc = "*Required features: 'Win32_System_Mmc'*"]
pub const ViewOption_ActionPaneHidden: _ViewOptions = 8i32;

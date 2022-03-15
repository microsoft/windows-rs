#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const ARRAY_SEP_CHAR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const FACILITY_WPC: u32 = 2457u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWPCGamesSettings(::windows::core::IUnknown);
impl IWPCGamesSettings {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoggingRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsLoggingRequired)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastSettingsChangeTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLastSettingsChangeTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<WPCFLAG_RESTRICTION> {
        let mut result__: WPCFLAG_RESTRICTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRestrictions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WPCFLAG_RESTRICTION>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn IsBlocked<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidappid: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsBlocked)(::core::mem::transmute_copy(self), guidappid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWPCGamesSettings> for ::windows::core::IUnknown {
    fn from(value: IWPCGamesSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCGamesSettings> for ::windows::core::IUnknown {
    fn from(value: &IWPCGamesSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWPCGamesSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWPCGamesSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWPCGamesSettings> for IWPCSettings {
    fn from(value: IWPCGamesSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCGamesSettings> for IWPCSettings {
    fn from(value: &IWPCGamesSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWPCSettings> for IWPCGamesSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IWPCSettings> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWPCSettings> for &'a IWPCGamesSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IWPCSettings> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWPCGamesSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWPCGamesSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCGamesSettings {}
impl ::core::fmt::Debug for IWPCGamesSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCGamesSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWPCGamesSettings {
    type Vtable = IWPCGamesSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95e87780_e158_489e_b452_bbb850790715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCGamesSettings_Vtbl {
    pub base: IWPCSettings_Vtbl,
    pub IsBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidappid: ::windows::core::GUID, pdwreasons: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWPCProviderConfig(::windows::core::IUnknown);
impl IWPCProviderConfig {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserSummary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsid: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserSummary)(::core::mem::transmute_copy(self), bstrsid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configure<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwnd: Param0, bstrsid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Configure)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), bstrsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestOverride<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwnd: Param0, bstrpath: Param1, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestOverride)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), bstrpath.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IWPCProviderConfig> for ::windows::core::IUnknown {
    fn from(value: IWPCProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCProviderConfig> for ::windows::core::IUnknown {
    fn from(value: &IWPCProviderConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWPCProviderConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWPCProviderConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWPCProviderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWPCProviderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCProviderConfig {}
impl ::core::fmt::Debug for IWPCProviderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCProviderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWPCProviderConfig {
    type Vtable = IWPCProviderConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbef54196_2d02_4a26_b6e5_d65af295d0f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderConfig_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserSummary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrusersummary: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserSummary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Configure: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestOverride: usize,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWPCProviderState(::windows::core::IUnknown);
impl IWPCProviderState {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disable)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWPCProviderState> for ::windows::core::IUnknown {
    fn from(value: IWPCProviderState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCProviderState> for ::windows::core::IUnknown {
    fn from(value: &IWPCProviderState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWPCProviderState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWPCProviderState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWPCProviderState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWPCProviderState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCProviderState {}
impl ::core::fmt::Debug for IWPCProviderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCProviderState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWPCProviderState {
    type Vtable = IWPCProviderState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50b6a267_c4bd_450b_adb5_759073837c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderState_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWPCProviderSupport(::windows::core::IUnknown);
impl IWPCProviderSupport {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IWPCProviderSupport> for ::windows::core::IUnknown {
    fn from(value: IWPCProviderSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCProviderSupport> for ::windows::core::IUnknown {
    fn from(value: &IWPCProviderSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWPCProviderSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWPCProviderSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWPCProviderSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWPCProviderSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCProviderSupport {}
impl ::core::fmt::Debug for IWPCProviderSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCProviderSupport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWPCProviderSupport {
    type Vtable = IWPCProviderSupport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41eba572_23ed_4779_bec1_8df96206c44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderSupport_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWPCSettings(::windows::core::IUnknown);
impl IWPCSettings {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoggingRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsLoggingRequired)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastSettingsChangeTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastSettingsChangeTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<WPCFLAG_RESTRICTION> {
        let mut result__: WPCFLAG_RESTRICTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRestrictions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WPCFLAG_RESTRICTION>(result__)
    }
}
impl ::core::convert::From<IWPCSettings> for ::windows::core::IUnknown {
    fn from(value: IWPCSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCSettings> for ::windows::core::IUnknown {
    fn from(value: &IWPCSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWPCSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWPCSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWPCSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWPCSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCSettings {}
impl ::core::fmt::Debug for IWPCSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWPCSettings {
    type Vtable = IWPCSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fdf6ca1_0189_47e4_b670_1a8a4636e340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCSettings_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLoggingRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLoggingRequired: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastSettingsChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastSettingsChangeTime: usize,
    pub GetRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWPCWebSettings(::windows::core::IUnknown);
impl IWPCWebSettings {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoggingRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsLoggingRequired)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastSettingsChangeTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLastSettingsChangeTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<WPCFLAG_RESTRICTION> {
        let mut result__: WPCFLAG_RESTRICTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRestrictions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WPCFLAG_RESTRICTION>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetSettings(&self) -> ::windows::core::Result<WPCFLAG_WEB_SETTING> {
        let mut result__: WPCFLAG_WEB_SETTING = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettings)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WPCFLAG_WEB_SETTING>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestURLOverride<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hwnd: Param0, pcszurl: Param1, ppcszsuburls: &[::windows::core::PWSTR]) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RequestURLOverride)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), pcszurl.into_param().abi(), ppcszsuburls.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppcszsuburls)), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWPCWebSettings> for ::windows::core::IUnknown {
    fn from(value: IWPCWebSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCWebSettings> for ::windows::core::IUnknown {
    fn from(value: &IWPCWebSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWPCWebSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWPCWebSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWPCWebSettings> for IWPCSettings {
    fn from(value: IWPCWebSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWPCWebSettings> for IWPCSettings {
    fn from(value: &IWPCWebSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWPCSettings> for IWPCWebSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IWPCSettings> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWPCSettings> for &'a IWPCWebSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IWPCSettings> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWPCWebSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWPCWebSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCWebSettings {}
impl ::core::fmt::Debug for IWPCWebSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCWebSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWPCWebSettings {
    type Vtable = IWPCWebSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffccbdb8_0992_4c30_b0f1_1cbb09c240aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCWebSettings_Vtbl {
    pub base: IWPCSettings_Vtbl,
    pub GetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestURLOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: ::windows::core::PCWSTR, curls: u32, ppcszsuburls: *const ::windows::core::PWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestURLOverride: usize,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWindowsParentalControls(::windows::core::IUnknown);
impl IWindowsParentalControls {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetVisibility(&self) -> ::windows::core::Result<WPCFLAG_VISIBILITY> {
        let mut result__: WPCFLAG_VISIBILITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVisibility)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WPCFLAG_VISIBILITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetUserSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pcszsid: Param0) -> ::windows::core::Result<IWPCSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetUserSettings)(::core::mem::transmute_copy(self), pcszsid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWPCSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetWebSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pcszsid: Param0) -> ::windows::core::Result<IWPCWebSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetWebSettings)(::core::mem::transmute_copy(self), pcszsid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWPCWebSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetWebFilterInfo(&self, pguidid: *mut ::windows::core::GUID, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetWebFilterInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidid), ::core::mem::transmute(ppszname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetGamesSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pcszsid: Param0) -> ::windows::core::Result<IWPCGamesSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGamesSettings)(::core::mem::transmute_copy(self), pcszsid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWPCGamesSettings>(result__)
    }
}
impl ::core::convert::From<IWindowsParentalControls> for ::windows::core::IUnknown {
    fn from(value: IWindowsParentalControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsParentalControls> for ::windows::core::IUnknown {
    fn from(value: &IWindowsParentalControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsParentalControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsParentalControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWindowsParentalControls> for IWindowsParentalControlsCore {
    fn from(value: IWindowsParentalControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsParentalControls> for IWindowsParentalControlsCore {
    fn from(value: &IWindowsParentalControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWindowsParentalControlsCore> for IWindowsParentalControls {
    fn into_param(self) -> ::windows::core::Param<'a, IWindowsParentalControlsCore> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWindowsParentalControlsCore> for &'a IWindowsParentalControls {
    fn into_param(self) -> ::windows::core::Param<'a, IWindowsParentalControlsCore> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsParentalControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsParentalControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsParentalControls {}
impl ::core::fmt::Debug for IWindowsParentalControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsParentalControls").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsParentalControls {
    type Vtable = IWindowsParentalControls_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b4d88b_e072_49e6_804d_26edbe21a7b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsParentalControls_Vtbl {
    pub base: IWindowsParentalControlsCore_Vtbl,
    pub GetGamesSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcszsid: ::windows::core::PCWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
pub struct IWindowsParentalControlsCore(::windows::core::IUnknown);
impl IWindowsParentalControlsCore {
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetVisibility(&self) -> ::windows::core::Result<WPCFLAG_VISIBILITY> {
        let mut result__: WPCFLAG_VISIBILITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVisibility)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WPCFLAG_VISIBILITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetUserSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pcszsid: Param0) -> ::windows::core::Result<IWPCSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserSettings)(::core::mem::transmute_copy(self), pcszsid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWPCSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetWebSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pcszsid: Param0) -> ::windows::core::Result<IWPCWebSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWebSettings)(::core::mem::transmute_copy(self), pcszsid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWPCWebSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
    pub unsafe fn GetWebFilterInfo(&self, pguidid: *mut ::windows::core::GUID, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWebFilterInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidid), ::core::mem::transmute(ppszname)).ok()
    }
}
impl ::core::convert::From<IWindowsParentalControlsCore> for ::windows::core::IUnknown {
    fn from(value: IWindowsParentalControlsCore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsParentalControlsCore> for ::windows::core::IUnknown {
    fn from(value: &IWindowsParentalControlsCore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsParentalControlsCore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsParentalControlsCore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsParentalControlsCore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsParentalControlsCore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsParentalControlsCore {}
impl ::core::fmt::Debug for IWindowsParentalControlsCore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsParentalControlsCore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsParentalControlsCore {
    type Vtable = IWindowsParentalControlsCore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff40a0f_3f3b_4d7c_a41b_4f39d7b44d05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsParentalControlsCore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows::core::HRESULT,
    pub GetUserSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcszsid: ::windows::core::PCWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetWebSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcszsid: ::windows::core::PCWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetWebFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_AppBlocked: i32 = -1342177264i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_AppOverride: i32 = -1342177263i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_Application: i32 = -1342177260i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_ComputerUsage: i32 = -1342177259i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_ContentUsage: i32 = -1342177258i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_Custom: i32 = -1342177267i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_EmailContact: i32 = -1342177266i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_EmailReceived: i32 = -1342177276i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_EmailSent: i32 = -1342177275i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_FileDownload: i32 = -1342177270i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_GameStart: i32 = -1342177278i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_IMContact: i32 = -1342177265i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_IMFeature: i32 = -1342177269i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_IMInvitation: i32 = -1342177273i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_IMJoin: i32 = -1342177272i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_IMLeave: i32 = -1342177271i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_MediaPlayback: i32 = -1342177274i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_SettingChange: i32 = -1342177279i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_UrlVisit: i32 = -1342177277i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_WebOverride: i32 = -1342177262i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Event_WebsiteVisit: i32 = -1342177261i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Keyword_ThirdParty: i32 = 268435462i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Keyword_WPC: i32 = 268435461i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Opcode_Launch: i32 = 805306390i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Opcode_Locate: i32 = 805306388i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Opcode_Modify: i32 = 805306389i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Opcode_System: i32 = 805306391i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Opcode_Web: i32 = 805306392i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Publisher_Name: i32 = -1879048191i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_AppBlocked: i32 = 1879048208i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_AppOverride: i32 = 1879048209i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_Application: i32 = 1879048212i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_ComputerUsage: i32 = 1879048213i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_ContentUsage: i32 = 1879048214i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_Custom: i32 = 1879048205i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_EmailContact: i32 = 1879048206i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_EmailReceived: i32 = 1879048196i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_EmailSent: i32 = 1879048197i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_FileDownload: i32 = 1879048202i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_GameStart: i32 = 1879048194i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_IMContact: i32 = 1879048207i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_IMFeature: i32 = 1879048203i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_IMInvitation: i32 = 1879048199i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_IMJoin: i32 = 1879048200i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_IMLeave: i32 = 1879048201i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_MediaPlayback: i32 = 1879048198i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_SettingChange: i32 = 1879048193i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_UrlVisit: i32 = 1879048195i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_WebOverride: i32 = 1879048210i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const MSG_Task_WebsiteVisit: i32 = 1879048211i32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCCHANNEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_APPLICATION_value: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_APPOVERRIDE_value: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_COMPUTERUSAGE_value: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_CONTENTUSAGE_value: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_CUSTOM_value: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_EMAIL_CONTACT_value: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_EMAIL_RECEIVED_value: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_EMAIL_SENT_value: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_GAME_START_value: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_IM_CONTACT_value: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_IM_FEATURE_value: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_IM_INVITATION_value: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_IM_JOIN_value: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_IM_LEAVE_value: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_MEDIA_PLAYBACK_value: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_SYSTEM_APPBLOCKED_value: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_SYS_SETTINGCHANGE_value: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_WEBOVERRIDE_value: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_WEB_FILEDOWNLOAD_value: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_WEB_URLVISIT_value: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCEVENT_WEB_WEBSITEVISIT_value: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_IM_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_NONE: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_VIDEO: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_AUDIO: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_GAME: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_SMS: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_FILESWAP: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(16i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_URLSWAP: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(32i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_SENDING: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_FEATURE_ALL: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(-1i32);
impl ::core::marker::Copy for WPCFLAG_IM_FEATURE {}
impl ::core::clone::Clone for WPCFLAG_IM_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_IM_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_IM_FEATURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_IM_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_IM_FEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_IM_LEAVE(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_LEAVE_NORMAL: WPCFLAG_IM_LEAVE = WPCFLAG_IM_LEAVE(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_LEAVE_FORCED: WPCFLAG_IM_LEAVE = WPCFLAG_IM_LEAVE(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_IM_LEAVE_CONVERSATION_END: WPCFLAG_IM_LEAVE = WPCFLAG_IM_LEAVE(2i32);
impl ::core::marker::Copy for WPCFLAG_IM_LEAVE {}
impl ::core::clone::Clone for WPCFLAG_IM_LEAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_IM_LEAVE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_IM_LEAVE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_IM_LEAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_IM_LEAVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_ISBLOCKED(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_NOTBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_IMBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_EMAILBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_MEDIAPLAYBACKBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_WEBBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_GAMESBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(16i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_CONTACTBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(32i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_FEATUREBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(64i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_DOWNLOADBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(128i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_RATINGBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(256i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_DESCRIPTORBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(512i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_EXPLICITBLOCK: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(1024i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_BADPASS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(2048i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_MAXHOURS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(4096i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_SPECHOURS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(8192i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_SETTINGSCHANGEBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(16384i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_ATTACHMENTBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(32768i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_SENDERBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(65536i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_RECEIVERBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(131072i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_NOTEXPLICITLYALLOWED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(262144i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_NOTINLIST: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(524288i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_CATEGORYBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(1048576i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_CATEGORYNOTINLIST: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(2097152i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_NOTKIDS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(4194304i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_UNRATED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(8388608i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_NOACCESS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(16777216i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_ISBLOCKED_INTERNALERROR: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(-1i32);
impl ::core::marker::Copy for WPCFLAG_ISBLOCKED {}
impl ::core::clone::Clone for WPCFLAG_ISBLOCKED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_ISBLOCKED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_ISBLOCKED {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_ISBLOCKED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_ISBLOCKED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_LOGOFF_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_LOGOFF_TYPE_LOGOUT: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_LOGOFF_TYPE_RESTART: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_LOGOFF_TYPE_SHUTDOWN: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_LOGOFF_TYPE_FUS: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_LOGOFF_TYPE_FORCEDFUS: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(8i32);
impl ::core::marker::Copy for WPCFLAG_LOGOFF_TYPE {}
impl ::core::clone::Clone for WPCFLAG_LOGOFF_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_LOGOFF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_LOGOFF_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_LOGOFF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_LOGOFF_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_OVERRIDE(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_APPLICATION: WPCFLAG_OVERRIDE = WPCFLAG_OVERRIDE(1i32);
impl ::core::marker::Copy for WPCFLAG_OVERRIDE {}
impl ::core::clone::Clone for WPCFLAG_OVERRIDE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_OVERRIDE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_OVERRIDE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_OVERRIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_OVERRIDE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_RESTRICTION(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_NO_RESTRICTION: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_LOGGING_REQUIRED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_WEB_FILTERED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_HOURS_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_GAMES_BLOCKED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_APPS_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(16i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_TIME_ALLOWANCE_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(32i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_GAMES_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(64i32);
impl ::core::marker::Copy for WPCFLAG_RESTRICTION {}
impl ::core::clone::Clone for WPCFLAG_RESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_RESTRICTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_RESTRICTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_RESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_RESTRICTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_VISIBILITY(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_WPC_VISIBLE: WPCFLAG_VISIBILITY = WPCFLAG_VISIBILITY(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_WPC_HIDDEN: WPCFLAG_VISIBILITY = WPCFLAG_VISIBILITY(1i32);
impl ::core::marker::Copy for WPCFLAG_VISIBILITY {}
impl ::core::clone::Clone for WPCFLAG_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_VISIBILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_VISIBILITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPCFLAG_WEB_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_WEB_SETTING_NOTBLOCKED: WPCFLAG_WEB_SETTING = WPCFLAG_WEB_SETTING(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCFLAG_WEB_SETTING_DOWNLOADSBLOCKED: WPCFLAG_WEB_SETTING = WPCFLAG_WEB_SETTING(1i32);
impl ::core::marker::Copy for WPCFLAG_WEB_SETTING {}
impl ::core::clone::Clone for WPCFLAG_WEB_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCFLAG_WEB_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCFLAG_WEB_SETTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCFLAG_WEB_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_WEB_SETTING").field(&self.0).finish()
    }
}
pub const WPCPROV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01090065_b467_4503_9b28_533766761087);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_KEYWORD_ThirdParty: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_KEYWORD_WPC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_AppBlocked: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_AppOverride: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_Application: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_ComputerUsage: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_ContentUsage: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_Custom: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_EmailContact: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_EmailReceived: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_EmailSent: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_FileDownload: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_GameStart: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_IMContact: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_IMFeature: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_IMInvitation: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_IMJoin: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_IMLeave: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_MediaPlayback: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_SettingChange: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_UrlVisit: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_WebOverride: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPCPROV_TASK_WebsiteVisit: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_APP_LAUNCH: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_APPLICATIONEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPLICATIONEVENT_SERIALIZEDAPPLICATION: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPLICATIONEVENT_DECISION: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPLICATIONEVENT_PROCESSID: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPLICATIONEVENT_CREATIONTIME: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPLICATIONEVENT_TIMEUSED: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPLICATIONEVENT_CARGS: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(5i32);
impl ::core::marker::Copy for WPC_ARGS_APPLICATIONEVENT {}
impl ::core::clone::Clone for WPC_ARGS_APPLICATIONEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_APPLICATIONEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_APPLICATIONEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_APPLICATIONEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_APPLICATIONEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_APPOVERRIDEEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPOVERRIDEEVENT_USERID: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPOVERRIDEEVENT_PATH: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPOVERRIDEEVENT_REASON: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_APPOVERRIDEEVENT_CARGS: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(3i32);
impl ::core::marker::Copy for WPC_ARGS_APPOVERRIDEEVENT {}
impl ::core::clone::Clone for WPC_ARGS_APPOVERRIDEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_APPOVERRIDEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_APPOVERRIDEEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_APPOVERRIDEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_APPOVERRIDEEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_COMPUTERUSAGEEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_COMPUTERUSAGEEVENT_ID: WPC_ARGS_COMPUTERUSAGEEVENT = WPC_ARGS_COMPUTERUSAGEEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_COMPUTERUSAGEEVENT_TIMEUSED: WPC_ARGS_COMPUTERUSAGEEVENT = WPC_ARGS_COMPUTERUSAGEEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_COMPUTERUSAGEEVENT_CARGS: WPC_ARGS_COMPUTERUSAGEEVENT = WPC_ARGS_COMPUTERUSAGEEVENT(2i32);
impl ::core::marker::Copy for WPC_ARGS_COMPUTERUSAGEEVENT {}
impl ::core::clone::Clone for WPC_ARGS_COMPUTERUSAGEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_COMPUTERUSAGEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_COMPUTERUSAGEEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_COMPUTERUSAGEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_COMPUTERUSAGEEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_CONTENTUSAGEEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_CONTENTPROVIDERID: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_CONTENTPROVIDERTITLE: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_ID: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_TITLE: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_CATEGORY: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_RATINGS: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_DECISION: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONTENTUSAGEEVENT_CARGS: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(7i32);
impl ::core::marker::Copy for WPC_ARGS_CONTENTUSAGEEVENT {}
impl ::core::clone::Clone for WPC_ARGS_CONTENTUSAGEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_CONTENTUSAGEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_CONTENTUSAGEEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_CONTENTUSAGEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONTENTUSAGEEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_CONVERSATIONINITEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_APPNAME: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_APPVERSION: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_ACCOUNTNAME: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_CONVID: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_REQUESTINGIP: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_SENDER: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_REASON: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_RECIPCOUNT: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_RECIPIENT: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONINITEVENT_CARGS: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(9i32);
impl ::core::marker::Copy for WPC_ARGS_CONVERSATIONINITEVENT {}
impl ::core::clone::Clone for WPC_ARGS_CONVERSATIONINITEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_CONVERSATIONINITEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_CONVERSATIONINITEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_CONVERSATIONINITEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONVERSATIONINITEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_CONVERSATIONJOINEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_APPNAME: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_APPVERSION: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_ACCOUNTNAME: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_CONVID: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_JOININGIP: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_JOININGUSER: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_REASON: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_MEMBERCOUNT: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_MEMBER: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_SENDER: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONJOINEVENT_CARGS: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(10i32);
impl ::core::marker::Copy for WPC_ARGS_CONVERSATIONJOINEVENT {}
impl ::core::clone::Clone for WPC_ARGS_CONVERSATIONJOINEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_CONVERSATIONJOINEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_CONVERSATIONJOINEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_CONVERSATIONJOINEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONVERSATIONJOINEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_CONVERSATIONLEAVEEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_APPNAME: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_APPVERSION: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_ACCOUNTNAME: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_CONVID: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_LEAVINGIP: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_LEAVINGUSER: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_REASON: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_MEMBERCOUNT: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_MEMBER: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_FLAGS: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_CARGS: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(10i32);
impl ::core::marker::Copy for WPC_ARGS_CONVERSATIONLEAVEEVENT {}
impl ::core::clone::Clone for WPC_ARGS_CONVERSATIONLEAVEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_CONVERSATIONLEAVEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_CONVERSATIONLEAVEEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_CONVERSATIONLEAVEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONVERSATIONLEAVEEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_CUSTOMEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_PUBLISHER: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_APPNAME: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_APPVERSION: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_EVENT: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_VALUE1: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_VALUE2: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_VALUE3: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_BLOCKED: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_REASON: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_CUSTOMEVENT_CARGS: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(9i32);
impl ::core::marker::Copy for WPC_ARGS_CUSTOMEVENT {}
impl ::core::clone::Clone for WPC_ARGS_CUSTOMEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_CUSTOMEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_CUSTOMEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_CUSTOMEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CUSTOMEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_EMAILCONTACTEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_APPNAME: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_APPVERSION: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_OLDNAME: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_OLDID: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_NEWNAME: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_NEWID: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_REASON: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_EMAILACCOUNT: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILCONTACTEVENT_CARGS: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(8i32);
impl ::core::marker::Copy for WPC_ARGS_EMAILCONTACTEVENT {}
impl ::core::clone::Clone for WPC_ARGS_EMAILCONTACTEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_EMAILCONTACTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_EMAILCONTACTEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_EMAILCONTACTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_EMAILCONTACTEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_EMAILRECEIEVEDEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_SENDER: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_APPNAME: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_APPVERSION: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_SUBJECT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_REASON: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_RECIPCOUNT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_RECIPIENT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_ATTACHCOUNT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_ATTACHMENTNAME: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_RECEIVEDTIME: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_EMAILACCOUNT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(10i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_CARGS: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(11i32);
impl ::core::marker::Copy for WPC_ARGS_EMAILRECEIEVEDEVENT {}
impl ::core::clone::Clone for WPC_ARGS_EMAILRECEIEVEDEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_EMAILRECEIEVEDEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_EMAILRECEIEVEDEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_EMAILRECEIEVEDEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_EMAILRECEIEVEDEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_EMAILSENTEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_SENDER: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_APPNAME: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_APPVERSION: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_SUBJECT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_REASON: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_RECIPCOUNT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_RECIPIENT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_ATTACHCOUNT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_ATTACHMENTNAME: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_EMAILACCOUNT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_EMAILSENTEVENT_CARGS: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(10i32);
impl ::core::marker::Copy for WPC_ARGS_EMAILSENTEVENT {}
impl ::core::clone::Clone for WPC_ARGS_EMAILSENTEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_EMAILSENTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_EMAILSENTEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_EMAILSENTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_EMAILSENTEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_FILEDOWNLOADEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_FILEDOWNLOADEVENT_URL: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_FILEDOWNLOADEVENT_APPNAME: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_FILEDOWNLOADEVENT_VERSION: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_FILEDOWNLOADEVENT_BLOCKED: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_FILEDOWNLOADEVENT_PATH: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_FILEDOWNLOADEVENT_CARGS: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(5i32);
impl ::core::marker::Copy for WPC_ARGS_FILEDOWNLOADEVENT {}
impl ::core::clone::Clone for WPC_ARGS_FILEDOWNLOADEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_FILEDOWNLOADEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_FILEDOWNLOADEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_FILEDOWNLOADEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_FILEDOWNLOADEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_GAMESTARTEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_APPID: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_INSTANCEID: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_APPVERSION: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_PATH: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_RATING: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_RATINGSYSTEM: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_REASON: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_DESCCOUNT: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_DESCRIPTOR: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_PID: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_GAMESTARTEVENT_CARGS: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(10i32);
impl ::core::marker::Copy for WPC_ARGS_GAMESTARTEVENT {}
impl ::core::clone::Clone for WPC_ARGS_GAMESTARTEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_GAMESTARTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_GAMESTARTEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_GAMESTARTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_GAMESTARTEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_IMCONTACTEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_APPNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_APPVERSION: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_ACCOUNTNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_OLDNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_OLDID: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_NEWNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_NEWID: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_REASON: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMCONTACTEVENT_CARGS: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(8i32);
impl ::core::marker::Copy for WPC_ARGS_IMCONTACTEVENT {}
impl ::core::clone::Clone for WPC_ARGS_IMCONTACTEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_IMCONTACTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_IMCONTACTEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_IMCONTACTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_IMCONTACTEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_IMFEATUREEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_APPNAME: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_APPVERSION: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_ACCOUNTNAME: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_CONVID: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_MEDIATYPE: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_REASON: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_RECIPCOUNT: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_RECIPIENT: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_SENDER: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_SENDERIP: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_DATA: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(10i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_IMFEATUREEVENT_CARGS: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(11i32);
impl ::core::marker::Copy for WPC_ARGS_IMFEATUREEVENT {}
impl ::core::clone::Clone for WPC_ARGS_IMFEATUREEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_IMFEATUREEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_IMFEATUREEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_IMFEATUREEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_IMFEATUREEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_MEDIADOWNLOADEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_APPNAME: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_APPVERSION: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_MEDIATYPE: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_PATH: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_TITLE: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_PML: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_ALBUM: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_EXPLICIT: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_REASON: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIADOWNLOADEVENT_CARGS: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(9i32);
impl ::core::marker::Copy for WPC_ARGS_MEDIADOWNLOADEVENT {}
impl ::core::clone::Clone for WPC_ARGS_MEDIADOWNLOADEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_MEDIADOWNLOADEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_MEDIADOWNLOADEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_MEDIADOWNLOADEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_MEDIADOWNLOADEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_MEDIAPLAYBACKEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_APPNAME: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_APPVERSION: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_MEDIATYPE: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_PATH: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_TITLE: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_PML: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_ALBUM: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_EXPLICIT: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_REASON: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_CARGS: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(9i32);
impl ::core::marker::Copy for WPC_ARGS_MEDIAPLAYBACKEVENT {}
impl ::core::clone::Clone for WPC_ARGS_MEDIAPLAYBACKEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_MEDIAPLAYBACKEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_MEDIAPLAYBACKEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_MEDIAPLAYBACKEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_MEDIAPLAYBACKEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_SAFERAPPBLOCKED(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SAFERAPPBLOCKED_TIMESTAMP: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SAFERAPPBLOCKED_USERID: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SAFERAPPBLOCKED_PATH: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SAFERAPPBLOCKED_RULEID: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SAFERAPPBLOCKED_CARGS: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(4i32);
impl ::core::marker::Copy for WPC_ARGS_SAFERAPPBLOCKED {}
impl ::core::clone::Clone for WPC_ARGS_SAFERAPPBLOCKED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_SAFERAPPBLOCKED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_SAFERAPPBLOCKED {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_SAFERAPPBLOCKED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_SAFERAPPBLOCKED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_SETTINGSCHANGEEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_CLASS: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_SETTING: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_OWNER: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_OLDVAL: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_NEWVAL: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_REASON: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_OPTIONAL: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_SETTINGSCHANGEEVENT_CARGS: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(7i32);
impl ::core::marker::Copy for WPC_ARGS_SETTINGSCHANGEEVENT {}
impl ::core::clone::Clone for WPC_ARGS_SETTINGSCHANGEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_SETTINGSCHANGEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_SETTINGSCHANGEEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_SETTINGSCHANGEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_SETTINGSCHANGEEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_URLVISITEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_URL: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_APPNAME: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_VERSION: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_REASON: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_RATINGSYSTEMID: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_CATCOUNT: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_CATEGORY: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_URLVISITEVENT_CARGS: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(7i32);
impl ::core::marker::Copy for WPC_ARGS_URLVISITEVENT {}
impl ::core::clone::Clone for WPC_ARGS_URLVISITEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_URLVISITEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_URLVISITEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_URLVISITEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_URLVISITEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_WEBOVERRIDEEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBOVERRIDEEVENT_USERID: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBOVERRIDEEVENT_URL: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBOVERRIDEEVENT_REASON: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBOVERRIDEEVENT_CARGS: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(3i32);
impl ::core::marker::Copy for WPC_ARGS_WEBOVERRIDEEVENT {}
impl ::core::clone::Clone for WPC_ARGS_WEBOVERRIDEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_WEBOVERRIDEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_WEBOVERRIDEEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_WEBOVERRIDEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_WEBOVERRIDEEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_ARGS_WEBSITEVISITEVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_URL: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_DECISION: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_CATEGORIES: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_BLOCKEDCATEGORIES: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_SERIALIZEDAPPLICATION: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_TITLE: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_CONTENTTYPE: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_REFERRER: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_TELEMETRY: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_ARGS_WEBSITEVISITEVENT_CARGS: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(9i32);
impl ::core::marker::Copy for WPC_ARGS_WEBSITEVISITEVENT {}
impl ::core::clone::Clone for WPC_ARGS_WEBSITEVISITEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_ARGS_WEBSITEVISITEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_ARGS_WEBSITEVISITEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_ARGS_WEBSITEVISITEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_WEBSITEVISITEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_MEDIA_EXPLICIT(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_EXPLICIT_FALSE: WPC_MEDIA_EXPLICIT = WPC_MEDIA_EXPLICIT(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_EXPLICIT_TRUE: WPC_MEDIA_EXPLICIT = WPC_MEDIA_EXPLICIT(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_EXPLICIT_UNKNOWN: WPC_MEDIA_EXPLICIT = WPC_MEDIA_EXPLICIT(2i32);
impl ::core::marker::Copy for WPC_MEDIA_EXPLICIT {}
impl ::core::clone::Clone for WPC_MEDIA_EXPLICIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_MEDIA_EXPLICIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_MEDIA_EXPLICIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_MEDIA_EXPLICIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_MEDIA_EXPLICIT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_OTHER: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_DVD: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_RECORDED_TV: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_AUDIO_FILE: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_CD_AUDIO: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_VIDEO_FILE: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_PICTURE_FILE: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_MEDIA_TYPE_MAX: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(7i32);
impl ::core::marker::Copy for WPC_MEDIA_TYPE {}
impl ::core::clone::Clone for WPC_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPC_SETTINGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_EXTENSION_PATH: WPC_SETTINGS = WPC_SETTINGS(0i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_EXTENSION_SILO: WPC_SETTINGS = WPC_SETTINGS(1i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_EXTENSION_IMAGE_PATH: WPC_SETTINGS = WPC_SETTINGS(2i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_EXTENSION_DISABLEDIMAGE_PATH: WPC_SETTINGS = WPC_SETTINGS(3i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_EXTENSION_NAME: WPC_SETTINGS = WPC_SETTINGS(4i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_EXTENSION_SUB_TITLE: WPC_SETTINGS = WPC_SETTINGS(5i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_CURRENT_RATING_SYSTEM: WPC_SETTINGS = WPC_SETTINGS(6i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_LAST_LOG_VIEW: WPC_SETTINGS = WPC_SETTINGS(7i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_LOG_VIEW_REMINDER_INTERVAL: WPC_SETTINGS = WPC_SETTINGS(8i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_HTTP_EXEMPTION_LIST: WPC_SETTINGS = WPC_SETTINGS(9i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_URL_EXEMPTION_LIST: WPC_SETTINGS = WPC_SETTINGS(10i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_FILTER_ID: WPC_SETTINGS = WPC_SETTINGS(11i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_FILTER_NAME: WPC_SETTINGS = WPC_SETTINGS(12i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_SYSTEM_LOCALE: WPC_SETTINGS = WPC_SETTINGS(13i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_ALLOW_BLOCK: WPC_SETTINGS = WPC_SETTINGS(14i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_GAME_BLOCKED: WPC_SETTINGS = WPC_SETTINGS(15i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_GAME_ALLOW_UNRATED: WPC_SETTINGS = WPC_SETTINGS(16i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_GAME_MAX_ALLOWED: WPC_SETTINGS = WPC_SETTINGS(17i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_GAME_DENIED_DESCRIPTORS: WPC_SETTINGS = WPC_SETTINGS(18i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_WPC_ENABLED: WPC_SETTINGS = WPC_SETTINGS(19i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_LOGGING_REQUIRED: WPC_SETTINGS = WPC_SETTINGS(20i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_HOURLY_RESTRICTIONS: WPC_SETTINGS = WPC_SETTINGS(21i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_OVERRRIDE_REQUESTS: WPC_SETTINGS = WPC_SETTINGS(22i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_LOGON_HOURS: WPC_SETTINGS = WPC_SETTINGS(23i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_APP_RESTRICTIONS: WPC_SETTINGS = WPC_SETTINGS(24i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WEB_FILTER_ON: WPC_SETTINGS = WPC_SETTINGS(25i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WEB_DOWNLOAD_BLOCKED: WPC_SETTINGS = WPC_SETTINGS(26i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WEB_FILTER_LEVEL: WPC_SETTINGS = WPC_SETTINGS(27i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WEB_BLOCKED_CATEGORY_LIST: WPC_SETTINGS = WPC_SETTINGS(28i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WEB_BLOCK_UNRATED: WPC_SETTINGS = WPC_SETTINGS(29i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_ENABLED: WPC_SETTINGS = WPC_SETTINGS(30i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_LOGGING_REQUIRED: WPC_SETTINGS = WPC_SETTINGS(31i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_RATING_SYSTEM_PATH: WPC_SETTINGS = WPC_SETTINGS(32i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_WPC_PROVIDER_CURRENT: WPC_SETTINGS = WPC_SETTINGS(33i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_TIME_ALLOWANCE: WPC_SETTINGS = WPC_SETTINGS(34i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_USER_TIME_ALLOWANCE_RESTRICTIONS: WPC_SETTINGS = WPC_SETTINGS(35i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_GAME_RESTRICTED: WPC_SETTINGS = WPC_SETTINGS(36i32);
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTING_COUNT: WPC_SETTINGS = WPC_SETTINGS(37i32);
impl ::core::marker::Copy for WPC_SETTINGS {}
impl ::core::clone::Clone for WPC_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPC_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPC_SETTINGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPC_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_SETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_LOCATE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SETTINGS_MODIFY: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_SYSTEM: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`*"]
pub const WPC_WEB: u32 = 24u32;
pub const WindowsParentalControls: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe77cc89b_7401_4c04_8ced_149db35add04);
pub const WpcProviderSupport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb18c7a0_2186_4be0_97d8_04847b628e02);
pub const WpcSettingsProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x355dffaa_3b9f_435c_b428_5d44290bc5f2);
#[cfg(feature = "implement")]
::core::include!("impl.rs");

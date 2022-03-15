#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_HASSETTINGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_SETTINGSMODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IADesktopP2(::windows::core::IUnknown);
impl IADesktopP2 {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn ReReadWallpaper(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReReadWallpaper)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetADObjectFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwflags), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn MakeDynamicChanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Ole::IOleObject>>(&self, poleobj: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeDynamicChanges)(::core::mem::transmute_copy(self), poleobj.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IADesktopP2> for ::windows::core::IUnknown {
    fn from(value: IADesktopP2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IADesktopP2> for ::windows::core::IUnknown {
    fn from(value: &IADesktopP2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IADesktopP2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IADesktopP2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IADesktopP2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IADesktopP2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADesktopP2 {}
impl ::core::fmt::Debug for IADesktopP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADesktopP2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IADesktopP2 {
    type Vtable = IADesktopP2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22754e2_4574_11d1_9888_006097deacf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ReReadWallpaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetADObjectFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub MakeDynamicChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    MakeDynamicChanges: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IActiveDesktopP(::windows::core::IUnknown);
impl IActiveDesktopP {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSafeMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn EnsureUpdateHTML(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnsureUpdateHTML)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn SetScheme<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszschemename: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScheme)(::core::mem::transmute_copy(self), pwszschemename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn GetScheme(&self, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScheme)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszschemename), ::core::mem::transmute(pdwcchbuffer), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IActiveDesktopP> for ::windows::core::IUnknown {
    fn from(value: IActiveDesktopP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveDesktopP> for ::windows::core::IUnknown {
    fn from(value: &IActiveDesktopP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveDesktopP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActiveDesktopP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveDesktopP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveDesktopP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveDesktopP {}
impl ::core::fmt::Debug for IActiveDesktopP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveDesktopP").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveDesktopP {
    type Vtable = IActiveDesktopP_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetSafeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IBriefcaseInitiator(::windows::core::IUnknown);
impl IBriefcaseInitiator {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsMonikerInBriefcase<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IMoniker>>(&self, pmk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsMonikerInBriefcase)(::core::mem::transmute_copy(self), pmk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBriefcaseInitiator> for ::windows::core::IUnknown {
    fn from(value: IBriefcaseInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBriefcaseInitiator> for ::windows::core::IUnknown {
    fn from(value: &IBriefcaseInitiator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBriefcaseInitiator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBriefcaseInitiator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBriefcaseInitiator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBriefcaseInitiator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBriefcaseInitiator {}
impl ::core::fmt::Debug for IBriefcaseInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBriefcaseInitiator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBriefcaseInitiator {
    type Vtable = IBriefcaseInitiator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99180164_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBriefcaseInitiator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsMonikerInBriefcase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsMonikerInBriefcase: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCache(::windows::core::IUnknown);
impl IEmptyVolumeCache {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn GetSpaceUsed<'a, Param1: ::windows::core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, pdwlspaceused: *mut u64, picb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpaceUsed)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwlspaceused), picb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn Purge<'a, Param1: ::windows::core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, dwlspacetofree: u64, picb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Purge)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlspacetofree), picb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowProperties)(::core::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Deactivate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEmptyVolumeCache> for ::windows::core::IUnknown {
    fn from(value: IEmptyVolumeCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache> for ::windows::core::IUnknown {
    fn from(value: &IEmptyVolumeCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEmptyVolumeCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEmptyVolumeCache {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEmptyVolumeCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCache {}
impl ::core::fmt::Debug for IEmptyVolumeCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCache").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEmptyVolumeCache {
    type Vtable = IEmptyVolumeCache_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fce5227_04da_11d1_a004_00805f8abe06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetSpaceUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowProperties: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCache2(::windows::core::IUnknown);
impl IEmptyVolumeCache2 {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn GetSpaceUsed<'a, Param1: ::windows::core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, pdwlspaceused: *mut u64, picb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSpaceUsed)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwlspaceused), picb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn Purge<'a, Param1: ::windows::core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, dwlspacetofree: u64, picb: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Purge)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlspacetofree), picb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ShowProperties)(::core::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Deactivate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn InitializeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, pcwszkeyname: Param2, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeEx)(::core::mem::transmute_copy(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), pcwszkeyname.into_param().abi(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(ppwszbtntext), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<IEmptyVolumeCache2> for ::windows::core::IUnknown {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache2> for ::windows::core::IUnknown {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IEmptyVolumeCache> for IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::core::Param<'a, IEmptyVolumeCache> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IEmptyVolumeCache> for &'a IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::core::Param<'a, IEmptyVolumeCache> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEmptyVolumeCache2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCache2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCache2 {}
impl ::core::fmt::Debug for IEmptyVolumeCache2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCache2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEmptyVolumeCache2 {
    type Vtable = IEmptyVolumeCache2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b7e3ba_4db3_11d2_b2d9_00c04f8eec8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache2_Vtbl {
    pub base: IEmptyVolumeCache_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, pcwszkeyname: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCacheCallBack(::windows::core::IUnknown);
impl IEmptyVolumeCacheCallBack {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn ScanProgress<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScanProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlspaceused), ::core::mem::transmute(dwflags), pcwszstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn PurgeProgress<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PurgeProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlspacefreed), ::core::mem::transmute(dwlspacetofree), ::core::mem::transmute(dwflags), pcwszstatus.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEmptyVolumeCacheCallBack> for ::windows::core::IUnknown {
    fn from(value: IEmptyVolumeCacheCallBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCacheCallBack> for ::windows::core::IUnknown {
    fn from(value: &IEmptyVolumeCacheCallBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEmptyVolumeCacheCallBack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEmptyVolumeCacheCallBack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEmptyVolumeCacheCallBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCacheCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCacheCallBack {}
impl ::core::fmt::Debug for IEmptyVolumeCacheCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCacheCallBack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEmptyVolumeCacheCallBack {
    type Vtable = IEmptyVolumeCacheCallBack_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e793361_73c6_11d0_8469_00aa00442901);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCacheCallBack_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ScanProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IReconcilableObject(::windows::core::IUnknown);
impl IReconcilableObject {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Reconcile<'a, Param0: ::windows::core::IntoParam<'a, IReconcileInitiator>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param7: ::windows::core::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>>(&self, pinitiator: Param0, dwflags: u32, hwndowner: Param2, hwndprogressfeedback: Param3, rgpmkotherinput: &mut [::core::option::Option<super::super::System::Com::IMoniker>], ploutindex: *mut i32, pstgnewresidues: Param7, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reconcile)(::core::mem::transmute_copy(self), pinitiator.into_param().abi(), ::core::mem::transmute(dwflags), hwndowner.into_param().abi(), hwndprogressfeedback.into_param().abi(), rgpmkotherinput.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgpmkotherinput)), ::core::mem::transmute(ploutindex), pstgnewresidues.into_param().abi(), ::core::mem::transmute(pvreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProgressFeedbackMaxEstimate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IReconcilableObject> for ::windows::core::IUnknown {
    fn from(value: IReconcilableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReconcilableObject> for ::windows::core::IUnknown {
    fn from(value: &IReconcilableObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReconcilableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReconcilableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReconcilableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReconcilableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReconcilableObject {}
impl ::core::fmt::Debug for IReconcilableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReconcilableObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReconcilableObject {
    type Vtable = IReconcilableObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99180162_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcilableObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Reconcile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiator: ::windows::core::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows::core::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows::core::RawPtr, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Reconcile: usize,
    pub GetProgressFeedbackMaxEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IReconcileInitiator(::windows::core::IUnknown);
impl IReconcileInitiator {
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn SetAbortCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkforabort: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAbortCallback)(::core::mem::transmute_copy(self), punkforabort.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProgressFeedback)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax)).ok()
    }
}
impl ::core::convert::From<IReconcileInitiator> for ::windows::core::IUnknown {
    fn from(value: IReconcileInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReconcileInitiator> for ::windows::core::IUnknown {
    fn from(value: &IReconcileInitiator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReconcileInitiator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReconcileInitiator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReconcileInitiator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReconcileInitiator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReconcileInitiator {}
impl ::core::fmt::Debug for IReconcileInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReconcileInitiator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReconcileInitiator {
    type Vtable = IReconcileInitiator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99180161_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcileInitiator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetAbortCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProgressFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_ABORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147217408i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_INEEDTODOTHEUPDATES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147217404i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_NOCALLBACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147217407i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_NORESIDUES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147217406i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_TOODIFFERENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147217405i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_S_IDIDTHEUPDATES: ::windows::core::HRESULT = ::windows::core::HRESULT(266240i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_S_NOTCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(266241i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows::core::HRESULT = ::windows::core::HRESULT(266242i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const STATEBITS_FLAT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _reconcilef(pub i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_MAYBOTHERUSER: _reconcilef = _reconcilef(1i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_FEEDBACKWINDOWVALID: _reconcilef = _reconcilef(2i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_NORESIDUESOK: _reconcilef = _reconcilef(4i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_OMITSELFRESIDUE: _reconcilef = _reconcilef(8i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_RESUMERECONCILIATION: _reconcilef = _reconcilef(16i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_YOUMAYDOTHEUPDATES: _reconcilef = _reconcilef(32i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_ONLYYOUWERECHANGED: _reconcilef = _reconcilef(64i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const ALL_RECONCILE_FLAGS: _reconcilef = _reconcilef(127i32);
impl ::core::marker::Copy for _reconcilef {}
impl ::core::clone::Clone for _reconcilef {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _reconcilef {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _reconcilef {
    type Abi = Self;
}
impl ::core::fmt::Debug for _reconcilef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_reconcilef").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

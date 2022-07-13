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
    pub unsafe fn ReReadWallpaper(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReReadWallpaper)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetADObjectFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwflags), dwmask).ok()
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn MakeDynamicChanges<'a, P0>(&self, poleobj: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Ole::IOleObject>>,
    {
        (::windows::core::Interface::vtable(self).MakeDynamicChanges)(::windows::core::Interface::as_raw(self), poleobj.into().abi()).ok()
    }
}
impl ::core::convert::From<IADesktopP2> for ::windows::core::IUnknown {
    fn from(value: IADesktopP2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IADesktopP2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IADesktopP2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IADesktopP2> for ::windows::core::IUnknown {
    fn from(value: &IADesktopP2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub ReReadWallpaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetADObjectFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub MakeDynamicChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poleobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    MakeDynamicChanges: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IActiveDesktopP(::windows::core::IUnknown);
impl IActiveDesktopP {
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSafeMode)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnsureUpdateHTML)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetScheme<'a, P0>(&self, pwszschemename: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetScheme)(::windows::core::Interface::as_raw(self), pwszschemename.into(), dwflags).ok()
    }
    pub unsafe fn GetScheme(&self, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScheme)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszschemename), ::core::mem::transmute(pdwcchbuffer), dwflags).ok()
    }
}
impl ::core::convert::From<IActiveDesktopP> for ::windows::core::IUnknown {
    fn from(value: IActiveDesktopP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActiveDesktopP> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IActiveDesktopP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveDesktopP> for ::windows::core::IUnknown {
    fn from(value: &IActiveDesktopP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetSafeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IBriefcaseInitiator(::windows::core::IUnknown);
impl IBriefcaseInitiator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsMonikerInBriefcase<'a, P0>(&self, pmk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IMoniker>>,
    {
        (::windows::core::Interface::vtable(self).IsMonikerInBriefcase)(::windows::core::Interface::as_raw(self), pmk.into().abi()).ok()
    }
}
impl ::core::convert::From<IBriefcaseInitiator> for ::windows::core::IUnknown {
    fn from(value: IBriefcaseInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBriefcaseInitiator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBriefcaseInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBriefcaseInitiator> for ::windows::core::IUnknown {
    fn from(value: &IBriefcaseInitiator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsMonikerInBriefcase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsMonikerInBriefcase: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCache(::windows::core::IUnknown);
impl IEmptyVolumeCache {
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, P0, P1>(&self, hkregkey: P0, pcwszvolume: P1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), hkregkey.into(), pcwszvolume.into(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetSpaceUsed<'a, P0>(&self, pdwlspaceused: *mut u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Interface::vtable(self).GetSpaceUsed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwlspaceused), picb.into().abi()).ok()
    }
    pub unsafe fn Purge<'a, P0>(&self, dwlspacetofree: u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Interface::vtable(self).Purge)(::windows::core::Interface::as_raw(self), dwlspacetofree, picb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<'a, P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).ShowProperties)(::windows::core::Interface::as_raw(self), hwnd.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEmptyVolumeCache> for ::windows::core::IUnknown {
    fn from(value: IEmptyVolumeCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEmptyVolumeCache> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEmptyVolumeCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache> for ::windows::core::IUnknown {
    fn from(value: &IEmptyVolumeCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetSpaceUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, P0, P1>(&self, hkregkey: P0, pcwszvolume: P1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), hkregkey.into(), pcwszvolume.into(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetSpaceUsed<'a, P0>(&self, pdwlspaceused: *mut u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Interface::vtable(self).base__.GetSpaceUsed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwlspaceused), picb.into().abi()).ok()
    }
    pub unsafe fn Purge<'a, P0>(&self, dwlspacetofree: u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Interface::vtable(self).base__.Purge)(::windows::core::Interface::as_raw(self), dwlspacetofree, picb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<'a, P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.ShowProperties)(::windows::core::Interface::as_raw(self), hwnd.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Deactivate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn InitializeEx<'a, P0, P1, P2>(&self, hkregkey: P0, pcwszvolume: P1, pcwszkeyname: P2, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InitializeEx)(::windows::core::Interface::as_raw(self), hkregkey.into(), pcwszvolume.into(), pcwszkeyname.into(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(ppwszbtntext), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<IEmptyVolumeCache2> for ::windows::core::IUnknown {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEmptyVolumeCache2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache2> for ::windows::core::IUnknown {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEmptyVolumeCache2> for &'a IEmptyVolumeCache {
    fn from(value: &'a IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: IEmptyVolumeCache_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, pcwszkeyname: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCacheCallBack(::windows::core::IUnknown);
impl IEmptyVolumeCacheCallBack {
    pub unsafe fn ScanProgress<'a, P0>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ScanProgress)(::windows::core::Interface::as_raw(self), dwlspaceused, dwflags, pcwszstatus.into()).ok()
    }
    pub unsafe fn PurgeProgress<'a, P0>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).PurgeProgress)(::windows::core::Interface::as_raw(self), dwlspacefreed, dwlspacetofree, dwflags, pcwszstatus.into()).ok()
    }
}
impl ::core::convert::From<IEmptyVolumeCacheCallBack> for ::windows::core::IUnknown {
    fn from(value: IEmptyVolumeCacheCallBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEmptyVolumeCacheCallBack> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEmptyVolumeCacheCallBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCacheCallBack> for ::windows::core::IUnknown {
    fn from(value: &IEmptyVolumeCacheCallBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub ScanProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IReconcilableObject(::windows::core::IUnknown);
impl IReconcilableObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Reconcile<'a, P0, P1, P2, P3>(&self, pinitiator: P0, dwflags: u32, hwndowner: P1, hwndprogressfeedback: P2, rgpmkotherinput: &mut [::core::option::Option<super::super::System::Com::IMoniker>], ploutindex: *mut i32, pstgnewresidues: P3, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IReconcileInitiator>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<super::super::Foundation::HWND>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::StructuredStorage::IStorage>>,
    {
        (::windows::core::Interface::vtable(self).Reconcile)(::windows::core::Interface::as_raw(self), pinitiator.into().abi(), dwflags, hwndowner.into(), hwndprogressfeedback.into(), rgpmkotherinput.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgpmkotherinput)), ::core::mem::transmute(ploutindex), pstgnewresidues.into().abi(), ::core::mem::transmute(pvreserved)).ok()
    }
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProgressFeedbackMaxEstimate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IReconcilableObject> for ::windows::core::IUnknown {
    fn from(value: IReconcilableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IReconcilableObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IReconcilableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReconcilableObject> for ::windows::core::IUnknown {
    fn from(value: &IReconcilableObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Reconcile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiator: *mut ::core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut ::core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Reconcile: usize,
    pub GetProgressFeedbackMaxEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IReconcileInitiator(::windows::core::IUnknown);
impl IReconcileInitiator {
    pub unsafe fn SetAbortCallback<'a, P0>(&self, punkforabort: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetAbortCallback)(::windows::core::Interface::as_raw(self), punkforabort.into().abi()).ok()
    }
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProgressFeedback)(::windows::core::Interface::as_raw(self), ulprogress, ulprogressmax).ok()
    }
}
impl ::core::convert::From<IReconcileInitiator> for ::windows::core::IUnknown {
    fn from(value: IReconcileInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IReconcileInitiator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IReconcileInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReconcileInitiator> for ::windows::core::IUnknown {
    fn from(value: &IReconcileInitiator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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

#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IADesktopP2(::windows::core::IUnknown);
impl IADesktopP2 {
    pub unsafe fn ReReadWallpaper(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReReadWallpaper)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetADObjectFlags)(::windows::core::Vtable::as_raw(self), pdwflags, dwmask).ok()
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateAllDesktopSubscriptions)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn MakeDynamicChanges<P0>(&self, poleobj: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
    {
        (::windows::core::Vtable::vtable(self).MakeDynamicChanges)(::windows::core::Vtable::as_raw(self), poleobj.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IADesktopP2, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IADesktopP2 {
    type Vtable = IADesktopP2_Vtbl;
}
unsafe impl ::windows::core::Interface for IADesktopP2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22754e2_4574_11d1_9888_006097deacf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
        (::windows::core::Vtable::vtable(self).SetSafeMode)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnsureUpdateHTML)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetScheme<P0>(&self, pwszschemename: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetScheme)(::windows::core::Vtable::as_raw(self), pwszschemename.into().abi(), dwflags).ok()
    }
    pub unsafe fn GetScheme(&self, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetScheme)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszschemename), pdwcchbuffer, dwflags).ok()
    }
}
::windows::core::interface_hierarchy!(IActiveDesktopP, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IActiveDesktopP {
    type Vtable = IActiveDesktopP_Vtbl;
}
unsafe impl ::windows::core::Interface for IActiveDesktopP {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn IsMonikerInBriefcase<P0>(&self, pmk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).IsMonikerInBriefcase)(::windows::core::Vtable::as_raw(self), pmk.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IBriefcaseInitiator, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IBriefcaseInitiator {
    type Vtable = IBriefcaseInitiator_Vtbl;
}
unsafe impl ::windows::core::Interface for IBriefcaseInitiator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99180164_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBriefcaseInitiator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn Initialize<P0, P1>(&self, hkregkey: P0, pcwszvolume: P1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), hkregkey.into(), pcwszvolume.into().abi(), ppwszdisplayname, ppwszdescription, pdwflags).ok()
    }
    pub unsafe fn GetSpaceUsed<P0>(&self, pdwlspaceused: *mut u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Vtable::vtable(self).GetSpaceUsed)(::windows::core::Vtable::as_raw(self), pdwlspaceused, picb.into().abi()).ok()
    }
    pub unsafe fn Purge<P0>(&self, dwlspacetofree: u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Vtable::vtable(self).Purge)(::windows::core::Vtable::as_raw(self), dwlspacetofree, picb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).ShowProperties)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<EMPTY_VOLUME_CACHE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Deactivate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEmptyVolumeCache, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IEmptyVolumeCache {
    type Vtable = IEmptyVolumeCache_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmptyVolumeCache {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fce5227_04da_11d1_a004_00805f8abe06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetSpaceUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowProperties: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCache2(::windows::core::IUnknown);
impl IEmptyVolumeCache2 {
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0, P1>(&self, hkregkey: P0, pcwszvolume: P1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), hkregkey.into(), pcwszvolume.into().abi(), ppwszdisplayname, ppwszdescription, pdwflags).ok()
    }
    pub unsafe fn GetSpaceUsed<P0>(&self, pdwlspaceused: *mut u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSpaceUsed)(::windows::core::Vtable::as_raw(self), pdwlspaceused, picb.into().abi()).ok()
    }
    pub unsafe fn Purge<P0>(&self, dwlspacetofree: u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Purge)(::windows::core::Vtable::as_raw(self), dwlspacetofree, picb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShowProperties)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<EMPTY_VOLUME_CACHE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn InitializeEx<P0, P1, P2>(&self, hkregkey: P0, pcwszvolume: P1, pcwszkeyname: P2, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).InitializeEx)(::windows::core::Vtable::as_raw(self), hkregkey.into(), pcwszvolume.into().abi(), pcwszkeyname.into().abi(), ppwszdisplayname, ppwszdescription, ppwszbtntext, pdwflags).ok()
    }
}
::windows::core::interface_hierarchy!(IEmptyVolumeCache2, ::windows::core::IUnknown, IEmptyVolumeCache);
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
unsafe impl ::windows::core::Vtable for IEmptyVolumeCache2 {
    type Vtable = IEmptyVolumeCache2_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmptyVolumeCache2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b7e3ba_4db3_11d2_b2d9_00c04f8eec8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache2_Vtbl {
    pub base__: IEmptyVolumeCache_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, pcwszkeyname: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IEmptyVolumeCacheCallBack(::windows::core::IUnknown);
impl IEmptyVolumeCacheCallBack {
    pub unsafe fn ScanProgress<P0>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ScanProgress)(::windows::core::Vtable::as_raw(self), dwlspaceused, dwflags, pcwszstatus.into().abi()).ok()
    }
    pub unsafe fn PurgeProgress<P0>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).PurgeProgress)(::windows::core::Vtable::as_raw(self), dwlspacefreed, dwlspacetofree, dwflags, pcwszstatus.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IEmptyVolumeCacheCallBack, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IEmptyVolumeCacheCallBack {
    type Vtable = IEmptyVolumeCacheCallBack_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmptyVolumeCacheCallBack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e793361_73c6_11d0_8469_00aa00442901);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCacheCallBack_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ScanProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
pub struct IReconcilableObject(::windows::core::IUnknown);
impl IReconcilableObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Reconcile<P0, P1, P2, P3>(&self, pinitiator: P0, dwflags: u32, hwndowner: P1, hwndprogressfeedback: P2, rgpmkotherinput: &mut [::core::option::Option<super::super::System::Com::IMoniker>], ploutindex: *mut i32, pstgnewresidues: P3, pvreserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IReconcileInitiator>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<super::super::Foundation::HWND>,
        P3: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::StructuredStorage::IStorage>>,
    {
        (::windows::core::Vtable::vtable(self).Reconcile)(::windows::core::Vtable::as_raw(self), pinitiator.into().abi(), dwflags, hwndowner.into(), hwndprogressfeedback.into(), rgpmkotherinput.len() as _, ::core::mem::transmute(rgpmkotherinput.as_ptr()), ploutindex, pstgnewresidues.into().abi(), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProgressFeedbackMaxEstimate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IReconcilableObject, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IReconcilableObject {
    type Vtable = IReconcilableObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IReconcilableObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99180162_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcilableObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn SetAbortCallback<P0>(&self, punkforabort: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetAbortCallback)(::windows::core::Vtable::as_raw(self), punkforabort.into().abi()).ok()
    }
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProgressFeedback)(::windows::core::Vtable::as_raw(self), ulprogress, ulprogressmax).ok()
    }
}
::windows::core::interface_hierarchy!(IReconcileInitiator, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IReconcileInitiator {
    type Vtable = IReconcileInitiator_Vtbl;
}
unsafe impl ::windows::core::Interface for IReconcileInitiator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99180161_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcileInitiator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAbortCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProgressFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
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
pub struct EMPTY_VOLUME_CACHE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_HASSETTINGS: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_ENABLEBYDEFAULT: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_REMOVEFROMLIST: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_ENABLEBYDEFAULT_AUTO: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_DONTSHOWIFZERO: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_SETTINGSMODE: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_OUTOFDISKSPACE: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_USERCONSENTOBTAINED: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_SYSTEMAUTORUN: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(256u32);
impl ::core::marker::Copy for EMPTY_VOLUME_CACHE_FLAGS {}
impl ::core::clone::Clone for EMPTY_VOLUME_CACHE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMPTY_VOLUME_CACHE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EMPTY_VOLUME_CACHE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EMPTY_VOLUME_CACHE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMPTY_VOLUME_CACHE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RECONCILEF(pub i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_MAYBOTHERUSER: RECONCILEF = RECONCILEF(1i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_FEEDBACKWINDOWVALID: RECONCILEF = RECONCILEF(2i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_NORESIDUESOK: RECONCILEF = RECONCILEF(4i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_OMITSELFRESIDUE: RECONCILEF = RECONCILEF(8i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_RESUMERECONCILIATION: RECONCILEF = RECONCILEF(16i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_YOUMAYDOTHEUPDATES: RECONCILEF = RECONCILEF(32i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_ONLYYOUWERECHANGED: RECONCILEF = RECONCILEF(64i32);
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const ALL_RECONCILE_FLAGS: RECONCILEF = RECONCILEF(127i32);
impl ::core::marker::Copy for RECONCILEF {}
impl ::core::clone::Clone for RECONCILEF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RECONCILEF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RECONCILEF {
    type Abi = Self;
}
impl ::core::fmt::Debug for RECONCILEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RECONCILEF").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_HASSETTINGS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_SETTINGSMODE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IADesktopP2(pub ::windows::runtime::IUnknown);
impl IADesktopP2 {
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn ReReadWallpaper(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwflags), ::std::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_System_Ole`*"]
    pub unsafe fn MakeDynamicChanges<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IOleObject>>(&self, poleobj: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), poleobj.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IADesktopP2 {
    type Vtable = IADesktopP2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2988922082, 17780, 4561, [152, 136, 0, 96, 151, 222, 172, 249]);
}
impl ::std::convert::From<IADesktopP2> for ::windows::runtime::IUnknown {
    fn from(value: IADesktopP2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IADesktopP2> for ::windows::runtime::IUnknown {
    fn from(value: &IADesktopP2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IADesktopP2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IADesktopP2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32, dwmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poleobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IActiveDesktopP(pub ::windows::runtime::IUnknown);
impl IActiveDesktopP {
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn EnsureUpdateHTML(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`*"]
    pub unsafe fn SetScheme<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszschemename: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszschemename.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`*"]
    pub unsafe fn GetScheme(&self, pwszschemename: super::super::Foundation::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszschemename), ::std::mem::transmute(pdwcchbuffer), ::std::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IActiveDesktopP {
    type Vtable = IActiveDesktopP_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1380986592, 60544, 4560, [137, 171, 0, 192, 79, 194, 151, 45]);
}
impl ::std::convert::From<IActiveDesktopP> for ::windows::runtime::IUnknown {
    fn from(value: IActiveDesktopP) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IActiveDesktopP> for ::windows::runtime::IUnknown {
    fn from(value: &IActiveDesktopP) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActiveDesktopP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IActiveDesktopP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszschemename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszschemename: super::super::Foundation::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBriefcaseInitiator(pub ::windows::runtime::IUnknown);
impl IBriefcaseInitiator {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_System_Com`*"]
    pub unsafe fn IsMonikerInBriefcase<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMoniker>>(&self, pmk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pmk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBriefcaseInitiator {
    type Vtable = IBriefcaseInitiator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2568487268, 55830, 4122, [147, 92, 68, 69, 83, 84, 0, 0]);
}
impl ::std::convert::From<IBriefcaseInitiator> for ::windows::runtime::IUnknown {
    fn from(value: IBriefcaseInitiator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBriefcaseInitiator> for ::windows::runtime::IUnknown {
    fn from(value: &IBriefcaseInitiator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBriefcaseInitiator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBriefcaseInitiator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBriefcaseInitiator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEmptyVolumeCache(pub ::windows::runtime::IUnknown);
impl IEmptyVolumeCache {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`, `Win32_System_Registry`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), ::std::mem::transmute(ppwszdisplayname), ::std::mem::transmute(ppwszdescription), ::std::mem::transmute(pdwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn GetSpaceUsed<'a, Param1: ::windows::runtime::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, pdwlspaceused: *mut u64, picb: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwlspaceused), picb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn Purge<'a, Param1: ::windows::runtime::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, dwlspacetofree: u64, picb: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlspacetofree), picb.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`*"]
    pub unsafe fn ShowProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEmptyVolumeCache {
    type Vtable = IEmptyVolumeCache_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2412663335, 1242, 4561, [160, 4, 0, 128, 95, 138, 190, 6]);
}
impl ::std::convert::From<IEmptyVolumeCache> for ::windows::runtime::IUnknown {
    fn from(value: IEmptyVolumeCache) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEmptyVolumeCache> for ::windows::runtime::IUnknown {
    fn from(value: &IEmptyVolumeCache) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEmptyVolumeCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEmptyVolumeCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Registry")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlspaceused: *mut u64, picb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlspacetofree: u64, picb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEmptyVolumeCache2(pub ::windows::runtime::IUnknown);
impl IEmptyVolumeCache2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`, `Win32_System_Registry`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), ::std::mem::transmute(ppwszdisplayname), ::std::mem::transmute(ppwszdescription), ::std::mem::transmute(pdwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn GetSpaceUsed<'a, Param1: ::windows::runtime::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, pdwlspaceused: *mut u64, picb: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwlspaceused), picb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn Purge<'a, Param1: ::windows::runtime::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, dwlspacetofree: u64, picb: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlspacetofree), picb.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`*"]
    pub unsafe fn ShowProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`, `Win32_System_Registry`*"]
    pub unsafe fn InitializeEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        hkregkey: Param0,
        pcwszvolume: Param1,
        pcwszkeyname: Param2,
        ppwszdisplayname: *mut super::super::Foundation::PWSTR,
        ppwszdescription: *mut super::super::Foundation::PWSTR,
        ppwszbtntext: *mut super::super::Foundation::PWSTR,
        pdwflags: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), pcwszkeyname.into_param().abi(), ::std::mem::transmute(ppwszdisplayname), ::std::mem::transmute(ppwszdescription), ::std::mem::transmute(ppwszbtntext), ::std::mem::transmute(pdwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEmptyVolumeCache2 {
    type Vtable = IEmptyVolumeCache2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(45605818, 19891, 4562, [178, 217, 0, 192, 79, 142, 236, 140]);
}
impl ::std::convert::From<IEmptyVolumeCache2> for ::windows::runtime::IUnknown {
    fn from(value: IEmptyVolumeCache2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEmptyVolumeCache2> for ::windows::runtime::IUnknown {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEmptyVolumeCache> for IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEmptyVolumeCache> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEmptyVolumeCache> for &IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEmptyVolumeCache> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Registry")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlspaceused: *mut u64, picb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlspacetofree: u64, picb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, pcwszkeyname: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, ppwszbtntext: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Registry")))] usize,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEmptyVolumeCacheCallBack(pub ::windows::runtime::IUnknown);
impl IEmptyVolumeCacheCallBack {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`*"]
    pub unsafe fn ScanProgress<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlspaceused), ::std::mem::transmute(dwflags), pcwszstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`*"]
    pub unsafe fn PurgeProgress<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlspacefreed), ::std::mem::transmute(dwlspacetofree), ::std::mem::transmute(dwflags), pcwszstatus.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEmptyVolumeCacheCallBack {
    type Vtable = IEmptyVolumeCacheCallBack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1853436769, 29638, 4560, [132, 105, 0, 170, 0, 68, 41, 1]);
}
impl ::std::convert::From<IEmptyVolumeCacheCallBack> for ::windows::runtime::IUnknown {
    fn from(value: IEmptyVolumeCacheCallBack) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEmptyVolumeCacheCallBack> for ::windows::runtime::IUnknown {
    fn from(value: &IEmptyVolumeCacheCallBack) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEmptyVolumeCacheCallBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEmptyVolumeCacheCallBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCacheCallBack_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlspaceused: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IReconcilableObject(pub ::windows::runtime::IUnknown);
impl IReconcilableObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Reconcile<'a, Param0: ::windows::runtime::IntoParam<'a, IReconcileInitiator>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param7: ::windows::runtime::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>>(
        &self,
        pinitiator: Param0,
        dwflags: u32,
        hwndowner: Param2,
        hwndprogressfeedback: Param3,
        ulcinput: u32,
        rgpmkotherinput: *mut ::std::option::Option<super::super::System::Com::IMoniker>,
        ploutindex: *mut i32,
        pstgnewresidues: Param7,
        pvreserved: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pinitiator.into_param().abi(),
            ::std::mem::transmute(dwflags),
            hwndowner.into_param().abi(),
            hwndprogressfeedback.into_param().abi(),
            ::std::mem::transmute(ulcinput),
            ::std::mem::transmute(rgpmkotherinput),
            ::std::mem::transmute(ploutindex),
            pstgnewresidues.into_param().abi(),
            ::std::mem::transmute(pvreserved),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IReconcilableObject {
    type Vtable = IReconcilableObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2568487266, 55830, 4122, [147, 92, 68, 69, 83, 84, 0, 0]);
}
impl ::std::convert::From<IReconcilableObject> for ::windows::runtime::IUnknown {
    fn from(value: IReconcilableObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReconcilableObject> for ::windows::runtime::IUnknown {
    fn from(value: &IReconcilableObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReconcilableObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReconcilableObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcilableObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinitiator: ::windows::runtime::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows::runtime::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows::runtime::RawPtr, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulprogressmax: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IReconcileInitiator(pub ::windows::runtime::IUnknown);
impl IReconcileInitiator {
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn SetAbortCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkforabort: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkforabort.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulprogress), ::std::mem::transmute(ulprogressmax)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReconcileInitiator {
    type Vtable = IReconcileInitiator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2568487265, 55830, 4122, [147, 92, 68, 69, 83, 84, 0, 0]);
}
impl ::std::convert::From<IReconcileInitiator> for ::windows::runtime::IUnknown {
    fn from(value: IReconcileInitiator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReconcileInitiator> for ::windows::runtime::IUnknown {
    fn from(value: &IReconcileInitiator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReconcileInitiator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReconcileInitiator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcileInitiator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkforabort: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulprogress: u32, ulprogressmax: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_ABORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147217408i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_INEEDTODOTHEUPDATES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147217404i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_NOCALLBACK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147217407i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_NORESIDUES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147217406i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_TOODIFFERENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147217405i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_S_IDIDTHEUPDATES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(266240i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_S_NOTCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(266241i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(266242i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const STATEBITS_FLAT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _reconcilef(pub i32);
pub const RECONCILEF_MAYBOTHERUSER: _reconcilef = _reconcilef(1i32);
pub const RECONCILEF_FEEDBACKWINDOWVALID: _reconcilef = _reconcilef(2i32);
pub const RECONCILEF_NORESIDUESOK: _reconcilef = _reconcilef(4i32);
pub const RECONCILEF_OMITSELFRESIDUE: _reconcilef = _reconcilef(8i32);
pub const RECONCILEF_RESUMERECONCILIATION: _reconcilef = _reconcilef(16i32);
pub const RECONCILEF_YOUMAYDOTHEUPDATES: _reconcilef = _reconcilef(32i32);
pub const RECONCILEF_ONLYYOUWERECHANGED: _reconcilef = _reconcilef(64i32);
pub const ALL_RECONCILE_FLAGS: _reconcilef = _reconcilef(127i32);
impl ::std::convert::From<i32> for _reconcilef {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _reconcilef {
    type Abi = Self;
}

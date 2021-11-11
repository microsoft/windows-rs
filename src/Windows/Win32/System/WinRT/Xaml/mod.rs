#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDesktopWindowXamlSourceNative(pub ::windows::runtime::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn WindowHandle(&self) -> ::windows::runtime::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554);
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentwnd: super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDesktopWindowXamlSourceNative2(pub ::windows::runtime::IUnknown);
impl IDesktopWindowXamlSourceNative2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn WindowHandle(&self) -> ::windows::runtime::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn PreTranslateMessage(&self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowXamlSourceNative2 {
    type Vtable = IDesktopWindowXamlSourceNative2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe3dcd8c7_3057_4692_99c3_7b7720afda31);
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative2> for ::windows::runtime::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative2> for ::windows::runtime::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative2> for IDesktopWindowXamlSourceNative {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative2> for IDesktopWindowXamlSourceNative {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDesktopWindowXamlSourceNative> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDesktopWindowXamlSourceNative> for &IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentwnd: super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFindReferenceTargetsCallback(pub ::windows::runtime::IUnknown);
impl IFindReferenceTargetsCallback {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn FoundTrackerTarget<'a, Param0: ::windows::runtime::IntoParam<'a, IReferenceTrackerTarget>>(&self, target: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
impl ::core::convert::From<IFindReferenceTargetsCallback> for ::windows::runtime::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFindReferenceTargetsCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTracker(pub ::windows::runtime::IUnknown);
impl IReferenceTracker {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn FindTrackerTargets<'a, Param0: ::windows::runtime::IntoParam<'a, IFindReferenceTargetsCallback>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn GetReferenceTrackerManager(&self) -> ::windows::runtime::Result<IReferenceTrackerManager> {
        let mut result__: <IReferenceTrackerManager as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IReferenceTrackerManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
impl ::core::convert::From<IReferenceTracker> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTracker> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReferenceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerExtension(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerExtension {}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
impl ::core::convert::From<IReferenceTrackerExtension> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerExtension> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerHost(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerHost {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn GetTrackerTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, unknown: Param0) -> ::windows::runtime::Result<IReferenceTrackerTarget> {
        let mut result__: <IReferenceTrackerTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), unknown.into_param().abi(), &mut result__).from_abi::<IReferenceTrackerTarget>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytesallocated)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytesallocated)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
impl ::core::convert::From<IReferenceTrackerHost> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerHost> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReferenceTrackerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unknown: ::windows::runtime::RawPtr, newreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bytesallocated: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bytesallocated: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerManager(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerManager {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(findfailed)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn SetReferenceTrackerHost<'a, Param0: ::windows::runtime::IntoParam<'a, IReferenceTrackerHost>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
impl ::core::convert::From<IReferenceTrackerManager> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerManager> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTrackerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReferenceTrackerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, findfailed: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerTarget(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerTarget {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn Peg(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn Unpeg(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
impl ::core::convert::From<IReferenceTrackerTarget> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurfaceImageSourceManagerNative(pub ::windows::runtime::IUnknown);
impl ISurfaceImageSourceManagerNative {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn FlushAllSurfacesWithDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4c8798b7_1d88_4a0f_b59b_b93f600de8c8);
}
impl ::core::convert::From<ISurfaceImageSourceManagerNative> for ::windows::runtime::IUnknown {
    fn from(value: ISurfaceImageSourceManagerNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurfaceImageSourceManagerNative> for ::windows::runtime::IUnknown {
    fn from(value: &ISurfaceImageSourceManagerNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurfaceImageSourceNative(pub ::windows::runtime::IUnknown);
impl ISurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf2e9edc1_d307_4525_9886_0fafaa44163c);
}
impl ::core::convert::From<ISurfaceImageSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: ISurfaceImageSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: &ISurfaceImageSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::runtime::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurfaceImageSourceNativeWithD2D(pub ::windows::runtime::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x54298223_41e1_4a41_9c08_02e8256864a1);
}
impl ::core::convert::From<ISurfaceImageSourceNativeWithD2D> for ::windows::runtime::IUnknown {
    fn from(value: ISurfaceImageSourceNativeWithD2D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNativeWithD2D> for ::windows::runtime::IUnknown {
    fn from(value: &ISurfaceImageSourceNativeWithD2D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainBackgroundPanelNative(pub ::windows::runtime::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43bebd4e_add5_4035_8f85_5608d08e9dc9);
}
impl ::core::convert::From<ISwapChainBackgroundPanelNative> for ::windows::runtime::IUnknown {
    fn from(value: ISwapChainBackgroundPanelNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainBackgroundPanelNative> for ::windows::runtime::IUnknown {
    fn from(value: &ISwapChainBackgroundPanelNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainPanelNative(pub ::windows::runtime::IUnknown);
impl ISwapChainPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf92f19d2_3ade_45a6_a20c_f6f1ea90554b);
}
impl ::core::convert::From<ISwapChainPanelNative> for ::windows::runtime::IUnknown {
    fn from(value: ISwapChainPanelNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainPanelNative> for ::windows::runtime::IUnknown {
    fn from(value: &ISwapChainPanelNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISwapChainPanelNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISwapChainPanelNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainPanelNative2(pub ::windows::runtime::IUnknown);
impl ISwapChainPanelNative2 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn SetSwapChainHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, swapchainhandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), swapchainhandle.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5a2f60c_37b2_44a2_937b_8d8eb9726821);
}
impl ::core::convert::From<ISwapChainPanelNative2> for ::windows::runtime::IUnknown {
    fn from(value: ISwapChainPanelNative2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainPanelNative2> for ::windows::runtime::IUnknown {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISwapChainPanelNative2> for ISwapChainPanelNative {
    fn from(value: ISwapChainPanelNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainPanelNative2> for ISwapChainPanelNative {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISwapChainPanelNative> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISwapChainPanelNative> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISwapChainPanelNative> for &ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISwapChainPanelNative> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITrackerOwner(pub ::windows::runtime::IUnknown);
impl ITrackerOwner {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn CreateTrackerHandle(&self) -> ::windows::runtime::Result<*mut TrackerHandle__> {
        let mut result__: <*mut TrackerHandle__ as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut TrackerHandle__>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn SetTrackerValue<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, handle: *const TrackerHandle__, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::core::option::Option<::windows::runtime::IUnknown>) -> u8 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(returnvalue)))
    }
}
unsafe impl ::windows::runtime::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
impl ::core::convert::From<ITrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, returnvalue: *mut *mut TrackerHandle__) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: *const TrackerHandle__) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: *const TrackerHandle__, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: *const TrackerHandle__, returnvalue: *mut ::windows::runtime::RawPtr) -> u8,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVirtualSurfaceImageSourceNative(pub ::windows::runtime::IUnknown);
impl IVirtualSurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn Invalidate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), updaterect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn GetUpdateRectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn GetUpdateRects(&self, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(updates), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`, `Win32_Foundation`*"]
    pub unsafe fn GetVisibleBounds(&self) -> ::windows::runtime::Result<super::super::super::Foundation::RECT> {
        let mut result__: <super::super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn RegisterForUpdatesNeeded<'a, Param0: ::windows::runtime::IntoParam<'a, IVirtualSurfaceUpdatesCallbackNative>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newwidth), ::core::mem::transmute(newheight)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe9550983_360b_4f53_b391_afd695078691);
}
impl ::core::convert::From<IVirtualSurfaceImageSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVirtualSurfaceImageSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVirtualSurfaceImageSourceNative> for ISurfaceImageSourceNative {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualSurfaceImageSourceNative> for ISurfaceImageSourceNative {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISurfaceImageSourceNative> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISurfaceImageSourceNative> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISurfaceImageSourceNative> for &IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISurfaceImageSourceNative> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::runtime::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bounds: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newwidth: i32, newheight: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVirtualSurfaceUpdatesCallbackNative(pub ::windows::runtime::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    #[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
    pub unsafe fn UpdatesNeeded(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdbf2e947_8e6c_4254_9eee_7738f71386c9);
}
impl ::core::convert::From<IVirtualSurfaceUpdatesCallbackNative> for ::windows::runtime::IUnknown {
    fn from(value: IVirtualSurfaceUpdatesCallbackNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVirtualSurfaceUpdatesCallbackNative> for ::windows::runtime::IUnknown {
    fn from(value: &IVirtualSurfaceUpdatesCallbackNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl TrackerHandle__ {}
impl ::core::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TrackerHandle__").field("unused", &self.unused).finish()
    }
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for TrackerHandle__ {}
unsafe impl ::windows::runtime::Abi for TrackerHandle__ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT_Xaml`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XAML_REFERENCETRACKER_DISCONNECT(pub i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(1i32);
impl ::core::convert::From<i32> for XAML_REFERENCETRACKER_DISCONNECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XAML_REFERENCETRACKER_DISCONNECT {
    type Abi = Self;
}

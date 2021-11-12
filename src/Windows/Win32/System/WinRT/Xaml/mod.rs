#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDesktopWindowXamlSourceNative(pub ::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554);
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDesktopWindowXamlSourceNative2(pub ::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn PreTranslateMessage(&self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative2 {
    type Vtable = IDesktopWindowXamlSourceNative2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3dcd8c7_3057_4692_99c3_7b7720afda31);
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative2> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative2> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDesktopWindowXamlSourceNative> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDesktopWindowXamlSourceNative> for &IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFindReferenceTargetsCallback(pub ::windows::core::IUnknown);
impl IFindReferenceTargetsCallback {
    pub unsafe fn FoundTrackerTarget<'a, Param0: ::windows::core::IntoParam<'a, IReferenceTrackerTarget>>(&self, target: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
impl ::core::convert::From<IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTracker(pub ::windows::core::IUnknown);
impl IReferenceTracker {
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn FindTrackerTargets<'a, Param0: ::windows::core::IntoParam<'a, IFindReferenceTargetsCallback>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn GetReferenceTrackerManager(&self) -> ::windows::core::Result<IReferenceTrackerManager> {
        let mut result__: <IReferenceTrackerManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IReferenceTrackerManager>(result__)
    }
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
impl ::core::convert::From<IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerExtension(pub ::windows::core::IUnknown);
impl IReferenceTrackerExtension {}
unsafe impl ::windows::core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
impl ::core::convert::From<IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerHost(pub ::windows::core::IUnknown);
impl IReferenceTrackerHost {
    pub unsafe fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetTrackerTarget<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, unknown: Param0) -> ::windows::core::Result<IReferenceTrackerTarget> {
        let mut result__: <IReferenceTrackerTarget as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), unknown.into_param().abi(), &mut result__).from_abi::<IReferenceTrackerTarget>(result__)
    }
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytesallocated)).ok()
    }
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytesallocated)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
impl ::core::convert::From<IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, unknown: ::windows::core::RawPtr, newreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bytesallocated: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bytesallocated: u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerManager(pub ::windows::core::IUnknown);
impl IReferenceTrackerManager {
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(findfailed)).ok()
    }
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetReferenceTrackerHost<'a, Param0: ::windows::core::IntoParam<'a, IReferenceTrackerHost>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
impl ::core::convert::From<IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, findfailed: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceTrackerTarget(pub ::windows::core::IUnknown);
impl IReferenceTrackerTarget {
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Peg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unpeg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
impl ::core::convert::From<IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurfaceImageSourceManagerNative(pub ::windows::core::IUnknown);
impl ISurfaceImageSourceManagerNative {
    pub unsafe fn FlushAllSurfacesWithDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8798b7_1d88_4a0f_b59b_b93f600de8c8);
}
impl ::core::convert::From<ISurfaceImageSourceManagerNative> for ::windows::core::IUnknown {
    fn from(value: ISurfaceImageSourceManagerNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurfaceImageSourceManagerNative> for ::windows::core::IUnknown {
    fn from(value: &ISurfaceImageSourceManagerNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurfaceImageSourceNative(pub ::windows::core::IUnknown);
impl ISurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e9edc1_d307_4525_9886_0fafaa44163c);
}
impl ::core::convert::From<ISurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: ISurfaceImageSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: &ISurfaceImageSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurfaceImageSourceNativeWithD2D(pub ::windows::core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    pub unsafe fn SetDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54298223_41e1_4a41_9c08_02e8256864a1);
}
impl ::core::convert::From<ISurfaceImageSourceNativeWithD2D> for ::windows::core::IUnknown {
    fn from(value: ISurfaceImageSourceNativeWithD2D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNativeWithD2D> for ::windows::core::IUnknown {
    fn from(value: &ISurfaceImageSourceNativeWithD2D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainBackgroundPanelNative(pub ::windows::core::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bebd4e_add5_4035_8f85_5608d08e9dc9);
}
impl ::core::convert::From<ISwapChainBackgroundPanelNative> for ::windows::core::IUnknown {
    fn from(value: ISwapChainBackgroundPanelNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainBackgroundPanelNative> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainBackgroundPanelNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainPanelNative(pub ::windows::core::IUnknown);
impl ISwapChainPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92f19d2_3ade_45a6_a20c_f6f1ea90554b);
}
impl ::core::convert::From<ISwapChainPanelNative> for ::windows::core::IUnknown {
    fn from(value: ISwapChainPanelNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainPanelNative> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainPanelNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISwapChainPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainPanelNative2(pub ::windows::core::IUnknown);
impl ISwapChainPanelNative2 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSwapChainHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, swapchainhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), swapchainhandle.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5a2f60c_37b2_44a2_937b_8d8eb9726821);
}
impl ::core::convert::From<ISwapChainPanelNative2> for ::windows::core::IUnknown {
    fn from(value: ISwapChainPanelNative2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainPanelNative2> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ISwapChainPanelNative> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISwapChainPanelNative> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISwapChainPanelNative> for &ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISwapChainPanelNative> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITrackerOwner(pub ::windows::core::IUnknown);
impl ITrackerOwner {
    pub unsafe fn CreateTrackerHandle(&self) -> ::windows::core::Result<*mut TrackerHandle__> {
        let mut result__: <*mut TrackerHandle__ as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut TrackerHandle__>(result__)
    }
    pub unsafe fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle)).ok()
    }
    pub unsafe fn SetTrackerValue<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, handle: *const TrackerHandle__, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), value.into_param().abi()).ok()
    }
    pub unsafe fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(returnvalue)))
    }
}
unsafe impl ::windows::core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
impl ::core::convert::From<ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, returnvalue: *mut *mut TrackerHandle__) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handle: *const TrackerHandle__) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handle: *const TrackerHandle__, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handle: *const TrackerHandle__, returnvalue: *mut ::windows::core::RawPtr) -> u8,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVirtualSurfaceImageSourceNative(pub ::windows::core::IUnknown);
impl IVirtualSurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Invalidate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), updaterect.into_param().abi()).ok()
    }
    pub unsafe fn GetUpdateRectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUpdateRects(&self, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(updates), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVisibleBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::RECT> {
        let mut result__: <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::RECT>(result__)
    }
    pub unsafe fn RegisterForUpdatesNeeded<'a, Param0: ::windows::core::IntoParam<'a, IVirtualSurfaceUpdatesCallbackNative>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newwidth), ::core::mem::transmute(newheight)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9550983_360b_4f53_b391_afd695078691);
}
impl ::core::convert::From<IVirtualSurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVirtualSurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ISurfaceImageSourceNative> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ISurfaceImageSourceNative> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISurfaceImageSourceNative> for &IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ISurfaceImageSourceNative> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updaterect: super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bounds: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newwidth: i32, newheight: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVirtualSurfaceUpdatesCallbackNative(pub ::windows::core::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    pub unsafe fn UpdatesNeeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbf2e947_8e6c_4254_9eee_7738f71386c9);
}
impl ::core::convert::From<IVirtualSurfaceUpdatesCallbackNative> for ::windows::core::IUnknown {
    fn from(value: IVirtualSurfaceUpdatesCallbackNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVirtualSurfaceUpdatesCallbackNative> for ::windows::core::IUnknown {
    fn from(value: &IVirtualSurfaceUpdatesCallbackNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for TrackerHandle__ {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for XAML_REFERENCETRACKER_DISCONNECT {
    type Abi = Self;
}

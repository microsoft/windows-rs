#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowXamlSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowXamlSourceNative {}
impl ::core::fmt::Debug for IDesktopWindowXamlSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowXamlSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNativeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative2(::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative2 {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn PreTranslateMessage(&self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(result)).ok()
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
impl ::core::convert::From<IDesktopWindowXamlSourceNative2> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative2> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowXamlSourceNative2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowXamlSourceNative2 {}
impl ::core::fmt::Debug for IDesktopWindowXamlSourceNative2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowXamlSourceNative2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative2 {
    type Vtable = IDesktopWindowXamlSourceNative2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3dcd8c7_3057_4692_99c3_7b7720afda31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IFindReferenceTargetsCallback(::windows::core::IUnknown);
impl IFindReferenceTargetsCallback {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn FoundTrackerTarget<'a, Param0: ::windows::core::IntoParam<'a, IReferenceTrackerTarget>>(&self, target: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFindReferenceTargetsCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFindReferenceTargetsCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindReferenceTargetsCallback {}
impl ::core::fmt::Debug for IFindReferenceTargetsCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFindReferenceTargetsCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IReferenceTracker(::windows::core::IUnknown);
impl IReferenceTracker {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn FindTrackerTargets<'a, Param0: ::windows::core::IntoParam<'a, IFindReferenceTargetsCallback>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn GetReferenceTrackerManager(&self) -> ::windows::core::Result<IReferenceTrackerManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IReferenceTrackerManager>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTracker {}
impl ::core::fmt::Debug for IReferenceTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTracker {
    type Vtable = IReferenceTrackerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IReferenceTrackerExtension(::windows::core::IUnknown);
impl IReferenceTrackerExtension {}
impl ::core::convert::From<IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerExtension {}
impl ::core::fmt::Debug for IReferenceTrackerExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtensionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtensionVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IReferenceTrackerHost(::windows::core::IUnknown);
impl IReferenceTrackerHost {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn GetTrackerTarget<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, unknown: Param0) -> ::windows::core::Result<IReferenceTrackerTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), unknown.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IReferenceTrackerTarget>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytesallocated)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytesallocated)).ok()
    }
}
impl ::core::convert::From<IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerHost {}
impl ::core::fmt::Debug for IReferenceTrackerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHostVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut ::core::ffi::c_void, newreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IReferenceTrackerManager(::windows::core::IUnknown);
impl IReferenceTrackerManager {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(findfailed)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn SetReferenceTrackerHost<'a, Param0: ::windows::core::IntoParam<'a, IReferenceTrackerHost>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IReferenceTrackerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerManager {}
impl ::core::fmt::Debug for IReferenceTrackerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findfailed: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IReferenceTrackerTarget(::windows::core::IUnknown);
impl IReferenceTrackerTarget {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn Peg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn Unpeg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerTarget {}
impl ::core::fmt::Debug for IReferenceTrackerTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTargetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTargetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ISurfaceImageSourceManagerNative(::windows::core::IUnknown);
impl ISurfaceImageSourceManagerNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn FlushAllSurfacesWithDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISurfaceImageSourceManagerNative> for ::windows::core::IUnknown {
    fn from(value: ISurfaceImageSourceManagerNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurfaceImageSourceManagerNative> for ::windows::core::IUnknown {
    fn from(value: &ISurfaceImageSourceManagerNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurfaceImageSourceManagerNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurfaceImageSourceManagerNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfaceImageSourceManagerNative {}
impl ::core::fmt::Debug for ISurfaceImageSourceManagerNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfaceImageSourceManagerNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8798b7_1d88_4a0f_b59b_b93f600de8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNativeVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ISurfaceImageSourceNative(::windows::core::IUnknown);
impl ISurfaceImageSourceNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Graphics_Dxgi'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation', 'Win32_Graphics_Dxgi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: ISurfaceImageSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: &ISurfaceImageSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurfaceImageSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurfaceImageSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfaceImageSourceNative {}
impl ::core::fmt::Debug for ISurfaceImageSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfaceImageSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e9edc1_d307_4525_9886_0fafaa44163c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ISurfaceImageSourceNativeWithD2D(::windows::core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISurfaceImageSourceNativeWithD2D> for ::windows::core::IUnknown {
    fn from(value: ISurfaceImageSourceNativeWithD2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNativeWithD2D> for ::windows::core::IUnknown {
    fn from(value: &ISurfaceImageSourceNativeWithD2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurfaceImageSourceNativeWithD2D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurfaceImageSourceNativeWithD2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfaceImageSourceNativeWithD2D {}
impl ::core::fmt::Debug for ISurfaceImageSourceNativeWithD2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfaceImageSourceNativeWithD2D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54298223_41e1_4a41_9c08_02e8256864a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ISwapChainBackgroundPanelNative(::windows::core::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Graphics_Dxgi'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainBackgroundPanelNative> for ::windows::core::IUnknown {
    fn from(value: ISwapChainBackgroundPanelNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainBackgroundPanelNative> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainBackgroundPanelNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainBackgroundPanelNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainBackgroundPanelNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainBackgroundPanelNative {}
impl ::core::fmt::Debug for ISwapChainBackgroundPanelNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainBackgroundPanelNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bebd4e_add5_4035_8f85_5608d08e9dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNativeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ISwapChainPanelNative(::windows::core::IUnknown);
impl ISwapChainPanelNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Graphics_Dxgi'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainPanelNative> for ::windows::core::IUnknown {
    fn from(value: ISwapChainPanelNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainPanelNative> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainPanelNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISwapChainPanelNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainPanelNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainPanelNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainPanelNative {}
impl ::core::fmt::Debug for ISwapChainPanelNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainPanelNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92f19d2_3ade_45a6_a20c_f6f1ea90554b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNativeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ISwapChainPanelNative2(::windows::core::IUnknown);
impl ISwapChainPanelNative2 {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Graphics_Dxgi'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSwapChainHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, swapchainhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), swapchainhandle.into_param().abi()).ok()
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
impl ::core::convert::From<ISwapChainPanelNative2> for ::windows::core::IUnknown {
    fn from(value: ISwapChainPanelNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainPanelNative2> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainPanelNative2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainPanelNative2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainPanelNative2 {}
impl ::core::fmt::Debug for ISwapChainPanelNative2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainPanelNative2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5a2f60c_37b2_44a2_937b_8d8eb9726821);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct ITrackerOwner(::windows::core::IUnknown);
impl ITrackerOwner {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn CreateTrackerHandle(&self) -> ::windows::core::Result<*mut TrackerHandle__> {
        let mut result__: *mut TrackerHandle__ = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TrackerHandle__>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn SetTrackerValue<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, handle: *const TrackerHandle__, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(returnvalue)))
    }
}
impl ::core::convert::From<ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrackerOwner {}
impl ::core::fmt::Debug for ITrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrackerOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwnerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwnerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, returnvalue: *mut *mut TrackerHandle__) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceNative(::windows::core::IUnknown);
impl IVirtualSurfaceImageSourceNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Graphics_Dxgi'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation', 'Win32_Graphics_Dxgi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Invalidate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, updaterect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), updaterect.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn GetUpdateRectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUpdateRects(&self, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(updates), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVisibleBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::RECT> {
        let mut result__: super::super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn RegisterForUpdatesNeeded<'a, Param0: ::windows::core::IntoParam<'a, IVirtualSurfaceUpdatesCallbackNative>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newwidth), ::core::mem::transmute(newheight)).ok()
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
impl ::core::convert::From<IVirtualSurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualSurfaceImageSourceNative> for ::windows::core::IUnknown {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVirtualSurfaceImageSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVirtualSurfaceImageSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualSurfaceImageSourceNative {}
impl ::core::fmt::Debug for IVirtualSurfaceImageSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualSurfaceImageSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9550983_360b_4f53_b391_afd695078691);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNativeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newwidth: i32, newheight: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
#[repr(transparent)]
pub struct IVirtualSurfaceUpdatesCallbackNative(::windows::core::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
    pub unsafe fn UpdatesNeeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IVirtualSurfaceUpdatesCallbackNative> for ::windows::core::IUnknown {
    fn from(value: IVirtualSurfaceUpdatesCallbackNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualSurfaceUpdatesCallbackNative> for ::windows::core::IUnknown {
    fn from(value: &IVirtualSurfaceUpdatesCallbackNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVirtualSurfaceUpdatesCallbackNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVirtualSurfaceUpdatesCallbackNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualSurfaceUpdatesCallbackNative {}
impl ::core::fmt::Debug for IVirtualSurfaceUpdatesCallbackNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualSurfaceUpdatesCallbackNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbf2e947_8e6c_4254_9eee_7738f71386c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNativeVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for TrackerHandle__ {}
impl ::core::clone::Clone for TrackerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TrackerHandle__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for TrackerHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TrackerHandle__>()) == 0 }
    }
}
impl ::core::cmp::Eq for TrackerHandle__ {}
impl ::core::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
pub type XAML_REFERENCETRACKER_DISCONNECT = i32;
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = 0i32;
#[doc = "*Required features: 'Win32_System_WinRT_Xaml'*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = 1i32;

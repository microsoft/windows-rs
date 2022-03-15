#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct HostMessageReceivedCallback(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HostMessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = HostMessageReceivedCallbackBox::<F> { vtable: &HostMessageReceivedCallbackBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct HostMessageReceivedCallbackBox<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const HostMessageReceivedCallback_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> HostMessageReceivedCallbackBox<F> {
    const VTABLE: HostMessageReceivedCallback_Vtbl = HostMessageReceivedCallback_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<HostMessageReceivedCallback as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&receiverid), ::core::mem::transmute(&message)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for HostMessageReceivedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for HostMessageReceivedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for HostMessageReceivedCallback {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for HostMessageReceivedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostMessageReceivedCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for HostMessageReceivedCallback {
    type Vtable = HostMessageReceivedCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf26ffa_8ce1_4cc1_b278_322d31a5e4a3);
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for HostMessageReceivedCallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{faf26ffa-8ce1-4cc1-b278-322d31a5e4a3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct HostMessageReceivedCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d24597_c328_4467_b37f_4dfc6f60b6bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartProcessSilentlyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProcessSilentlyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartProcessSilentlyWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProcessSilentlyWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostfolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFolderWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostfolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestoptions: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFolderWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFileWithUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFileWithUIAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFileWithUIAndTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFileWithUIAndTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TerminateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TerminateWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TerminateWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, messagereceivedcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterMessageReceiver: usize,
    pub UnregisterMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment2 {
    type Vtable = IIsolatedWindowsEnvironment2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d365f39_88bd_4ab4_93cf_7e2bcef337c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiverAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiverWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiverWithTelemetryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment3 {
    type Vtable = IIsolatedWindowsEnvironment3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb7fc7d2_d06e_4c26_8ada_dacdaaad03f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShareFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFileWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFileWithTelemetryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentCreateResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9a5e58_dcd7_45c2_9c85_ab642a715e8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Environment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFactory {
    type Vtable = IIsolatedWindowsEnvironmentFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aca93e7_e804_454d_8466_f9897c20b0f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithTelemetryAsync: usize,
    pub GetById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindByOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentownerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindByOwnerId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d5ae1ef_029f_4101_8c35_fe91bf9cd5f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HostPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFile2 {
    type Vtable = IIsolatedWindowsEnvironmentFile2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4eeb8dec_ad5d_4b0a_b754_f36c3d46d684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GuestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentHostStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentHostStatics {
    type Vtable = IIsolatedWindowsEnvironmentHostStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0e22c7_05a0_517a_b81c_6ee8790c381f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x685d4176_f6e0_4569_b1aa_215c0ff5b257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71d98f7_61f0_4008_b207_0bf9eb2d76f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnvironmentOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetEnvironmentOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AllowedClipboardFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub SetAllowedClipboardFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub ClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT,
    pub SetClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT,
    pub AvailablePrinters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT,
    pub SetAvailablePrinters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT,
    pub SharedHostFolderPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SharedFolderNameInEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShareHostFolderForUntrustedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharedhostfolderpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharefoldernameinenvironment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PersistUserProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPersistUserProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions2 {
    type Vtable = IIsolatedWindowsEnvironmentOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10d7cc31_8b8f_4b9d_b22c_617103b55b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WindowAnnotationOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetWindowAnnotationOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf888ec22_e8cf_56c0_b1df_90af4ad80e84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ShareableFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShareableFolders: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessesRunnableAsSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessesRunnableAsSystem: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessesRunnableAsUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessesRunnableAsUser: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivationFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivationFileExtensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dab9451_6169_55df_8f51_790e99d7277d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10951754_204b_5ec9_9de3_df792d074a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ownerregistrationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dfa28fa_2ef0_4d8f_b341_3171b2df93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentProcess(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa858c3ef_8172_4f10_af93_cbe60af88d09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub WaitForExit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForExitWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutmilliseconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WaitForExitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForExitAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9190ed8_0fd0_4946_bb88_117a60737b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec7caa7_9ac6_4bf5_8b91_5c1adf0d7d00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc405eb7d_7053_4f6a_9b87_746846ed19b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x556ba72e_ca9d_4211_b143_1cedc86eb2fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa1dc2f_57da_4bb5_9c06_fa072d2032e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebdb3cab_7a3a_4524_a0f4_f96e284d33cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentUserInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9c75ae_69ba_4001_96fc_19a02703b340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnvironmentUserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EnvironmentUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryWaitForSignInAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryWaitForSignInAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsHostMessengerStatics {
    type Vtable = IIsolatedWindowsHostMessengerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06e444bb_53c0_4889_8fa3_53592e37cf21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiver: usize,
    pub GetFileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIsolatedWindowsHostMessengerStatics2 {
    type Vtable = IIsolatedWindowsHostMessengerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ef9ebc_0444_42ad_832d_1b89c089d1ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterHostMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, hostmessagereceivedcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterHostMessageReceiver: usize,
    pub UnregisterHostMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironment(::windows::core::IUnknown);
impl IsolatedWindowsEnvironment {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartProcessSilentlyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, hostexepath: Param0, arguments: Param1, activator: IsolatedWindowsEnvironmentActivator) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartProcessSilentlyAsync)(::core::mem::transmute_copy(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartProcessSilentlyWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, hostexepath: Param0, arguments: Param1, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartProcessSilentlyWithTelemetryAsync)(::core::mem::transmute_copy(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>>(&self, hostfolder: Param0, requestoptions: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShareFolderAsync)(::core::mem::transmute_copy(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFolderWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>, Param2: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, hostfolder: Param0, requestoptions: Param1, telemetryparameters: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShareFolderWithTelemetryAsync)(::core::mem::transmute_copy(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFileWithUIAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appexepath: Param0, argumentstemplate: Param1, filepath: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFileWithUIAsync)(::core::mem::transmute_copy(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFileWithUIAndTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, appexepath: Param0, argumentstemplate: Param1, filepath: Param2, telemetryparameters: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFileWithUIAndTelemetryAsync)(::core::mem::transmute_copy(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TerminateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TerminateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TerminateWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, telemetryparameters: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TerminateWithTelemetryAsync)(::core::mem::transmute_copy(this), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, MessageReceivedCallback>>(&self, receiverid: Param0, messagereceivedcallback: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterMessageReceiver)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), messagereceivedcallback.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn UnregisterMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, receiverid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnregisterMessageReceiver)(::core::mem::transmute_copy(this), receiverid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiverAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PostMessageToReceiverAsync)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiverWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, Param2: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, receiverid: Param0, message: Param1, telemetryparameters: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PostMessageToReceiverWithTelemetryAsync)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn GetUserInfo(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentUserInfo> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUserInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentUserInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>>(&self, filepath: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShareFileAsync)(::core::mem::transmute_copy(this), filepath.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFileWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>, Param2: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, filepath: Param0, options: Param1, telemetryparameters: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShareFileWithTelemetryAsync)(::core::mem::transmute_copy(this), filepath.into_param().abi(), options.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentOptions>>(options: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentOptions>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(options: Param0, telemetryparameters: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithTelemetryAsync)(::core::mem::transmute_copy(this), options.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(environmentid: Param0) -> ::windows::core::Result<IsolatedWindowsEnvironment> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetById)(::core::mem::transmute_copy(this), environmentid.into_param().abi(), &mut result__).from_abi::<IsolatedWindowsEnvironment>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindByOwnerId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(environmentownerid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindByOwnerId)(::core::mem::transmute_copy(this), environmentownerid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsEnvironmentFactory<R, F: FnOnce(&IIsolatedWindowsEnvironmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironment, IIsolatedWindowsEnvironmentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironment {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironment;{41d24597-c328-4467-b37f-4dfc6f60b6bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironment";
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironment {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironment {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
impl IsolatedWindowsEnvironmentActivator {
    pub const System: Self = Self(0i32);
    pub const User: Self = Self(1i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentActivator {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentActivator {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentActivator {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentActivator {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentActivator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAllowedClipboardFormats {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAllowedClipboardFormats").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: Self = Self(0u32);
    pub const Local: Self = Self(1u32);
    pub const Network: Self = Self(2u32);
    pub const SystemPrintToPdf: Self = Self(4u32);
    pub const SystemPrintToXps: Self = Self(8u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAvailablePrinters {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAvailablePrinters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentAvailablePrinters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentAvailablePrinters {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentAvailablePrinters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAvailablePrinters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentAvailablePrinters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: Self = Self(0u32);
    pub const HostToIsolatedWindowsEnvironment: Self = Self(1u32);
    pub const IsolatedWindowsEnvironmentToHost: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentClipboardCopyPasteDirections").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsEnvironmentCreateProgress {
    pub State: IsolatedWindowsEnvironmentProgressState,
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedWindowsEnvironmentCreateProgress").field("State", &self.State).field("PercentComplete", &self.PercentComplete).finish()
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreateProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress;enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolatedWindowsEnvironmentCreateProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentCreateResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentCreateStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentCreateStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentCreateStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Environment(&self) -> ::windows::core::Result<IsolatedWindowsEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Environment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironment>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult;{ef9a5e58-dcd7-45c2-9c85-ab642a715e8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentCreateResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentCreateResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentCreateResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: Self = Self(0i32);
    pub const FailureByPolicy: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentFile {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn HostPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn GuestPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GuestPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentFile {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentFile;{4d5ae1ef-029f-4101-8c35-fe91bf9cd5f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentFile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentFile {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentFile {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsEnvironmentHost {}
impl IsolatedWindowsEnvironmentHost {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn IsReady() -> ::windows::core::Result<bool> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReady)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostErrors() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostErrors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsEnvironmentHostStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentHostStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentHost, IIsolatedWindowsEnvironmentHostStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentHost {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: Self = Self(0i32);
    pub const FeatureNotInstalled: Self = Self(1i32);
    pub const HardwareRequirementsNotMet: Self = Self(2i32);
    pub const RebootRequired: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentHostError {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentHostError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentHostError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentHostError {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentHostError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentHostError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentHostError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentLaunchFileResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentLaunchFileStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentLaunchFileStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentLaunchFileStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentLaunchFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentLaunchFileResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult;{685d4176-f6e0-4569-b1aa-215c0ff5b257})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentLaunchFileResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentLaunchFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentLaunchFileResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const TimedOut: Self = Self(4i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentLaunchFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentLaunchFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOptions(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn EnvironmentOwnerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnvironmentOwnerId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetEnvironmentOwnerId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnvironmentOwnerId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn AllowedClipboardFormats(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentAllowedClipboardFormats = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowedClipboardFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentAllowedClipboardFormats>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowedClipboardFormats)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ClipboardCopyPasteDirections(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClipboardCopyPasteDirections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClipboardCopyPasteDirections)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn AvailablePrinters(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAvailablePrinters> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentAvailablePrinters = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailablePrinters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentAvailablePrinters>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAvailablePrinters)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SharedHostFolderPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SharedHostFolderPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SharedFolderNameInEnvironment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SharedFolderNameInEnvironment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ShareHostFolderForUntrustedItems<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, sharedhostfolderpath: Param0, sharefoldernameinenvironment: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShareHostFolderForUntrustedItems)(::core::mem::transmute_copy(this), sharedhostfolderpath.into_param().abi(), sharefoldernameinenvironment.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn PersistUserProfile(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PersistUserProfile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetPersistUserProfile(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPersistUserProfile)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn AllowGraphicsHardwareAcceleration(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowGraphicsHardwareAcceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowGraphicsHardwareAcceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn AllowCameraAndMicrophoneAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowCameraAndMicrophoneAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowCameraAndMicrophoneAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn WindowAnnotationOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowAnnotationOverride)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetWindowAnnotationOverride<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWindowAnnotationOverride)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions;{b71d98f7-61f0-4008-b207-0bf9eb2d76f2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOptions {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsEnvironmentOwnerRegistration {}
impl IsolatedWindowsEnvironmentOwnerRegistration {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Register<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentOwnerRegistrationData>>(ownername: Param0, ownerregistrationdata: Param1) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Register)(::core::mem::transmute_copy(this), ownername.into_param().abi(), ownerregistrationdata.into_param().abi(), &mut result__).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationResult>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Unregister<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(ownername: Param0) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Unregister)(::core::mem::transmute_copy(this), ownername.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsEnvironmentOwnerRegistrationStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentOwnerRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistration, IIsolatedWindowsEnvironmentOwnerRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistration {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentOwnerRegistrationData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistrationData, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShareableFolders(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShareableFolders)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessesRunnableAsSystem(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessesRunnableAsSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessesRunnableAsUser(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessesRunnableAsUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivationFileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivationFileExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOwnerRegistrationData {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData;{f888ec22-e8cf-56c0-b1df-90af4ad80e84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationData {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentOwnerRegistrationResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentOwnerRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult;{6dab9451-6169-55df-8f51-790e99d7277d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidArgument: Self = Self(1i32);
    pub const AccessDenied: Self = Self(2i32);
    pub const InsufficientMemory: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOwnerRegistrationStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentPostMessageResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentPostMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentPostMessageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentPostMessageStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentPostMessageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentPostMessageResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult;{0dfa28fa-2ef0-4d8f-b341-3171b2df93b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentPostMessageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentPostMessageResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentPostMessageResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentPostMessageStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentPostMessageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentPostMessageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentPostMessageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentProcess {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn State(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcessState> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentProcessState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentProcessState>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn WaitForExit(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).WaitForExit)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).WaitForExitWithTimeout)(::core::mem::transmute_copy(this), timeoutmilliseconds).ok() }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitForExitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WaitForExitAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentProcess {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProcess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess;{a858c3ef-8172-4f10-af93-cbe60af88d09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentProcess as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentProcess {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentProcess {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProcessState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcessState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentProcessState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentProcessState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProcessState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcessState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProcessState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProgressState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentProgressState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentProgressState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProgressState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProgressState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProgressState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFileRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentShareFileRequestOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn AllowWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowWrite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowWrite)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFileRequestOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions;{c9190ed8-0fd0-4946-bb88-117a60737b61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFileRequestOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFileResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFileStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentShareFileStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentShareFileStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFileResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult;{aec7caa7-9ac6-4bf5-8b91-5c1adf0d7d00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFileResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(3i32);
    pub const FileNotFound: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentShareFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentShareFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentShareFolderRequestOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn AllowWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowWrite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowWrite)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions;{c405eb7d-7053-4f6a-9b87-746846ed19b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFolderResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFolderStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentShareFolderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentShareFolderStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFolderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFolderResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult;{556ba72e-ca9d-4211-b143-1cedc86eb2fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFolderResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FolderNotFound: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFolderStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentShareFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentShareFolderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentStartProcessResult {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentStartProcessStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentStartProcessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentStartProcessStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn Process(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcess> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Process)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentProcess>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentStartProcessResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentStartProcessResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult;{8fa1dc2f-57da-4bb5-9c06-fa072d2032e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentStartProcessResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentStartProcessResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentStartProcessResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const AppNotRegistered: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentStartProcessStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentStartProcessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentStartProcessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentStartProcessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentTelemetryParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentTelemetryParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCorrelationId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentTelemetryParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentTelemetryParameters {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentTelemetryParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters;{ebdb3cab-7a3a-4524-a0f4-f96e284d33cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentTelemetryParameters {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentTelemetryParameters {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentUserInfo {
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn EnvironmentUserSid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnvironmentUserSid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn EnvironmentUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnvironmentUserName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryWaitForSignInAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryWaitForSignInAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentUserInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentUserInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentUserInfo {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentUserInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentUserInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo;{8a9c75ae-69ba-4001-96fc-19a02703b340})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentUserInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentUserInfo {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentUserInfo {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsHostMessenger {}
impl IsolatedWindowsHostMessenger {
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>(receiverid: Param0, message: Param1) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).PostMessageToReceiver)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn GetFileId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileId)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterHostMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, HostMessageReceivedCallback>>(receiverid: Param0, hostmessagereceivedcallback: Param1) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterHostMessageReceiver)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), hostmessagereceivedcallback.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_Isolation\"`*"]
    pub fn UnregisterHostMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(receiverid: Param0) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).UnregisterHostMessageReceiver)(::core::mem::transmute_copy(this), receiverid.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsHostMessengerStatics<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsHostMessengerStatics2<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsHostMessenger {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsHostMessenger";
}
#[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct MessageReceivedCallback(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedCallbackBox::<F> { vtable: &MessageReceivedCallbackBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct MessageReceivedCallbackBox<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageReceivedCallback_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MessageReceivedCallbackBox<F> {
    const VTABLE: MessageReceivedCallback_Vtbl = MessageReceivedCallback_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MessageReceivedCallback as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&receiverid), ::core::mem::transmute(&message)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for MessageReceivedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for MessageReceivedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for MessageReceivedCallback {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for MessageReceivedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageReceivedCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MessageReceivedCallback {
    type Vtable = MessageReceivedCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b4c8ff_1d9d_4995_9fea_4d15257c0757);
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MessageReceivedCallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f5b4c8ff-1d9d-4995-9fea-4d15257c0757}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

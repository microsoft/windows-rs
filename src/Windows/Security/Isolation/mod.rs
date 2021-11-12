#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HostMessageReceivedCallback(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HostMessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = HostMessageReceivedCallback_box::<F> {
            vtable: &HostMessageReceivedCallback_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for HostMessageReceivedCallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({faf26ffa-8ce1-4cc1-b278-322d31a5e4a3})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for HostMessageReceivedCallback {
    type Vtable = HostMessageReceivedCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf26ffa_8ce1_4cc1_b278_322d31a5e4a3);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct HostMessageReceivedCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct HostMessageReceivedCallback_box<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const HostMessageReceivedCallback_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + 'static> HostMessageReceivedCallback_box<F> {
    const VTABLE: HostMessageReceivedCallback_abi = HostMessageReceivedCallback_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<HostMessageReceivedCallback as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            &*(&message as *const <super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d24597_c328_4467_b37f_4dfc6f60b6bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostfolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostfolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestoptions: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, messagereceivedcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment2 {
    type Vtable = IIsolatedWindowsEnvironment2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d365f39_88bd_4ab4_93cf_7e2bcef337c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment3 {
    type Vtable = IIsolatedWindowsEnvironment3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb7fc7d2_d06e_4c26_8ada_dacdaaad03f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9a5e58_dcd7_45c2_9c85_ab642a715e8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFactory {
    type Vtable = IIsolatedWindowsEnvironmentFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aca93e7_e804_454d_8466_f9897c20b0f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, environmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, environmentownerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d5ae1ef_029f_4101_8c35_fe91bf9cd5f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFile2 {
    type Vtable = IIsolatedWindowsEnvironmentFile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4eeb8dec_ad5d_4b0a_b754_f36c3d46d684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentHostStatics {
    type Vtable = IIsolatedWindowsEnvironmentHostStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0e22c7_05a0_517a_b81c_6ee8790c381f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x685d4176_f6e0_4569_b1aa_215c0ff5b257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71d98f7_61f0_4008_b207_0bf9eb2d76f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharedhostfolderpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharefoldernameinenvironment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions2 {
    type Vtable = IIsolatedWindowsEnvironmentOptions2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10d7cc31_8b8f_4b9d_b22c_617103b55b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf888ec22_e8cf_56c0_b1df_90af4ad80e84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dab9451_6169_55df_8f51_790e99d7277d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10951754_204b_5ec9_9de3_df792d074a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ownername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ownerregistrationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ownername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dfa28fa_2ef0_4d8f_b341_3171b2df93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa858c3ef_8172_4f10_af93_cbe60af88d09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeoutmilliseconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9190ed8_0fd0_4946_bb88_117a60737b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec7caa7_9ac6_4bf5_8b91_5c1adf0d7d00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc405eb7d_7053_4f6a_9b87_746846ed19b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x556ba72e_ca9d_4211_b143_1cedc86eb2fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa1dc2f_57da_4bb5_9c06_fa072d2032e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebdb3cab_7a3a_4524_a0f4_f96e284d33cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9c75ae_69ba_4001_96fc_19a02703b340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsHostMessengerStatics {
    type Vtable = IIsolatedWindowsHostMessengerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06e444bb_53c0_4889_8fa3_53592e37cf21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsolatedWindowsHostMessengerStatics2 {
    type Vtable = IIsolatedWindowsHostMessengerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ef9ebc_0444_42ad_832d_1b89c089d1ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, hostmessagereceivedcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironment(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironment {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartProcessSilentlyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, hostexepath: Param0, arguments: Param1, activator: IsolatedWindowsEnvironmentActivator) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartProcessSilentlyWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        hostexepath: Param0,
        arguments: Param1,
        activator: IsolatedWindowsEnvironmentActivator,
        telemetryparameters: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShareFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>>(&self, hostfolder: Param0, requestoptions: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShareFolderWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>, Param2: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, hostfolder: Param0, requestoptions: Param1, telemetryparameters: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFileWithUIAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appexepath: Param0, argumentstemplate: Param1, filepath: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFileWithUIAndTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        appexepath: Param0,
        argumentstemplate: Param1,
        filepath: Param2,
        telemetryparameters: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TerminateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TerminateWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, telemetryparameters: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, MessageReceivedCallback>>(&self, receiverid: Param0, messagereceivedcallback: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), messagereceivedcallback.into_param().abi()).ok() }
    }
    pub fn UnregisterMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, receiverid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), receiverid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentOptions>>(options: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentOptions>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(options: Param0, telemetryparameters: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    pub fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(environmentid: Param0) -> ::windows::core::Result<IsolatedWindowsEnvironment> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), environmentid.into_param().abi(), &mut result__).from_abi::<IsolatedWindowsEnvironment>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindByOwnerId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(environmentownerid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), environmentownerid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn PostMessageToReceiverAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn PostMessageToReceiverWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, Param2: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        receiverid: Param0,
        message: Param1,
        telemetryparameters: Param2,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    pub fn GetUserInfo(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentUserInfo> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentUserInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShareFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>>(&self, filepath: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filepath.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShareFileWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>, Param2: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, filepath: Param0, options: Param1, telemetryparameters: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), filepath.into_param().abi(), options.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    pub fn IIsolatedWindowsEnvironmentFactory<R, F: FnOnce(&IIsolatedWindowsEnvironmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironment, IIsolatedWindowsEnvironmentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironment;{41d24597-c328-4467-b37f-4dfc6f60b6bc})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d24597_c328_4467_b37f_4dfc6f60b6bc);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironment";
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironment {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironment {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
impl IsolatedWindowsEnvironmentActivator {
    pub const System: IsolatedWindowsEnvironmentActivator = IsolatedWindowsEnvironmentActivator(0i32);
    pub const User: IsolatedWindowsEnvironmentActivator = IsolatedWindowsEnvironmentActivator(1i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentActivator {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentActivator {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentActivator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentActivator {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: IsolatedWindowsEnvironmentAllowedClipboardFormats = IsolatedWindowsEnvironmentAllowedClipboardFormats(0u32);
    pub const Text: IsolatedWindowsEnvironmentAllowedClipboardFormats = IsolatedWindowsEnvironmentAllowedClipboardFormats(1u32);
    pub const Image: IsolatedWindowsEnvironmentAllowedClipboardFormats = IsolatedWindowsEnvironmentAllowedClipboardFormats(2u32);
}
impl ::core::convert::From<u32> for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats;u4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(0u32);
    pub const Local: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(1u32);
    pub const Network: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(2u32);
    pub const SystemPrintToPdf: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(4u32);
    pub const SystemPrintToXps: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(8u32);
}
impl ::core::convert::From<u32> for IsolatedWindowsEnvironmentAvailablePrinters {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentAvailablePrinters {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentAvailablePrinters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters;u4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentAvailablePrinters {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = IsolatedWindowsEnvironmentClipboardCopyPasteDirections(0u32);
    pub const HostToIsolatedWindowsEnvironment: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = IsolatedWindowsEnvironmentClipboardCopyPasteDirections(1u32);
    pub const IsolatedWindowsEnvironmentToHost: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = IsolatedWindowsEnvironmentClipboardCopyPasteDirections(2u32);
}
impl ::core::convert::From<u32> for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections;u4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IsolatedWindowsEnvironmentContract(pub u8);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateProgress {
    pub State: IsolatedWindowsEnvironmentProgressState,
    pub PercentComplete: u32,
}
impl IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateProgress {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IsolatedWindowsEnvironmentCreateProgress").field("State", &self.State).field("PercentComplete", &self.PercentComplete).finish()
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.PercentComplete == other.PercentComplete
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateProgress {}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreateProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress;enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4);u4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentCreateProgress {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentCreateResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentCreateResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentCreateStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentCreateStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentCreateStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn Environment(&self) -> ::windows::core::Result<IsolatedWindowsEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironment>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult;{ef9a5e58-dcd7-45c2-9c85-ab642a715e8e})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9a5e58_dcd7_45c2_9c85_ab642a715e8e);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentCreateResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentCreateResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: IsolatedWindowsEnvironmentCreateStatus = IsolatedWindowsEnvironmentCreateStatus(0i32);
    pub const FailureByPolicy: IsolatedWindowsEnvironmentCreateStatus = IsolatedWindowsEnvironmentCreateStatus(1i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentCreateStatus = IsolatedWindowsEnvironmentCreateStatus(2i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentCreateStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreateStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentCreateStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentFile(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentFile {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn HostPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn GuestPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentFile;{4d5ae1ef-029f-4101-8c35-fe91bf9cd5f0})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d5ae1ef_029f_4101_8c35_fe91bf9cd5f0);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentFile {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentFile {}
pub struct IsolatedWindowsEnvironmentHost {}
impl IsolatedWindowsEnvironmentHost {
    pub fn IsReady() -> ::windows::core::Result<bool> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostErrors() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>>(result__)
        })
    }
    pub fn IIsolatedWindowsEnvironmentHostStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentHostStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentHost, IIsolatedWindowsEnvironmentHostStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentHost {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(0i32);
    pub const FeatureNotInstalled: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(1i32);
    pub const HardwareRequirementsNotMet: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(2i32);
    pub const RebootRequired: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(3i32);
    pub const UnknownError: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(4i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentHostError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentHostError {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentHostError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentHostError {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentLaunchFileResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentLaunchFileStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentLaunchFileStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentLaunchFileStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult;{685d4176-f6e0-4569-b1aa-215c0ff5b257})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x685d4176_f6e0_4569_b1aa_215c0ff5b257);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentLaunchFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentLaunchFileResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(2i32);
    pub const FileNotFound: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(3i32);
    pub const TimedOut: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(4i32);
    pub const AlreadySharedWithConflictingOptions: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(5i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentLaunchFileStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentLaunchFileStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentOptions(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EnvironmentOwnerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEnvironmentOwnerId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AllowedClipboardFormats(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentAllowedClipboardFormats = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentAllowedClipboardFormats>(result__)
        }
    }
    pub fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ClipboardCopyPasteDirections(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>(result__)
        }
    }
    pub fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AvailablePrinters(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAvailablePrinters> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentAvailablePrinters = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentAvailablePrinters>(result__)
        }
    }
    pub fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SharedHostFolderPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SharedFolderNameInEnvironment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ShareHostFolderForUntrustedItems<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, sharedhostfolderpath: Param0, sharefoldernameinenvironment: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), sharedhostfolderpath.into_param().abi(), sharefoldernameinenvironment.into_param().abi()).ok() }
    }
    pub fn PersistUserProfile(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPersistUserProfile(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowGraphicsHardwareAcceleration(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowCameraAndMicrophoneAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WindowAnnotationOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetWindowAnnotationOverride<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions;{b71d98f7-61f0-4008-b207-0bf9eb2d76f2})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71d98f7_61f0_4008_b207_0bf9eb2d76f2);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOptions {}
pub struct IsolatedWindowsEnvironmentOwnerRegistration {}
impl IsolatedWindowsEnvironmentOwnerRegistration {
    pub fn Register<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IsolatedWindowsEnvironmentOwnerRegistrationData>>(ownername: Param0, ownerregistrationdata: Param1) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ownername.into_param().abi(), ownerregistrationdata.into_param().abi(), &mut result__).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationResult>(result__)
        })
    }
    pub fn Unregister<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(ownername: Param0) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ownername.into_param().abi()).ok() })
    }
    pub fn IIsolatedWindowsEnvironmentOwnerRegistrationStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentOwnerRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistration, IIsolatedWindowsEnvironmentOwnerRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistration {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentOwnerRegistrationData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistrationData, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShareableFolders(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessesRunnableAsSystem(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessesRunnableAsUser(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivationFileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData;{f888ec22-e8cf-56c0-b1df-90af4ad80e84})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf888ec22_e8cf_56c0_b1df_90af4ad80e84);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationData {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentOwnerRegistrationResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentOwnerRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult;{6dab9451-6169-55df-8f51-790e99d7277d})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dab9451_6169_55df_8f51_790e99d7277d);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(0i32);
    pub const InvalidArgument: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(1i32);
    pub const AccessDenied: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(2i32);
    pub const InsufficientMemory: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(3i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(4i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentPostMessageResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentPostMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentPostMessageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentPostMessageStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult;{0dfa28fa-2ef0-4d8f-b341-3171b2df93b1})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dfa28fa_2ef0_4d8f_b341_3171b2df93b1);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentPostMessageResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentPostMessageResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: IsolatedWindowsEnvironmentPostMessageStatus = IsolatedWindowsEnvironmentPostMessageStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentPostMessageStatus = IsolatedWindowsEnvironmentPostMessageStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentPostMessageStatus = IsolatedWindowsEnvironmentPostMessageStatus(2i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentPostMessageStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentPostMessageStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentPostMessageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentPostMessageStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentProcess(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentProcess {
    pub fn State(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcessState> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentProcessState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentProcessState>(result__)
        }
    }
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn WaitForExit(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), timeoutmilliseconds).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WaitForExitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProcess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess;{a858c3ef-8172-4f10-af93-cbe60af88d09})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa858c3ef_8172_4f10_af93_cbe60af88d09);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentProcess {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentProcess {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: IsolatedWindowsEnvironmentProcessState = IsolatedWindowsEnvironmentProcessState(1i32);
    pub const Aborted: IsolatedWindowsEnvironmentProcessState = IsolatedWindowsEnvironmentProcessState(2i32);
    pub const Completed: IsolatedWindowsEnvironmentProcessState = IsolatedWindowsEnvironmentProcessState(3i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentProcessState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentProcessState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProcessState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentProcessState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: IsolatedWindowsEnvironmentProgressState = IsolatedWindowsEnvironmentProgressState(0i32);
    pub const Processing: IsolatedWindowsEnvironmentProgressState = IsolatedWindowsEnvironmentProgressState(1i32);
    pub const Completed: IsolatedWindowsEnvironmentProgressState = IsolatedWindowsEnvironmentProgressState(2i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentProgressState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentProgressState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProgressState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentProgressState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentShareFileRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentShareFileRequestOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions;{c9190ed8-0fd0-4946-bb88-117a60737b61})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9190ed8_0fd0_4946_bb88_117a60737b61);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFileResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentShareFileResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFileStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentShareFileStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentShareFileStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult;{aec7caa7-9ac6-4bf5-8b91-5c1adf0d7d00})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec7caa7_9ac6_4bf5_8b91_5c1adf0d7d00);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(2i32);
    pub const AlreadySharedWithConflictingOptions: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(3i32);
    pub const FileNotFound: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(4i32);
    pub const AccessDenied: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(5i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentShareFileStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentShareFileStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentShareFileStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentShareFolderRequestOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions;{c405eb7d-7053-4f6a-9b87-746846ed19b2})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc405eb7d_7053_4f6a_9b87_746846ed19b2);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentShareFolderResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFolderStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentShareFolderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentShareFolderStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult;{556ba72e-ca9d-4211-b143-1cedc86eb2fe})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x556ba72e_ca9d_4211_b143_1cedc86eb2fe);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(2i32);
    pub const FolderNotFound: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(3i32);
    pub const AccessDenied: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(4i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentShareFolderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentShareFolderStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentShareFolderStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentStartProcessResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentStartProcessStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentStartProcessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentStartProcessStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn Process(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcess> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentProcess>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult;{8fa1dc2f-57da-4bb5-9c06-fa072d2032e2})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa1dc2f_57da_4bb5_9c06_fa072d2032e2);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentStartProcessResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentStartProcessResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(2i32);
    pub const FileNotFound: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(3i32);
    pub const AppNotRegistered: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(4i32);
}
impl ::core::convert::From<i32> for IsolatedWindowsEnvironmentStartProcessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentStartProcessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentStartProcessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus;i4)");
}
impl ::windows::core::DefaultType for IsolatedWindowsEnvironmentStartProcessStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentTelemetryParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentTelemetryParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters;{ebdb3cab-7a3a-4524-a0f4-f96e284d33cd})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebdb3cab_7a3a_4524_a0f4_f96e284d33cd);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentTelemetryParameters {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentTelemetryParameters {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentUserInfo(pub ::windows::core::IInspectable);
impl IsolatedWindowsEnvironmentUserInfo {
    pub fn EnvironmentUserSid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EnvironmentUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryWaitForSignInAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo;{8a9c75ae-69ba-4001-96fc-19a02703b340})");
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9c75ae_69ba_4001_96fc_19a02703b340);
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows::core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentUserInfo {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentUserInfo {}
pub struct IsolatedWindowsHostMessenger {}
impl IsolatedWindowsHostMessenger {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>(receiverid: Param0, message: Param1) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() })
    }
    pub fn GetFileId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterHostMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, HostMessageReceivedCallback>>(receiverid: Param0, hostmessagereceivedcallback: Param1) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), hostmessagereceivedcallback.into_param().abi()).ok() })
    }
    pub fn UnregisterHostMessageReceiver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(receiverid: Param0) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), receiverid.into_param().abi()).ok() })
    }
    pub fn IIsolatedWindowsHostMessengerStatics<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IIsolatedWindowsHostMessengerStatics2<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsHostMessenger {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsHostMessenger";
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageReceivedCallback(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = MessageReceivedCallback_box::<F> {
            vtable: &MessageReceivedCallback_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MessageReceivedCallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({f5b4c8ff-1d9d-4995-9fea-4d15257c0757})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MessageReceivedCallback {
    type Vtable = MessageReceivedCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b4c8ff_1d9d_4995_9fea_4d15257c0757);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct MessageReceivedCallback_box<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const MessageReceivedCallback_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + 'static> MessageReceivedCallback_box<F> {
    const VTABLE: MessageReceivedCallback_abi = MessageReceivedCallback_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MessageReceivedCallback as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            &*(&message as *const <super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}

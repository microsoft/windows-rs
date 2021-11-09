#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HostMessageReceivedCallback(::windows::runtime::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HostMessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::runtime::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        unsafe {
            let object = ::windows::runtime::heap_alloc(core::mem::size_of::<HostMessageReceivedCallback_box<F>>()).expect("Could not successfully allocate delegate") as *mut HostMessageReceivedCallback_box<F>;
            *object = HostMessageReceivedCallback_box::<F> {
                vtable: &HostMessageReceivedCallback_box::<F>::VTABLE,
                count: ::windows::runtime::RefCount::new(1),
                invoke,
            };
            core::mem::transmute(object)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for HostMessageReceivedCallback {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({faf26ffa-8ce1-4cc1-b278-322d31a5e4a3})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for HostMessageReceivedCallback {
    type Vtable = HostMessageReceivedCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4210192378, 36065, 19649, [178, 120, 50, 45, 49, 165, 228, 163]);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct HostMessageReceivedCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct HostMessageReceivedCallback_box<F: FnMut(&::windows::runtime::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const HostMessageReceivedCallback_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::runtime::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> + 'static> HostMessageReceivedCallback_box<F> {
    const VTABLE: HostMessageReceivedCallback_abi = HostMessageReceivedCallback_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<HostMessageReceivedCallback as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(ptr: ::windows::runtime::RawPtr) -> u32 {
        let this = ptr as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::heap_free(ptr);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&receiverid as *const <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType),
            &*(&message as *const <super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable> as ::windows::runtime::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable> as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1104299415, 49960, 17511, [179, 127, 77, 252, 111, 96, 182, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostexepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostexepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostfolder: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, requestoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostfolder: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, requestoptions: ::windows::runtime::RawPtr, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appexepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appexepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, messagereceivedcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironment2 {
    type Vtable = IIsolatedWindowsEnvironment2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(758538041, 35005, 19124, [147, 207, 126, 43, 206, 243, 55, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironment3 {
    type Vtable = IIsolatedWindowsEnvironment3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3414149074, 53358, 19494, [138, 218, 218, 205, 170, 173, 3, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4019871320, 56535, 17858, [156, 133, 171, 100, 42, 113, 94, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentFactory {
    type Vtable = IIsolatedWindowsEnvironmentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(449483751, 59396, 17741, [132, 102, 249, 137, 124, 32, 176, 246]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, telemetryparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environmentid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environmentownerid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297801711, 671, 16641, [140, 53, 254, 145, 191, 156, 213, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentFile2 {
    type Vtable = IIsolatedWindowsEnvironmentFile2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1324060140, 44381, 19210, [183, 84, 243, 108, 61, 70, 214, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentHostStatics {
    type Vtable = IIsolatedWindowsEnvironmentHostStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(739123911, 1440, 20858, [184, 28, 110, 232, 121, 12, 56, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750942070, 63200, 17769, [177, 170, 33, 92, 15, 245, 178, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072170231, 25072, 16392, [178, 7, 11, 249, 235, 45, 118, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharedhostfolderpath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sharefoldernameinenvironment: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentOptions2 {
    type Vtable = IIsolatedWindowsEnvironmentOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(282577969, 35727, 19357, [178, 44, 97, 113, 3, 181, 91, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4169722914, 59599, 22208, [177, 223, 144, 175, 74, 216, 14, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1839961169, 24937, 21983, [143, 81, 121, 14, 153, 215, 39, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(278206292, 8267, 24265, [157, 227, 223, 121, 45, 7, 74, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownername: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, ownerregistrationdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownername: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(234498298, 12016, 19855, [179, 65, 49, 113, 178, 223, 147, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2824389615, 33138, 20240, [175, 147, 203, 230, 10, 248, 141, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeoutmilliseconds: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3373862616, 4048, 18758, [187, 136, 17, 122, 96, 115, 123, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2932329127, 39622, 19445, [139, 145, 92, 26, 223, 13, 125, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3288722301, 28755, 20330, [155, 135, 116, 104, 70, 237, 25, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1433118510, 51869, 16913, [177, 67, 28, 237, 200, 110, 178, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409749551, 22490, 19381, [156, 6, 250, 7, 45, 32, 50, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3957013675, 31290, 17700, [160, 244, 249, 110, 40, 77, 51, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2325509550, 27066, 16385, [150, 252, 25, 160, 39, 3, 179, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsHostMessengerStatics {
    type Vtable = IIsolatedWindowsHostMessengerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(115623099, 21440, 18569, [143, 163, 83, 89, 46, 55, 207, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIsolatedWindowsHostMessengerStatics2 {
    type Vtable = IIsolatedWindowsHostMessengerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1441767100, 1092, 17069, [131, 45, 27, 137, 192, 137, 209, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, hostmessagereceivedcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironment(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironment {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn StartProcessSilentlyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, hostexepath: Param0, arguments: Param1, activator: IsolatedWindowsEnvironmentActivator) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn StartProcessSilentlyWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        hostexepath: Param0,
        arguments: Param1,
        activator: IsolatedWindowsEnvironmentActivator,
        telemetryparameters: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn ShareFolderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>>(&self, hostfolder: Param0, requestoptions: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn ShareFolderWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>, Param2: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        hostfolder: Param0,
        requestoptions: Param1,
        telemetryparameters: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn LaunchFileWithUIAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, appexepath: Param0, argumentstemplate: Param1, filepath: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn LaunchFileWithUIAndTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        appexepath: Param0,
        argumentstemplate: Param1,
        filepath: Param2,
        telemetryparameters: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn TerminateAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn TerminateWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, telemetryparameters: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn RegisterMessageReceiver<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, MessageReceivedCallback>>(&self, receiverid: Param0, messagereceivedcallback: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), messagereceivedcallback.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn UnregisterMessageReceiver<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, receiverid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), receiverid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn CreateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentOptions>>(options: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn CreateWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentOptions>, Param1: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(options: Param0, telemetryparameters: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn GetById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(environmentid: Param0) -> ::windows::runtime::Result<IsolatedWindowsEnvironment> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), environmentid.into_param().abi(), &mut result__).from_abi::<IsolatedWindowsEnvironment>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn FindByOwnerId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(environmentownerid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), environmentownerid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`, `Foundation_Collections`*"]
    pub fn PostMessageToReceiverAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`, `Foundation_Collections`*"]
    pub fn PostMessageToReceiverWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::IInspectable>>, Param2: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(
        &self,
        receiverid: Param0,
        message: Param1,
        telemetryparameters: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn GetUserInfo(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentUserInfo> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentUserInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn ShareFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>>(&self, filepath: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filepath.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn ShareFileWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>, Param2: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, filepath: Param0, options: Param1, telemetryparameters: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), filepath.into_param().abi(), options.into_param().abi(), telemetryparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    pub fn IIsolatedWindowsEnvironmentFactory<R, F: FnOnce(&IIsolatedWindowsEnvironmentFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironment, IIsolatedWindowsEnvironmentFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironment;{41d24597-c328-4467-b37f-4dfc6f60b6bc})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1104299415, 49960, 17511, [179, 127, 77, 252, 111, 96, 182, 188]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironment";
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironment {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironment {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentActivator {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentActivator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentActivator {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats;u4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
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
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentAvailablePrinters {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentAvailablePrinters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters;u4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentAvailablePrinters {
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
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections;u4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
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
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentCreateProgress {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentCreateProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress;enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4);u4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentCreateProgress {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentCreateResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentCreateResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentCreateStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentCreateStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentCreateStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Environment(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironment>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult;{ef9a5e58-dcd7-45c2-9c85-ab642a715e8e})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4019871320, 56535, 17858, [156, 133, 171, 100, 42, 113, 94, 142]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentCreateResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentCreateResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentCreateStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentCreateStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentCreateStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentFile(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentFile {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn HostPath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn GuestPath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentFile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentFile;{4d5ae1ef-029f-4101-8c35-fe91bf9cd5f0})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297801711, 671, 16641, [140, 53, 254, 145, 191, 156, 213, 240]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentFile {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentFile {}
#[doc = "*Required features: `Security_Isolation`*"]
pub struct IsolatedWindowsEnvironmentHost {}
impl IsolatedWindowsEnvironmentHost {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn IsReady() -> ::windows::runtime::Result<bool> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn HostErrors() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>>(result__)
        })
    }
    pub fn IIsolatedWindowsEnvironmentHostStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentHostStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentHost, IIsolatedWindowsEnvironmentHostStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentHost {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentHostError {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentHostError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentHostError {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentLaunchFileResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentLaunchFileStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentLaunchFileStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentLaunchFileStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn File(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult;{685d4176-f6e0-4569-b1aa-215c0ff5b257})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750942070, 63200, 17769, [177, 170, 33, 92, 15, 245, 178, 87]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentLaunchFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentLaunchFileResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentLaunchFileStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentLaunchFileStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentLaunchFileStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentOptions(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn EnvironmentOwnerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetEnvironmentOwnerId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn AllowedClipboardFormats(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentAllowedClipboardFormats = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentAllowedClipboardFormats>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ClipboardCopyPasteDirections(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn AvailablePrinters(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentAvailablePrinters> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentAvailablePrinters = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentAvailablePrinters>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SharedHostFolderPath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SharedFolderNameInEnvironment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ShareHostFolderForUntrustedItems<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, sharedhostfolderpath: Param0, sharefoldernameinenvironment: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), sharedhostfolderpath.into_param().abi(), sharefoldernameinenvironment.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn PersistUserProfile(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetPersistUserProfile(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn AllowGraphicsHardwareAcceleration(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn AllowCameraAndMicrophoneAccess(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn WindowAnnotationOverride(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetWindowAnnotationOverride<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions;{b71d98f7-61f0-4008-b207-0bf9eb2d76f2})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072170231, 25072, 16392, [178, 7, 11, 249, 235, 45, 118, 242]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOptions {}
#[doc = "*Required features: `Security_Isolation`*"]
pub struct IsolatedWindowsEnvironmentOwnerRegistration {}
impl IsolatedWindowsEnvironmentOwnerRegistration {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Register<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, IsolatedWindowsEnvironmentOwnerRegistrationData>>(ownername: Param0, ownerregistrationdata: Param1) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ownername.into_param().abi(), ownerregistrationdata.into_param().abi(), &mut result__).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationResult>(result__)
        })
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Unregister<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(ownername: Param0) -> ::windows::runtime::Result<()> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ownername.into_param().abi()).ok() })
    }
    pub fn IIsolatedWindowsEnvironmentOwnerRegistrationStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentOwnerRegistrationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistration, IIsolatedWindowsEnvironmentOwnerRegistrationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistration {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentOwnerRegistrationData {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistrationData, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn ShareableFolders(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn ProcessesRunnableAsSystem(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn ProcessesRunnableAsUser(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn ActivationFileExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData;{f888ec22-e8cf-56c0-b1df-90af4ad80e84})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4169722914, 59599, 22208, [177, 223, 144, 175, 74, 216, 14, 132]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationData {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentOwnerRegistrationResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentOwnerRegistrationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult;{6dab9451-6169-55df-8f51-790e99d7277d})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1839961169, 24937, 21983, [143, 81, 121, 14, 153, 215, 39, 125]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentPostMessageResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentPostMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentPostMessageStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentPostMessageStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult;{0dfa28fa-2ef0-4d8f-b341-3171b2df93b1})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(234498298, 12016, 19855, [179, 65, 49, 113, 178, 223, 147, 177]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentPostMessageResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentPostMessageResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentPostMessageStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentPostMessageStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentPostMessageStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentProcess(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentProcess {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn State(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentProcessState> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentProcessState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentProcessState>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExitCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn WaitForExit(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), timeoutmilliseconds).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn WaitForExitAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentProcess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess;{a858c3ef-8172-4f10-af93-cbe60af88d09})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2824389615, 33138, 20240, [175, 147, 203, 230, 10, 248, 141, 9]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentProcess {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentProcess {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentProcessState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentProcessState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentProcessState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentProgressState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentProgressState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentProgressState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentShareFileRequestOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentShareFileRequestOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn AllowWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions;{c9190ed8-0fd0-4946-bb88-117a60737b61})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3373862616, 4048, 18758, [187, 136, 17, 122, 96, 115, 123, 97]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFileResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentShareFileResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentShareFileStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentShareFileStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentShareFileStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn File(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult;{aec7caa7-9ac6-4bf5-8b91-5c1adf0d7d00})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2932329127, 39622, 19445, [139, 145, 92, 26, 223, 13, 125, 0]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentShareFileStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentShareFileStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentShareFileStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentShareFolderRequestOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn AllowWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions;{c405eb7d-7053-4f6a-9b87-746846ed19b2})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3288722301, 28755, 20330, [155, 135, 116, 104, 70, 237, 25, 178]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentShareFolderResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentShareFolderStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentShareFolderStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentShareFolderStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult;{556ba72e-ca9d-4211-b143-1cedc86eb2fe})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1433118510, 51869, 16913, [177, 67, 28, 237, 200, 110, 178, 254]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentShareFolderStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentShareFolderStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentShareFolderStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentStartProcessResult {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentStartProcessStatus> {
        let this = self;
        unsafe {
            let mut result__: IsolatedWindowsEnvironmentStartProcessStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentStartProcessStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn Process(&self) -> ::windows::runtime::Result<IsolatedWindowsEnvironmentProcess> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsolatedWindowsEnvironmentProcess>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult;{8fa1dc2f-57da-4bb5-9c06-fa072d2032e2})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409749551, 22490, 19381, [156, 6, 250, 7, 45, 32, 50, 226]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentStartProcessResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentStartProcessResult {}
#[doc = "*Required features: `Security_Isolation`*"]
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
unsafe impl ::windows::runtime::Abi for IsolatedWindowsEnvironmentStartProcessStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentStartProcessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus;i4)");
}
impl ::windows::runtime::DefaultType for IsolatedWindowsEnvironmentStartProcessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentTelemetryParameters {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsEnvironmentTelemetryParameters, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn SetCorrelationId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters;{ebdb3cab-7a3a-4524-a0f4-f96e284d33cd})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3957013675, 31290, 17700, [160, 244, 249, 110, 40, 77, 51, 205]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentTelemetryParameters {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentTelemetryParameters {}
#[doc = "*Required features: `Security_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsolatedWindowsEnvironmentUserInfo(pub ::windows::runtime::IInspectable);
impl IsolatedWindowsEnvironmentUserInfo {
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn EnvironmentUserSid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn EnvironmentUserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation`*"]
    pub fn TryWaitForSignInAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo;{8a9c75ae-69ba-4001-96fc-19a02703b340})");
}
unsafe impl ::windows::runtime::Interface for IsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2325509550, 27066, 16385, [150, 252, 25, 160, 39, 3, 179, 64]);
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows::runtime::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows::runtime::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows::runtime::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentUserInfo {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentUserInfo {}
#[doc = "*Required features: `Security_Isolation`*"]
pub struct IsolatedWindowsHostMessenger {}
impl IsolatedWindowsHostMessenger {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn PostMessageToReceiver<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>>(receiverid: Param0, message: Param1) -> ::windows::runtime::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn GetFileId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filepath: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn RegisterHostMessageReceiver<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, HostMessageReceivedCallback>>(receiverid: Param0, hostmessagereceivedcallback: Param1) -> ::windows::runtime::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), hostmessagereceivedcallback.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Security_Isolation`*"]
    pub fn UnregisterHostMessageReceiver<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(receiverid: Param0) -> ::windows::runtime::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), receiverid.into_param().abi()).ok() })
    }
    pub fn IIsolatedWindowsHostMessengerStatics<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IIsolatedWindowsHostMessengerStatics2<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for IsolatedWindowsHostMessenger {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsHostMessenger";
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageReceivedCallback(::windows::runtime::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::runtime::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        unsafe {
            let object = ::windows::runtime::heap_alloc(core::mem::size_of::<MessageReceivedCallback_box<F>>()).expect("Could not successfully allocate delegate") as *mut MessageReceivedCallback_box<F>;
            *object = MessageReceivedCallback_box::<F> {
                vtable: &MessageReceivedCallback_box::<F>::VTABLE,
                count: ::windows::runtime::RefCount::new(1),
                invoke,
            };
            core::mem::transmute(object)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Isolation`, `Foundation_Collections`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for MessageReceivedCallback {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({f5b4c8ff-1d9d-4995-9fea-4d15257c0757})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for MessageReceivedCallback {
    type Vtable = MessageReceivedCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4122265855, 7581, 18837, [159, 234, 77, 21, 37, 124, 7, 87]);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct MessageReceivedCallback_box<F: FnMut(&::windows::runtime::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const MessageReceivedCallback_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::runtime::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> + 'static> MessageReceivedCallback_box<F> {
    const VTABLE: MessageReceivedCallback_abi = MessageReceivedCallback_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MessageReceivedCallback as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(ptr: ::windows::runtime::RawPtr) -> u32 {
        let this = ptr as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::heap_free(ptr);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, receiverid: ::windows::runtime::GUID, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&receiverid as *const <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType),
            &*(&message as *const <super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable> as ::windows::runtime::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::runtime::IInspectable> as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}

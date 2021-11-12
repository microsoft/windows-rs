#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IServiceDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IServiceDeviceStatics {
    type Vtable = IServiceDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa88214e1_59c7_4a20_aba6_9f6707937230);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicetype: ServiceDeviceType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageDeviceStatics {
    type Vtable = IStorageDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ece44ee_1b23_4dd2_8652_bc164f003128);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct PortableDeviceContract(pub u8);
pub struct ServiceDevice {}
impl ServiceDevice {
    pub fn GetDeviceSelector(servicetype: ServiceDeviceType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), servicetype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromServiceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(serviceid: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IServiceDeviceStatics<R, F: FnOnce(&IServiceDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ServiceDevice, IServiceDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ServiceDevice {
    const NAME: &'static str = "Windows.Devices.Portable.ServiceDevice";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ServiceDeviceType(pub i32);
impl ServiceDeviceType {
    pub const CalendarService: ServiceDeviceType = ServiceDeviceType(0i32);
    pub const ContactsService: ServiceDeviceType = ServiceDeviceType(1i32);
    pub const DeviceStatusService: ServiceDeviceType = ServiceDeviceType(2i32);
    pub const NotesService: ServiceDeviceType = ServiceDeviceType(3i32);
    pub const RingtonesService: ServiceDeviceType = ServiceDeviceType(4i32);
    pub const SmsService: ServiceDeviceType = ServiceDeviceType(5i32);
    pub const TasksService: ServiceDeviceType = ServiceDeviceType(6i32);
}
impl ::core::convert::From<i32> for ServiceDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ServiceDeviceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ServiceDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Portable.ServiceDeviceType;i4)");
}
impl ::windows::core::DefaultType for ServiceDeviceType {
    type DefaultType = Self;
}
pub struct StorageDevice {}
impl StorageDevice {
    #[cfg(feature = "Storage")]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IStorageDeviceStatics<R, F: FnOnce(&IStorageDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageDevice, IStorageDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageDevice {
    const NAME: &'static str = "Windows.Devices.Portable.StorageDevice";
}

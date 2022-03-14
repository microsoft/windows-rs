#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServiceDeviceStatics {
    type Vtable = IServiceDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa88214e1_59c7_4a20_aba6_9f6707937230);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicetype: ServiceDeviceType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageDeviceStatics {
    type Vtable = IStorageDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ece44ee_1b23_4dd2_8652_bc164f003128);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    FromId: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
pub struct ServiceDevice {}
impl ServiceDevice {
    #[doc = "*Required features: `\"Devices_Portable\"`*"]
    pub fn GetDeviceSelector(servicetype: ServiceDeviceType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), servicetype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Portable\"`*"]
    pub fn GetDeviceSelectorFromServiceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(serviceid: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromServiceId)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IServiceDeviceStatics<R, F: FnOnce(&IServiceDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ServiceDevice, IServiceDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ServiceDevice {
    const NAME: &'static str = "Windows.Devices.Portable.ServiceDevice";
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ServiceDeviceType(pub i32);
impl ServiceDeviceType {
    pub const CalendarService: Self = Self(0i32);
    pub const ContactsService: Self = Self(1i32);
    pub const DeviceStatusService: Self = Self(2i32);
    pub const NotesService: Self = Self(3i32);
    pub const RingtonesService: Self = Self(4i32);
    pub const SmsService: Self = Self(5i32);
    pub const TasksService: Self = Self(6i32);
}
impl ::core::marker::Copy for ServiceDeviceType {}
impl ::core::clone::Clone for ServiceDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ServiceDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ServiceDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ServiceDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServiceDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Portable.ServiceDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
pub struct StorageDevice {}
impl StorageDevice {
    #[doc = "*Required features: `\"Devices_Portable\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromId)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Portable\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageDeviceStatics<R, F: FnOnce(&IStorageDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageDevice, IStorageDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageDevice {
    const NAME: &'static str = "Windows.Devices.Portable.StorageDevice";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

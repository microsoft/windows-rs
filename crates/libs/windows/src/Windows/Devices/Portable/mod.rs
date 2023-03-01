#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServiceDeviceStatics {
    type Vtable = IServiceDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IServiceDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IServiceDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa88214e1_59c7_4a20_aba6_9f6707937230);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicetype: ServiceDeviceType, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::GUID, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageDeviceStatics {
    type Vtable = IStorageDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IStorageDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ece44ee_1b23_4dd2_8652_bc164f003128);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    FromId: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
pub struct ServiceDevice;
impl ServiceDevice {
    pub fn GetDeviceSelector(servicetype: ServiceDeviceType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), servicetype, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorFromServiceId(serviceid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromServiceId)(::windows::core::Interface::as_raw(this), serviceid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IServiceDeviceStatics<R, F: FnOnce(&IServiceDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ServiceDevice, IServiceDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ServiceDevice {
    const NAME: &'static str = "Windows.Devices.Portable.ServiceDevice";
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
pub struct StorageDevice;
impl StorageDevice {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn FromId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFolder>();
            (::windows::core::Interface::vtable(this).FromId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageDeviceStatics<R, F: FnOnce(&IStorageDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageDevice, IStorageDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StorageDevice {
    const NAME: &'static str = "Windows.Devices.Portable.StorageDevice";
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for ServiceDeviceType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ServiceDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceDeviceType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ServiceDeviceType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Portable.ServiceDeviceType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

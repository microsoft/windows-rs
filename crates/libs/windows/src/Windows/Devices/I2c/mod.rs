#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cConnectionSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for II2cConnectionSettings {
    type Vtable = II2cConnectionSettings_Vtbl;
}
impl ::core::clone::Clone for II2cConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for II2cConnectionSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2db1307_ab6f_4639_a767_54536dc3460f);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut I2cBusSpeed) -> ::windows::core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: I2cBusSpeed) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut I2cSharingMode) -> ::windows::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: I2cSharingMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cConnectionSettingsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for II2cConnectionSettingsFactory {
    type Vtable = II2cConnectionSettingsFactory_Vtbl;
}
impl ::core::clone::Clone for II2cConnectionSettingsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for II2cConnectionSettingsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81b586b3_9693_41b1_a243_ded4f6e66926);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettingsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slaveaddress: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for II2cController {
    type Vtable = II2cController_Vtbl;
}
impl ::core::clone::Clone for II2cController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for II2cController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc48ab1b2_87a0_4166_8e3e_b4b8f97cd729);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for II2cControllerStatics {
    type Vtable = II2cControllerStatics_Vtbl;
}
impl ::core::clone::Clone for II2cControllerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for II2cControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40fc0365_5f05_4e7e_84bd_100db8e0aec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for II2cDevice {
    type Vtable = II2cDevice_Vtbl;
}
impl ::core::clone::Clone for II2cDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for II2cDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8636c136_b9c5_4f70_9449_cc46dc6f57eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct II2cDeviceStatics(::windows::core::IUnknown);
impl II2cDeviceStatics {
    pub fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, settings: &I2cConnectionSettings) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<I2cDevice>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), ::core::mem::transmute_copy(settings), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(II2cDeviceStatics, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for II2cDeviceStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cDeviceStatics {}
impl ::core::fmt::Debug for II2cDeviceStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cDeviceStatics").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for II2cDeviceStatics {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{91a33be3-7334-4512-96bc-fbae9459f5f6}");
}
unsafe impl ::windows::core::Interface for II2cDeviceStatics {
    type Vtable = II2cDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for II2cDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for II2cDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a33be3_7334_4512_96bc_fbae9459f5f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct I2cConnectionSettings(::windows::core::IUnknown);
impl I2cConnectionSettings {
    pub fn SlaveAddress(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).SlaveAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSlaveAddress)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn BusSpeed(&self) -> ::windows::core::Result<I2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cBusSpeed>();
            (::windows::core::Interface::vtable(this).BusSpeed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: I2cBusSpeed) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBusSpeed)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<I2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cSharingMode>();
            (::windows::core::Interface::vtable(this).SharingMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSharingMode(&self, value: I2cSharingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSharingMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(slaveaddress: i32) -> ::windows::core::Result<I2cConnectionSettings> {
        Self::II2cConnectionSettingsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cConnectionSettings>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), slaveaddress, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn II2cConnectionSettingsFactory<R, F: FnOnce(&II2cConnectionSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<I2cConnectionSettings, II2cConnectionSettingsFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for I2cConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cConnectionSettings {}
impl ::core::fmt::Debug for I2cConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cConnectionSettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for I2cConnectionSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cConnectionSettings;{f2db1307-ab6f-4639-a767-54536dc3460f})");
}
impl ::core::clone::Clone for I2cConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for I2cConnectionSettings {
    type Vtable = II2cConnectionSettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for I2cConnectionSettings {
    const IID: ::windows::core::GUID = <II2cConnectionSettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for I2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.I2cConnectionSettings";
}
::windows::imp::interface_hierarchy!(I2cConnectionSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for I2cConnectionSettings {}
unsafe impl ::core::marker::Sync for I2cConnectionSettings {}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct I2cController(::windows::core::IUnknown);
impl I2cController {
    pub fn GetDevice(&self, settings: &I2cConnectionSettings) -> ::windows::core::Result<I2cDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cDevice>();
            (::windows::core::Interface::vtable(this).GetDevice)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(settings), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_I2c_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<P0>(provider: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>>
    where
        P0: ::windows::core::TryIntoParam<Provider::II2cProvider>,
    {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>>();
            (::windows::core::Interface::vtable(this).GetControllersAsync)(::windows::core::Interface::as_raw(this), provider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cController>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<I2cController>>();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn II2cControllerStatics<R, F: FnOnce(&II2cControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<I2cController, II2cControllerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for I2cController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cController {}
impl ::core::fmt::Debug for I2cController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cController").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for I2cController {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cController;{c48ab1b2-87a0-4166-8e3e-b4b8f97cd729})");
}
impl ::core::clone::Clone for I2cController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for I2cController {
    type Vtable = II2cController_Vtbl;
}
unsafe impl ::windows::core::ComInterface for I2cController {
    const IID: ::windows::core::GUID = <II2cController as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for I2cController {
    const NAME: &'static str = "Windows.Devices.I2c.I2cController";
}
::windows::imp::interface_hierarchy!(I2cController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for I2cController {}
unsafe impl ::core::marker::Sync for I2cController {}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct I2cDevice(::windows::core::IUnknown);
impl I2cDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConnectionSettings(&self) -> ::windows::core::Result<I2cConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cConnectionSettings>();
            (::windows::core::Interface::vtable(this).ConnectionSettings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Write)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_ptr()).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> ::windows::core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cTransferResult>();
            (::windows::core::Interface::vtable(this).WritePartial)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Read)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr()).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> ::windows::core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cTransferResult>();
            (::windows::core::Interface::vtable(this).ReadPartial)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).WriteRead)(::windows::core::Interface::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr()).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2cTransferResult>();
            (::windows::core::Interface::vtable(this).WriteReadPartial)(::windows::core::Interface::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName(friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING, settings: &I2cConnectionSettings) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<I2cDevice>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), ::core::mem::transmute_copy(settings), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn II2cDeviceStatics<R, F: FnOnce(&II2cDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<I2cDevice, II2cDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for I2cDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cDevice {}
impl ::core::fmt::Debug for I2cDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cDevice").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for I2cDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cDevice;{8636c136-b9c5-4f70-9449-cc46dc6f57eb})");
}
impl ::core::clone::Clone for I2cDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for I2cDevice {
    type Vtable = II2cDevice_Vtbl;
}
unsafe impl ::windows::core::ComInterface for I2cDevice {
    const IID: ::windows::core::GUID = <II2cDevice as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for I2cDevice {
    const NAME: &'static str = "Windows.Devices.I2c.I2cDevice";
}
::windows::imp::interface_hierarchy!(I2cDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for I2cDevice {}
unsafe impl ::core::marker::Send for I2cDevice {}
unsafe impl ::core::marker::Sync for I2cDevice {}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cBusSpeed {}
impl ::core::clone::Clone for I2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cBusSpeed {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for I2cBusSpeed {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for I2cBusSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cBusSpeed").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for I2cBusSpeed {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cBusSpeed;i4)");
}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cSharingMode {}
impl ::core::clone::Clone for I2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for I2cSharingMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for I2cSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cSharingMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for I2cSharingMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cSharingMode;i4)");
}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for I2cTransferStatus {}
impl ::core::clone::Clone for I2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for I2cTransferStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for I2cTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cTransferStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for I2cTransferStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cTransferStatus;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_I2c\"`*"]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for I2cTransferResult {}
impl ::core::clone::Clone for I2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for I2cTransferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
impl ::windows::core::TypeKind for I2cTransferResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for I2cTransferResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.I2cTransferResult;enum(Windows.Devices.I2c.I2cTransferStatus;i4);u4)");
}
impl ::core::cmp::PartialEq for I2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::core::cmp::Eq for I2cTransferResult {}
impl ::core::default::Default for I2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

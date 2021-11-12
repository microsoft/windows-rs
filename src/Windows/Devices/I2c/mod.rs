#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: I2cBusSpeed = I2cBusSpeed(0i32);
    pub const FastMode: I2cBusSpeed = I2cBusSpeed(1i32);
}
impl ::core::convert::From<i32> for I2cBusSpeed {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for I2cBusSpeed {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for I2cBusSpeed {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cBusSpeed;i4)");
}
impl ::windows::core::DefaultType for I2cBusSpeed {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct I2cConnectionSettings(pub ::windows::core::IInspectable);
impl I2cConnectionSettings {
    pub fn SlaveAddress(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BusSpeed(&self) -> ::windows::core::Result<I2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__: I2cBusSpeed = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2cBusSpeed>(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: I2cBusSpeed) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<I2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__: I2cSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2cSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: I2cSharingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Create(slaveaddress: i32) -> ::windows::core::Result<I2cConnectionSettings> {
        Self::II2cConnectionSettingsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), slaveaddress, &mut result__).from_abi::<I2cConnectionSettings>(result__)
        })
    }
    pub fn II2cConnectionSettingsFactory<R, F: FnOnce(&II2cConnectionSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<I2cConnectionSettings, II2cConnectionSettingsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for I2cConnectionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cConnectionSettings;{f2db1307-ab6f-4639-a767-54536dc3460f})");
}
unsafe impl ::windows::core::Interface for I2cConnectionSettings {
    type Vtable = II2cConnectionSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2db1307_ab6f_4639_a767_54536dc3460f);
}
impl ::windows::core::RuntimeName for I2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.I2cConnectionSettings";
}
impl ::core::convert::From<I2cConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: I2cConnectionSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&I2cConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: &I2cConnectionSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for I2cConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a I2cConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<I2cConnectionSettings> for ::windows::core::IInspectable {
    fn from(value: I2cConnectionSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&I2cConnectionSettings> for ::windows::core::IInspectable {
    fn from(value: &I2cConnectionSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for I2cConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a I2cConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for I2cConnectionSettings {}
unsafe impl ::core::marker::Sync for I2cConnectionSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct I2cController(pub ::windows::core::IInspectable);
impl I2cController {
    pub fn GetDevice<'a, Param0: ::windows::core::IntoParam<'a, I2cConnectionSettings>>(&self, settings: Param0) -> ::windows::core::Result<I2cDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<I2cDevice>(result__)
        }
    }
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, Param0: ::windows::core::IntoParam<'a, Provider::II2cProvider>>(provider: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cController>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<I2cController>>(result__)
        })
    }
    pub fn II2cControllerStatics<R, F: FnOnce(&II2cControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<I2cController, II2cControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for I2cController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cController;{c48ab1b2-87a0-4166-8e3e-b4b8f97cd729})");
}
unsafe impl ::windows::core::Interface for I2cController {
    type Vtable = II2cController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc48ab1b2_87a0_4166_8e3e_b4b8f97cd729);
}
impl ::windows::core::RuntimeName for I2cController {
    const NAME: &'static str = "Windows.Devices.I2c.I2cController";
}
impl ::core::convert::From<I2cController> for ::windows::core::IUnknown {
    fn from(value: I2cController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&I2cController> for ::windows::core::IUnknown {
    fn from(value: &I2cController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for I2cController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a I2cController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<I2cController> for ::windows::core::IInspectable {
    fn from(value: I2cController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&I2cController> for ::windows::core::IInspectable {
    fn from(value: &I2cController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for I2cController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a I2cController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for I2cController {}
unsafe impl ::core::marker::Sync for I2cController {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct I2cDevice(pub ::windows::core::IInspectable);
impl I2cDevice {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionSettings(&self) -> ::windows::core::Result<I2cConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2cConnectionSettings>(result__)
        }
    }
    pub fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr())).ok() }
    }
    pub fn WritePartial(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__: I2cTransferResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), &mut result__).from_abi::<I2cTransferResult>(result__)
        }
    }
    pub fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer)).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__: I2cTransferResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer), &mut result__).from_abi::<I2cTransferResult>(result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer)).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__: I2cTransferResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer), &mut result__).from_abi::<I2cTransferResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(friendlyname: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, I2cConnectionSettings>>(deviceid: Param0, settings: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<I2cDevice>>(result__)
        })
    }
    pub fn II2cDeviceStatics<R, F: FnOnce(&II2cDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<I2cDevice, II2cDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for I2cDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cDevice;{8636c136-b9c5-4f70-9449-cc46dc6f57eb})");
}
unsafe impl ::windows::core::Interface for I2cDevice {
    type Vtable = II2cDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8636c136_b9c5_4f70_9449_cc46dc6f57eb);
}
impl ::windows::core::RuntimeName for I2cDevice {
    const NAME: &'static str = "Windows.Devices.I2c.I2cDevice";
}
impl ::core::convert::From<I2cDevice> for ::windows::core::IUnknown {
    fn from(value: I2cDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&I2cDevice> for ::windows::core::IUnknown {
    fn from(value: &I2cDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for I2cDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a I2cDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<I2cDevice> for ::windows::core::IInspectable {
    fn from(value: I2cDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&I2cDevice> for ::windows::core::IInspectable {
    fn from(value: &I2cDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for I2cDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a I2cDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<I2cDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: I2cDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&I2cDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &I2cDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for I2cDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &I2cDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for I2cDevice {}
unsafe impl ::core::marker::Sync for I2cDevice {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: I2cSharingMode = I2cSharingMode(0i32);
    pub const Shared: I2cSharingMode = I2cSharingMode(1i32);
}
impl ::core::convert::From<i32> for I2cSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for I2cSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for I2cSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cSharingMode;i4)");
}
impl ::windows::core::DefaultType for I2cSharingMode {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl I2cTransferResult {}
impl ::core::default::Default for I2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for I2cTransferResult {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("I2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
impl ::core::cmp::PartialEq for I2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::core::cmp::Eq for I2cTransferResult {}
unsafe impl ::windows::core::Abi for I2cTransferResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for I2cTransferResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.I2cTransferResult;enum(Windows.Devices.I2c.I2cTransferStatus;i4);u4)");
}
impl ::windows::core::DefaultType for I2cTransferResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: I2cTransferStatus = I2cTransferStatus(0i32);
    pub const PartialTransfer: I2cTransferStatus = I2cTransferStatus(1i32);
    pub const SlaveAddressNotAcknowledged: I2cTransferStatus = I2cTransferStatus(2i32);
    pub const ClockStretchTimeout: I2cTransferStatus = I2cTransferStatus(3i32);
    pub const UnknownError: I2cTransferStatus = I2cTransferStatus(4i32);
}
impl ::core::convert::From<i32> for I2cTransferStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for I2cTransferStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for I2cTransferStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cTransferStatus;i4)");
}
impl ::windows::core::DefaultType for I2cTransferStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct II2cConnectionSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for II2cConnectionSettings {
    type Vtable = II2cConnectionSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2db1307_ab6f_4639_a767_54536dc3460f);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut I2cBusSpeed) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: I2cBusSpeed) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut I2cSharingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: I2cSharingMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct II2cConnectionSettingsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for II2cConnectionSettingsFactory {
    type Vtable = II2cConnectionSettingsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81b586b3_9693_41b1_a243_ded4f6e66926);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettingsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, slaveaddress: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct II2cController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for II2cController {
    type Vtable = II2cController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc48ab1b2_87a0_4166_8e3e_b4b8f97cd729);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct II2cControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for II2cControllerStatics {
    type Vtable = II2cControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40fc0365_5f05_4e7e_84bd_100db8e0aec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_I2c_Provider", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct II2cDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for II2cDevice {
    type Vtable = II2cDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8636c136_b9c5_4f70_9449_cc46dc6f57eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *const u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct II2cDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for II2cDeviceStatics {
    type Vtable = II2cDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a33be3_7334_4512_96bc_fbae9459f5f6);
}
impl II2cDeviceStatics {
    pub fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDeviceSelectorFromFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, friendlyname: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, I2cConnectionSettings>>(&self, deviceid: Param0, settings: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<I2cDevice>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for II2cDeviceStatics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{91a33be3-7334-4512-96bc-fbae9459f5f6}");
}
impl ::core::convert::From<II2cDeviceStatics> for ::windows::core::IUnknown {
    fn from(value: II2cDeviceStatics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&II2cDeviceStatics> for ::windows::core::IUnknown {
    fn from(value: &II2cDeviceStatics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for II2cDeviceStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a II2cDeviceStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<II2cDeviceStatics> for ::windows::core::IInspectable {
    fn from(value: II2cDeviceStatics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&II2cDeviceStatics> for ::windows::core::IInspectable {
    fn from(value: &II2cDeviceStatics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for II2cDeviceStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a II2cDeviceStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);

#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct II2cControllerProvider(::windows::core::IUnknown);
impl II2cControllerProvider {
    pub fn GetDeviceProvider(&self, settings: &ProviderI2cConnectionSettings) -> ::windows::core::Result<II2cDeviceProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceProvider)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(II2cControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for II2cControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cControllerProvider {}
impl ::core::fmt::Debug for II2cControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for II2cControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61c2bb82-4510-4163-a87c-4e15a9558980}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for II2cControllerProvider {
    type Vtable = II2cControllerProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for II2cControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c2bb82_4510_4163_a87c_4e15a9558980);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct II2cDeviceProvider(::windows::core::IUnknown);
impl II2cDeviceProvider {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Write)(::windows::core::Vtable::as_raw(this), buffer.len() as u32, buffer.as_ptr()).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> ::windows::core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WritePartial)(::windows::core::Vtable::as_raw(this), buffer.len() as u32, buffer.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Read)(::windows::core::Vtable::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr()).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> ::windows::core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadPartial)(::windows::core::Vtable::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteRead)(::windows::core::Vtable::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr()).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteReadPartial)(::windows::core::Vtable::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(II2cDeviceProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<II2cDeviceProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: II2cDeviceProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&II2cDeviceProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &II2cDeviceProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&II2cDeviceProvider> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &II2cDeviceProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for II2cDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cDeviceProvider {}
impl ::core::fmt::Debug for II2cDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cDeviceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for II2cDeviceProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ad342654-57e8-453e-8329-d1e447d103a9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for II2cDeviceProvider {
    type Vtable = II2cDeviceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for II2cDeviceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad342654_57e8_453e_8329_d1e447d103a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows::core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct II2cProvider(::windows::core::IUnknown);
impl II2cProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetControllersAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(II2cProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for II2cProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cProvider {}
impl ::core::fmt::Debug for II2cProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for II2cProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6f13083e-bf62-4fe2-a95a-f08999669818}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for II2cProvider {
    type Vtable = II2cProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for II2cProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f13083e_bf62_4fe2_a95a_f08999669818);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProviderI2cConnectionSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProviderI2cConnectionSettings {
    type Vtable = IProviderI2cConnectionSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderI2cConnectionSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9db4e34_e510_44b7_809d_f2f85b555339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderI2cConnectionSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderI2cBusSpeed) -> ::windows::core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderI2cBusSpeed) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderI2cSharingMode) -> ::windows::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderI2cSharingMode) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderI2cConnectionSettings(::windows::core::IUnknown);
impl ProviderI2cConnectionSettings {
    pub fn SlaveAddress(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SlaveAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSlaveAddress)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BusSpeed(&self) -> ::windows::core::Result<ProviderI2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BusSpeed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: ProviderI2cBusSpeed) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBusSpeed)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<ProviderI2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SharingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSharingMode(&self, value: ProviderI2cSharingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSharingMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ProviderI2cConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProviderI2cConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProviderI2cConnectionSettings {}
impl ::core::fmt::Debug for ProviderI2cConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cConnectionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderI2cConnectionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings;{e9db4e34-e510-44b7-809d-f2f85b555339})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProviderI2cConnectionSettings {
    type Vtable = IProviderI2cConnectionSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for ProviderI2cConnectionSettings {
    const IID: ::windows::core::GUID = <IProviderI2cConnectionSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProviderI2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings";
}
::windows::core::interface_hierarchy!(ProviderI2cConnectionSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProviderI2cConnectionSettings {}
unsafe impl ::core::marker::Sync for ProviderI2cConnectionSettings {}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cBusSpeed {}
impl ::core::clone::Clone for ProviderI2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderI2cBusSpeed {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProviderI2cBusSpeed {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderI2cBusSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cBusSpeed").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderI2cBusSpeed {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cBusSpeed;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cSharingMode {}
impl ::core::clone::Clone for ProviderI2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderI2cSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProviderI2cSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderI2cSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderI2cSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
}
impl ::core::marker::Copy for ProviderI2cTransferStatus {}
impl ::core::clone::Clone for ProviderI2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderI2cTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProviderI2cTransferStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderI2cTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cTransferStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderI2cTransferStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
pub struct ProviderI2cTransferResult {
    pub Status: ProviderI2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for ProviderI2cTransferResult {}
impl ::core::clone::Clone for ProviderI2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ProviderI2cTransferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProviderI2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
unsafe impl ::windows::core::Abi for ProviderI2cTransferResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderI2cTransferResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.Provider.ProviderI2cTransferResult;enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ProviderI2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::core::cmp::Eq for ProviderI2cTransferResult {}
impl ::core::default::Default for ProviderI2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

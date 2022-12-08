#[doc(hidden)]
#[repr(transparent)]
pub struct IProviderSpiConnectionSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProviderSpiConnectionSettings {
    type Vtable = IProviderSpiConnectionSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderSpiConnectionSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6034550_a542_4ec0_9601_a4dd68f8697b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChipSelectLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetChipSelectLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderSpiMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderSpiMode) -> ::windows::core::HRESULT,
    pub DataBitLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDataBitLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ClockFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetClockFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderSpiSharingMode) -> ::windows::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderSpiSharingMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProviderSpiConnectionSettingsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProviderSpiConnectionSettingsFactory {
    type Vtable = IProviderSpiConnectionSettingsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderSpiConnectionSettingsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66456b5a_0c79_43e3_9f3c_e59780ac18fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettingsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chipselectline: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
pub struct ISpiControllerProvider(::windows::core::IUnknown);
impl ISpiControllerProvider {
    pub fn GetDeviceProvider(&self, settings: &ProviderSpiConnectionSettings) -> ::windows::core::Result<ISpiDeviceProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceProvider)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISpiControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ISpiControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpiControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpiControllerProvider {}
impl ::core::fmt::Debug for ISpiControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpiControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c1686504-02ce-4226-a385-4f11fb04b41b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISpiControllerProvider {
    type Vtable = ISpiControllerProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpiControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1686504_02ce_4226_a385_4f11fb04b41b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
pub struct ISpiDeviceProvider(::windows::core::IUnknown);
impl ISpiDeviceProvider {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConnectionSettings(&self) -> ::windows::core::Result<ProviderSpiConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionSettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Write)(::windows::core::Vtable::as_raw(this), buffer.len() as u32, buffer.as_ptr()).ok() }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Read)(::windows::core::Vtable::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr()).ok() }
    }
    pub fn TransferSequential(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).TransferSequential)(::windows::core::Vtable::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr()).ok() }
    }
    pub fn TransferFullDuplex(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).TransferFullDuplex)(::windows::core::Vtable::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(ISpiDeviceProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ISpiDeviceProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ISpiDeviceProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ISpiDeviceProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISpiDeviceProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ISpiDeviceProvider> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISpiDeviceProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ISpiDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpiDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpiDeviceProvider {}
impl ::core::fmt::Debug for ISpiDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpiDeviceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiDeviceProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0d1c3443-304b-405c-b4f7-f5ab1074461e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISpiDeviceProvider {
    type Vtable = ISpiDeviceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpiDeviceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d1c3443_304b_405c_b4f7_f5ab1074461e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDeviceProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub TransferSequential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub TransferFullDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
pub struct ISpiProvider(::windows::core::IUnknown);
impl ISpiProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetControllersAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISpiProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ISpiProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpiProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpiProvider {}
impl ::core::fmt::Debug for ISpiProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpiProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{96b461e2-77d4-48ce-aaa0-75715a8362cf}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISpiProvider {
    type Vtable = ISpiProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpiProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96b461e2_77d4_48ce_aaa0_75715a8362cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllersAsync: usize,
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderSpiConnectionSettings(::windows::core::IUnknown);
impl ProviderSpiConnectionSettings {
    pub fn ChipSelectLine(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChipSelectLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChipSelectLine(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetChipSelectLine)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Mode(&self) -> ::windows::core::Result<ProviderSpiMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: ProviderSpiMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DataBitLength(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataBitLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDataBitLength(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDataBitLength)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ClockFrequency(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClockFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetClockFrequency(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetClockFrequency)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<ProviderSpiSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SharingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSharingMode(&self, value: ProviderSpiSharingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSharingMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(chipselectline: i32) -> ::windows::core::Result<ProviderSpiConnectionSettings> {
        Self::IProviderSpiConnectionSettingsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), chipselectline, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProviderSpiConnectionSettingsFactory<R, F: FnOnce(&IProviderSpiConnectionSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProviderSpiConnectionSettings, IProviderSpiConnectionSettingsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ProviderSpiConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProviderSpiConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProviderSpiConnectionSettings {}
impl ::core::fmt::Debug for ProviderSpiConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderSpiConnectionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderSpiConnectionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings;{f6034550-a542-4ec0-9601-a4dd68f8697b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProviderSpiConnectionSettings {
    type Vtable = IProviderSpiConnectionSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for ProviderSpiConnectionSettings {
    const IID: ::windows::core::GUID = <IProviderSpiConnectionSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProviderSpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings";
}
::windows::core::interface_hierarchy!(ProviderSpiConnectionSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProviderSpiConnectionSettings {}
unsafe impl ::core::marker::Sync for ProviderSpiConnectionSettings {}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for ProviderSpiMode {}
impl ::core::clone::Clone for ProviderSpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderSpiMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProviderSpiMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderSpiMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderSpiMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderSpiMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.Provider.ProviderSpiMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderSpiSharingMode {}
impl ::core::clone::Clone for ProviderSpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderSpiSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProviderSpiSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderSpiSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderSpiSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderSpiSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.Provider.ProviderSpiSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

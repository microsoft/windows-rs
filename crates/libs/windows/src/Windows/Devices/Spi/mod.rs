#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpiBusInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpiBusInfo {
    type Vtable = ISpiBusInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9929444a_54f2_48c6_b952_9c32fc02c669);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiBusInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ChipSelectLineCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MinClockFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxClockFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDataBitLengths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDataBitLengths: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpiConnectionSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpiConnectionSettings {
    type Vtable = ISpiConnectionSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5283a37f_f935_4b9f_a7a7_3a7890afa5ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiConnectionSettings_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ChipSelectLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetChipSelectLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpiMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpiMode) -> ::windows::core::HRESULT,
    pub DataBitLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDataBitLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ClockFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetClockFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpiSharingMode) -> ::windows::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpiSharingMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpiConnectionSettingsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpiConnectionSettingsFactory {
    type Vtable = ISpiConnectionSettingsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff99081e_10c4_44b7_9fea_a748b5a46f31);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiConnectionSettingsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chipselectline: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpiController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpiController {
    type Vtable = ISpiController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8d3c829_9895_4159_a934_8741f1ee6d27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpiControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpiControllerStatics {
    type Vtable = ISpiControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d5229e2_138b_4e48_b964_4f2f79b9c5a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(all(feature = "Devices_Spi_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Spi_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpiDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpiDevice {
    type Vtable = ISpiDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05d5356d_11b6_4d39_84d5_95dfb4c9f2ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDevice_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub TransferSequential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub TransferFullDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct ISpiDeviceStatics(::windows::core::IUnknown);
impl ISpiDeviceStatics {
    pub fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetBusInfo(&self, busid: &::windows::core::HSTRING) -> ::windows::core::Result<SpiBusInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetBusInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(busid), result__.as_mut_ptr()).from_abi::<SpiBusInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, P0>(&self, busid: &::windows::core::HSTRING, settings: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiDevice>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SpiConnectionSettings>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(busid), settings.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<SpiDevice>>(result__)
        }
    }
}
impl ::core::convert::From<ISpiDeviceStatics> for ::windows::core::IUnknown {
    fn from(value: ISpiDeviceStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISpiDeviceStatics> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISpiDeviceStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpiDeviceStatics> for ::windows::core::IUnknown {
    fn from(value: &ISpiDeviceStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISpiDeviceStatics> for ::windows::core::IInspectable {
    fn from(value: ISpiDeviceStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISpiDeviceStatics> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISpiDeviceStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpiDeviceStatics> for ::windows::core::IInspectable {
    fn from(value: &ISpiDeviceStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISpiDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpiDeviceStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpiDeviceStatics {}
impl ::core::fmt::Debug for ISpiDeviceStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpiDeviceStatics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiDeviceStatics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a278e559-5720-4d3f-bd93-56f5ff5a5879}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISpiDeviceStatics {
    type Vtable = ISpiDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa278e559_5720_4d3f_bd93_56f5ff5a5879);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetBusInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct SpiBusInfo(::windows::core::IUnknown);
impl SpiBusInfo {
    pub fn ChipSelectLineCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChipSelectLineCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MinClockFrequency(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MinClockFrequency)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxClockFrequency(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxClockFrequency)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDataBitLengths(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedDataBitLengths)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpiBusInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpiBusInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpiBusInfo {}
impl ::core::fmt::Debug for SpiBusInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpiBusInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpiBusInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Spi.SpiBusInfo;{9929444a-54f2-48c6-b952-9c32fc02c669})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpiBusInfo {
    type Vtable = ISpiBusInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISpiBusInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpiBusInfo {
    const NAME: &'static str = "Windows.Devices.Spi.SpiBusInfo";
}
impl ::core::convert::From<SpiBusInfo> for ::windows::core::IUnknown {
    fn from(value: SpiBusInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiBusInfo> for ::windows::core::IUnknown {
    fn from(value: &SpiBusInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiBusInfo> for &::windows::core::IUnknown {
    fn from(value: &SpiBusInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SpiBusInfo> for ::windows::core::IInspectable {
    fn from(value: SpiBusInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiBusInfo> for ::windows::core::IInspectable {
    fn from(value: &SpiBusInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiBusInfo> for &::windows::core::IInspectable {
    fn from(value: &SpiBusInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for SpiBusInfo {}
unsafe impl ::core::marker::Sync for SpiBusInfo {}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct SpiConnectionSettings(::windows::core::IUnknown);
impl SpiConnectionSettings {
    pub fn ChipSelectLine(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChipSelectLine)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetChipSelectLine(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetChipSelectLine)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Mode(&self) -> ::windows::core::Result<SpiMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpiMode>(result__)
        }
    }
    pub fn SetMode(&self, value: SpiMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataBitLength(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DataBitLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDataBitLength(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataBitLength)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClockFrequency(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ClockFrequency)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetClockFrequency(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClockFrequency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<SpiSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SharingMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpiSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: SpiSharingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSharingMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(chipselectline: i32) -> ::windows::core::Result<SpiConnectionSettings> {
        Self::ISpiConnectionSettingsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), chipselectline, result__.as_mut_ptr()).from_abi::<SpiConnectionSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpiConnectionSettingsFactory<R, F: FnOnce(&ISpiConnectionSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpiConnectionSettings, ISpiConnectionSettingsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SpiConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpiConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpiConnectionSettings {}
impl ::core::fmt::Debug for SpiConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpiConnectionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpiConnectionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Spi.SpiConnectionSettings;{5283a37f-f935-4b9f-a7a7-3a7890afa5ce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpiConnectionSettings {
    type Vtable = ISpiConnectionSettings_Vtbl;
    const IID: ::windows::core::GUID = <ISpiConnectionSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.SpiConnectionSettings";
}
impl ::core::convert::From<SpiConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: SpiConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: &SpiConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiConnectionSettings> for &::windows::core::IUnknown {
    fn from(value: &SpiConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SpiConnectionSettings> for ::windows::core::IInspectable {
    fn from(value: SpiConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiConnectionSettings> for ::windows::core::IInspectable {
    fn from(value: &SpiConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiConnectionSettings> for &::windows::core::IInspectable {
    fn from(value: &SpiConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for SpiConnectionSettings {}
unsafe impl ::core::marker::Sync for SpiConnectionSettings {}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct SpiController(::windows::core::IUnknown);
impl SpiController {
    pub fn GetDevice<'a, P0>(&self, settings: P0) -> ::windows::core::Result<SpiDevice>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SpiConnectionSettings>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDevice)(::windows::core::Interface::as_raw(this), settings.into().abi(), result__.as_mut_ptr()).from_abi::<SpiDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiController>> {
        Self::ISpiControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<SpiController>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Spi_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Spi_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, P0, E0>(provider: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SpiController>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, Provider::ISpiProvider>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ISpiControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetControllersAsync)(::windows::core::Interface::as_raw(this), provider.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SpiController>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpiControllerStatics<R, F: FnOnce(&ISpiControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpiController, ISpiControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SpiController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpiController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpiController {}
impl ::core::fmt::Debug for SpiController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpiController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpiController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Spi.SpiController;{a8d3c829-9895-4159-a934-8741f1ee6d27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpiController {
    type Vtable = ISpiController_Vtbl;
    const IID: ::windows::core::GUID = <ISpiController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpiController {
    const NAME: &'static str = "Windows.Devices.Spi.SpiController";
}
impl ::core::convert::From<SpiController> for ::windows::core::IUnknown {
    fn from(value: SpiController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiController> for ::windows::core::IUnknown {
    fn from(value: &SpiController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiController> for &::windows::core::IUnknown {
    fn from(value: &SpiController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SpiController> for ::windows::core::IInspectable {
    fn from(value: SpiController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiController> for ::windows::core::IInspectable {
    fn from(value: &SpiController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiController> for &::windows::core::IInspectable {
    fn from(value: &SpiController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for SpiController {}
unsafe impl ::core::marker::Sync for SpiController {}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct SpiDevice(::windows::core::IUnknown);
impl SpiDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionSettings(&self) -> ::windows::core::Result<SpiConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionSettings)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpiConnectionSettings>(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Write)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_ptr()).ok() }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Read)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr()).ok() }
    }
    pub fn TransferSequential(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TransferSequential)(::windows::core::Interface::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr()).ok() }
    }
    pub fn TransferFullDuplex(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TransferFullDuplex)(::windows::core::Interface::as_raw(this), writebuffer.len() as u32, writebuffer.as_ptr(), readbuffer.len() as u32, readbuffer.as_mut_ptr()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName(friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetBusInfo(busid: &::windows::core::HSTRING) -> ::windows::core::Result<SpiBusInfo> {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetBusInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(busid), result__.as_mut_ptr()).from_abi::<SpiBusInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, P0>(busid: &::windows::core::HSTRING, settings: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiDevice>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SpiConnectionSettings>>,
    {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(busid), settings.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<SpiDevice>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpiDeviceStatics<R, F: FnOnce(&ISpiDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpiDevice, ISpiDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SpiDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpiDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpiDevice {}
impl ::core::fmt::Debug for SpiDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpiDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpiDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Spi.SpiDevice;{05d5356d-11b6-4d39-84d5-95dfb4c9f2ce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpiDevice {
    type Vtable = ISpiDevice_Vtbl;
    const IID: ::windows::core::GUID = <ISpiDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpiDevice {
    const NAME: &'static str = "Windows.Devices.Spi.SpiDevice";
}
impl ::core::convert::From<SpiDevice> for ::windows::core::IUnknown {
    fn from(value: SpiDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiDevice> for ::windows::core::IUnknown {
    fn from(value: &SpiDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiDevice> for &::windows::core::IUnknown {
    fn from(value: &SpiDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SpiDevice> for ::windows::core::IInspectable {
    fn from(value: SpiDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpiDevice> for ::windows::core::IInspectable {
    fn from(value: &SpiDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SpiDevice> for &::windows::core::IInspectable {
    fn from(value: &SpiDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SpiDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SpiDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SpiDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpiDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&SpiDevice> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpiDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SpiDevice {}
unsafe impl ::core::marker::Sync for SpiDevice {}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpiMode(pub i32);
impl SpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for SpiMode {}
impl ::core::clone::Clone for SpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpiMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpiMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpiMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpiMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpiMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.SpiMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpiSharingMode(pub i32);
impl SpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for SpiSharingMode {}
impl ::core::clone::Clone for SpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpiSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpiSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpiSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpiSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpiSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.SpiSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

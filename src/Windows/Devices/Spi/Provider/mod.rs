#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProviderSpiConnectionSettings {
    type Vtable = IProviderSpiConnectionSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6034550_a542_4ec0_9601_a4dd68f8697b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ProviderSpiMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ProviderSpiMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ProviderSpiSharingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ProviderSpiSharingMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettingsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProviderSpiConnectionSettingsFactory {
    type Vtable = IProviderSpiConnectionSettingsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66456b5a_0c79_43e3_9f3c_e59780ac18fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettingsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, chipselectline: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Spi_Provider`*"]
pub struct ISpiControllerProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpiControllerProvider {
    type Vtable = ISpiControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1686504_02ce_4226_a385_4f11fb04b41b);
}
impl ISpiControllerProvider {
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn GetDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, ProviderSpiConnectionSettings>>(&self, settings: Param0) -> ::windows::core::Result<ISpiDeviceProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<ISpiDeviceProvider>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c1686504-02ce-4226-a385-4f11fb04b41b}");
}
impl ::core::convert::From<ISpiControllerProvider> for ::windows::core::IUnknown {
    fn from(value: ISpiControllerProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISpiControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &ISpiControllerProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpiControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpiControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISpiControllerProvider> for ::windows::core::IInspectable {
    fn from(value: ISpiControllerProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpiControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &ISpiControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISpiControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISpiControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Spi_Provider`*"]
pub struct ISpiDeviceProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpiDeviceProvider {
    type Vtable = ISpiDeviceProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d1c3443_304b_405c_b4f7_f5ab1074461e);
}
impl ISpiDeviceProvider {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Spi_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn ConnectionSettings(&self) -> ::windows::core::Result<ProviderSpiConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderSpiConnectionSettings>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer)).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn TransferSequential(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer)).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn TransferFullDuplex(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiDeviceProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0d1c3443-304b-405c-b4f7-f5ab1074461e}");
}
impl ::core::convert::From<ISpiDeviceProvider> for ::windows::core::IUnknown {
    fn from(value: ISpiDeviceProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISpiDeviceProvider> for ::windows::core::IUnknown {
    fn from(value: &ISpiDeviceProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpiDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpiDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISpiDeviceProvider> for ::windows::core::IInspectable {
    fn from(value: ISpiDeviceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpiDeviceProvider> for ::windows::core::IInspectable {
    fn from(value: &ISpiDeviceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISpiDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISpiDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for ISpiDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &ISpiDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDeviceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Spi_Provider`*"]
pub struct ISpiProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpiProvider {
    type Vtable = ISpiProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96b461e2_77d4_48ce_aaa0_75715a8362cf);
}
impl ISpiProvider {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Spi_Provider`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ISpiProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{96b461e2-77d4-48ce-aaa0-75715a8362cf}");
}
impl ::core::convert::From<ISpiProvider> for ::windows::core::IUnknown {
    fn from(value: ISpiProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISpiProvider> for ::windows::core::IUnknown {
    fn from(value: &ISpiProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpiProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpiProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISpiProvider> for ::windows::core::IInspectable {
    fn from(value: ISpiProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpiProvider> for ::windows::core::IInspectable {
    fn from(value: &ISpiProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISpiProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISpiProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc = "*Required features: `Devices_Spi_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProviderSpiConnectionSettings(pub ::windows::core::IInspectable);
impl ProviderSpiConnectionSettings {
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn ChipSelectLine(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn SetChipSelectLine(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn Mode(&self) -> ::windows::core::Result<ProviderSpiMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderSpiMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderSpiMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn SetMode(&self, value: ProviderSpiMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn DataBitLength(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn SetDataBitLength(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn ClockFrequency(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn SetClockFrequency(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn SharingMode(&self) -> ::windows::core::Result<ProviderSpiSharingMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderSpiSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderSpiSharingMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn SetSharingMode(&self, value: ProviderSpiSharingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Spi_Provider`*"]
    pub fn Create(chipselectline: i32) -> ::windows::core::Result<ProviderSpiConnectionSettings> {
        Self::IProviderSpiConnectionSettingsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), chipselectline, &mut result__).from_abi::<ProviderSpiConnectionSettings>(result__)
        })
    }
    pub fn IProviderSpiConnectionSettingsFactory<R, F: FnOnce(&IProviderSpiConnectionSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProviderSpiConnectionSettings, IProviderSpiConnectionSettingsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderSpiConnectionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings;{f6034550-a542-4ec0-9601-a4dd68f8697b})");
}
unsafe impl ::windows::core::Interface for ProviderSpiConnectionSettings {
    type Vtable = IProviderSpiConnectionSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6034550_a542_4ec0_9601_a4dd68f8697b);
}
impl ::windows::core::RuntimeName for ProviderSpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings";
}
impl ::core::convert::From<ProviderSpiConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: ProviderSpiConnectionSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProviderSpiConnectionSettings> for ::windows::core::IUnknown {
    fn from(value: &ProviderSpiConnectionSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProviderSpiConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProviderSpiConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProviderSpiConnectionSettings> for ::windows::core::IInspectable {
    fn from(value: ProviderSpiConnectionSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProviderSpiConnectionSettings> for ::windows::core::IInspectable {
    fn from(value: &ProviderSpiConnectionSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProviderSpiConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProviderSpiConnectionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProviderSpiConnectionSettings {}
unsafe impl ::core::marker::Sync for ProviderSpiConnectionSettings {}
#[doc = "*Required features: `Devices_Spi_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: ProviderSpiMode = ProviderSpiMode(0i32);
    pub const Mode1: ProviderSpiMode = ProviderSpiMode(1i32);
    pub const Mode2: ProviderSpiMode = ProviderSpiMode(2i32);
    pub const Mode3: ProviderSpiMode = ProviderSpiMode(3i32);
}
impl ::core::convert::From<i32> for ProviderSpiMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProviderSpiMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderSpiMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.Provider.ProviderSpiMode;i4)");
}
impl ::windows::core::DefaultType for ProviderSpiMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Spi_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: ProviderSpiSharingMode = ProviderSpiSharingMode(0i32);
    pub const Shared: ProviderSpiSharingMode = ProviderSpiSharingMode(1i32);
}
impl ::core::convert::From<i32> for ProviderSpiSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProviderSpiSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderSpiSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.Provider.ProviderSpiSharingMode;i4)");
}
impl ::windows::core::DefaultType for ProviderSpiSharingMode {
    type DefaultType = Self;
}

#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_Adc_Provider`*"]
pub struct IAdcControllerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdcControllerProvider {
    type Vtable = IAdcControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3193198632, 33133, 19941, [160, 72, 171, 160, 105, 88, 170, 168]);
}
impl IAdcControllerProvider {
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn ChannelCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn ResolutionInBits(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn MinValue(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn MaxValue(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn ChannelMode(&self) -> ::windows::runtime::Result<ProviderAdcChannelMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderAdcChannelMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProviderAdcChannelMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), channelmode, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn AcquireChannel(&self, channel: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), channel).ok() }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn ReleaseChannel(&self, channel: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), channel).ok() }
    }
    #[doc = "*Required features: `Devices_Adc_Provider`*"]
    pub fn ReadValue(&self, channelnumber: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), channelnumber, &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAdcControllerProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{be545828-816d-4de5-a048-aba06958aaa8}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ProviderAdcChannelMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ProviderAdcChannelMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channelnumber: i32, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_Adc_Provider`*"]
pub struct IAdcProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdcProvider {
    type Vtable = IAdcProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(680867432, 37721, 19543, [188, 136, 226, 117, 232, 22, 56, 201]);
}
impl IAdcProvider {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Adc_Provider`, `Foundation_Collections`*"]
    pub fn GetControllers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAdcProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{28953668-9359-4c57-bc88-e275e81638c9}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Devices_Adc_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderAdcChannelMode(pub i32);
impl ProviderAdcChannelMode {
    pub const SingleEnded: ProviderAdcChannelMode = ProviderAdcChannelMode(0i32);
    pub const Differential: ProviderAdcChannelMode = ProviderAdcChannelMode(1i32);
}
impl ::std::convert::From<i32> for ProviderAdcChannelMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProviderAdcChannelMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProviderAdcChannelMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.Provider.ProviderAdcChannelMode;i4)");
}
impl ::windows::runtime::DefaultType for ProviderAdcChannelMode {
    type DefaultType = Self;
}

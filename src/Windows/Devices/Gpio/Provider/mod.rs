#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GpioPinProviderValueChangedEventArgs(pub ::windows::core::IInspectable);
impl GpioPinProviderValueChangedEventArgs {
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn Edge(&self) -> ::windows::core::Result<ProviderGpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__: ProviderGpioPinEdge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderGpioPinEdge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn Create(edge: ProviderGpioPinEdge) -> ::windows::core::Result<GpioPinProviderValueChangedEventArgs> {
        Self::IGpioPinProviderValueChangedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), edge, &mut result__).from_abi::<GpioPinProviderValueChangedEventArgs>(result__)
        })
    }
    pub fn IGpioPinProviderValueChangedEventArgsFactory<R, F: FnOnce(&IGpioPinProviderValueChangedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GpioPinProviderValueChangedEventArgs, IGpioPinProviderValueChangedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinProviderValueChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs;{32a6d6f2-3d5b-44cd-8fbe-13a69f2edb24})");
}
unsafe impl ::windows::core::Interface for GpioPinProviderValueChangedEventArgs {
    type Vtable = IGpioPinProviderValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32a6d6f2_3d5b_44cd_8fbe_13a69f2edb24);
}
impl ::windows::core::RuntimeName for GpioPinProviderValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs";
}
impl ::core::convert::From<GpioPinProviderValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GpioPinProviderValueChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GpioPinProviderValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GpioPinProviderValueChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GpioPinProviderValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GpioPinProviderValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GpioPinProviderValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GpioPinProviderValueChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GpioPinProviderValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GpioPinProviderValueChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GpioPinProviderValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GpioPinProviderValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GpioPinProviderValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GpioPinProviderValueChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
pub struct IGpioControllerProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioControllerProvider {
    type Vtable = IGpioControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad11cec7_19ea_4b21_874f_b91aed4a25db);
}
impl IGpioControllerProvider {
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows::core::Result<IGpioPinProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pin, sharingmode, &mut result__).from_abi::<IGpioPinProvider>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGpioControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ad11cec7-19ea-4b21-874f-b91aed4a25db}");
}
impl ::core::convert::From<IGpioControllerProvider> for ::windows::core::IUnknown {
    fn from(value: IGpioControllerProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGpioControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &IGpioControllerProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGpioControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGpioControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGpioControllerProvider> for ::windows::core::IInspectable {
    fn from(value: IGpioControllerProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGpioControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &IGpioControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGpioControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGpioControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
pub struct IGpioPinProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioPinProvider {
    type Vtable = IGpioPinProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42344cb7_6abc_40ff_9ce7_73b85301b900);
}
impl IGpioPinProvider {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Gpio_Provider`, `Foundation`*"]
    pub fn ValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Gpio_Provider`, `Foundation`*"]
    pub fn RemoveValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Gpio_Provider`, `Foundation`*"]
    pub fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Gpio_Provider`, `Foundation`*"]
    pub fn SetDebounceTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn PinNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn SharingMode(&self) -> ::windows::core::Result<ProviderGpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderGpioSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderGpioSharingMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn IsDriveModeSupported(&self, drivemode: ProviderGpioPinDriveMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), drivemode, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn GetDriveMode(&self) -> ::windows::core::Result<ProviderGpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderGpioPinDriveMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderGpioPinDriveMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn Write(&self, value: ProviderGpioPinValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Gpio_Provider`*"]
    pub fn Read(&self) -> ::windows::core::Result<ProviderGpioPinValue> {
        let this = self;
        unsafe {
            let mut result__: ProviderGpioPinValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderGpioPinValue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGpioPinProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{42344cb7-6abc-40ff-9ce7-73b85301b900}");
}
impl ::core::convert::From<IGpioPinProvider> for ::windows::core::IUnknown {
    fn from(value: IGpioPinProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGpioPinProvider> for ::windows::core::IUnknown {
    fn from(value: &IGpioPinProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGpioPinProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGpioPinProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGpioPinProvider> for ::windows::core::IInspectable {
    fn from(value: IGpioPinProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGpioPinProvider> for ::windows::core::IInspectable {
    fn from(value: &IGpioPinProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGpioPinProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGpioPinProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ProviderGpioSharingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ProviderGpioPinDriveMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ProviderGpioPinDriveMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ProviderGpioPinValue) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ProviderGpioPinValue) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioPinProviderValueChangedEventArgs {
    type Vtable = IGpioPinProviderValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32a6d6f2_3d5b_44cd_8fbe_13a69f2edb24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ProviderGpioPinEdge) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioPinProviderValueChangedEventArgsFactory {
    type Vtable = IGpioPinProviderValueChangedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ecb0b59_568c_4392_b24a_8a59a902b1f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edge: ProviderGpioPinEdge, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
pub struct IGpioProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGpioProvider {
    type Vtable = IGpioProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44e82707_08ca_434a_afe0_d61580446f7e);
}
impl IGpioProvider {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Gpio_Provider`, `Foundation_Collections`*"]
    pub fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGpioProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{44e82707-08ca-434a-afe0-d61580446f7e}");
}
impl ::core::convert::From<IGpioProvider> for ::windows::core::IUnknown {
    fn from(value: IGpioProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGpioProvider> for ::windows::core::IUnknown {
    fn from(value: &IGpioProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGpioProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGpioProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGpioProvider> for ::windows::core::IInspectable {
    fn from(value: IGpioProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGpioProvider> for ::windows::core::IInspectable {
    fn from(value: &IGpioProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGpioProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGpioProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderGpioPinDriveMode(pub i32);
impl ProviderGpioPinDriveMode {
    pub const Input: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(0i32);
    pub const Output: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(1i32);
    pub const InputPullUp: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(2i32);
    pub const InputPullDown: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(3i32);
    pub const OutputOpenDrain: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(4i32);
    pub const OutputOpenDrainPullUp: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(5i32);
    pub const OutputOpenSource: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(6i32);
    pub const OutputOpenSourcePullDown: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(7i32);
}
impl ::core::convert::From<i32> for ProviderGpioPinDriveMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProviderGpioPinDriveMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderGpioPinDriveMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinDriveMode;i4)");
}
impl ::windows::core::DefaultType for ProviderGpioPinDriveMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderGpioPinEdge(pub i32);
impl ProviderGpioPinEdge {
    pub const FallingEdge: ProviderGpioPinEdge = ProviderGpioPinEdge(0i32);
    pub const RisingEdge: ProviderGpioPinEdge = ProviderGpioPinEdge(1i32);
}
impl ::core::convert::From<i32> for ProviderGpioPinEdge {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProviderGpioPinEdge {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderGpioPinEdge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinEdge;i4)");
}
impl ::windows::core::DefaultType for ProviderGpioPinEdge {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderGpioPinValue(pub i32);
impl ProviderGpioPinValue {
    pub const Low: ProviderGpioPinValue = ProviderGpioPinValue(0i32);
    pub const High: ProviderGpioPinValue = ProviderGpioPinValue(1i32);
}
impl ::core::convert::From<i32> for ProviderGpioPinValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProviderGpioPinValue {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderGpioPinValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinValue;i4)");
}
impl ::windows::core::DefaultType for ProviderGpioPinValue {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Gpio_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProviderGpioSharingMode(pub i32);
impl ProviderGpioSharingMode {
    pub const Exclusive: ProviderGpioSharingMode = ProviderGpioSharingMode(0i32);
    pub const SharedReadOnly: ProviderGpioSharingMode = ProviderGpioSharingMode(1i32);
}
impl ::core::convert::From<i32> for ProviderGpioSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProviderGpioSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProviderGpioSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioSharingMode;i4)");
}
impl ::windows::core::DefaultType for ProviderGpioSharingMode {
    type DefaultType = Self;
}

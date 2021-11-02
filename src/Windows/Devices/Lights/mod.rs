#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILamp(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILamp {
    type Vtable = ILamp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(75324314, 59973, 19243, [177, 162, 20, 223, 240, 11, 222, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILamp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILampArray(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArray {
    type Vtable = ILampArray_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2060359559, 51360, 20117, [161, 224, 213, 134, 118, 83, 134, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArray_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampArrayKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lampindex: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::System::VirtualKey, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, purposes: LampPurposes, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lampindex: i32, desiredcolor: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(all(feature = "System", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))] usize,
    #[cfg(all(feature = "System", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, keys_array_size: u32, keys: *const super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::UI::Color, purposes: LampPurposes) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageid: i32, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageid: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILampArrayStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayStatics {
    type Vtable = ILampArrayStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2075707789, 24513, 17709, [187, 31, 74, 212, 16, 211, 152, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILampAvailabilityChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampAvailabilityChangedEventArgs {
    type Vtable = ILampAvailabilityChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1332624877, 1954, 18845, [146, 96, 103, 227, 4, 83, 43, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampAvailabilityChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILampInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampInfo {
    type Vtable = ILampInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(817582620, 2767, 18906, [140, 16, 21, 11, 156, 246, 39, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampPurposes) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::UI::Color, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILampStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampStatics {
    type Vtable = ILampStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2820817260, 34949, 16414, [184, 33, 142, 139, 56, 168, 232, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `Devices_Lights`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Lamp(::windows::runtime::IInspectable);
impl Lamp {
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn BrightnessLevel(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn SetBrightnessLevel(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn IsColorSettable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn AvailabilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ILampStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Lamp>> {
        Self::ILampStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Lamp>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Lamp>> {
        Self::ILampStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Lamp>>(result__)
        })
    }
    pub fn ILampStatics<R, F: FnOnce(&ILampStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Lamp, ILampStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Lamp {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Lamp;{047d5b9a-ea45-4b2b-b1a2-14dff00bde7b})");
}
unsafe impl ::windows::runtime::Interface for Lamp {
    type Vtable = ILamp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(75324314, 59973, 19243, [177, 162, 20, 223, 240, 11, 222, 123]);
}
impl ::windows::runtime::RuntimeName for Lamp {
    const NAME: &'static str = "Windows.Devices.Lights.Lamp";
}
impl ::std::convert::From<Lamp> for ::windows::runtime::IUnknown {
    fn from(value: Lamp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Lamp> for ::windows::runtime::IUnknown {
    fn from(value: &Lamp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Lamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Lamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Lamp> for ::windows::runtime::IInspectable {
    fn from(value: Lamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Lamp> for ::windows::runtime::IInspectable {
    fn from(value: &Lamp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Lamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Lamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<Lamp> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Lamp) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&Lamp> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Lamp) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for Lamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &Lamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Lamp {}
unsafe impl ::std::marker::Sync for Lamp {}
#[doc = "*Required features: `Devices_Lights`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArray(::windows::runtime::IInspectable);
impl LampArray {
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn HardwareVendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn HardwareProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn HardwareVersion(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn LampArrayKind(&self) -> ::windows::runtime::Result<LampArrayKind> {
        let this = self;
        unsafe {
            let mut result__: LampArrayKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn LampCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn MinUpdateInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation_Numerics`*"]
    pub fn BoundingBox(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn BrightnessLevel(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn SetBrightnessLevel(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn IsConnected(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn SupportsVirtualKeys(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn GetLampInfo(&self, lampindex: i32) -> ::windows::runtime::Result<LampInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), lampindex, &mut result__).from_abi::<LampInfo>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Devices_Lights`, `System`*"]
    pub fn GetIndicesForKey(&self, key: super::super::System::VirtualKey) -> ::windows::runtime::Result<::windows::runtime::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<i32> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), key, ::windows::runtime::Array::<i32>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn GetIndicesForPurposes(&self, purposes: LampPurposes) -> ::windows::runtime::Result<::windows::runtime::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<i32> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), purposes, ::windows::runtime::Array::<i32>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn SetColorForIndex<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, lampindex: i32, desiredcolor: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), lampindex, desiredcolor.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn SetSingleColorForIndices<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0, lampindexes: &[<i32 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn SetColorsForIndices(&self, desiredcolors: &[<super::super::UI::Color as ::windows::runtime::Abi>::DefaultType], lampindexes: &[<i32 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), desiredcolors.len() as u32, ::std::mem::transmute(desiredcolors.as_ptr()), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[cfg(all(feature = "System", feature = "UI"))]
    #[doc = "*Required features: `Devices_Lights`, `System`, `UI`*"]
    pub fn SetColorsForKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0, key: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi(), key).ok() }
    }
    #[cfg(all(feature = "System", feature = "UI"))]
    #[doc = "*Required features: `Devices_Lights`, `System`, `UI`*"]
    pub fn SetColorsForKeys(&self, desiredcolors: &[<super::super::UI::Color as ::windows::runtime::Abi>::DefaultType], keys: &[<super::super::System::VirtualKey as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), desiredcolors.len() as u32, ::std::mem::transmute(desiredcolors.as_ptr()), keys.len() as u32, ::std::mem::transmute(keys.as_ptr())).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn SetColorsForPurposes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0, purposes: LampPurposes) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi(), purposes).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`, `Storage_Streams`*"]
    pub fn SendMessageAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, messageid: i32, message: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), messageid, message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestMessageAsync(&self, messageid: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), messageid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ILampArrayStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LampArray>> {
        Self::ILampArrayStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LampArray>>(result__)
        })
    }
    pub fn ILampArrayStatics<R, F: FnOnce(&ILampArrayStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArray, ILampArrayStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArray {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampArray;{7ace9787-c8a0-4e95-a1e0-d58676538649})");
}
unsafe impl ::windows::runtime::Interface for LampArray {
    type Vtable = ILampArray_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2060359559, 51360, 20117, [161, 224, 213, 134, 118, 83, 134, 73]);
}
impl ::windows::runtime::RuntimeName for LampArray {
    const NAME: &'static str = "Windows.Devices.Lights.LampArray";
}
impl ::std::convert::From<LampArray> for ::windows::runtime::IUnknown {
    fn from(value: LampArray) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArray> for ::windows::runtime::IUnknown {
    fn from(value: &LampArray) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArray> for ::windows::runtime::IInspectable {
    fn from(value: LampArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArray> for ::windows::runtime::IInspectable {
    fn from(value: &LampArray) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArray {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LampArray {}
unsafe impl ::std::marker::Sync for LampArray {}
#[doc = "*Required features: `Devices_Lights`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LampArrayKind(pub i32);
impl LampArrayKind {
    pub const Undefined: LampArrayKind = LampArrayKind(0i32);
    pub const Keyboard: LampArrayKind = LampArrayKind(1i32);
    pub const Mouse: LampArrayKind = LampArrayKind(2i32);
    pub const GameController: LampArrayKind = LampArrayKind(3i32);
    pub const Peripheral: LampArrayKind = LampArrayKind(4i32);
    pub const Scene: LampArrayKind = LampArrayKind(5i32);
    pub const Notification: LampArrayKind = LampArrayKind(6i32);
    pub const Chassis: LampArrayKind = LampArrayKind(7i32);
    pub const Wearable: LampArrayKind = LampArrayKind(8i32);
    pub const Furniture: LampArrayKind = LampArrayKind(9i32);
    pub const Art: LampArrayKind = LampArrayKind(10i32);
}
impl ::std::convert::From<i32> for LampArrayKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LampArrayKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.LampArrayKind;i4)");
}
#[doc = "*Required features: `Devices_Lights`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampAvailabilityChangedEventArgs(::windows::runtime::IInspectable);
impl LampAvailabilityChangedEventArgs {
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn IsAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampAvailabilityChangedEventArgs;{4f6e3ded-07a2-499d-9260-67e304532ba4})");
}
unsafe impl ::windows::runtime::Interface for LampAvailabilityChangedEventArgs {
    type Vtable = ILampAvailabilityChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1332624877, 1954, 18845, [146, 96, 103, 227, 4, 83, 43, 164]);
}
impl ::windows::runtime::RuntimeName for LampAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.LampAvailabilityChangedEventArgs";
}
impl ::std::convert::From<LampAvailabilityChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampAvailabilityChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampAvailabilityChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LampAvailabilityChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampAvailabilityChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LampAvailabilityChangedEventArgs {}
unsafe impl ::std::marker::Sync for LampAvailabilityChangedEventArgs {}
#[doc = "*Required features: `Devices_Lights`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampInfo(::windows::runtime::IInspectable);
impl LampInfo {
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn Index(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn Purposes(&self) -> ::windows::runtime::Result<LampPurposes> {
        let this = self;
        unsafe {
            let mut result__: LampPurposes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampPurposes>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn RedLevelCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn GreenLevelCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn BlueLevelCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights`*"]
    pub fn GainLevelCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`, `UI`*"]
    pub fn FixedColor(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::UI::Color>>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights`, `UI`*"]
    pub fn GetNearestSupportedColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi(), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights`, `Foundation`*"]
    pub fn UpdateLatency(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampInfo;{30bb521c-0acf-49da-8c10-150b9cf62713})");
}
unsafe impl ::windows::runtime::Interface for LampInfo {
    type Vtable = ILampInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(817582620, 2767, 18906, [140, 16, 21, 11, 156, 246, 39, 19]);
}
impl ::windows::runtime::RuntimeName for LampInfo {
    const NAME: &'static str = "Windows.Devices.Lights.LampInfo";
}
impl ::std::convert::From<LampInfo> for ::windows::runtime::IUnknown {
    fn from(value: LampInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampInfo> for ::windows::runtime::IUnknown {
    fn from(value: &LampInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampInfo> for ::windows::runtime::IInspectable {
    fn from(value: LampInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampInfo> for ::windows::runtime::IInspectable {
    fn from(value: &LampInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LampInfo {}
unsafe impl ::std::marker::Sync for LampInfo {}
#[doc = "*Required features: `Devices_Lights`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LampPurposes(pub u32);
impl LampPurposes {
    pub const Undefined: LampPurposes = LampPurposes(0u32);
    pub const Control: LampPurposes = LampPurposes(1u32);
    pub const Accent: LampPurposes = LampPurposes(2u32);
    pub const Branding: LampPurposes = LampPurposes(4u32);
    pub const Status: LampPurposes = LampPurposes(8u32);
    pub const Illumination: LampPurposes = LampPurposes(16u32);
    pub const Presentation: LampPurposes = LampPurposes(32u32);
}
impl ::std::convert::From<u32> for LampPurposes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LampPurposes {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LampPurposes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.LampPurposes;u4)");
}
impl ::std::ops::BitOr for LampPurposes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LampPurposes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LampPurposes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LampPurposes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LampPurposes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}

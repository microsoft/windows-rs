#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILampImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn BrightnessLevel(&self) -> ::windows::core::Result<f32>;
    fn SetBrightnessLevel(&self, value: f32) -> ::windows::core::Result<()>;
    fn IsColorSettable(&self) -> ::windows::core::Result<bool>;
    fn Color(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AvailabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HardwareVendorId(&self) -> ::windows::core::Result<u16>;
    fn HardwareProductId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVersion(&self) -> ::windows::core::Result<u16>;
    fn LampArrayKind(&self) -> ::windows::core::Result<LampArrayKind>;
    fn LampCount(&self) -> ::windows::core::Result<i32>;
    fn MinUpdateInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn BrightnessLevel(&self) -> ::windows::core::Result<f64>;
    fn SetBrightnessLevel(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
    fn SupportsVirtualKeys(&self) -> ::windows::core::Result<bool>;
    fn GetLampInfo(&self, lampindex: i32) -> ::windows::core::Result<LampInfo>;
    fn GetIndicesForKey(&self, key: super::super::System::VirtualKey) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn GetIndicesForPurposes(&self, purposes: LampPurposes) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn SetColor(&self, desiredcolor: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetColorForIndex(&self, lampindex: i32, desiredcolor: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetSingleColorForIndices(&self, desiredcolor: &super::super::UI::Color, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForIndices(&self, desiredcolors: &[<super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForKey(&self, desiredcolor: &super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn SetColorsForKeys(&self, desiredcolors: &[<super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], keys: &[<super::super::System::VirtualKey as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForPurposes(&self, desiredcolor: &super::super::UI::Color, purposes: LampPurposes) -> ::windows::core::Result<()>;
    fn SendMessageAsync(&self, messageid: i32, message: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestMessageAsync(&self, messageid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LampArray>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampAvailabilityChangedEventArgsImpl: Sized {
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampInfoImpl: Sized {
    fn Index(&self) -> ::windows::core::Result<i32>;
    fn Purposes(&self) -> ::windows::core::Result<LampPurposes>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn RedLevelCount(&self) -> ::windows::core::Result<i32>;
    fn GreenLevelCount(&self) -> ::windows::core::Result<i32>;
    fn BlueLevelCount(&self) -> ::windows::core::Result<i32>;
    fn GainLevelCount(&self) -> ::windows::core::Result<i32>;
    fn FixedColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>>;
    fn GetNearestSupportedColor(&self, desiredcolor: &super::super::UI::Color) -> ::windows::core::Result<super::super::UI::Color>;
    fn UpdateLatency(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Lamp>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Lamp>>;
}

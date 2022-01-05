#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitorImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionKind(&self) -> ::windows::core::Result<DisplayMonitorConnectionKind>;
    fn PhysicalConnector(&self) -> ::windows::core::Result<DisplayMonitorPhysicalConnectorKind>;
    fn DisplayAdapterDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayAdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId>;
    fn DisplayAdapterTargetId(&self) -> ::windows::core::Result<u32>;
    fn UsageKind(&self) -> ::windows::core::Result<DisplayMonitorUsageKind>;
    fn NativeResolutionInRawPixels(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn PhysicalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Size>>;
    fn RawDpiX(&self) -> ::windows::core::Result<f32>;
    fn RawDpiY(&self) -> ::windows::core::Result<f32>;
    fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitor2Impl: Sized {
    fn IsDolbyVisionSupportedInHdrMode(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitorStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>>;
    fn FromInterfaceIdAsync(&self, deviceinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>>;
}

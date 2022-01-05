#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()>;
    fn ControlDescription(&self) -> ::windows::core::Result<HidBooleanControlDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlDescriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn ReportId(&self) -> ::windows::core::Result<u16>;
    fn ReportType(&self) -> ::windows::core::Result<HidReportType>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlDescription2Impl: Sized {
    fn IsAbsolute(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Type(&self) -> ::windows::core::Result<HidCollectionType>;
    fn UsagePage(&self) -> ::windows::core::Result<u32>;
    fn UsageId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHidDeviceImpl: Sized + IClosableImpl {
    fn VendorId(&self) -> ::windows::core::Result<u16>;
    fn ProductId(&self) -> ::windows::core::Result<u16>;
    fn Version(&self) -> ::windows::core::Result<u16>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn GetInputReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>>;
    fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>>;
    fn GetFeatureReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>>;
    fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>>;
    fn CreateOutputReport(&self) -> ::windows::core::Result<HidOutputReport>;
    fn CreateOutputReportById(&self, reportid: u16) -> ::windows::core::Result<HidOutputReport>;
    fn CreateFeatureReport(&self) -> ::windows::core::Result<HidFeatureReport>;
    fn CreateFeatureReportById(&self, reportid: u16) -> ::windows::core::Result<HidFeatureReport>;
    fn SendOutputReportAsync(&self, outputreport: &::core::option::Option<HidOutputReport>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendFeatureReportAsync(&self, featurereport: &::core::option::Option<HidFeatureReport>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>;
    fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>;
    fn InputReportReceived(&self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputReportReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorVidPid(&self, usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, accessmode: super::super::Storage::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidFeatureReportImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u16>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidInputReportImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u16>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ActivatedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>;
    fn TransitionedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>;
    fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidInputReportReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<HidInputReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidNumericControlImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn IsGrouped(&self) -> ::windows::core::Result<bool>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn Value(&self) -> ::windows::core::Result<i64>;
    fn SetValue(&self, value: i64) -> ::windows::core::Result<()>;
    fn ScaledValue(&self) -> ::windows::core::Result<i64>;
    fn SetScaledValue(&self, value: i64) -> ::windows::core::Result<()>;
    fn ControlDescription(&self) -> ::windows::core::Result<HidNumericControlDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidNumericControlDescriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn ReportId(&self) -> ::windows::core::Result<u16>;
    fn ReportType(&self) -> ::windows::core::Result<HidReportType>;
    fn ReportSize(&self) -> ::windows::core::Result<u32>;
    fn ReportCount(&self) -> ::windows::core::Result<u32>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn LogicalMinimum(&self) -> ::windows::core::Result<i32>;
    fn LogicalMaximum(&self) -> ::windows::core::Result<i32>;
    fn PhysicalMinimum(&self) -> ::windows::core::Result<i32>;
    fn PhysicalMaximum(&self) -> ::windows::core::Result<i32>;
    fn UnitExponent(&self) -> ::windows::core::Result<u32>;
    fn Unit(&self) -> ::windows::core::Result<u32>;
    fn IsAbsolute(&self) -> ::windows::core::Result<bool>;
    fn HasNull(&self) -> ::windows::core::Result<bool>;
    fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidOutputReportImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u16>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}

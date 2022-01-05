#[cfg(feature = "implement_exclusive")]
pub trait IAddAppointmentOperationImpl: Sized {
    fn AppointmentInformation(&self) -> ::windows::core::Result<super::Appointment>;
    fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self, itemid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReportCanceled(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentsProviderLaunchActionVerbsStaticsImpl: Sized {
    fn AddAppointment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReplaceAppointment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoveAppointment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowTimeFrame(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentsProviderLaunchActionVerbsStatics2Impl: Sized {
    fn ShowAppointmentDetails(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoveAppointmentOperationImpl: Sized {
    fn AppointmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
    fn ReportCanceled(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReplaceAppointmentOperationImpl: Sized {
    fn AppointmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentInformation(&self) -> ::windows::core::Result<super::Appointment>;
    fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self, itemid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReportCanceled(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissUI(&self) -> ::windows::core::Result<()>;
}

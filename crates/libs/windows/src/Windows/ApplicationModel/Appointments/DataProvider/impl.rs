#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarCancelMeetingRequestImpl: Sized {
    fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NotifyInvitees(&self) -> ::windows::core::Result<bool>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarCancelMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppointmentCalendarCancelMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarCreateOrUpdateAppointmentRequestImpl: Sized {
    fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Appointment(&self) -> ::windows::core::Result<super::Appointment>;
    fn NotifyInvitees(&self) -> ::windows::core::Result<bool>;
    fn ChangedProperties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ReportCompletedAsync(&self, createdorupdatedappointment: &::core::option::Option<super::Appointment>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppointmentCalendarCreateOrUpdateAppointmentRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarForwardMeetingRequestImpl: Sized {
    fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn Invitees(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::AppointmentInvitee>>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ForwardHeader(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarForwardMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppointmentCalendarForwardMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarProposeNewTimeForMeetingRequestImpl: Sized {
    fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn NewStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NewDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppointmentCalendarProposeNewTimeForMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarSyncManagerSyncRequestImpl: Sized {
    fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppointmentCalendarSyncManagerSyncRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarUpdateMeetingResponseRequestImpl: Sized {
    fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn Response(&self) -> ::windows::core::Result<super::AppointmentParticipantResponse>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SendUpdate(&self) -> ::windows::core::Result<bool>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarUpdateMeetingResponseRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppointmentCalendarUpdateMeetingResponseRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentDataProviderConnectionImpl: Sized {
    fn SyncRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateOrUpdateAppointmentRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateOrUpdateAppointmentRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CancelMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCancelMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCancelMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ForwardMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarForwardMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveForwardMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProposeNewTimeForMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProposeNewTimeForMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UpdateMeetingResponseRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarUpdateMeetingResponseRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateMeetingResponseRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<AppointmentDataProviderConnection>;
}

#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDetails(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Reminder(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetReminder(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Organizer(&self) -> ::windows::core::Result<AppointmentOrganizer>;
    fn SetOrganizer(&self, value: &::core::option::Option<AppointmentOrganizer>) -> ::windows::core::Result<()>;
    fn Invitees(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AppointmentInvitee>>;
    fn Recurrence(&self) -> ::windows::core::Result<AppointmentRecurrence>;
    fn SetRecurrence(&self, value: &::core::option::Option<AppointmentRecurrence>) -> ::windows::core::Result<()>;
    fn BusyStatus(&self) -> ::windows::core::Result<AppointmentBusyStatus>;
    fn SetBusyStatus(&self, value: AppointmentBusyStatus) -> ::windows::core::Result<()>;
    fn AllDay(&self) -> ::windows::core::Result<bool>;
    fn SetAllDay(&self, value: bool) -> ::windows::core::Result<()>;
    fn Sensitivity(&self) -> ::windows::core::Result<AppointmentSensitivity>;
    fn SetSensitivity(&self, value: AppointmentSensitivity) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointment2Impl: Sized + IAppointmentImpl {
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CalendarId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRoamingId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OriginalStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn IsResponseRequested(&self) -> ::windows::core::Result<bool>;
    fn SetIsResponseRequested(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowNewTimeProposal(&self) -> ::windows::core::Result<bool>;
    fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows::core::Result<()>;
    fn OnlineMeetingLink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOnlineMeetingLink(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReplyTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetReplyTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn UserResponse(&self) -> ::windows::core::Result<AppointmentParticipantResponse>;
    fn SetUserResponse(&self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()>;
    fn HasInvitees(&self) -> ::windows::core::Result<bool>;
    fn IsCanceledMeeting(&self) -> ::windows::core::Result<bool>;
    fn SetIsCanceledMeeting(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOrganizedByUser(&self) -> ::windows::core::Result<bool>;
    fn SetIsOrganizedByUser(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointment3Impl: Sized + IAppointmentImpl + IAppointment2Impl {
    fn ChangeNumber(&self) -> ::windows::core::Result<u64>;
    fn RemoteChangeNumber(&self) -> ::windows::core::Result<u64>;
    fn SetRemoteChangeNumber(&self, value: u64) -> ::windows::core::Result<()>;
    fn DetailsKind(&self) -> ::windows::core::Result<AppointmentDetailsKind>;
    fn SetDetailsKind(&self, value: AppointmentDetailsKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarImpl: Sized {
    fn DisplayColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsHidden(&self) -> ::windows::core::Result<bool>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<AppointmentCalendarOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&self) -> ::windows::core::Result<AppointmentCalendarOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SummaryCardView(&self) -> ::windows::core::Result<AppointmentSummaryCardView>;
    fn SetSummaryCardView(&self, value: AppointmentSummaryCardView) -> ::windows::core::Result<()>;
    fn FindAppointmentsAsync(&self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindAppointmentsAsyncWithOptions(&self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan, options: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindExceptionsFromMasterAsync(&self, masterlocalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>>;
    fn FindAllInstancesAsync(&self, masterlocalid: &::windows::core::HSTRING, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindAllInstancesAsyncWithOptions(&self, masterlocalid: &::windows::core::HSTRING, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan, poptions: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn GetAppointmentAsync(&self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn GetAppointmentInstanceAsync(&self, localid: &::windows::core::HSTRING, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn FindUnexpandedAppointmentsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindUnexpandedAppointmentsAsyncWithOptions(&self, options: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAppointmentAsync(&self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAppointmentInstanceAsync(&self, localid: &::windows::core::HSTRING, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAppointmentAsync(&self, pappointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendar2Impl: Sized + IAppointmentCalendarImpl {
    fn SyncManager(&self) -> ::windows::core::Result<AppointmentCalendarSyncManager>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetDisplayColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetIsHidden(&self, value: bool) -> ::windows::core::Result<()>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanCreateOrUpdateAppointments(&self) -> ::windows::core::Result<bool>;
    fn SetCanCreateOrUpdateAppointments(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanCancelMeetings(&self) -> ::windows::core::Result<bool>;
    fn SetCanCancelMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanForwardMeetings(&self) -> ::windows::core::Result<bool>;
    fn SetCanForwardMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanProposeNewTimeForMeetings(&self) -> ::windows::core::Result<bool>;
    fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanUpdateMeetingResponses(&self) -> ::windows::core::Result<bool>;
    fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanNotifyInvitees(&self) -> ::windows::core::Result<bool>;
    fn SetCanNotifyInvitees(&self, value: bool) -> ::windows::core::Result<()>;
    fn MustNofityInvitees(&self) -> ::windows::core::Result<bool>;
    fn SetMustNofityInvitees(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryCreateOrUpdateAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCancelMeetingAsync(&self, meeting: &::core::option::Option<Appointment>, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryForwardMeetingAsync(&self, meeting: &::core::option::Option<Appointment>, invitees: &::core::option::Option<super::super::Foundation::Collections::IIterable<AppointmentInvitee>>, subject: &::windows::core::HSTRING, forwardheader: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryProposeNewTimeForMeetingAsync(&self, meeting: &::core::option::Option<Appointment>, newstarttime: &super::super::Foundation::DateTime, newduration: &super::super::Foundation::TimeSpan, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryUpdateMeetingResponseAsync(&self, meeting: &::core::option::Option<Appointment>, response: AppointmentParticipantResponse, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendar3Impl: Sized {
    fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarSyncManagerImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AppointmentCalendarSyncStatus>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentCalendarSyncManager2Impl: Sized {
    fn SetStatus(&self, value: AppointmentCalendarSyncStatus) -> ::windows::core::Result<()>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetLastAttemptedSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentConflictResultImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<AppointmentConflictType>;
    fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentExceptionImpl: Sized {
    fn Appointment(&self) -> ::windows::core::Result<Appointment>;
    fn ExceptionProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IsDeleted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentInviteeImpl: Sized + IAppointmentParticipantImpl {
    fn Role(&self) -> ::windows::core::Result<AppointmentParticipantRole>;
    fn SetRole(&self, value: AppointmentParticipantRole) -> ::windows::core::Result<()>;
    fn Response(&self) -> ::windows::core::Result<AppointmentParticipantResponse>;
    fn SetResponse(&self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentManagerForUserImpl: Sized {
    fn ShowAddAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowAddAppointmentWithPlacementAsync(&self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentAsync(&self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAsync(&self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAndDateAsync(&self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowRemoveAppointmentAsync(&self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAsync(&self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowTimeFrameAsync(&self, timetoshow: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsAsync(&self, appointmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsWithDateAsync(&self, appointmentid: &::windows::core::HSTRING, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowEditNewAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentManagerStaticsImpl: Sized {
    fn ShowAddAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowAddAppointmentWithPlacementAsync(&self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentAsync(&self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAsync(&self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAndDateAsync(&self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowRemoveAppointmentAsync(&self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAsync(&self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowTimeFrameAsync(&self, timetoshow: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentManagerStatics2Impl: Sized {
    fn ShowAppointmentDetailsAsync(&self, appointmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsWithDateAsync(&self, appointmentid: &::windows::core::HSTRING, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowEditNewAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentManagerStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppointmentManagerForUser>;
}
pub trait IAppointmentParticipantImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentPropertiesStaticsImpl: Sized {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StartTime(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Duration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reminder(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BusyStatus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sensitivity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OriginalStartTime(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsResponseRequested(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowNewTimeProposal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllDay(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnlineMeetingLink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReplyTime(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Organizer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserResponse(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasInvitees(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsCanceledMeeting(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOrganizedByUser(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recurrence(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Invitees(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentPropertiesStatics2Impl: Sized + IAppointmentPropertiesStaticsImpl {
    fn ChangeNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteChangeNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetailsKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentRecurrenceImpl: Sized {
    fn Unit(&self) -> ::windows::core::Result<AppointmentRecurrenceUnit>;
    fn SetUnit(&self, value: AppointmentRecurrenceUnit) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetOccurrences(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetUntil(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<u32>;
    fn SetInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn DaysOfWeek(&self) -> ::windows::core::Result<AppointmentDaysOfWeek>;
    fn SetDaysOfWeek(&self, value: AppointmentDaysOfWeek) -> ::windows::core::Result<()>;
    fn WeekOfMonth(&self) -> ::windows::core::Result<AppointmentWeekOfMonth>;
    fn SetWeekOfMonth(&self, value: AppointmentWeekOfMonth) -> ::windows::core::Result<()>;
    fn Month(&self) -> ::windows::core::Result<u32>;
    fn SetMonth(&self, value: u32) -> ::windows::core::Result<()>;
    fn Day(&self) -> ::windows::core::Result<u32>;
    fn SetDay(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentRecurrence2Impl: Sized + IAppointmentRecurrenceImpl {
    fn RecurrenceType(&self) -> ::windows::core::Result<RecurrenceType>;
    fn TimeZone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTimeZone(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentRecurrence3Impl: Sized + IAppointmentRecurrenceImpl + IAppointmentRecurrence2Impl {
    fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreImpl: Sized {
    fn ChangeTracker(&self) -> ::windows::core::Result<AppointmentStoreChangeTracker>;
    fn CreateAppointmentCalendarAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>;
    fn GetAppointmentCalendarAsync(&self, calendarid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>;
    fn GetAppointmentAsync(&self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn GetAppointmentInstanceAsync(&self, localid: &::windows::core::HSTRING, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn FindAppointmentCalendarsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>;
    fn FindAppointmentCalendarsAsyncWithOptions(&self, options: FindAppointmentCalendarsOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>;
    fn FindAppointmentsAsync(&self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindAppointmentsAsyncWithOptions(&self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan, options: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindConflictAsync(&self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>;
    fn FindConflictAsyncWithInstanceStart(&self, appointment: &::core::option::Option<Appointment>, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>;
    fn MoveAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>, destinationcalendar: &::core::option::Option<AppointmentCalendar>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentAsync(&self, localid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAndDateAsync(&self, localid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowRemoveAppointmentAsync(&self, localid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, localid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowAppointmentDetailsAsync(&self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsWithDateAsync(&self, localid: &::windows::core::HSTRING, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowEditNewAppointmentAsync(&self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn FindLocalIdsFromRoamingIdAsync(&self, roamingid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStore2Impl: Sized + IAppointmentStoreImpl {
    fn StoreChanged(&self, phandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStoreChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateAppointmentCalendarInAccountAsync(&self, name: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStore3Impl: Sized {
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<AppointmentStoreChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeImpl: Sized {
    fn Appointment(&self) -> ::windows::core::Result<Appointment>;
    fn ChangeType(&self) -> ::windows::core::Result<AppointmentStoreChangeType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChange2Impl: Sized + IAppointmentStoreChangeImpl {
    fn AppointmentCalendar(&self) -> ::windows::core::Result<AppointmentCalendar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>>;
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoaccept: &::core::option::Option<AppointmentStoreChange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeTrackerImpl: Sized {
    fn GetChangeReader(&self) -> ::windows::core::Result<AppointmentStoreChangeReader>;
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeTracker2Impl: Sized {
    fn IsTracking(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<AppointmentStoreChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreNotificationTriggerDetailsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFindAppointmentsOptionsImpl: Sized {
    fn CalendarIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn FetchProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IncludeHidden(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeHidden(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxCount(&self) -> ::windows::core::Result<u32>;
    fn SetMaxCount(&self, value: u32) -> ::windows::core::Result<()>;
}

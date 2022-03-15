#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct Appointment(::windows::core::IUnknown);
impl Appointment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Appointment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubject)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDetails<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDetails)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Reminder(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reminder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReminder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReminder)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Organizer(&self) -> ::windows::core::Result<AppointmentOrganizer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Organizer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentOrganizer>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetOrganizer<'a, Param0: ::windows::core::IntoParam<'a, AppointmentOrganizer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOrganizer)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invitees(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<AppointmentInvitee>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Recurrence(&self) -> ::windows::core::Result<AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Recurrence)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentRecurrence>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetRecurrence<'a, Param0: ::windows::core::IntoParam<'a, AppointmentRecurrence>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecurrence)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn BusyStatus(&self) -> ::windows::core::Result<AppointmentBusyStatus> {
        let this = self;
        unsafe {
            let mut result__: AppointmentBusyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusyStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentBusyStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetBusyStatus(&self, value: AppointmentBusyStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBusyStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AllDay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllDay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetAllDay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllDay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Sensitivity(&self) -> ::windows::core::Result<AppointmentSensitivity> {
        let this = self;
        unsafe {
            let mut result__: AppointmentSensitivity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Sensitivity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentSensitivity>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetSensitivity(&self, value: AppointmentSensitivity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSensitivity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CalendarId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalendarId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoamingId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetRoamingId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRoamingId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OriginalStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginalStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsResponseRequested(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsResponseRequested)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetIsResponseRequested(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsResponseRequested)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AllowNewTimeProposal(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowNewTimeProposal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowNewTimeProposal)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn OnlineMeetingLink(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OnlineMeetingLink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetOnlineMeetingLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOnlineMeetingLink)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReplyTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReplyTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReplyTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReplyTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn UserResponse(&self) -> ::windows::core::Result<AppointmentParticipantResponse> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: AppointmentParticipantResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserResponse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentParticipantResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetUserResponse(&self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUserResponse)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn HasInvitees(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasInvitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsCanceledMeeting(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceledMeeting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetIsCanceledMeeting(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCanceledMeeting)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsOrganizedByUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOrganizedByUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetIsOrganizedByUser(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsOrganizedByUser)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn ChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn RemoteChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteChangeNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetRemoteChangeNumber(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteChangeNumber)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DetailsKind(&self) -> ::windows::core::Result<AppointmentDetailsKind> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__: AppointmentDetailsKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetailsKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentDetailsKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDetailsKind(&self, value: AppointmentDetailsKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDetailsKind)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for Appointment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Appointment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Appointment {}
impl ::core::fmt::Debug for Appointment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Appointment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Appointment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.Appointment;{dd002f2f-2bdd-4076-90a3-22c275312965})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Appointment {
    type Vtable = IAppointment_Vtbl;
    const IID: ::windows::core::GUID = <IAppointment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Appointment {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.Appointment";
}
impl ::core::convert::From<Appointment> for ::windows::core::IUnknown {
    fn from(value: Appointment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Appointment> for ::windows::core::IUnknown {
    fn from(value: &Appointment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Appointment> for ::windows::core::IInspectable {
    fn from(value: Appointment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Appointment> for ::windows::core::IInspectable {
    fn from(value: &Appointment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Appointment {}
unsafe impl ::core::marker::Sync for Appointment {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentBusyStatus(pub i32);
impl AppointmentBusyStatus {
    pub const Busy: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Free: Self = Self(2i32);
    pub const OutOfOffice: Self = Self(3i32);
    pub const WorkingElsewhere: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentBusyStatus {}
impl ::core::clone::Clone for AppointmentBusyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentBusyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentBusyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentBusyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentBusyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentBusyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentBusyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendar(::windows::core::IUnknown);
impl AppointmentCalendar {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsHidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHidden)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<AppointmentCalendarOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: AppointmentCalendarOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OtherAppReadAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetOtherAppReadAccess(&self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppReadAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn OtherAppWriteAccess(&self) -> ::windows::core::Result<AppointmentCalendarOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__: AppointmentCalendarOtherAppWriteAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OtherAppWriteAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarOtherAppWriteAccess>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetOtherAppWriteAccess(&self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppWriteAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SummaryCardView(&self) -> ::windows::core::Result<AppointmentSummaryCardView> {
        let this = self;
        unsafe {
            let mut result__: AppointmentSummaryCardView = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SummaryCardView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentSummaryCardView>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetSummaryCardView(&self, value: AppointmentSummaryCardView) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSummaryCardView)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentsAsync)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, rangestart: Param0, rangelength: Param1, options: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentsAsyncWithOptions)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindExceptionsFromMasterAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, masterlocalid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindExceptionsFromMasterAsync)(::core::mem::transmute_copy(this), masterlocalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllInstancesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, masterlocalid: Param0, rangestart: Param1, rangelength: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllInstancesAsync)(::core::mem::transmute_copy(this), masterlocalid.into_param().abi(), rangestart.into_param().abi(), rangelength.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllInstancesAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param3: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, masterlocalid: Param0, rangestart: Param1, rangelength: Param2, poptions: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllInstancesAsyncWithOptions)(::core::mem::transmute_copy(this), masterlocalid.into_param().abi(), rangestart.into_param().abi(), rangelength.into_param().abi(), poptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppointmentAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppointmentInstanceAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUnexpandedAppointmentsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindUnexpandedAppointmentsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUnexpandedAppointmentsAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, options: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindUnexpandedAppointmentsAsyncWithOptions)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAppointmentAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAppointmentInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAppointmentInstanceAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, pappointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveAppointmentAsync)(::core::mem::transmute_copy(this), pappointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SyncManager(&self) -> ::windows::core::Result<AppointmentCalendarSyncManager> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncManager)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarSyncManager>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetDisplayColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetIsHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsHidden)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserDataAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CanCreateOrUpdateAppointments(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCreateOrUpdateAppointments)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetCanCreateOrUpdateAppointments(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanCreateOrUpdateAppointments)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CanCancelMeetings(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCancelMeetings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetCanCancelMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanCancelMeetings)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CanForwardMeetings(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanForwardMeetings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetCanForwardMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanForwardMeetings)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CanProposeNewTimeForMeetings(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanProposeNewTimeForMeetings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanProposeNewTimeForMeetings)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CanUpdateMeetingResponses(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanUpdateMeetingResponses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanUpdateMeetingResponses)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CanNotifyInvitees(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanNotifyInvitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetCanNotifyInvitees(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanNotifyInvitees)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn MustNofityInvitees(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MustNofityInvitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetMustNofityInvitees(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMustNofityInvitees)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateOrUpdateAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateOrUpdateAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), notifyinvitees, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelMeetingAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, meeting: Param0, subject: Param1, comment: Param2, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCancelMeetingAsync)(::core::mem::transmute_copy(this), meeting.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), notifyinvitees, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryForwardMeetingAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<AppointmentInvitee>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, meeting: Param0, invitees: Param1, subject: Param2, forwardheader: Param3, comment: Param4) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryForwardMeetingAsync)(::core::mem::transmute_copy(this), meeting.into_param().abi(), invitees.into_param().abi(), subject.into_param().abi(), forwardheader.into_param().abi(), comment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryProposeNewTimeForMeetingAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, meeting: Param0, newstarttime: Param1, newduration: Param2, subject: Param3, comment: Param4) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryProposeNewTimeForMeetingAsync)(::core::mem::transmute_copy(this), meeting.into_param().abi(), newstarttime.into_param().abi(), newduration.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateMeetingResponseAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, meeting: Param0, response: AppointmentParticipantResponse, subject: Param2, comment: Param3, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdateMeetingResponseAsync)(::core::mem::transmute_copy(this), meeting.into_param().abi(), response, subject.into_param().abi(), comment.into_param().abi(), sendupdate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterSyncManagerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendar {}
impl ::core::fmt::Debug for AppointmentCalendar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendar;{5273819d-8339-3d4f-a02f-64084452bb5d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendar {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendar";
}
impl ::core::convert::From<AppointmentCalendar> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendar> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendar> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendar> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendar {}
unsafe impl ::core::marker::Sync for AppointmentCalendar {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppReadAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentCalendarOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentCalendarOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppWriteAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentCalendarOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentCalendarOtherAppWriteAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppWriteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarOtherAppWriteAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarSyncManager(::windows::core::IUnknown);
impl AppointmentCalendarSyncManager {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AppointmentCalendarSyncStatus> {
        let this = self;
        unsafe {
            let mut result__: AppointmentCalendarSyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarSyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastSuccessfulSyncTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastAttemptedSyncTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSyncStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetStatus(&self, value: AppointmentCalendarSyncStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLastAttemptedSyncTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentCalendarSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarSyncManager {}
impl ::core::fmt::Debug for AppointmentCalendarSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager;{2b21b3a0-4aff-4392-bc5f-5645ffcffb17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarSyncManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
}
impl ::core::convert::From<AppointmentCalendarSyncManager> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManager> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarSyncManager> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManager> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarSyncManager {}
unsafe impl ::core::marker::Sync for AppointmentCalendarSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentCalendarSyncStatus {}
impl ::core::clone::Clone for AppointmentCalendarSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentCalendarSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentCalendarSyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentCalendarSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarSyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentConflictResult(::windows::core::IUnknown);
impl AppointmentConflictResult {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<AppointmentConflictType> {
        let this = self;
        unsafe {
            let mut result__: AppointmentConflictType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentConflictType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Date)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentConflictResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentConflictResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentConflictResult {}
impl ::core::fmt::Debug for AppointmentConflictResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentConflictResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentConflictResult;{d5cdf0be-2f2f-3b7d-af0a-a7e20f3a46e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentConflictResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentConflictResult {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentConflictResult";
}
impl ::core::convert::From<AppointmentConflictResult> for ::windows::core::IUnknown {
    fn from(value: AppointmentConflictResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentConflictResult> for ::windows::core::IUnknown {
    fn from(value: &AppointmentConflictResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentConflictResult> for ::windows::core::IInspectable {
    fn from(value: AppointmentConflictResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentConflictResult> for ::windows::core::IInspectable {
    fn from(value: &AppointmentConflictResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentConflictResult {}
unsafe impl ::core::marker::Sync for AppointmentConflictResult {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: Self = Self(0i32);
    pub const Adjacent: Self = Self(1i32);
    pub const Overlap: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentConflictType {}
impl ::core::clone::Clone for AppointmentConflictType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentConflictType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentConflictType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentConflictType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentConflictType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentConflictType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl ::core::marker::Copy for AppointmentDaysOfWeek {}
impl ::core::clone::Clone for AppointmentDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentDaysOfWeek {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDaysOfWeek").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AppointmentDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AppointmentDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AppointmentDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentDaysOfWeek {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentDetailsKind {}
impl ::core::clone::Clone for AppointmentDetailsKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentDetailsKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentDetailsKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentDetailsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDetailsKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentDetailsKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDetailsKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentException(::windows::core::IUnknown);
impl AppointmentException {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Appointment(&self) -> ::windows::core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Appointment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Appointment>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExceptionProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExceptionProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsDeleted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDeleted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentException {}
impl ::core::fmt::Debug for AppointmentException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentException").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentException {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentException;{a2076767-16f6-4bce-9f5a-8600b8019fcb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentException {
    type Vtable = IAppointmentException_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentException as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentException {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentException";
}
impl ::core::convert::From<AppointmentException> for ::windows::core::IUnknown {
    fn from(value: AppointmentException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentException> for ::windows::core::IUnknown {
    fn from(value: &AppointmentException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentException> for ::windows::core::IInspectable {
    fn from(value: AppointmentException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentException> for ::windows::core::IInspectable {
    fn from(value: &AppointmentException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentException {}
unsafe impl ::core::marker::Sync for AppointmentException {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentInvitee(::windows::core::IUnknown);
impl AppointmentInvitee {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentInvitee, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Role(&self) -> ::windows::core::Result<AppointmentParticipantRole> {
        let this = self;
        unsafe {
            let mut result__: AppointmentParticipantRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Role)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentParticipantRole>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetRole(&self, value: AppointmentParticipantRole) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRole)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Response(&self) -> ::windows::core::Result<AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__: AppointmentParticipantResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Response)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentParticipantResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetResponse(&self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResponse)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentInvitee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentInvitee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentInvitee {}
impl ::core::fmt::Debug for AppointmentInvitee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentInvitee").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentInvitee {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentInvitee;{13bf0796-9842-495b-b0e7-ef8f79c0701d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentInvitee as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentInvitee {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentInvitee";
}
impl ::core::convert::From<AppointmentInvitee> for ::windows::core::IUnknown {
    fn from(value: AppointmentInvitee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentInvitee> for ::windows::core::IUnknown {
    fn from(value: &AppointmentInvitee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentInvitee> for ::windows::core::IInspectable {
    fn from(value: AppointmentInvitee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentInvitee> for ::windows::core::IInspectable {
    fn from(value: &AppointmentInvitee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentInvitee> for IAppointmentParticipant {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentInvitee) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentInvitee> for IAppointmentParticipant {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentInvitee) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentParticipant> for AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentParticipant> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentParticipant> for &AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentParticipant> {
        ::core::convert::TryInto::<IAppointmentParticipant>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentInvitee {}
unsafe impl ::core::marker::Sync for AppointmentInvitee {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
pub struct AppointmentManager {}
impl AppointmentManager {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointment: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAddAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointment: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAppointmentWithPlacementAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, appointment: Param1, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param4) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowTimeFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(timetoshow: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowTimeFrameAsync)(::core::mem::transmute_copy(this), timetoshow.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appointmentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetailsAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(appointmentid: Param0, instancestartdate: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowEditNewAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentStore>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<AppointmentManagerForUser> {
        Self::IAppointmentManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppointmentManagerForUser>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics<R, F: FnOnce(&IAppointmentManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentManager, IAppointmentManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics2<R, F: FnOnce(&IAppointmentManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentManager, IAppointmentManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics3<R, F: FnOnce(&IAppointmentManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentManager, IAppointmentManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppointmentManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManager";
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentManagerForUser(::windows::core::IUnknown);
impl AppointmentManagerForUser {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointment: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAddAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointment: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAppointmentWithPlacementAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param4) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowTimeFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timetoshow: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowTimeFrameAsync)(::core::mem::transmute_copy(this), timetoshow.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appointmentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetailsAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointmentid: Param0, instancestartdate: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowEditNewAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentStore>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentManagerForUser {}
impl ::core::fmt::Debug for AppointmentManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentManagerForUser;{70261423-73cc-4660-b318-b01365302a03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
}
impl ::core::convert::From<AppointmentManagerForUser> for ::windows::core::IUnknown {
    fn from(value: AppointmentManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &AppointmentManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentManagerForUser> for ::windows::core::IInspectable {
    fn from(value: AppointmentManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &AppointmentManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentManagerForUser {}
unsafe impl ::core::marker::Sync for AppointmentManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentOrganizer(::windows::core::IUnknown);
impl AppointmentOrganizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentOrganizer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentOrganizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentOrganizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentOrganizer {}
impl ::core::fmt::Debug for AppointmentOrganizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentOrganizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentOrganizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentOrganizer;{615e2902-9718-467b-83fb-b293a19121de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentOrganizer {
    type Vtable = IAppointmentParticipant_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentParticipant as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentOrganizer {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentOrganizer";
}
impl ::core::convert::From<AppointmentOrganizer> for ::windows::core::IUnknown {
    fn from(value: AppointmentOrganizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for ::windows::core::IUnknown {
    fn from(value: &AppointmentOrganizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentOrganizer> for ::windows::core::IInspectable {
    fn from(value: AppointmentOrganizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for ::windows::core::IInspectable {
    fn from(value: &AppointmentOrganizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentOrganizer> for IAppointmentParticipant {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentOrganizer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentOrganizer> for IAppointmentParticipant {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentOrganizer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentParticipant> for AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentParticipant> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentParticipant> for &AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentParticipant> {
        ::core::convert::TryInto::<IAppointmentParticipant>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentOrganizer {}
unsafe impl ::core::marker::Sync for AppointmentOrganizer {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentParticipantResponse(pub i32);
impl AppointmentParticipantResponse {
    pub const None: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Accepted: Self = Self(2i32);
    pub const Declined: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentParticipantResponse {}
impl ::core::clone::Clone for AppointmentParticipantResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentParticipantResponse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentParticipantResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentParticipantResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentParticipantResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: Self = Self(0i32);
    pub const OptionalAttendee: Self = Self(1i32);
    pub const Resource: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentParticipantRole {}
impl ::core::clone::Clone for AppointmentParticipantRole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentParticipantRole {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentParticipantRole {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentParticipantRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantRole").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentParticipantRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantRole;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
pub struct AppointmentProperties {}
impl AppointmentProperties {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Subject() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Location() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn StartTime() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Duration() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Reminder() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reminder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn BusyStatus() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusyStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Sensitivity() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Sensitivity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn OriginalStartTime() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginalStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsResponseRequested() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsResponseRequested)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AllowNewTimeProposal() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowNewTimeProposal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AllDay() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllDay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Details() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn OnlineMeetingLink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OnlineMeetingLink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn ReplyTime() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReplyTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Organizer() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Organizer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn UserResponse() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserResponse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn HasInvitees() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasInvitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsCanceledMeeting() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceledMeeting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsOrganizedByUser() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOrganizedByUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Recurrence() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Recurrence)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Uri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Invitees() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DefaultProperties() -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn ChangeNumber() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn RemoteChangeNumber() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteChangeNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DetailsKind() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetailsKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentPropertiesStatics<R, F: FnOnce(&IAppointmentPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppointmentPropertiesStatics2<R, F: FnOnce(&IAppointmentPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppointmentProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentProperties";
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentRecurrence(::windows::core::IUnknown);
impl AppointmentRecurrence {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentRecurrence, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Unit(&self) -> ::windows::core::Result<AppointmentRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__: AppointmentRecurrenceUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentRecurrenceUnit>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetUnit(&self, value: AppointmentRecurrenceUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occurrences)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOccurrences<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOccurrences)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Until)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUntil<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUntil)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Interval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Interval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInterval)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DaysOfWeek(&self) -> ::windows::core::Result<AppointmentDaysOfWeek> {
        let this = self;
        unsafe {
            let mut result__: AppointmentDaysOfWeek = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DaysOfWeek)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentDaysOfWeek>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDaysOfWeek(&self, value: AppointmentDaysOfWeek) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDaysOfWeek)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn WeekOfMonth(&self) -> ::windows::core::Result<AppointmentWeekOfMonth> {
        let this = self;
        unsafe {
            let mut result__: AppointmentWeekOfMonth = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WeekOfMonth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentWeekOfMonth>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetWeekOfMonth(&self, value: AppointmentWeekOfMonth) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWeekOfMonth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Month(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Month)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetMonth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMonth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Day(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Day)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDay(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn RecurrenceType(&self) -> ::windows::core::Result<RecurrenceType> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__: RecurrenceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecurrenceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RecurrenceType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn TimeZone(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimeZone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetTimeZone<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTimeZone)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalendarIdentifier)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentRecurrence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentRecurrence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentRecurrence {}
impl ::core::fmt::Debug for AppointmentRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentRecurrence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentRecurrence;{d87b3e83-15a6-487b-b959-0c361e60e954})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentRecurrence as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentRecurrence {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentRecurrence";
}
impl ::core::convert::From<AppointmentRecurrence> for ::windows::core::IUnknown {
    fn from(value: AppointmentRecurrence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentRecurrence> for ::windows::core::IUnknown {
    fn from(value: &AppointmentRecurrence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentRecurrence> for ::windows::core::IInspectable {
    fn from(value: AppointmentRecurrence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentRecurrence> for ::windows::core::IInspectable {
    fn from(value: &AppointmentRecurrence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentRecurrence {}
unsafe impl ::core::marker::Sync for AppointmentRecurrence {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl ::core::marker::Copy for AppointmentRecurrenceUnit {}
impl ::core::clone::Clone for AppointmentRecurrenceUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentRecurrenceUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentRecurrenceUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentRecurrenceUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrenceUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentRecurrenceUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSensitivity {}
impl ::core::clone::Clone for AppointmentSensitivity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentSensitivity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentSensitivity {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentSensitivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSensitivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentSensitivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSensitivity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStore(::windows::core::IUnknown);
impl AppointmentStore {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn ChangeTracker(&self) -> ::windows::core::Result<AppointmentStoreChangeTracker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeTracker)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangeTracker>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAppointmentCalendarAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAppointmentCalendarAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentCalendarAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, calendarid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppointmentCalendarAsync)(::core::mem::transmute_copy(this), calendarid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppointmentAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppointmentInstanceAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentCalendarsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentCalendarsAsyncWithOptions(&self, options: FindAppointmentCalendarsOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentCalendarsAsyncWithOptions)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentsAsync)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, rangestart: Param0, rangelength: Param1, options: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentsAsyncWithOptions)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindConflictAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindConflictAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindConflictAsyncWithInstanceStart<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointment: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindConflictAsyncWithInstanceStart)(::core::mem::transmute_copy(this), appointment.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, AppointmentCalendar>>(&self, appointment: Param0, destinationcalendar: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), destinationcalendar.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointment: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, localid: Param0, appointment: Param1, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param4) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, localid: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetailsAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestartdate: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowEditNewAppointmentAsync)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindLocalIdsFromRoamingIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, roamingid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindLocalIdsFromRoamingIdAsync)(::core::mem::transmute_copy(this), roamingid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>>(&self, phandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreChanged)(::core::mem::transmute_copy(this), phandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStoreChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAppointmentCalendarInAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAppointmentCalendarInAccountAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn GetChangeTracker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, identity: Param0) -> ::windows::core::Result<AppointmentStoreChangeTracker> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetChangeTracker)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<AppointmentStoreChangeTracker>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStore {}
impl ::core::fmt::Debug for AppointmentStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStore;{a461918c-7a47-4d96-96c9-15cd8a05a735})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStore {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStore";
}
impl ::core::convert::From<AppointmentStore> for ::windows::core::IUnknown {
    fn from(value: AppointmentStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStore> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStore> for ::windows::core::IInspectable {
    fn from(value: AppointmentStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStore> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStore {}
unsafe impl ::core::marker::Sync for AppointmentStore {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: Self = Self(0i32);
    pub const AllCalendarsReadOnly: Self = Self(1i32);
    pub const AllCalendarsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentStoreAccessType {}
impl ::core::clone::Clone for AppointmentStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreChange(::windows::core::IUnknown);
impl AppointmentStoreChange {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Appointment(&self) -> ::windows::core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Appointment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Appointment>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn ChangeType(&self) -> ::windows::core::Result<AppointmentStoreChangeType> {
        let this = self;
        unsafe {
            let mut result__: AppointmentStoreChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangeType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AppointmentCalendar(&self) -> ::windows::core::Result<AppointmentCalendar> {
        let this = &::windows::core::Interface::cast::<IAppointmentStoreChange2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendar)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendar>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStoreChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChange {}
impl ::core::fmt::Debug for AppointmentStoreChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChange;{a5a6e035-0a33-3654-8463-b543e90c3b79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStoreChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreChange {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChange";
}
impl ::core::convert::From<AppointmentStoreChange> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChange> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChange> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChange> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChange {}
unsafe impl ::core::marker::Sync for AppointmentStoreChange {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreChangeReader(::windows::core::IUnknown);
impl AppointmentStoreChangeReader {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AcceptChanges(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AcceptChanges)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn AcceptChangesThrough<'a, Param0: ::windows::core::IntoParam<'a, AppointmentStoreChange>>(&self, lastchangetoaccept: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AcceptChangesThrough)(::core::mem::transmute_copy(this), lastchangetoaccept.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangeReader {}
impl ::core::fmt::Debug for AppointmentStoreChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader;{8b2409f1-65f3-42a0-961d-4c209bf30370})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStoreChangeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
}
impl ::core::convert::From<AppointmentStoreChangeReader> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeReader> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangeReader> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeReader> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangeReader {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeReader {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreChangeTracker(::windows::core::IUnknown);
impl AppointmentStoreChangeTracker {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn GetChangeReader(&self) -> ::windows::core::Result<AppointmentStoreChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetChangeReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangeReader>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Enable)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IsTracking(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentStoreChangeTracker2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTracking)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangeTracker {}
impl ::core::fmt::Debug for AppointmentStoreChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker;{1b25f4b1-8ece-4f17-93c8-e6412458fd5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStoreChangeTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
}
impl ::core::convert::From<AppointmentStoreChangeTracker> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeTracker> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangeTracker> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeTracker> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangeTracker {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeTracker {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentStoreChangeType(pub i32);
impl AppointmentStoreChangeType {
    pub const AppointmentCreated: Self = Self(0i32);
    pub const AppointmentModified: Self = Self(1i32);
    pub const AppointmentDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
    pub const CalendarCreated: Self = Self(4i32);
    pub const CalendarModified: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentStoreChangeType {}
impl ::core::clone::Clone for AppointmentStoreChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentStoreChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentStoreChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentStoreChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreChangedDeferral(::windows::core::IUnknown);
impl AppointmentStoreChangedDeferral {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangedDeferral {}
impl ::core::fmt::Debug for AppointmentStoreChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral;{4cb82026-fedb-4bc3-9662-95a9befdf4df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStoreChangedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
}
impl ::core::convert::From<AppointmentStoreChangedDeferral> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedDeferral> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangedDeferral> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedDeferral> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangedDeferral {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreChangedEventArgs(::windows::core::IUnknown);
impl AppointmentStoreChangedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<AppointmentStoreChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangedEventArgs {}
impl ::core::fmt::Debug for AppointmentStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs;{2285f8b9-0791-417e-bfea-cc6d41636c8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStoreChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
}
impl ::core::convert::From<AppointmentStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreNotificationTriggerDetails(::windows::core::IUnknown);
impl AppointmentStoreNotificationTriggerDetails {}
impl ::core::clone::Clone for AppointmentStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreNotificationTriggerDetails {}
impl ::core::fmt::Debug for AppointmentStoreNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails;{9b33cb11-c301-421e-afef-047ecfa76adb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentStoreNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
}
impl ::core::convert::From<AppointmentStoreNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentSummaryCardView(pub i32);
impl AppointmentSummaryCardView {
    pub const System: Self = Self(0i32);
    pub const App: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSummaryCardView {}
impl ::core::clone::Clone for AppointmentSummaryCardView {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentSummaryCardView {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentSummaryCardView {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentSummaryCardView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSummaryCardView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentSummaryCardView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSummaryCardView;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentWeekOfMonth {}
impl ::core::clone::Clone for AppointmentWeekOfMonth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentWeekOfMonth {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppointmentWeekOfMonth {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentWeekOfMonth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentWeekOfMonth").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentWeekOfMonth {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeHidden: Self = Self(1u32);
}
impl ::core::marker::Copy for FindAppointmentCalendarsOptions {}
impl ::core::clone::Clone for FindAppointmentCalendarsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FindAppointmentCalendarsOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FindAppointmentCalendarsOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FindAppointmentCalendarsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentCalendarsOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FindAppointmentCalendarsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FindAppointmentCalendarsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for FindAppointmentCalendarsOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct FindAppointmentsOptions(::windows::core::IUnknown);
impl FindAppointmentsOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FindAppointmentsOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CalendarIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalendarIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FetchProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FetchProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn IncludeHidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncludeHidden)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetIncludeHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeHidden)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn MaxCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetMaxCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxCount)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for FindAppointmentsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindAppointmentsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindAppointmentsOptions {}
impl ::core::fmt::Debug for FindAppointmentsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentsOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FindAppointmentsOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.FindAppointmentsOptions;{55f7dc55-9942-3086-82b5-2cb29f64d5f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
    const IID: ::windows::core::GUID = <IFindAppointmentsOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FindAppointmentsOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
}
impl ::core::convert::From<FindAppointmentsOptions> for ::windows::core::IUnknown {
    fn from(value: FindAppointmentsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindAppointmentsOptions> for ::windows::core::IUnknown {
    fn from(value: &FindAppointmentsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FindAppointmentsOptions> for ::windows::core::IInspectable {
    fn from(value: FindAppointmentsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindAppointmentsOptions> for ::windows::core::IInspectable {
    fn from(value: &FindAppointmentsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FindAppointmentsOptions {}
unsafe impl ::core::marker::Sync for FindAppointmentsOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointment {
    type Vtable = IAppointment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd002f2f_2bdd_4076_90a3_22c275312965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reminder: usize,
    #[cfg(feature = "Foundation")]
    pub SetReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReminder: usize,
    pub Organizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOrganizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invitees: usize,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRecurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentBusyStatus) -> ::windows::core::HRESULT,
    pub SetBusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentBusyStatus) -> ::windows::core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSensitivity) -> ::windows::core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentSensitivity) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointment2 {
    type Vtable = IAppointment2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e85983c_540f_3452_9b5c_0dd7ad4c65a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CalendarId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalStartTime: usize,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetOnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReplyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReplyTime: usize,
    pub UserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub SetUserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointment3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointment3 {
    type Vtable = IAppointment3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcc45a9_8961_4991_934b_c48768e5a96c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDetailsKind) -> ::windows::core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentDetailsKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5273819d_8339_3d4f_a02f_64084452bb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SummaryCardView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSummaryCardView) -> ::windows::core::HRESULT,
    pub SetSummaryCardView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentSummaryCardView) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindExceptionsFromMasterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindExceptionsFromMasterAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAppointmentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendar2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendar2 {
    type Vtable = IAppointmentCalendar2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18e7e422_2467_4e1c_a459_d8a29303d092);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDisplayColor: usize,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanCancelMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanCancelMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanNotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanNotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MustNofityInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMustNofityInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, notifyinvitees: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, notifyinvitees: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelMeetingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, invitees: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, forwardheader: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryProposeNewTimeForMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, response: AppointmentParticipantResponse, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sendupdate: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateMeetingResponseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendar3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendar3 {
    type Vtable = IAppointmentCalendar3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb23d22b_a685_42ae_8495_b3119adb4167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b21b3a0_4aff_4392_bc5f_5645ffcffb17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarSyncManager2 {
    type Vtable = IAppointmentCalendarSyncManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x647528ad_0d29_4c7c_aaa7_bf996805537c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentConflictResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5cdf0be_2f2f_3b7d_af0a_a7e20f3a46e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentConflictResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentConflictType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentException(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentException {
    type Vtable = IAppointmentException_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2076767_16f6_4bce_9f5a_8600b8019fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentException_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExceptionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExceptionProperties: usize,
    pub IsDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentInvitee(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13bf0796_9842_495b_b0e7_ef8f79c0701d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentInvitee_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantRole) -> ::windows::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantRole) -> ::windows::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70261423_73cc_4660_b318_b01365302a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerForUser_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAddAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowTimeFrameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentManagerStatics {
    type Vtable = IAppointmentManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a30fa01_5c40_499d_b33f_a43050f74fc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAddAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowTimeFrameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentManagerStatics2 {
    type Vtable = IAppointmentManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a81f60d_d04f_4034_af72_a36573b45ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentManagerStatics3 {
    type Vtable = IAppointmentManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9ae09c_b34c_4dc7_a35d_cafd88ae3ec6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct IAppointmentParticipant(::windows::core::IUnknown);
impl IAppointmentParticipant {
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IAppointmentParticipant> for ::windows::core::IUnknown {
    fn from(value: IAppointmentParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentParticipant> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentParticipant> for ::windows::core::IInspectable {
    fn from(value: IAppointmentParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentParticipant> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAppointmentParticipant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentParticipant {}
impl ::core::fmt::Debug for IAppointmentParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentParticipant").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentParticipant {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{615e2902-9718-467b-83fb-b293a19121de}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentParticipant {
    type Vtable = IAppointmentParticipant_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x615e2902_9718_467b_83fb_b293a19121de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentParticipant_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentPropertiesStatics {
    type Vtable = IAppointmentPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25141fe9_68ae_3aae_855f_bc4441caa234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Organizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DefaultProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DefaultProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentPropertiesStatics2 {
    type Vtable = IAppointmentPropertiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdffc434b_b017_45dd_8af5_d163d10801bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentRecurrence(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd87b3e83_15a6_487b_b959_0c361e60e954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentRecurrenceUnit) -> ::windows::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentRecurrenceUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDaysOfWeek) -> ::windows::core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentDaysOfWeek) -> ::windows::core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentWeekOfMonth) -> ::windows::core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentWeekOfMonth) -> ::windows::core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentRecurrence2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentRecurrence2 {
    type Vtable = IAppointmentRecurrence2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df3a2e0_05a7_4f50_9f86_b03f9436254d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RecurrenceType) -> ::windows::core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentRecurrence3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentRecurrence3 {
    type Vtable = IAppointmentRecurrence3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89ff96d9_da4d_4a17_8dd2_1cebc2b5ff9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa461918c_7a47_4d96_96c9_15cd8a05a735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAppointmentCalendarAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calendarid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentCalendarAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FindAppointmentCalendarsOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub FindConflictAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindConflictAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FindConflictAsyncWithInstanceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindConflictAsyncWithInstanceStart: usize,
    #[cfg(feature = "Foundation")]
    pub MoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, destinationcalendar: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindLocalIdsFromRoamingIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roamingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindLocalIdsFromRoamingIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStore2 {
    type Vtable = IAppointmentStore2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25c48c20_1c41_424f_8084_67c1cfe0a854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAppointmentCalendarInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAppointmentCalendarInAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStore3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStore3 {
    type Vtable = IAppointmentStore3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4251940b_b078_470a_9a40_c2e01761f72f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5a6e035_0a33_3654_8463_b543e90c3b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentStoreChangeType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChange2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChange2 {
    type Vtable = IAppointmentStoreChange2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb37d0dce_5211_4402_a608_a96fe70b8ee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b2409f1_65f3_42a0_961d_4c209bf30370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoaccept: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangeTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25f4b1_8ece_4f17_93c8_e6412458fd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangeTracker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangeTracker2 {
    type Vtable = IAppointmentStoreChangeTracker2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb66aaf45_9542_4cf7_8550_eb370e0c08d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cb82026_fedb_4bc3_9662_95a9befdf4df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2285f8b9_0791_417e_bfea_cc6d41636c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b33cb11_c301_421e_afef_047ecfa76adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindAppointmentsOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55f7dc55_9942_3086_82b5_2cb29f64d5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAppointmentsOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CalendarIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CalendarIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FetchProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FetchProperties: usize,
    pub IncludeHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RecurrenceType(pub i32);
impl RecurrenceType {
    pub const Master: Self = Self(0i32);
    pub const Instance: Self = Self(1i32);
    pub const ExceptionInstance: Self = Self(2i32);
}
impl ::core::marker::Copy for RecurrenceType {}
impl ::core::clone::Clone for RecurrenceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RecurrenceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RecurrenceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RecurrenceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecurrenceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RecurrenceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.RecurrenceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

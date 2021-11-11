#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Appointment(pub ::windows::core::IInspectable);
impl Appointment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Appointment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDetails<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn Reminder(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetReminder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Organizer(&self) -> ::windows::core::Result<AppointmentOrganizer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentOrganizer>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetOrganizer<'a, Param0: ::windows::core::IntoParam<'a, AppointmentOrganizer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation_Collections`*"]
    pub fn Invitees(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<AppointmentInvitee>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Recurrence(&self) -> ::windows::core::Result<AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentRecurrence>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetRecurrence<'a, Param0: ::windows::core::IntoParam<'a, AppointmentRecurrence>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn BusyStatus(&self) -> ::windows::core::Result<AppointmentBusyStatus> {
        let this = self;
        unsafe {
            let mut result__: AppointmentBusyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentBusyStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetBusyStatus(&self, value: AppointmentBusyStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AllDay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetAllDay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Sensitivity(&self) -> ::windows::core::Result<AppointmentSensitivity> {
        let this = self;
        unsafe {
            let mut result__: AppointmentSensitivity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentSensitivity>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetSensitivity(&self, value: AppointmentSensitivity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CalendarId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetRoamingId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn OriginalStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsResponseRequested(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetIsResponseRequested(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AllowNewTimeProposal(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn OnlineMeetingLink(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetOnlineMeetingLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ReplyTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetReplyTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn UserResponse(&self) -> ::windows::core::Result<AppointmentParticipantResponse> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: AppointmentParticipantResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentParticipantResponse>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetUserResponse(&self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn HasInvitees(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsCanceledMeeting(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetIsCanceledMeeting(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsOrganizedByUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetIsOrganizedByUser(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn ChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn RemoteChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetRemoteChangeNumber(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DetailsKind(&self) -> ::windows::core::Result<AppointmentDetailsKind> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__: AppointmentDetailsKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentDetailsKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDetailsKind(&self, value: AppointmentDetailsKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for Appointment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.Appointment;{dd002f2f-2bdd-4076-90a3-22c275312965})");
}
unsafe impl ::windows::core::Interface for Appointment {
    type Vtable = IAppointment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd002f2f_2bdd_4076_90a3_22c275312965);
}
impl ::windows::core::RuntimeName for Appointment {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.Appointment";
}
impl ::core::convert::From<Appointment> for ::windows::core::IUnknown {
    fn from(value: Appointment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Appointment> for ::windows::core::IUnknown {
    fn from(value: &Appointment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Appointment> for ::windows::core::IInspectable {
    fn from(value: Appointment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Appointment> for ::windows::core::IInspectable {
    fn from(value: &Appointment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Appointment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Appointment {}
unsafe impl ::core::marker::Sync for Appointment {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentBusyStatus(pub i32);
impl AppointmentBusyStatus {
    pub const Busy: AppointmentBusyStatus = AppointmentBusyStatus(0i32);
    pub const Tentative: AppointmentBusyStatus = AppointmentBusyStatus(1i32);
    pub const Free: AppointmentBusyStatus = AppointmentBusyStatus(2i32);
    pub const OutOfOffice: AppointmentBusyStatus = AppointmentBusyStatus(3i32);
    pub const WorkingElsewhere: AppointmentBusyStatus = AppointmentBusyStatus(4i32);
}
impl ::core::convert::From<i32> for AppointmentBusyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentBusyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentBusyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentBusyStatus;i4)");
}
impl ::windows::core::DefaultType for AppointmentBusyStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentCalendar(pub ::windows::core::IInspectable);
impl AppointmentCalendar {
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `UI`*"]
    pub fn DisplayColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsHidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<AppointmentCalendarOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: AppointmentCalendarOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetOtherAppReadAccess(&self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn OtherAppWriteAccess(&self) -> ::windows::core::Result<AppointmentCalendarOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__: AppointmentCalendarOtherAppWriteAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarOtherAppWriteAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetOtherAppWriteAccess(&self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SummaryCardView(&self) -> ::windows::core::Result<AppointmentSummaryCardView> {
        let this = self;
        unsafe {
            let mut result__: AppointmentSummaryCardView = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentSummaryCardView>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetSummaryCardView(&self, value: AppointmentSummaryCardView) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentsAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, rangestart: Param0, rangelength: Param1, options: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindExceptionsFromMasterAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, masterlocalid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), masterlocalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllInstancesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, masterlocalid: Param0, rangestart: Param1, rangelength: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), masterlocalid.into_param().abi(), rangestart.into_param().abi(), rangelength.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllInstancesAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param3: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(
        &self,
        masterlocalid: Param0,
        rangestart: Param1,
        rangelength: Param2,
        poptions: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), masterlocalid.into_param().abi(), rangestart.into_param().abi(), rangelength.into_param().abi(), poptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn GetAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn GetAppointmentInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindUnexpandedAppointmentsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindUnexpandedAppointmentsAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, options: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn DeleteAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn DeleteAppointmentInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SaveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, pappointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), pappointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SyncManager(&self) -> ::windows::core::Result<AppointmentCalendarSyncManager> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarSyncManager>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `UI`*"]
    pub fn SetDisplayColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetIsHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CanCreateOrUpdateAppointments(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetCanCreateOrUpdateAppointments(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CanCancelMeetings(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetCanCancelMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CanForwardMeetings(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetCanForwardMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CanProposeNewTimeForMeetings(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CanUpdateMeetingResponses(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CanNotifyInvitees(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetCanNotifyInvitees(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn MustNofityInvitees(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetMustNofityInvitees(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn TryCreateOrUpdateAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), appointment.into_param().abi(), notifyinvitees, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn TryCancelMeetingAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, meeting: Param0, subject: Param1, comment: Param2, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), meeting.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), notifyinvitees, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn TryForwardMeetingAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<AppointmentInvitee>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        meeting: Param0,
        invitees: Param1,
        subject: Param2,
        forwardheader: Param3,
        comment: Param4,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), meeting.into_param().abi(), invitees.into_param().abi(), subject.into_param().abi(), forwardheader.into_param().abi(), comment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn TryProposeNewTimeForMeetingAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        meeting: Param0,
        newstarttime: Param1,
        newduration: Param2,
        subject: Param3,
        comment: Param4,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), meeting.into_param().abi(), newstarttime.into_param().abi(), newduration.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn TryUpdateMeetingResponseAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, meeting: Param0, response: AppointmentParticipantResponse, subject: Param2, comment: Param3, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), meeting.into_param().abi(), response, subject.into_param().abi(), comment.into_param().abi(), sendupdate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendar3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendar;{5273819d-8339-3d4f-a02f-64084452bb5d})");
}
unsafe impl ::windows::core::Interface for AppointmentCalendar {
    type Vtable = IAppointmentCalendar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5273819d_8339_3d4f_a02f_64084452bb5d);
}
impl ::windows::core::RuntimeName for AppointmentCalendar {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendar";
}
impl ::core::convert::From<AppointmentCalendar> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentCalendar> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentCalendar> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentCalendar> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendar {}
unsafe impl ::core::marker::Sync for AppointmentCalendar {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(0i32);
    pub const Limited: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(1i32);
    pub const Full: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(2i32);
    pub const None: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(3i32);
}
impl ::core::convert::From<i32> for AppointmentCalendarOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentCalendarOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess;i4)");
}
impl ::windows::core::DefaultType for AppointmentCalendarOtherAppReadAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: AppointmentCalendarOtherAppWriteAccess = AppointmentCalendarOtherAppWriteAccess(0i32);
    pub const SystemOnly: AppointmentCalendarOtherAppWriteAccess = AppointmentCalendarOtherAppWriteAccess(1i32);
    pub const Limited: AppointmentCalendarOtherAppWriteAccess = AppointmentCalendarOtherAppWriteAccess(2i32);
}
impl ::core::convert::From<i32> for AppointmentCalendarOtherAppWriteAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentCalendarOtherAppWriteAccess {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarOtherAppWriteAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess;i4)");
}
impl ::windows::core::DefaultType for AppointmentCalendarOtherAppWriteAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentCalendarSyncManager(pub ::windows::core::IInspectable);
impl AppointmentCalendarSyncManager {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Status(&self) -> ::windows::core::Result<AppointmentCalendarSyncStatus> {
        let this = self;
        unsafe {
            let mut result__: AppointmentCalendarSyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarSyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetStatus(&self, value: AppointmentCalendarSyncStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager;{2b21b3a0-4aff-4392-bc5f-5645ffcffb17})");
}
unsafe impl ::windows::core::Interface for AppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b21b3a0_4aff_4392_bc5f_5645ffcffb17);
}
impl ::windows::core::RuntimeName for AppointmentCalendarSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
}
impl ::core::convert::From<AppointmentCalendarSyncManager> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarSyncManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManager> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarSyncManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentCalendarSyncManager> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarSyncManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManager> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarSyncManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarSyncManager {}
unsafe impl ::core::marker::Sync for AppointmentCalendarSyncManager {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(0i32);
    pub const Syncing: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(1i32);
    pub const UpToDate: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(2i32);
    pub const AuthenticationError: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(3i32);
    pub const PolicyError: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(4i32);
    pub const UnknownError: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(5i32);
    pub const ManualAccountRemovalRequired: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(6i32);
}
impl ::core::convert::From<i32> for AppointmentCalendarSyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentCalendarSyncStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarSyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus;i4)");
}
impl ::windows::core::DefaultType for AppointmentCalendarSyncStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentConflictResult(pub ::windows::core::IInspectable);
impl AppointmentConflictResult {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Type(&self) -> ::windows::core::Result<AppointmentConflictType> {
        let this = self;
        unsafe {
            let mut result__: AppointmentConflictType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentConflictType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentConflictResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentConflictResult;{d5cdf0be-2f2f-3b7d-af0a-a7e20f3a46e3})");
}
unsafe impl ::windows::core::Interface for AppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5cdf0be_2f2f_3b7d_af0a_a7e20f3a46e3);
}
impl ::windows::core::RuntimeName for AppointmentConflictResult {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentConflictResult";
}
impl ::core::convert::From<AppointmentConflictResult> for ::windows::core::IUnknown {
    fn from(value: AppointmentConflictResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentConflictResult> for ::windows::core::IUnknown {
    fn from(value: &AppointmentConflictResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentConflictResult> for ::windows::core::IInspectable {
    fn from(value: AppointmentConflictResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentConflictResult> for ::windows::core::IInspectable {
    fn from(value: &AppointmentConflictResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentConflictResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentConflictResult {}
unsafe impl ::core::marker::Sync for AppointmentConflictResult {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: AppointmentConflictType = AppointmentConflictType(0i32);
    pub const Adjacent: AppointmentConflictType = AppointmentConflictType(1i32);
    pub const Overlap: AppointmentConflictType = AppointmentConflictType(2i32);
}
impl ::core::convert::From<i32> for AppointmentConflictType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentConflictType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentConflictType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentConflictType;i4)");
}
impl ::windows::core::DefaultType for AppointmentConflictType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: AppointmentDaysOfWeek = AppointmentDaysOfWeek(0u32);
    pub const Sunday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(1u32);
    pub const Monday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(2u32);
    pub const Tuesday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(4u32);
    pub const Wednesday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(8u32);
    pub const Thursday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(16u32);
    pub const Friday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(32u32);
    pub const Saturday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(64u32);
}
impl ::core::convert::From<u32> for AppointmentDaysOfWeek {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentDaysOfWeek {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentDaysOfWeek {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)");
}
impl ::windows::core::DefaultType for AppointmentDaysOfWeek {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AppointmentDaysOfWeek {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AppointmentDaysOfWeek {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AppointmentDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: AppointmentDetailsKind = AppointmentDetailsKind(0i32);
    pub const Html: AppointmentDetailsKind = AppointmentDetailsKind(1i32);
}
impl ::core::convert::From<i32> for AppointmentDetailsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentDetailsKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentDetailsKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDetailsKind;i4)");
}
impl ::windows::core::DefaultType for AppointmentDetailsKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentException(pub ::windows::core::IInspectable);
impl AppointmentException {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Appointment(&self) -> ::windows::core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Appointment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation_Collections`*"]
    pub fn ExceptionProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsDeleted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentException {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentException;{a2076767-16f6-4bce-9f5a-8600b8019fcb})");
}
unsafe impl ::windows::core::Interface for AppointmentException {
    type Vtable = IAppointmentException_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2076767_16f6_4bce_9f5a_8600b8019fcb);
}
impl ::windows::core::RuntimeName for AppointmentException {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentException";
}
impl ::core::convert::From<AppointmentException> for ::windows::core::IUnknown {
    fn from(value: AppointmentException) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentException> for ::windows::core::IUnknown {
    fn from(value: &AppointmentException) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentException> for ::windows::core::IInspectable {
    fn from(value: AppointmentException) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentException> for ::windows::core::IInspectable {
    fn from(value: &AppointmentException) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentException {}
unsafe impl ::core::marker::Sync for AppointmentException {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentInvitee(pub ::windows::core::IInspectable);
impl AppointmentInvitee {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentInvitee, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Role(&self) -> ::windows::core::Result<AppointmentParticipantRole> {
        let this = self;
        unsafe {
            let mut result__: AppointmentParticipantRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentParticipantRole>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetRole(&self, value: AppointmentParticipantRole) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Response(&self) -> ::windows::core::Result<AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__: AppointmentParticipantResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentParticipantResponse>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetResponse(&self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentInvitee {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentInvitee;{13bf0796-9842-495b-b0e7-ef8f79c0701d})");
}
unsafe impl ::windows::core::Interface for AppointmentInvitee {
    type Vtable = IAppointmentInvitee_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13bf0796_9842_495b_b0e7_ef8f79c0701d);
}
impl ::windows::core::RuntimeName for AppointmentInvitee {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentInvitee";
}
impl ::core::convert::From<AppointmentInvitee> for ::windows::core::IUnknown {
    fn from(value: AppointmentInvitee) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentInvitee> for ::windows::core::IUnknown {
    fn from(value: &AppointmentInvitee) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentInvitee> for ::windows::core::IInspectable {
    fn from(value: AppointmentInvitee) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentInvitee> for ::windows::core::IInspectable {
    fn from(value: &AppointmentInvitee) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentInvitee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
pub struct AppointmentManager {}
impl AppointmentManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointment: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowAddAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointment: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, appointment: Param1, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowReplaceAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(
        appointmentid: Param0,
        appointment: Param1,
        selection: Param2,
        preferredplacement: super::super::UI::Popups::Placement,
        instancestartdate: Param4,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowRemoveAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowTimeFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(timetoshow: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), timetoshow.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appointmentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(appointmentid: Param0, instancestartdate: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn RequestStoreAsync(options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentStore>>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<AppointmentManagerForUser> {
        Self::IAppointmentManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppointmentManagerForUser>(result__)
        })
    }
    pub fn IAppointmentManagerStatics<R, F: FnOnce(&IAppointmentManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentManager, IAppointmentManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppointmentManagerStatics2<R, F: FnOnce(&IAppointmentManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentManager, IAppointmentManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppointmentManagerStatics3<R, F: FnOnce(&IAppointmentManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentManager, IAppointmentManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppointmentManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManager";
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentManagerForUser(pub ::windows::core::IInspectable);
impl AppointmentManagerForUser {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointment: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowAddAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointment: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowReplaceAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(
        &self,
        appointmentid: Param0,
        appointment: Param1,
        selection: Param2,
        preferredplacement: super::super::UI::Popups::Placement,
        instancestartdate: Param4,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowRemoveAppointmentWithPlacementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointmentid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowTimeFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timetoshow: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), timetoshow.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appointmentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointmentid: Param0, instancestartdate: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), appointmentid.into_param().abi(), instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentManagerForUser;{70261423-73cc-4660-b318-b01365302a03})");
}
unsafe impl ::windows::core::Interface for AppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70261423_73cc_4660_b318_b01365302a03);
}
impl ::windows::core::RuntimeName for AppointmentManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
}
impl ::core::convert::From<AppointmentManagerForUser> for ::windows::core::IUnknown {
    fn from(value: AppointmentManagerForUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &AppointmentManagerForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentManagerForUser> for ::windows::core::IInspectable {
    fn from(value: AppointmentManagerForUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &AppointmentManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentManagerForUser {}
unsafe impl ::core::marker::Sync for AppointmentManagerForUser {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentOrganizer(pub ::windows::core::IInspectable);
impl AppointmentOrganizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentOrganizer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentOrganizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentOrganizer;{615e2902-9718-467b-83fb-b293a19121de})");
}
unsafe impl ::windows::core::Interface for AppointmentOrganizer {
    type Vtable = IAppointmentParticipant_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x615e2902_9718_467b_83fb_b293a19121de);
}
impl ::windows::core::RuntimeName for AppointmentOrganizer {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentOrganizer";
}
impl ::core::convert::From<AppointmentOrganizer> for ::windows::core::IUnknown {
    fn from(value: AppointmentOrganizer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for ::windows::core::IUnknown {
    fn from(value: &AppointmentOrganizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentOrganizer> for ::windows::core::IInspectable {
    fn from(value: AppointmentOrganizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for ::windows::core::IInspectable {
    fn from(value: &AppointmentOrganizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppointmentOrganizer> for IAppointmentParticipant {
    fn from(value: AppointmentOrganizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for IAppointmentParticipant {
    fn from(value: &AppointmentOrganizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentParticipant> for AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentParticipant> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentParticipant> for &AppointmentOrganizer {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentParticipant> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentOrganizer {}
unsafe impl ::core::marker::Sync for AppointmentOrganizer {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentParticipantResponse(pub i32);
impl AppointmentParticipantResponse {
    pub const None: AppointmentParticipantResponse = AppointmentParticipantResponse(0i32);
    pub const Tentative: AppointmentParticipantResponse = AppointmentParticipantResponse(1i32);
    pub const Accepted: AppointmentParticipantResponse = AppointmentParticipantResponse(2i32);
    pub const Declined: AppointmentParticipantResponse = AppointmentParticipantResponse(3i32);
    pub const Unknown: AppointmentParticipantResponse = AppointmentParticipantResponse(4i32);
}
impl ::core::convert::From<i32> for AppointmentParticipantResponse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentParticipantResponse {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentParticipantResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantResponse;i4)");
}
impl ::windows::core::DefaultType for AppointmentParticipantResponse {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: AppointmentParticipantRole = AppointmentParticipantRole(0i32);
    pub const OptionalAttendee: AppointmentParticipantRole = AppointmentParticipantRole(1i32);
    pub const Resource: AppointmentParticipantRole = AppointmentParticipantRole(2i32);
}
impl ::core::convert::From<i32> for AppointmentParticipantRole {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentParticipantRole {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentParticipantRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantRole;i4)");
}
impl ::windows::core::DefaultType for AppointmentParticipantRole {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
pub struct AppointmentProperties {}
impl AppointmentProperties {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Subject() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Location() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn StartTime() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Duration() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Reminder() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn BusyStatus() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Sensitivity() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn OriginalStartTime() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsResponseRequested() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AllowNewTimeProposal() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AllDay() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Details() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn OnlineMeetingLink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn ReplyTime() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Organizer() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn UserResponse() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn HasInvitees() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsCanceledMeeting() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsOrganizedByUser() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Recurrence() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Uri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Invitees() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation_Collections`*"]
    pub fn DefaultProperties() -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn ChangeNumber() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn RemoteChangeNumber() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DetailsKind() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IAppointmentPropertiesStatics<R, F: FnOnce(&IAppointmentPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppointmentPropertiesStatics2<R, F: FnOnce(&IAppointmentPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppointmentProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentProperties";
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentRecurrence(pub ::windows::core::IInspectable);
impl AppointmentRecurrence {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentRecurrence, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Unit(&self) -> ::windows::core::Result<AppointmentRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__: AppointmentRecurrenceUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentRecurrenceUnit>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetUnit(&self, value: AppointmentRecurrenceUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetOccurrences<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn SetUntil<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Interval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DaysOfWeek(&self) -> ::windows::core::Result<AppointmentDaysOfWeek> {
        let this = self;
        unsafe {
            let mut result__: AppointmentDaysOfWeek = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentDaysOfWeek>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDaysOfWeek(&self, value: AppointmentDaysOfWeek) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn WeekOfMonth(&self) -> ::windows::core::Result<AppointmentWeekOfMonth> {
        let this = self;
        unsafe {
            let mut result__: AppointmentWeekOfMonth = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentWeekOfMonth>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetWeekOfMonth(&self, value: AppointmentWeekOfMonth) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Month(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetMonth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Day(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDay(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn RecurrenceType(&self) -> ::windows::core::Result<RecurrenceType> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__: RecurrenceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RecurrenceType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn TimeZone(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetTimeZone<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentRecurrence3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentRecurrence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentRecurrence;{d87b3e83-15a6-487b-b959-0c361e60e954})");
}
unsafe impl ::windows::core::Interface for AppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd87b3e83_15a6_487b_b959_0c361e60e954);
}
impl ::windows::core::RuntimeName for AppointmentRecurrence {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentRecurrence";
}
impl ::core::convert::From<AppointmentRecurrence> for ::windows::core::IUnknown {
    fn from(value: AppointmentRecurrence) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentRecurrence> for ::windows::core::IUnknown {
    fn from(value: &AppointmentRecurrence) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentRecurrence> for ::windows::core::IInspectable {
    fn from(value: AppointmentRecurrence) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentRecurrence> for ::windows::core::IInspectable {
    fn from(value: &AppointmentRecurrence) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentRecurrence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentRecurrence {}
unsafe impl ::core::marker::Sync for AppointmentRecurrence {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(0i32);
    pub const Weekly: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(1i32);
    pub const Monthly: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(2i32);
    pub const MonthlyOnDay: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(3i32);
    pub const Yearly: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(4i32);
    pub const YearlyOnDay: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(5i32);
}
impl ::core::convert::From<i32> for AppointmentRecurrenceUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentRecurrenceUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentRecurrenceUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit;i4)");
}
impl ::windows::core::DefaultType for AppointmentRecurrenceUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: AppointmentSensitivity = AppointmentSensitivity(0i32);
    pub const Private: AppointmentSensitivity = AppointmentSensitivity(1i32);
}
impl ::core::convert::From<i32> for AppointmentSensitivity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentSensitivity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentSensitivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSensitivity;i4)");
}
impl ::windows::core::DefaultType for AppointmentSensitivity {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStore(pub ::windows::core::IInspectable);
impl AppointmentStore {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn ChangeTracker(&self) -> ::windows::core::Result<AppointmentStoreChangeTracker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangeTracker>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn CreateAppointmentCalendarAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn GetAppointmentCalendarAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, calendarid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), calendarid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn GetAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn GetAppointmentInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentCalendarsAsyncWithOptions(&self, options: FindAppointmentCalendarsOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentsAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, FindAppointmentsOptions>>(&self, rangestart: Param0, rangelength: Param1, options: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn FindConflictAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn FindConflictAsyncWithInstanceStart<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, appointment: Param0, instancestarttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), appointment.into_param().abi(), instancestarttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn MoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, AppointmentCalendar>>(&self, appointment: Param0, destinationcalendar: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), appointment.into_param().abi(), destinationcalendar.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, appointment: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, localid: Param0, appointment: Param1, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), localid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Appointment>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(
        &self,
        localid: Param0,
        appointment: Param1,
        selection: Param2,
        preferredplacement: super::super::UI::Popups::Placement,
        instancestartdate: Param4,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), localid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, localid: Param0, selection: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), localid.into_param().abi(), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `UI_Popups`*"]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), localid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), localid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, localid: Param0, instancestartdate: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), localid.into_param().abi(), instancestartdate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows::core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), appointment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindLocalIdsFromRoamingIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, roamingid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), roamingid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn StoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>>(&self, phandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), phandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn RemoveStoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`*"]
    pub fn CreateAppointmentCalendarInAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn GetChangeTracker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, identity: Param0) -> ::windows::core::Result<AppointmentStoreChangeTracker> {
        let this = &::windows::core::Interface::cast::<IAppointmentStore3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<AppointmentStoreChangeTracker>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStore;{a461918c-7a47-4d96-96c9-15cd8a05a735})");
}
unsafe impl ::windows::core::Interface for AppointmentStore {
    type Vtable = IAppointmentStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa461918c_7a47_4d96_96c9_15cd8a05a735);
}
impl ::windows::core::RuntimeName for AppointmentStore {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStore";
}
impl ::core::convert::From<AppointmentStore> for ::windows::core::IUnknown {
    fn from(value: AppointmentStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStore> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStore> for ::windows::core::IInspectable {
    fn from(value: AppointmentStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStore> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStore {}
unsafe impl ::core::marker::Sync for AppointmentStore {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: AppointmentStoreAccessType = AppointmentStoreAccessType(0i32);
    pub const AllCalendarsReadOnly: AppointmentStoreAccessType = AppointmentStoreAccessType(1i32);
    pub const AllCalendarsReadWrite: AppointmentStoreAccessType = AppointmentStoreAccessType(2i32);
}
impl ::core::convert::From<i32> for AppointmentStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreAccessType;i4)");
}
impl ::windows::core::DefaultType for AppointmentStoreAccessType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStoreChange(pub ::windows::core::IInspectable);
impl AppointmentStoreChange {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Appointment(&self) -> ::windows::core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Appointment>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn ChangeType(&self) -> ::windows::core::Result<AppointmentStoreChangeType> {
        let this = self;
        unsafe {
            let mut result__: AppointmentStoreChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangeType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AppointmentCalendar(&self) -> ::windows::core::Result<AppointmentCalendar> {
        let this = &::windows::core::Interface::cast::<IAppointmentStoreChange2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendar>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChange;{a5a6e035-0a33-3654-8463-b543e90c3b79})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5a6e035_0a33_3654_8463_b543e90c3b79);
}
impl ::windows::core::RuntimeName for AppointmentStoreChange {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChange";
}
impl ::core::convert::From<AppointmentStoreChange> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChange) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStoreChange> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChange) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStoreChange> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStoreChange> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChange {}
unsafe impl ::core::marker::Sync for AppointmentStoreChange {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStoreChangeReader(pub ::windows::core::IInspectable);
impl AppointmentStoreChangeReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AcceptChanges(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn AcceptChangesThrough<'a, Param0: ::windows::core::IntoParam<'a, AppointmentStoreChange>>(&self, lastchangetoaccept: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), lastchangetoaccept.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader;{8b2409f1-65f3-42a0-961d-4c209bf30370})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b2409f1_65f3_42a0_961d_4c209bf30370);
}
impl ::windows::core::RuntimeName for AppointmentStoreChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
}
impl ::core::convert::From<AppointmentStoreChangeReader> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangeReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStoreChangeReader> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangeReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStoreChangeReader> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangeReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStoreChangeReader> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangeReader {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeReader {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStoreChangeTracker(pub ::windows::core::IInspectable);
impl AppointmentStoreChangeTracker {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn GetChangeReader(&self) -> ::windows::core::Result<AppointmentStoreChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangeReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IsTracking(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppointmentStoreChangeTracker2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker;{1b25f4b1-8ece-4f17-93c8-e6412458fd5c})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25f4b1_8ece_4f17_93c8_e6412458fd5c);
}
impl ::windows::core::RuntimeName for AppointmentStoreChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
}
impl ::core::convert::From<AppointmentStoreChangeTracker> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangeTracker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStoreChangeTracker> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangeTracker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStoreChangeTracker> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangeTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStoreChangeTracker> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangeTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangeTracker {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeTracker {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentStoreChangeType(pub i32);
impl AppointmentStoreChangeType {
    pub const AppointmentCreated: AppointmentStoreChangeType = AppointmentStoreChangeType(0i32);
    pub const AppointmentModified: AppointmentStoreChangeType = AppointmentStoreChangeType(1i32);
    pub const AppointmentDeleted: AppointmentStoreChangeType = AppointmentStoreChangeType(2i32);
    pub const ChangeTrackingLost: AppointmentStoreChangeType = AppointmentStoreChangeType(3i32);
    pub const CalendarCreated: AppointmentStoreChangeType = AppointmentStoreChangeType(4i32);
    pub const CalendarModified: AppointmentStoreChangeType = AppointmentStoreChangeType(5i32);
    pub const CalendarDeleted: AppointmentStoreChangeType = AppointmentStoreChangeType(6i32);
}
impl ::core::convert::From<i32> for AppointmentStoreChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentStoreChangeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreChangeType;i4)");
}
impl ::windows::core::DefaultType for AppointmentStoreChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStoreChangedDeferral(pub ::windows::core::IInspectable);
impl AppointmentStoreChangedDeferral {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral;{4cb82026-fedb-4bc3-9662-95a9befdf4df})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cb82026_fedb_4bc3_9662_95a9befdf4df);
}
impl ::windows::core::RuntimeName for AppointmentStoreChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
}
impl ::core::convert::From<AppointmentStoreChangedDeferral> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangedDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStoreChangedDeferral> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangedDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStoreChangedDeferral> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangedDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStoreChangedDeferral> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangedDeferral {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedDeferral {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStoreChangedEventArgs(pub ::windows::core::IInspectable);
impl AppointmentStoreChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<AppointmentStoreChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentStoreChangedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs;{2285f8b9-0791-417e-bfea-cc6d41636c8c})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2285f8b9_0791_417e_bfea_cc6d41636c8c);
}
impl ::windows::core::RuntimeName for AppointmentStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
}
impl ::core::convert::From<AppointmentStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentStoreNotificationTriggerDetails(pub ::windows::core::IInspectable);
impl AppointmentStoreNotificationTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails;{9b33cb11-c301-421e-afef-047ecfa76adb})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b33cb11_c301_421e_afef_047ecfa76adb);
}
impl ::windows::core::RuntimeName for AppointmentStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
}
impl ::core::convert::From<AppointmentStoreNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentStoreNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentSummaryCardView(pub i32);
impl AppointmentSummaryCardView {
    pub const System: AppointmentSummaryCardView = AppointmentSummaryCardView(0i32);
    pub const App: AppointmentSummaryCardView = AppointmentSummaryCardView(1i32);
}
impl ::core::convert::From<i32> for AppointmentSummaryCardView {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentSummaryCardView {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentSummaryCardView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSummaryCardView;i4)");
}
impl ::windows::core::DefaultType for AppointmentSummaryCardView {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: AppointmentWeekOfMonth = AppointmentWeekOfMonth(0i32);
    pub const Second: AppointmentWeekOfMonth = AppointmentWeekOfMonth(1i32);
    pub const Third: AppointmentWeekOfMonth = AppointmentWeekOfMonth(2i32);
    pub const Fourth: AppointmentWeekOfMonth = AppointmentWeekOfMonth(3i32);
    pub const Last: AppointmentWeekOfMonth = AppointmentWeekOfMonth(4i32);
}
impl ::core::convert::From<i32> for AppointmentWeekOfMonth {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppointmentWeekOfMonth {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppointmentWeekOfMonth {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth;i4)");
}
impl ::windows::core::DefaultType for AppointmentWeekOfMonth {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: FindAppointmentCalendarsOptions = FindAppointmentCalendarsOptions(0u32);
    pub const IncludeHidden: FindAppointmentCalendarsOptions = FindAppointmentCalendarsOptions(1u32);
}
impl ::core::convert::From<u32> for FindAppointmentCalendarsOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FindAppointmentCalendarsOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FindAppointmentCalendarsOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions;u4)");
}
impl ::windows::core::DefaultType for FindAppointmentCalendarsOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for FindAppointmentCalendarsOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for FindAppointmentCalendarsOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FindAppointmentsOptions(pub ::windows::core::IInspectable);
impl FindAppointmentsOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FindAppointmentsOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation_Collections`*"]
    pub fn CalendarIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments`, `Foundation_Collections`*"]
    pub fn FetchProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn IncludeHidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetIncludeHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn MaxCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetMaxCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FindAppointmentsOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.FindAppointmentsOptions;{55f7dc55-9942-3086-82b5-2cb29f64d5f5})");
}
unsafe impl ::windows::core::Interface for FindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55f7dc55_9942_3086_82b5_2cb29f64d5f5);
}
impl ::windows::core::RuntimeName for FindAppointmentsOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
}
impl ::core::convert::From<FindAppointmentsOptions> for ::windows::core::IUnknown {
    fn from(value: FindAppointmentsOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FindAppointmentsOptions> for ::windows::core::IUnknown {
    fn from(value: &FindAppointmentsOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FindAppointmentsOptions> for ::windows::core::IInspectable {
    fn from(value: FindAppointmentsOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FindAppointmentsOptions> for ::windows::core::IInspectable {
    fn from(value: &FindAppointmentsOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FindAppointmentsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FindAppointmentsOptions {}
unsafe impl ::core::marker::Sync for FindAppointmentsOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointment {
    type Vtable = IAppointment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd002f2f_2bdd_4076_90a3_22c275312965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentBusyStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentBusyStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentSensitivity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentSensitivity) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointment2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointment2 {
    type Vtable = IAppointment2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e85983c_540f_3452_9b5c_0dd7ad4c65a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointment3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointment3 {
    type Vtable = IAppointment3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcc45a9_8961_4991_934b_c48768e5a96c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentDetailsKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentDetailsKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentCalendar {
    type Vtable = IAppointmentCalendar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5273819d_8339_3d4f_a02f_64084452bb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentCalendarOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentSummaryCardView) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentSummaryCardView) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pappointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendar2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentCalendar2 {
    type Vtable = IAppointmentCalendar2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18e7e422_2467_4e1c_a459_d8a29303d092);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, notifyinvitees: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, meeting: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, notifyinvitees: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, meeting: ::windows::core::RawPtr, invitees: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, forwardheader: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, meeting: ::windows::core::RawPtr, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, meeting: ::windows::core::RawPtr, response: AppointmentParticipantResponse, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sendupdate: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendar3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentCalendar3 {
    type Vtable = IAppointmentCalendar3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb23d22b_a685_42ae_8495_b3119adb4167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b21b3a0_4aff_4392_bc5f_5645ffcffb17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentCalendarSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentCalendarSyncManager2 {
    type Vtable = IAppointmentCalendarSyncManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x647528ad_0d29_4c7c_aaa7_bf996805537c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentCalendarSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentConflictResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5cdf0be_2f2f_3b7d_af0a_a7e20f3a46e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentConflictResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentConflictType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentException(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentException {
    type Vtable = IAppointmentException_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2076767_16f6_4bce_9f5a_8600b8019fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentException_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentInvitee(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentInvitee {
    type Vtable = IAppointmentInvitee_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13bf0796_9842_495b_b0e7_ef8f79c0701d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentInvitee_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentParticipantRole) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentParticipantRole) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentParticipantResponse) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentManagerForUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70261423_73cc_4660_b318_b01365302a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: AppointmentStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentManagerStatics {
    type Vtable = IAppointmentManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a30fa01_5c40_499d_b33f_a43050f74fc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentManagerStatics2 {
    type Vtable = IAppointmentManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a81f60d_d04f_4034_af72_a36573b45ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: AppointmentStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentManagerStatics3 {
    type Vtable = IAppointmentManagerStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9ae09c_b34c_4dc7_a35d_cafd88ae3ec6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
pub struct IAppointmentParticipant(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentParticipant {
    type Vtable = IAppointmentParticipant_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x615e2902_9718_467b_83fb_b293a19121de);
}
impl IAppointmentParticipant {
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentParticipant {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{615e2902-9718-467b-83fb-b293a19121de}");
}
impl ::core::convert::From<IAppointmentParticipant> for ::windows::core::IUnknown {
    fn from(value: IAppointmentParticipant) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentParticipant> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentParticipant) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentParticipant> for ::windows::core::IInspectable {
    fn from(value: IAppointmentParticipant) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentParticipant> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentParticipant) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentParticipant_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentPropertiesStatics {
    type Vtable = IAppointmentPropertiesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25141fe9_68ae_3aae_855f_bc4441caa234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentPropertiesStatics2 {
    type Vtable = IAppointmentPropertiesStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdffc434b_b017_45dd_8af5_d163d10801bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentRecurrence(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd87b3e83_15a6_487b_b959_0c361e60e954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentRecurrenceUnit) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentRecurrenceUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentDaysOfWeek) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentDaysOfWeek) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentWeekOfMonth) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppointmentWeekOfMonth) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentRecurrence2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentRecurrence2 {
    type Vtable = IAppointmentRecurrence2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df3a2e0_05a7_4f50_9f86_b03f9436254d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RecurrenceType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentRecurrence3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentRecurrence3 {
    type Vtable = IAppointmentRecurrence3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89ff96d9_da4d_4a17_8dd2_1cebc2b5ff9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStore {
    type Vtable = IAppointmentStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa461918c_7a47_4d96_96c9_15cd8a05a735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, calendarid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: FindAppointmentCalendarsOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, destinationcalendar: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, roamingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStore2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStore2 {
    type Vtable = IAppointmentStore2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25c48c20_1c41_424f_8084_67c1cfe0a854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStore3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStore3 {
    type Vtable = IAppointmentStore3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4251940b_b078_470a_9a40_c2e01761f72f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChange(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5a6e035_0a33_3654_8463_b543e90c3b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppointmentStoreChangeType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChange2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChange2 {
    type Vtable = IAppointmentStoreChange2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb37d0dce_5211_4402_a608_a96fe70b8ee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b2409f1_65f3_42a0_961d_4c209bf30370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lastchangetoaccept: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25f4b1_8ece_4f17_93c8_e6412458fd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangeTracker2 {
    type Vtable = IAppointmentStoreChangeTracker2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb66aaf45_9542_4cf7_8550_eb370e0c08d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cb82026_fedb_4bc3_9662_95a9befdf4df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2285f8b9_0791_417e_bfea_cc6d41636c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b33cb11_c301_421e_afef_047ecfa76adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFindAppointmentsOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55f7dc55_9942_3086_82b5_2cb29f64d5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAppointmentsOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Appointments`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RecurrenceType(pub i32);
impl RecurrenceType {
    pub const Master: RecurrenceType = RecurrenceType(0i32);
    pub const Instance: RecurrenceType = RecurrenceType(1i32);
    pub const ExceptionInstance: RecurrenceType = RecurrenceType(2i32);
}
impl ::core::convert::From<i32> for RecurrenceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RecurrenceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RecurrenceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.RecurrenceType;i4)");
}
impl ::windows::core::DefaultType for RecurrenceType {
    type DefaultType = Self;
}

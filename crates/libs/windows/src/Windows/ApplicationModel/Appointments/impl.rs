#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointmentImpl: Sized {
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subject(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Details(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDetails(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Reminder(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetReminder(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Organizer(&mut self) -> ::windows::core::Result<AppointmentOrganizer>;
    fn SetOrganizer(&mut self, value: &::core::option::Option<AppointmentOrganizer>) -> ::windows::core::Result<()>;
    fn Invitees(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AppointmentInvitee>>;
    fn Recurrence(&mut self) -> ::windows::core::Result<AppointmentRecurrence>;
    fn SetRecurrence(&mut self, value: &::core::option::Option<AppointmentRecurrence>) -> ::windows::core::Result<()>;
    fn BusyStatus(&mut self) -> ::windows::core::Result<AppointmentBusyStatus>;
    fn SetBusyStatus(&mut self, value: AppointmentBusyStatus) -> ::windows::core::Result<()>;
    fn AllDay(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllDay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Sensitivity(&mut self) -> ::windows::core::Result<AppointmentSensitivity>;
    fn SetSensitivity(&mut self, value: AppointmentSensitivity) -> ::windows::core::Result<()>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointment {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentVtbl {
        unsafe extern "system" fn StartTime<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Location<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subject<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Details<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDetails<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDetails(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Reminder<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reminder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReminder<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReminder(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Organizer<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Organizer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrganizer<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrganizer(&*(&value as *const <AppointmentOrganizer as ::windows::core::Abi>::Abi as *const <AppointmentOrganizer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Invitees<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recurrence<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recurrence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecurrence<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecurrence(&*(&value as *const <AppointmentRecurrence as ::windows::core::Abi>::Abi as *const <AppointmentRecurrence as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusyStatus<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentBusyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusyStatus<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentBusyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusyStatus(value).into()
        }
        unsafe extern "system" fn AllDay<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllDay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllDay<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllDay(value).into()
        }
        unsafe extern "system" fn Sensitivity<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSensitivity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSensitivity<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentSensitivity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSensitivity(value).into()
        }
        unsafe extern "system" fn Uri<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IAppointmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointment, BASE_OFFSET>(),
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            Details: Details::<Impl, IMPL_OFFSET>,
            SetDetails: SetDetails::<Impl, IMPL_OFFSET>,
            Reminder: Reminder::<Impl, IMPL_OFFSET>,
            SetReminder: SetReminder::<Impl, IMPL_OFFSET>,
            Organizer: Organizer::<Impl, IMPL_OFFSET>,
            SetOrganizer: SetOrganizer::<Impl, IMPL_OFFSET>,
            Invitees: Invitees::<Impl, IMPL_OFFSET>,
            Recurrence: Recurrence::<Impl, IMPL_OFFSET>,
            SetRecurrence: SetRecurrence::<Impl, IMPL_OFFSET>,
            BusyStatus: BusyStatus::<Impl, IMPL_OFFSET>,
            SetBusyStatus: SetBusyStatus::<Impl, IMPL_OFFSET>,
            AllDay: AllDay::<Impl, IMPL_OFFSET>,
            SetAllDay: SetAllDay::<Impl, IMPL_OFFSET>,
            Sensitivity: Sensitivity::<Impl, IMPL_OFFSET>,
            SetSensitivity: SetSensitivity::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointment2Impl: Sized + IAppointmentImpl {
    fn LocalId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CalendarId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRoamingId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OriginalStartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn IsResponseRequested(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsResponseRequested(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowNewTimeProposal(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowNewTimeProposal(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OnlineMeetingLink(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOnlineMeetingLink(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReplyTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetReplyTime(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn UserResponse(&mut self) -> ::windows::core::Result<AppointmentParticipantResponse>;
    fn SetUserResponse(&mut self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()>;
    fn HasInvitees(&mut self) -> ::windows::core::Result<bool>;
    fn IsCanceledMeeting(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCanceledMeeting(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsOrganizedByUser(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOrganizedByUser(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointment2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointment2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointment2Vtbl {
        unsafe extern "system" fn LocalId<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalendarId<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalendarId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingId<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoamingId<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoamingId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OriginalStartTime<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsResponseRequested<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResponseRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsResponseRequested<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsResponseRequested(value).into()
        }
        unsafe extern "system" fn AllowNewTimeProposal<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowNewTimeProposal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowNewTimeProposal<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowNewTimeProposal(value).into()
        }
        unsafe extern "system" fn OnlineMeetingLink<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnlineMeetingLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOnlineMeetingLink<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOnlineMeetingLink(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReplyTime<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplyTime<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReplyTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserResponse<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserResponse<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserResponse(value).into()
        }
        unsafe extern "system" fn HasInvitees<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasInvitees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCanceledMeeting<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCanceledMeeting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCanceledMeeting<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCanceledMeeting(value).into()
        }
        unsafe extern "system" fn IsOrganizedByUser<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOrganizedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOrganizedByUser<Impl: IAppointment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOrganizedByUser(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointment2, BASE_OFFSET>(),
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            CalendarId: CalendarId::<Impl, IMPL_OFFSET>,
            RoamingId: RoamingId::<Impl, IMPL_OFFSET>,
            SetRoamingId: SetRoamingId::<Impl, IMPL_OFFSET>,
            OriginalStartTime: OriginalStartTime::<Impl, IMPL_OFFSET>,
            IsResponseRequested: IsResponseRequested::<Impl, IMPL_OFFSET>,
            SetIsResponseRequested: SetIsResponseRequested::<Impl, IMPL_OFFSET>,
            AllowNewTimeProposal: AllowNewTimeProposal::<Impl, IMPL_OFFSET>,
            SetAllowNewTimeProposal: SetAllowNewTimeProposal::<Impl, IMPL_OFFSET>,
            OnlineMeetingLink: OnlineMeetingLink::<Impl, IMPL_OFFSET>,
            SetOnlineMeetingLink: SetOnlineMeetingLink::<Impl, IMPL_OFFSET>,
            ReplyTime: ReplyTime::<Impl, IMPL_OFFSET>,
            SetReplyTime: SetReplyTime::<Impl, IMPL_OFFSET>,
            UserResponse: UserResponse::<Impl, IMPL_OFFSET>,
            SetUserResponse: SetUserResponse::<Impl, IMPL_OFFSET>,
            HasInvitees: HasInvitees::<Impl, IMPL_OFFSET>,
            IsCanceledMeeting: IsCanceledMeeting::<Impl, IMPL_OFFSET>,
            SetIsCanceledMeeting: SetIsCanceledMeeting::<Impl, IMPL_OFFSET>,
            IsOrganizedByUser: IsOrganizedByUser::<Impl, IMPL_OFFSET>,
            SetIsOrganizedByUser: SetIsOrganizedByUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointment3Impl: Sized + IAppointmentImpl + IAppointment2Impl {
    fn ChangeNumber(&mut self) -> ::windows::core::Result<u64>;
    fn RemoteChangeNumber(&mut self) -> ::windows::core::Result<u64>;
    fn SetRemoteChangeNumber(&mut self, value: u64) -> ::windows::core::Result<()>;
    fn DetailsKind(&mut self) -> ::windows::core::Result<AppointmentDetailsKind>;
    fn SetDetailsKind(&mut self, value: AppointmentDetailsKind) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointment3 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointment3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointment3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointment3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointment3Vtbl {
        unsafe extern "system" fn ChangeNumber<Impl: IAppointment3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteChangeNumber<Impl: IAppointment3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteChangeNumber<Impl: IAppointment3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteChangeNumber(value).into()
        }
        unsafe extern "system" fn DetailsKind<Impl: IAppointment3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDetailsKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailsKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDetailsKind<Impl: IAppointment3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentDetailsKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDetailsKind(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointment3, BASE_OFFSET>(),
            ChangeNumber: ChangeNumber::<Impl, IMPL_OFFSET>,
            RemoteChangeNumber: RemoteChangeNumber::<Impl, IMPL_OFFSET>,
            SetRemoteChangeNumber: SetRemoteChangeNumber::<Impl, IMPL_OFFSET>,
            DetailsKind: DetailsKind::<Impl, IMPL_OFFSET>,
            SetDetailsKind: SetDetailsKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointment3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
pub trait IAppointmentCalendarImpl: Sized {
    fn DisplayColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LocalId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsHidden(&mut self) -> ::windows::core::Result<bool>;
    fn OtherAppReadAccess(&mut self) -> ::windows::core::Result<AppointmentCalendarOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&mut self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&mut self) -> ::windows::core::Result<AppointmentCalendarOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&mut self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SummaryCardView(&mut self) -> ::windows::core::Result<AppointmentSummaryCardView>;
    fn SetSummaryCardView(&mut self, value: AppointmentSummaryCardView) -> ::windows::core::Result<()>;
    fn FindAppointmentsAsync(&mut self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindAppointmentsAsyncWithOptions(&mut self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan, options: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindExceptionsFromMasterAsync(&mut self, masterlocalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>>;
    fn FindAllInstancesAsync(&mut self, masterlocalid: &::windows::core::HSTRING, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindAllInstancesAsyncWithOptions(&mut self, masterlocalid: &::windows::core::HSTRING, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan, poptions: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn GetAppointmentAsync(&mut self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn GetAppointmentInstanceAsync(&mut self, localid: &::windows::core::HSTRING, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn FindUnexpandedAppointmentsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindUnexpandedAppointmentsAsyncWithOptions(&mut self, options: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn DeleteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAppointmentAsync(&mut self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAppointmentInstanceAsync(&mut self, localid: &::windows::core::HSTRING, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAppointmentAsync(&mut self, pappointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentCalendar {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentCalendar";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
impl IAppointmentCalendarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentCalendarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentCalendarVtbl {
        unsafe extern "system" fn DisplayColor<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalId<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherAppReadAccess<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppReadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppReadAccess<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppReadAccess(value).into()
        }
        unsafe extern "system" fn OtherAppWriteAccess<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppWriteAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppWriteAccess<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppWriteAccess(value).into()
        }
        unsafe extern "system" fn SourceDisplayName<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SummaryCardView<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSummaryCardView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SummaryCardView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSummaryCardView<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentSummaryCardView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSummaryCardView(value).into()
        }
        unsafe extern "system" fn FindAppointmentsAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppointmentsAsync(&*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppointmentsAsyncWithOptions<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppointmentsAsyncWithOptions(
                &*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <FindAppointmentsOptions as ::windows::core::Abi>::Abi as *const <FindAppointmentsOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindExceptionsFromMasterAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindExceptionsFromMasterAsync(&*(&masterlocalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllInstancesAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllInstancesAsync(
                &*(&masterlocalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllInstancesAsyncWithOptions<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllInstancesAsyncWithOptions(
                &*(&masterlocalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&poptions as *const <FindAppointmentsOptions as ::windows::core::Abi>::Abi as *const <FindAppointmentsOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppointmentAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppointmentAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppointmentInstanceAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppointmentInstanceAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&instancestarttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUnexpandedAppointmentsAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUnexpandedAppointmentsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUnexpandedAppointmentsAsyncWithOptions<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUnexpandedAppointmentsAsyncWithOptions(&*(&options as *const <FindAppointmentsOptions as ::windows::core::Abi>::Abi as *const <FindAppointmentsOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAppointmentAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAppointmentAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAppointmentInstanceAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAppointmentInstanceAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&instancestarttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAppointmentAsync<Impl: IAppointmentCalendarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAppointmentAsync(&*(&pappointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentCalendar, BASE_OFFSET>(),
            DisplayColor: DisplayColor::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            IsHidden: IsHidden::<Impl, IMPL_OFFSET>,
            OtherAppReadAccess: OtherAppReadAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppReadAccess: SetOtherAppReadAccess::<Impl, IMPL_OFFSET>,
            OtherAppWriteAccess: OtherAppWriteAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppWriteAccess: SetOtherAppWriteAccess::<Impl, IMPL_OFFSET>,
            SourceDisplayName: SourceDisplayName::<Impl, IMPL_OFFSET>,
            SummaryCardView: SummaryCardView::<Impl, IMPL_OFFSET>,
            SetSummaryCardView: SetSummaryCardView::<Impl, IMPL_OFFSET>,
            FindAppointmentsAsync: FindAppointmentsAsync::<Impl, IMPL_OFFSET>,
            FindAppointmentsAsyncWithOptions: FindAppointmentsAsyncWithOptions::<Impl, IMPL_OFFSET>,
            FindExceptionsFromMasterAsync: FindExceptionsFromMasterAsync::<Impl, IMPL_OFFSET>,
            FindAllInstancesAsync: FindAllInstancesAsync::<Impl, IMPL_OFFSET>,
            FindAllInstancesAsyncWithOptions: FindAllInstancesAsyncWithOptions::<Impl, IMPL_OFFSET>,
            GetAppointmentAsync: GetAppointmentAsync::<Impl, IMPL_OFFSET>,
            GetAppointmentInstanceAsync: GetAppointmentInstanceAsync::<Impl, IMPL_OFFSET>,
            FindUnexpandedAppointmentsAsync: FindUnexpandedAppointmentsAsync::<Impl, IMPL_OFFSET>,
            FindUnexpandedAppointmentsAsyncWithOptions: FindUnexpandedAppointmentsAsyncWithOptions::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            DeleteAppointmentAsync: DeleteAppointmentAsync::<Impl, IMPL_OFFSET>,
            DeleteAppointmentInstanceAsync: DeleteAppointmentInstanceAsync::<Impl, IMPL_OFFSET>,
            SaveAppointmentAsync: SaveAppointmentAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentCalendar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
pub trait IAppointmentCalendar2Impl: Sized + IAppointmentCalendarImpl {
    fn SyncManager(&mut self) -> ::windows::core::Result<AppointmentCalendarSyncManager>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetDisplayColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetIsHidden(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn UserDataAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanCreateOrUpdateAppointments(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanCreateOrUpdateAppointments(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanCancelMeetings(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanCancelMeetings(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanForwardMeetings(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanForwardMeetings(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanProposeNewTimeForMeetings(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanProposeNewTimeForMeetings(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanUpdateMeetingResponses(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanUpdateMeetingResponses(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanNotifyInvitees(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanNotifyInvitees(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MustNofityInvitees(&mut self) -> ::windows::core::Result<bool>;
    fn SetMustNofityInvitees(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryCreateOrUpdateAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCancelMeetingAsync(&mut self, meeting: &::core::option::Option<Appointment>, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, notifyinvitees: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryForwardMeetingAsync(&mut self, meeting: &::core::option::Option<Appointment>, invitees: &::core::option::Option<super::super::Foundation::Collections::IIterable<AppointmentInvitee>>, subject: &::windows::core::HSTRING, forwardheader: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryProposeNewTimeForMeetingAsync(&mut self, meeting: &::core::option::Option<Appointment>, newstarttime: &super::super::Foundation::DateTime, newduration: &super::super::Foundation::TimeSpan, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryUpdateMeetingResponseAsync(&mut self, meeting: &::core::option::Option<Appointment>, response: AppointmentParticipantResponse, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentCalendar2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentCalendar2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
impl IAppointmentCalendar2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentCalendar2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentCalendar2Vtbl {
        unsafe extern "system" fn SyncManager<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteId<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDisplayColor<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIsHidden<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHidden(value).into()
        }
        unsafe extern "system" fn UserDataAccountId<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDataAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCreateOrUpdateAppointments<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCreateOrUpdateAppointments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanCreateOrUpdateAppointments<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanCreateOrUpdateAppointments(value).into()
        }
        unsafe extern "system" fn CanCancelMeetings<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCancelMeetings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanCancelMeetings<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanCancelMeetings(value).into()
        }
        unsafe extern "system" fn CanForwardMeetings<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanForwardMeetings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanForwardMeetings<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanForwardMeetings(value).into()
        }
        unsafe extern "system" fn CanProposeNewTimeForMeetings<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanProposeNewTimeForMeetings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanProposeNewTimeForMeetings<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanProposeNewTimeForMeetings(value).into()
        }
        unsafe extern "system" fn CanUpdateMeetingResponses<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanUpdateMeetingResponses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanUpdateMeetingResponses<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanUpdateMeetingResponses(value).into()
        }
        unsafe extern "system" fn CanNotifyInvitees<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanNotifyInvitees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanNotifyInvitees<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanNotifyInvitees(value).into()
        }
        unsafe extern "system" fn MustNofityInvitees<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MustNofityInvitees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMustNofityInvitees<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMustNofityInvitees(value).into()
        }
        unsafe extern "system" fn TryCreateOrUpdateAppointmentAsync<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, notifyinvitees: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateOrUpdateAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), notifyinvitees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCancelMeetingAsync<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, notifyinvitees: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancelMeetingAsync(&*(&meeting as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), notifyinvitees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryForwardMeetingAsync<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, invitees: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, forwardheader: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryForwardMeetingAsync(
                &*(&meeting as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&invitees as *const <super::super::Foundation::Collections::IIterable<AppointmentInvitee> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<AppointmentInvitee> as ::windows::core::DefaultType>::DefaultType),
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&forwardheader as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryProposeNewTimeForMeetingAsync<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryProposeNewTimeForMeetingAsync(
                &*(&meeting as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&newstarttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&newduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateMeetingResponseAsync<Impl: IAppointmentCalendar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, response: AppointmentParticipantResponse, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sendupdate: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateMeetingResponseAsync(&*(&meeting as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), response, &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sendupdate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentCalendar2, BASE_OFFSET>(),
            SyncManager: SyncManager::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            SetDisplayColor: SetDisplayColor::<Impl, IMPL_OFFSET>,
            SetIsHidden: SetIsHidden::<Impl, IMPL_OFFSET>,
            UserDataAccountId: UserDataAccountId::<Impl, IMPL_OFFSET>,
            CanCreateOrUpdateAppointments: CanCreateOrUpdateAppointments::<Impl, IMPL_OFFSET>,
            SetCanCreateOrUpdateAppointments: SetCanCreateOrUpdateAppointments::<Impl, IMPL_OFFSET>,
            CanCancelMeetings: CanCancelMeetings::<Impl, IMPL_OFFSET>,
            SetCanCancelMeetings: SetCanCancelMeetings::<Impl, IMPL_OFFSET>,
            CanForwardMeetings: CanForwardMeetings::<Impl, IMPL_OFFSET>,
            SetCanForwardMeetings: SetCanForwardMeetings::<Impl, IMPL_OFFSET>,
            CanProposeNewTimeForMeetings: CanProposeNewTimeForMeetings::<Impl, IMPL_OFFSET>,
            SetCanProposeNewTimeForMeetings: SetCanProposeNewTimeForMeetings::<Impl, IMPL_OFFSET>,
            CanUpdateMeetingResponses: CanUpdateMeetingResponses::<Impl, IMPL_OFFSET>,
            SetCanUpdateMeetingResponses: SetCanUpdateMeetingResponses::<Impl, IMPL_OFFSET>,
            CanNotifyInvitees: CanNotifyInvitees::<Impl, IMPL_OFFSET>,
            SetCanNotifyInvitees: SetCanNotifyInvitees::<Impl, IMPL_OFFSET>,
            MustNofityInvitees: MustNofityInvitees::<Impl, IMPL_OFFSET>,
            SetMustNofityInvitees: SetMustNofityInvitees::<Impl, IMPL_OFFSET>,
            TryCreateOrUpdateAppointmentAsync: TryCreateOrUpdateAppointmentAsync::<Impl, IMPL_OFFSET>,
            TryCancelMeetingAsync: TryCancelMeetingAsync::<Impl, IMPL_OFFSET>,
            TryForwardMeetingAsync: TryForwardMeetingAsync::<Impl, IMPL_OFFSET>,
            TryProposeNewTimeForMeetingAsync: TryProposeNewTimeForMeetingAsync::<Impl, IMPL_OFFSET>,
            TryUpdateMeetingResponseAsync: TryUpdateMeetingResponseAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentCalendar2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentCalendar3Impl: Sized {
    fn RegisterSyncManagerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentCalendar3 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentCalendar3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentCalendar3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentCalendar3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentCalendar3Vtbl {
        unsafe extern "system" fn RegisterSyncManagerAsync<Impl: IAppointmentCalendar3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterSyncManagerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentCalendar3, BASE_OFFSET>(),
            RegisterSyncManagerAsync: RegisterSyncManagerAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentCalendar3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentCalendarSyncManagerImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<AppointmentCalendarSyncStatus>;
    fn LastSuccessfulSyncTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastAttemptedSyncTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SyncAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentCalendarSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentCalendarSyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentCalendarSyncManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentCalendarSyncManagerVtbl {
        unsafe extern "system" fn Status<Impl: IAppointmentCalendarSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarSyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastSuccessfulSyncTime<Impl: IAppointmentCalendarSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSuccessfulSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastAttemptedSyncTime<Impl: IAppointmentCalendarSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastAttemptedSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncAsync<Impl: IAppointmentCalendarSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncStatusChanged<Impl: IAppointmentCalendarSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSyncStatusChanged<Impl: IAppointmentCalendarSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSyncStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentCalendarSyncManager, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            LastSuccessfulSyncTime: LastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            LastAttemptedSyncTime: LastAttemptedSyncTime::<Impl, IMPL_OFFSET>,
            SyncAsync: SyncAsync::<Impl, IMPL_OFFSET>,
            SyncStatusChanged: SyncStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveSyncStatusChanged: RemoveSyncStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentCalendarSyncManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentCalendarSyncManager2Impl: Sized {
    fn SetStatus(&mut self, value: AppointmentCalendarSyncStatus) -> ::windows::core::Result<()>;
    fn SetLastSuccessfulSyncTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetLastAttemptedSyncTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentCalendarSyncManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentCalendarSyncManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentCalendarSyncManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentCalendarSyncManager2Vtbl {
        unsafe extern "system" fn SetStatus<Impl: IAppointmentCalendarSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentCalendarSyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn SetLastSuccessfulSyncTime<Impl: IAppointmentCalendarSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastSuccessfulSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetLastAttemptedSyncTime<Impl: IAppointmentCalendarSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastAttemptedSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentCalendarSyncManager2, BASE_OFFSET>(),
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            SetLastSuccessfulSyncTime: SetLastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            SetLastAttemptedSyncTime: SetLastAttemptedSyncTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentCalendarSyncManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentConflictResultImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<AppointmentConflictType>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentConflictResult {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentConflictResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentConflictResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentConflictResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentConflictResultVtbl {
        unsafe extern "system" fn Type<Impl: IAppointmentConflictResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentConflictType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IAppointmentConflictResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentConflictResult, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentConflictResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointmentExceptionImpl: Sized {
    fn Appointment(&mut self) -> ::windows::core::Result<Appointment>;
    fn ExceptionProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IsDeleted(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentException {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentException";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointmentExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentExceptionVtbl {
        unsafe extern "system" fn Appointment<Impl: IAppointmentExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appointment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExceptionProperties<Impl: IAppointmentExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExceptionProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeleted<Impl: IAppointmentExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeleted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentException, BASE_OFFSET>(),
            Appointment: Appointment::<Impl, IMPL_OFFSET>,
            ExceptionProperties: ExceptionProperties::<Impl, IMPL_OFFSET>,
            IsDeleted: IsDeleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentException as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentInviteeImpl: Sized + IAppointmentParticipantImpl {
    fn Role(&mut self) -> ::windows::core::Result<AppointmentParticipantRole>;
    fn SetRole(&mut self, value: AppointmentParticipantRole) -> ::windows::core::Result<()>;
    fn Response(&mut self) -> ::windows::core::Result<AppointmentParticipantResponse>;
    fn SetResponse(&mut self, value: AppointmentParticipantResponse) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentInvitee {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentInvitee";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentInviteeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentInviteeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentInviteeVtbl {
        unsafe extern "system" fn Role<Impl: IAppointmentInviteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRole<Impl: IAppointmentInviteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentParticipantRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRole(value).into()
        }
        unsafe extern "system" fn Response<Impl: IAppointmentInviteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResponse<Impl: IAppointmentInviteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResponse(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentInvitee, BASE_OFFSET>(),
            Role: Role::<Impl, IMPL_OFFSET>,
            SetRole: SetRole::<Impl, IMPL_OFFSET>,
            Response: Response::<Impl, IMPL_OFFSET>,
            SetResponse: SetResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentInvitee as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait IAppointmentManagerForUserImpl: Sized {
    fn ShowAddAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowAddAppointmentWithPlacementAsync(&mut self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentAsync(&mut self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAsync(&mut self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAndDateAsync(&mut self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowRemoveAppointmentAsync(&mut self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAsync(&mut self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAndDateAsync(&mut self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowTimeFrameAsync(&mut self, timetoshow: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsAsync(&mut self, appointmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsWithDateAsync(&mut self, appointmentid: &::windows::core::HSTRING, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowEditNewAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestStoreAsync(&mut self, options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>>;
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentManagerForUser";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IAppointmentManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentManagerForUserVtbl {
        unsafe extern "system" fn ShowAddAppointmentAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAppointmentWithPlacementAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAppointmentWithPlacementAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentWithPlacementAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentWithPlacementAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentWithPlacementAndDateAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentWithPlacementAndDateAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentWithPlacementAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentWithPlacementAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentWithPlacementAndDateAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentWithPlacementAndDateAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowTimeFrameAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowTimeFrameAsync(&*(&timetoshow as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAppointmentDetailsAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetailsAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAppointmentDetailsWithDateAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetailsWithDateAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowEditNewAppointmentAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowEditNewAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStoreAsync<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAppointmentManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentManagerForUser, BASE_OFFSET>(),
            ShowAddAppointmentAsync: ShowAddAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowAddAppointmentWithPlacementAsync: ShowAddAppointmentWithPlacementAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentAsync: ShowReplaceAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentWithPlacementAsync: ShowReplaceAppointmentWithPlacementAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentWithPlacementAndDateAsync: ShowReplaceAppointmentWithPlacementAndDateAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentAsync: ShowRemoveAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentWithPlacementAsync: ShowRemoveAppointmentWithPlacementAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentWithPlacementAndDateAsync: ShowRemoveAppointmentWithPlacementAndDateAsync::<Impl, IMPL_OFFSET>,
            ShowTimeFrameAsync: ShowTimeFrameAsync::<Impl, IMPL_OFFSET>,
            ShowAppointmentDetailsAsync: ShowAppointmentDetailsAsync::<Impl, IMPL_OFFSET>,
            ShowAppointmentDetailsWithDateAsync: ShowAppointmentDetailsWithDateAsync::<Impl, IMPL_OFFSET>,
            ShowEditNewAppointmentAsync: ShowEditNewAppointmentAsync::<Impl, IMPL_OFFSET>,
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait IAppointmentManagerStaticsImpl: Sized {
    fn ShowAddAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowAddAppointmentWithPlacementAsync(&mut self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentAsync(&mut self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAsync(&mut self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAndDateAsync(&mut self, appointmentid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowRemoveAppointmentAsync(&mut self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAsync(&mut self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAndDateAsync(&mut self, appointmentid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowTimeFrameAsync(&mut self, timetoshow: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IAppointmentManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentManagerStaticsVtbl {
        unsafe extern "system" fn ShowAddAppointmentAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAppointmentWithPlacementAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAppointmentWithPlacementAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentWithPlacementAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentWithPlacementAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentWithPlacementAndDateAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentWithPlacementAndDateAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentWithPlacementAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentWithPlacementAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentWithPlacementAndDateAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentWithPlacementAndDateAsync(
                &*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowTimeFrameAsync<Impl: IAppointmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowTimeFrameAsync(&*(&timetoshow as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentManagerStatics, BASE_OFFSET>(),
            ShowAddAppointmentAsync: ShowAddAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowAddAppointmentWithPlacementAsync: ShowAddAppointmentWithPlacementAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentAsync: ShowReplaceAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentWithPlacementAsync: ShowReplaceAppointmentWithPlacementAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentWithPlacementAndDateAsync: ShowReplaceAppointmentWithPlacementAndDateAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentAsync: ShowRemoveAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentWithPlacementAsync: ShowRemoveAppointmentWithPlacementAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentWithPlacementAndDateAsync: ShowRemoveAppointmentWithPlacementAndDateAsync::<Impl, IMPL_OFFSET>,
            ShowTimeFrameAsync: ShowTimeFrameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentManagerStatics2Impl: Sized {
    fn ShowAppointmentDetailsAsync(&mut self, appointmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsWithDateAsync(&mut self, appointmentid: &::windows::core::HSTRING, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowEditNewAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestStoreAsync(&mut self, options: AppointmentStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentManagerStatics2Vtbl {
        unsafe extern "system" fn ShowAppointmentDetailsAsync<Impl: IAppointmentManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetailsAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAppointmentDetailsWithDateAsync<Impl: IAppointmentManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetailsWithDateAsync(&*(&appointmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowEditNewAppointmentAsync<Impl: IAppointmentManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowEditNewAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStoreAsync<Impl: IAppointmentManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentManagerStatics2, BASE_OFFSET>(),
            ShowAppointmentDetailsAsync: ShowAppointmentDetailsAsync::<Impl, IMPL_OFFSET>,
            ShowAppointmentDetailsWithDateAsync: ShowAppointmentDetailsWithDateAsync::<Impl, IMPL_OFFSET>,
            ShowEditNewAppointmentAsync: ShowEditNewAppointmentAsync::<Impl, IMPL_OFFSET>,
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppointmentManagerStatics3Impl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppointmentManagerForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppointmentManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentManagerStatics3Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IAppointmentManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentManagerStatics3, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentManagerStatics3 as ::windows::core::Interface>::IID
    }
}
pub trait IAppointmentParticipantImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Address(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAddress(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppointmentParticipant {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentParticipant";
}
impl IAppointmentParticipantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentParticipantImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentParticipantVtbl {
        unsafe extern "system" fn DisplayName<Impl: IAppointmentParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IAppointmentParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Address<Impl: IAppointmentParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IAppointmentParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentParticipant, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            SetAddress: SetAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentParticipant as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointmentPropertiesStaticsImpl: Sized {
    fn Subject(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StartTime(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Duration(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reminder(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BusyStatus(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sensitivity(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OriginalStartTime(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsResponseRequested(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowNewTimeProposal(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllDay(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Details(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnlineMeetingLink(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReplyTime(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Organizer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserResponse(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasInvitees(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsCanceledMeeting(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOrganizedByUser(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recurrence(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Invitees(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentPropertiesStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointmentPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentPropertiesStaticsVtbl {
        unsafe extern "system" fn Subject<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reminder<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reminder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BusyStatus<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sensitivity<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalStartTime<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsResponseRequested<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResponseRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowNewTimeProposal<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowNewTimeProposal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllDay<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllDay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Details<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnlineMeetingLink<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnlineMeetingLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplyTime<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Organizer<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Organizer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserResponse<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInvitees<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasInvitees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCanceledMeeting<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCanceledMeeting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOrganizedByUser<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOrganizedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recurrence<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recurrence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitees<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultProperties<Impl: IAppointmentPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentPropertiesStatics, BASE_OFFSET>(),
            Subject: Subject::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            Reminder: Reminder::<Impl, IMPL_OFFSET>,
            BusyStatus: BusyStatus::<Impl, IMPL_OFFSET>,
            Sensitivity: Sensitivity::<Impl, IMPL_OFFSET>,
            OriginalStartTime: OriginalStartTime::<Impl, IMPL_OFFSET>,
            IsResponseRequested: IsResponseRequested::<Impl, IMPL_OFFSET>,
            AllowNewTimeProposal: AllowNewTimeProposal::<Impl, IMPL_OFFSET>,
            AllDay: AllDay::<Impl, IMPL_OFFSET>,
            Details: Details::<Impl, IMPL_OFFSET>,
            OnlineMeetingLink: OnlineMeetingLink::<Impl, IMPL_OFFSET>,
            ReplyTime: ReplyTime::<Impl, IMPL_OFFSET>,
            Organizer: Organizer::<Impl, IMPL_OFFSET>,
            UserResponse: UserResponse::<Impl, IMPL_OFFSET>,
            HasInvitees: HasInvitees::<Impl, IMPL_OFFSET>,
            IsCanceledMeeting: IsCanceledMeeting::<Impl, IMPL_OFFSET>,
            IsOrganizedByUser: IsOrganizedByUser::<Impl, IMPL_OFFSET>,
            Recurrence: Recurrence::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Invitees: Invitees::<Impl, IMPL_OFFSET>,
            DefaultProperties: DefaultProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointmentPropertiesStatics2Impl: Sized + IAppointmentPropertiesStaticsImpl {
    fn ChangeNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteChangeNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetailsKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentPropertiesStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointmentPropertiesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentPropertiesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentPropertiesStatics2Vtbl {
        unsafe extern "system" fn ChangeNumber<Impl: IAppointmentPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteChangeNumber<Impl: IAppointmentPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailsKind<Impl: IAppointmentPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailsKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentPropertiesStatics2, BASE_OFFSET>(),
            ChangeNumber: ChangeNumber::<Impl, IMPL_OFFSET>,
            RemoteChangeNumber: RemoteChangeNumber::<Impl, IMPL_OFFSET>,
            DetailsKind: DetailsKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentRecurrenceImpl: Sized {
    fn Unit(&mut self) -> ::windows::core::Result<AppointmentRecurrenceUnit>;
    fn SetUnit(&mut self, value: AppointmentRecurrenceUnit) -> ::windows::core::Result<()>;
    fn Occurrences(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetOccurrences(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Until(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetUntil(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<u32>;
    fn SetInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DaysOfWeek(&mut self) -> ::windows::core::Result<AppointmentDaysOfWeek>;
    fn SetDaysOfWeek(&mut self, value: AppointmentDaysOfWeek) -> ::windows::core::Result<()>;
    fn WeekOfMonth(&mut self) -> ::windows::core::Result<AppointmentWeekOfMonth>;
    fn SetWeekOfMonth(&mut self, value: AppointmentWeekOfMonth) -> ::windows::core::Result<()>;
    fn Month(&mut self) -> ::windows::core::Result<u32>;
    fn SetMonth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Day(&mut self) -> ::windows::core::Result<u32>;
    fn SetDay(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentRecurrence {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentRecurrence";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentRecurrenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentRecurrenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentRecurrenceVtbl {
        unsafe extern "system" fn Unit<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentRecurrenceUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnit<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentRecurrenceUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnit(value).into()
        }
        unsafe extern "system" fn Occurrences<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occurrences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOccurrences<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOccurrences(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Until<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Until() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntil<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUntil(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Interval<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(value).into()
        }
        unsafe extern "system" fn DaysOfWeek<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDaysOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaysOfWeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysOfWeek<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentDaysOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysOfWeek(value).into()
        }
        unsafe extern "system" fn WeekOfMonth<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentWeekOfMonth) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekOfMonth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeekOfMonth<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppointmentWeekOfMonth) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeekOfMonth(value).into()
        }
        unsafe extern "system" fn Month<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Month() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonth(value).into()
        }
        unsafe extern "system" fn Day<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Day() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Impl: IAppointmentRecurrenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDay(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentRecurrence, BASE_OFFSET>(),
            Unit: Unit::<Impl, IMPL_OFFSET>,
            SetUnit: SetUnit::<Impl, IMPL_OFFSET>,
            Occurrences: Occurrences::<Impl, IMPL_OFFSET>,
            SetOccurrences: SetOccurrences::<Impl, IMPL_OFFSET>,
            Until: Until::<Impl, IMPL_OFFSET>,
            SetUntil: SetUntil::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            DaysOfWeek: DaysOfWeek::<Impl, IMPL_OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Impl, IMPL_OFFSET>,
            WeekOfMonth: WeekOfMonth::<Impl, IMPL_OFFSET>,
            SetWeekOfMonth: SetWeekOfMonth::<Impl, IMPL_OFFSET>,
            Month: Month::<Impl, IMPL_OFFSET>,
            SetMonth: SetMonth::<Impl, IMPL_OFFSET>,
            Day: Day::<Impl, IMPL_OFFSET>,
            SetDay: SetDay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentRecurrence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentRecurrence2Impl: Sized + IAppointmentRecurrenceImpl {
    fn RecurrenceType(&mut self) -> ::windows::core::Result<RecurrenceType>;
    fn TimeZone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTimeZone(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentRecurrence2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentRecurrence2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentRecurrence2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentRecurrence2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentRecurrence2Vtbl {
        unsafe extern "system" fn RecurrenceType<Impl: IAppointmentRecurrence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RecurrenceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecurrenceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeZone<Impl: IAppointmentRecurrence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeZone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeZone<Impl: IAppointmentRecurrence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeZone(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentRecurrence2, BASE_OFFSET>(),
            RecurrenceType: RecurrenceType::<Impl, IMPL_OFFSET>,
            TimeZone: TimeZone::<Impl, IMPL_OFFSET>,
            SetTimeZone: SetTimeZone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentRecurrence2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppointmentRecurrence3Impl: Sized + IAppointmentRecurrenceImpl + IAppointmentRecurrence2Impl {
    fn CalendarIdentifier(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentRecurrence3 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentRecurrence3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppointmentRecurrence3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentRecurrence3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentRecurrence3Vtbl {
        unsafe extern "system" fn CalendarIdentifier<Impl: IAppointmentRecurrence3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalendarIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentRecurrence3, BASE_OFFSET>(),
            CalendarIdentifier: CalendarIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentRecurrence3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait IAppointmentStoreImpl: Sized {
    fn ChangeTracker(&mut self) -> ::windows::core::Result<AppointmentStoreChangeTracker>;
    fn CreateAppointmentCalendarAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>;
    fn GetAppointmentCalendarAsync(&mut self, calendarid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>;
    fn GetAppointmentAsync(&mut self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn GetAppointmentInstanceAsync(&mut self, localid: &::windows::core::HSTRING, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Appointment>>;
    fn FindAppointmentCalendarsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>;
    fn FindAppointmentCalendarsAsyncWithOptions(&mut self, options: FindAppointmentCalendarsOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>>;
    fn FindAppointmentsAsync(&mut self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindAppointmentsAsyncWithOptions(&mut self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan, options: &::core::option::Option<FindAppointmentsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>;
    fn FindConflictAsync(&mut self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>;
    fn FindConflictAsyncWithInstanceStart(&mut self, appointment: &::core::option::Option<Appointment>, instancestarttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>;
    fn MoveAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>, destinationcalendar: &::core::option::Option<AppointmentCalendar>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentAsync(&mut self, localid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowReplaceAppointmentWithPlacementAndDateAsync(&mut self, localid: &::windows::core::HSTRING, appointment: &::core::option::Option<Appointment>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowRemoveAppointmentAsync(&mut self, localid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowRemoveAppointmentWithPlacementAndDateAsync(&mut self, localid: &::windows::core::HSTRING, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowAppointmentDetailsAsync(&mut self, localid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAppointmentDetailsWithDateAsync(&mut self, localid: &::windows::core::HSTRING, instancestartdate: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowEditNewAppointmentAsync(&mut self, appointment: &::core::option::Option<Appointment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn FindLocalIdsFromRoamingIdAsync(&mut self, roamingid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentStore {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStore";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IAppointmentStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreVtbl {
        unsafe extern "system" fn ChangeTracker<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeTracker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAppointmentCalendarAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAppointmentCalendarAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppointmentCalendarAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calendarid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppointmentCalendarAsync(&*(&calendarid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppointmentAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppointmentAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppointmentInstanceAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppointmentInstanceAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&instancestarttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppointmentCalendarsAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppointmentCalendarsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppointmentCalendarsAsyncWithOptions<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FindAppointmentCalendarsOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppointmentCalendarsAsyncWithOptions(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppointmentsAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppointmentsAsync(&*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppointmentsAsyncWithOptions<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppointmentsAsyncWithOptions(
                &*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <FindAppointmentsOptions as ::windows::core::Abi>::Abi as *const <FindAppointmentsOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConflictAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindConflictAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConflictAsyncWithInstanceStart<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, instancestarttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindConflictAsyncWithInstanceStart(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&instancestarttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveAppointmentAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, destinationcalendar: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&destinationcalendar as *const <AppointmentCalendar as ::windows::core::Abi>::Abi as *const <AppointmentCalendar as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAppointmentAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowReplaceAppointmentWithPlacementAndDateAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appointment: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowReplaceAppointmentWithPlacementAndDateAsync(
                &*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowRemoveAppointmentWithPlacementAndDateAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowRemoveAppointmentWithPlacementAndDateAsync(
                &*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAppointmentDetailsAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetailsAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAppointmentDetailsWithDateAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetailsWithDateAsync(&*(&localid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&instancestartdate as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowEditNewAppointmentAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowEditNewAppointmentAsync(&*(&appointment as *const <Appointment as ::windows::core::Abi>::Abi as *const <Appointment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindLocalIdsFromRoamingIdAsync<Impl: IAppointmentStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roamingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindLocalIdsFromRoamingIdAsync(&*(&roamingid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStore, BASE_OFFSET>(),
            ChangeTracker: ChangeTracker::<Impl, IMPL_OFFSET>,
            CreateAppointmentCalendarAsync: CreateAppointmentCalendarAsync::<Impl, IMPL_OFFSET>,
            GetAppointmentCalendarAsync: GetAppointmentCalendarAsync::<Impl, IMPL_OFFSET>,
            GetAppointmentAsync: GetAppointmentAsync::<Impl, IMPL_OFFSET>,
            GetAppointmentInstanceAsync: GetAppointmentInstanceAsync::<Impl, IMPL_OFFSET>,
            FindAppointmentCalendarsAsync: FindAppointmentCalendarsAsync::<Impl, IMPL_OFFSET>,
            FindAppointmentCalendarsAsyncWithOptions: FindAppointmentCalendarsAsyncWithOptions::<Impl, IMPL_OFFSET>,
            FindAppointmentsAsync: FindAppointmentsAsync::<Impl, IMPL_OFFSET>,
            FindAppointmentsAsyncWithOptions: FindAppointmentsAsyncWithOptions::<Impl, IMPL_OFFSET>,
            FindConflictAsync: FindConflictAsync::<Impl, IMPL_OFFSET>,
            FindConflictAsyncWithInstanceStart: FindConflictAsyncWithInstanceStart::<Impl, IMPL_OFFSET>,
            MoveAppointmentAsync: MoveAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowAddAppointmentAsync: ShowAddAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentAsync: ShowReplaceAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowReplaceAppointmentWithPlacementAndDateAsync: ShowReplaceAppointmentWithPlacementAndDateAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentAsync: ShowRemoveAppointmentAsync::<Impl, IMPL_OFFSET>,
            ShowRemoveAppointmentWithPlacementAndDateAsync: ShowRemoveAppointmentWithPlacementAndDateAsync::<Impl, IMPL_OFFSET>,
            ShowAppointmentDetailsAsync: ShowAppointmentDetailsAsync::<Impl, IMPL_OFFSET>,
            ShowAppointmentDetailsWithDateAsync: ShowAppointmentDetailsWithDateAsync::<Impl, IMPL_OFFSET>,
            ShowEditNewAppointmentAsync: ShowEditNewAppointmentAsync::<Impl, IMPL_OFFSET>,
            FindLocalIdsFromRoamingIdAsync: FindLocalIdsFromRoamingIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait IAppointmentStore2Impl: Sized + IAppointmentStoreImpl {
    fn StoreChanged(&mut self, phandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStoreChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateAppointmentCalendarInAccountAsync(&mut self, name: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStore2";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IAppointmentStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStore2Vtbl {
        unsafe extern "system" fn StoreChanged<Impl: IAppointmentStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreChanged(&*(&phandler as *const <super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStoreChanged<Impl: IAppointmentStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStoreChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateAppointmentCalendarInAccountAsync<Impl: IAppointmentStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAppointmentCalendarInAccountAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStore2, BASE_OFFSET>(),
            StoreChanged: StoreChanged::<Impl, IMPL_OFFSET>,
            RemoveStoreChanged: RemoveStoreChanged::<Impl, IMPL_OFFSET>,
            CreateAppointmentCalendarInAccountAsync: CreateAppointmentCalendarInAccountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStore3Impl: Sized {
    fn GetChangeTracker(&mut self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<AppointmentStoreChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStore3 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStore3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStore3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStore3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStore3Vtbl {
        unsafe extern "system" fn GetChangeTracker<Impl: IAppointmentStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeTracker(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStore3, BASE_OFFSET>(),
            GetChangeTracker: GetChangeTracker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStore3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeImpl: Sized {
    fn Appointment(&mut self) -> ::windows::core::Result<Appointment>;
    fn ChangeType(&mut self) -> ::windows::core::Result<AppointmentStoreChangeType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreChange {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChange";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChangeVtbl {
        unsafe extern "system" fn Appointment<Impl: IAppointmentStoreChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appointment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeType<Impl: IAppointmentStoreChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppointmentStoreChangeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChange, BASE_OFFSET>(),
            Appointment: Appointment::<Impl, IMPL_OFFSET>,
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChange2Impl: Sized + IAppointmentStoreChangeImpl {
    fn AppointmentCalendar(&mut self) -> ::windows::core::Result<AppointmentCalendar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreChange2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChange2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreChange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChange2Vtbl {
        unsafe extern "system" fn AppointmentCalendar<Impl: IAppointmentStoreChange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentCalendar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChange2, BASE_OFFSET>(),
            AppointmentCalendar: AppointmentCalendar::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppointmentStoreChangeReaderImpl: Sized {
    fn ReadBatchAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>>;
    fn AcceptChanges(&mut self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&mut self, lastchangetoaccept: &::core::option::Option<AppointmentStoreChange>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppointmentStoreChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppointmentStoreChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChangeReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChangeReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IAppointmentStoreChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptChanges<Impl: IAppointmentStoreChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChanges().into()
        }
        unsafe extern "system" fn AcceptChangesThrough<Impl: IAppointmentStoreChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchangetoaccept: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChangesThrough(&*(&lastchangetoaccept as *const <AppointmentStoreChange as ::windows::core::Abi>::Abi as *const <AppointmentStoreChange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChangeReader, BASE_OFFSET>(),
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
            AcceptChanges: AcceptChanges::<Impl, IMPL_OFFSET>,
            AcceptChangesThrough: AcceptChangesThrough::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChangeReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeTrackerImpl: Sized {
    fn GetChangeReader(&mut self) -> ::windows::core::Result<AppointmentStoreChangeReader>;
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreChangeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChangeTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChangeTrackerVtbl {
        unsafe extern "system" fn GetChangeReader<Impl: IAppointmentStoreChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IAppointmentStoreChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Reset<Impl: IAppointmentStoreChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChangeTracker, BASE_OFFSET>(),
            GetChangeReader: GetChangeReader::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChangeTracker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangeTracker2Impl: Sized {
    fn IsTracking(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreChangeTracker2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreChangeTracker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChangeTracker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChangeTracker2Vtbl {
        unsafe extern "system" fn IsTracking<Impl: IAppointmentStoreChangeTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTracking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChangeTracker2, BASE_OFFSET>(),
            IsTracking: IsTracking::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChangeTracker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangedDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreChangedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChangedDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChangedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IAppointmentStoreChangedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChangedDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChangedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreChangedEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<AppointmentStoreChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IAppointmentStoreChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreChangedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreNotificationTriggerDetailsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreNotificationTriggerDetailsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreNotificationTriggerDetails, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFindAppointmentsOptionsImpl: Sized {
    fn CalendarIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn FetchProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IncludeHidden(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeHidden(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MaxCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxCount(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFindAppointmentsOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IFindAppointmentsOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFindAppointmentsOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindAppointmentsOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindAppointmentsOptionsVtbl {
        unsafe extern "system" fn CalendarIds<Impl: IFindAppointmentsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalendarIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FetchProperties<Impl: IFindAppointmentsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FetchProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeHidden<Impl: IFindAppointmentsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeHidden<Impl: IFindAppointmentsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeHidden(value).into()
        }
        unsafe extern "system" fn MaxCount<Impl: IFindAppointmentsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCount<Impl: IFindAppointmentsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCount(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFindAppointmentsOptions, BASE_OFFSET>(),
            CalendarIds: CalendarIds::<Impl, IMPL_OFFSET>,
            FetchProperties: FetchProperties::<Impl, IMPL_OFFSET>,
            IncludeHidden: IncludeHidden::<Impl, IMPL_OFFSET>,
            SetIncludeHidden: SetIncludeHidden::<Impl, IMPL_OFFSET>,
            MaxCount: MaxCount::<Impl, IMPL_OFFSET>,
            SetMaxCount: SetMaxCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindAppointmentsOptions as ::windows::core::Interface>::IID
    }
}

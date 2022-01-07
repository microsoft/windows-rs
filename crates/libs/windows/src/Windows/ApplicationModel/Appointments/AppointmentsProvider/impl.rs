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
impl ::windows::core::RuntimeName for IAddAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.IAddAppointmentOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IAddAppointmentOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddAppointmentOperationImpl, const OFFSET: isize>() -> IAddAppointmentOperationVtbl {
        unsafe extern "system" fn AppointmentInformation<Impl: IAddAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePackageFamilyName<Impl: IAddAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IAddAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&itemid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReportCanceled<Impl: IAddAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCanceled().into()
        }
        unsafe extern "system" fn ReportError<Impl: IAddAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissUI<Impl: IAddAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissUI().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAddAppointmentOperation>, ::windows::core::GetTrustLevel, AppointmentInformation::<Impl, OFFSET>, SourcePackageFamilyName::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>, ReportCanceled::<Impl, OFFSET>, ReportError::<Impl, OFFSET>, DismissUI::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentsProviderLaunchActionVerbsStaticsImpl: Sized {
    fn AddAppointment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReplaceAppointment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoveAppointment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowTimeFrame(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentsProviderLaunchActionVerbsStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.IAppointmentsProviderLaunchActionVerbsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentsProviderLaunchActionVerbsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderLaunchActionVerbsStaticsImpl, const OFFSET: isize>() -> IAppointmentsProviderLaunchActionVerbsStaticsVtbl {
        unsafe extern "system" fn AddAppointment<Impl: IAppointmentsProviderLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAppointment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceAppointment<Impl: IAppointmentsProviderLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceAppointment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppointment<Impl: IAppointmentsProviderLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAppointment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowTimeFrame<Impl: IAppointmentsProviderLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowTimeFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderLaunchActionVerbsStatics>, ::windows::core::GetTrustLevel, AddAppointment::<Impl, OFFSET>, ReplaceAppointment::<Impl, OFFSET>, RemoveAppointment::<Impl, OFFSET>, ShowTimeFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentsProviderLaunchActionVerbsStatics2Impl: Sized {
    fn ShowAppointmentDetails(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentsProviderLaunchActionVerbsStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.IAppointmentsProviderLaunchActionVerbsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentsProviderLaunchActionVerbsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderLaunchActionVerbsStatics2Impl, const OFFSET: isize>() -> IAppointmentsProviderLaunchActionVerbsStatics2Vtbl {
        unsafe extern "system" fn ShowAppointmentDetails<Impl: IAppointmentsProviderLaunchActionVerbsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAppointmentDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderLaunchActionVerbsStatics2>, ::windows::core::GetTrustLevel, ShowAppointmentDetails::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IRemoveAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.IRemoveAppointmentOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoveAppointmentOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>() -> IRemoveAppointmentOperationVtbl {
        unsafe extern "system" fn AppointmentId<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceStartDate<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceStartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePackageFamilyName<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        unsafe extern "system" fn ReportCanceled<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCanceled().into()
        }
        unsafe extern "system" fn ReportError<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissUI<Impl: IRemoveAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissUI().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRemoveAppointmentOperation>,
            ::windows::core::GetTrustLevel,
            AppointmentId::<Impl, OFFSET>,
            InstanceStartDate::<Impl, OFFSET>,
            SourcePackageFamilyName::<Impl, OFFSET>,
            ReportCompleted::<Impl, OFFSET>,
            ReportCanceled::<Impl, OFFSET>,
            ReportError::<Impl, OFFSET>,
            DismissUI::<Impl, OFFSET>,
        )
    }
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReplaceAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.IReplaceAppointmentOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IReplaceAppointmentOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>() -> IReplaceAppointmentOperationVtbl {
        unsafe extern "system" fn AppointmentId<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppointmentInformation<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceStartDate<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceStartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePackageFamilyName<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&itemid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReportCanceled<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCanceled().into()
        }
        unsafe extern "system" fn ReportError<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissUI<Impl: IReplaceAppointmentOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissUI().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IReplaceAppointmentOperation>,
            ::windows::core::GetTrustLevel,
            AppointmentId::<Impl, OFFSET>,
            AppointmentInformation::<Impl, OFFSET>,
            InstanceStartDate::<Impl, OFFSET>,
            SourcePackageFamilyName::<Impl, OFFSET>,
            ReportCompleted::<Impl, OFFSET>,
            ReportCanceled::<Impl, OFFSET>,
            ReportError::<Impl, OFFSET>,
            DismissUI::<Impl, OFFSET>,
        )
    }
}

pub trait IActivatedEventArgsImpl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<ActivationKind>;
    fn PreviousExecutionState(&mut self) -> ::windows::core::Result<ApplicationExecutionState>;
    fn SplashScreen(&mut self) -> ::windows::core::Result<SplashScreen>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
impl IActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivatedEventArgsVtbl {
        unsafe extern "system" fn Kind<Impl: IActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousExecutionState<Impl: IActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousExecutionState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SplashScreen<Impl: IActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SplashScreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivatedEventArgs, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            PreviousExecutionState: PreviousExecutionState::<Impl, IMPL_OFFSET>,
            SplashScreen: SplashScreen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IActivatedEventArgsWithUserImpl: Sized + IActivatedEventArgsImpl {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
#[cfg(feature = "System")]
impl IActivatedEventArgsWithUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedEventArgsWithUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivatedEventArgsWithUserVtbl {
        unsafe extern "system" fn User<Impl: IActivatedEventArgsWithUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IActivatedEventArgsWithUser, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedEventArgsWithUser as ::windows::core::Interface>::IID
    }
}
pub trait IApplicationViewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CurrentlyShownApplicationViewId(&mut self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
impl IApplicationViewActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewActivatedEventArgsVtbl {
        unsafe extern "system" fn CurrentlyShownApplicationViewId<Impl: IApplicationViewActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentlyShownApplicationViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewActivatedEventArgs, BASE_OFFSET>(),
            CurrentlyShownApplicationViewId: CurrentlyShownApplicationViewId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IAppointmentsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
}
impl IAppointmentsProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentsProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Verb<Impl: IAppointmentsProviderActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentsProviderActivatedEventArgs, BASE_OFFSET>(),
            Verb: Verb::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn AddAppointmentOperation(&mut self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows::core::RuntimeName for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderAddAppointmentActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl {
        unsafe extern "system" fn AddAppointmentOperation<Impl: IAppointmentsProviderAddAppointmentActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentsProviderAddAppointmentActivatedEventArgs, BASE_OFFSET>(),
            AddAppointmentOperation: AddAppointmentOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn RemoveAppointmentOperation(&mut self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows::core::RuntimeName for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl {
        unsafe extern "system" fn RemoveAppointmentOperation<Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentsProviderRemoveAppointmentActivatedEventArgs, BASE_OFFSET>(),
            RemoveAppointmentOperation: RemoveAppointmentOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn ReplaceAppointmentOperation(&mut self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows::core::RuntimeName for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl {
        unsafe extern "system" fn ReplaceAppointmentOperation<Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentsProviderReplaceAppointmentActivatedEventArgs, BASE_OFFSET>(),
            ReplaceAppointmentOperation: ReplaceAppointmentOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn InstanceStartDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "Foundation")]
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl {
        unsafe extern "system" fn InstanceStartDate<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalId<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoamingId<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, BASE_OFFSET>(),
            InstanceStartDate: InstanceStartDate::<Impl, IMPL_OFFSET>,
            LocalId: LocalId::<Impl, IMPL_OFFSET>,
            RoamingId: RoamingId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn TimeToShow(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
#[cfg(feature = "Foundation")]
impl IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl {
        unsafe extern "system" fn TimeToShow<Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeToShow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentsProviderShowTimeFrameActivatedEventArgs, BASE_OFFSET>(),
            TimeToShow: TimeToShow::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Background")]
pub trait IBackgroundActivatedEventArgsImpl: Sized {
    fn TaskInstance(&mut self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance>;
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::RuntimeName for IBackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Background")]
impl IBackgroundActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundActivatedEventArgsVtbl {
        unsafe extern "system" fn TaskInstance<Impl: IBackgroundActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundActivatedEventArgs, BASE_OFFSET>(),
            TaskInstance: TaskInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IBarcodeScannerPreviewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ConnectionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
}
impl IBarcodeScannerPreviewActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerPreviewActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerPreviewActivatedEventArgsVtbl {
        unsafe extern "system" fn ConnectionId<Impl: IBarcodeScannerPreviewActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerPreviewActivatedEventArgs, BASE_OFFSET>(),
            ConnectionId: ConnectionId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerPreviewActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Provider")]
pub trait ICachedFileUpdaterActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CachedFileUpdaterUI(&mut self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
#[cfg(feature = "Storage_Provider")]
impl ::windows::core::RuntimeName for ICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "Storage_Provider")]
impl ICachedFileUpdaterActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileUpdaterActivatedEventArgsVtbl {
        unsafe extern "system" fn CachedFileUpdaterUI<Impl: ICachedFileUpdaterActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFileUpdaterUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileUpdaterActivatedEventArgs, BASE_OFFSET>(),
            CachedFileUpdaterUI: CachedFileUpdaterUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ICameraSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn VideoDeviceController(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn VideoDeviceExtension(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
}
impl ICameraSettingsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraSettingsActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraSettingsActivatedEventArgsVtbl {
        unsafe extern "system" fn VideoDeviceController<Impl: ICameraSettingsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceExtension<Impl: ICameraSettingsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraSettingsActivatedEventArgs, BASE_OFFSET>(),
            VideoDeviceController: VideoDeviceController::<Impl, IMPL_OFFSET>,
            VideoDeviceExtension: VideoDeviceExtension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraSettingsActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ICommandLineActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&mut self) -> ::windows::core::Result<CommandLineActivationOperation>;
}
impl ::windows::core::RuntimeName for ICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
}
impl ICommandLineActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandLineActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandLineActivatedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: ICommandLineActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandLineActivatedEventArgs, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandLineActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICommandLineActivationOperationImpl: Sized {
    fn Arguments(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentDirectoryPath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitCode(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ExitCode(&mut self) -> ::windows::core::Result<i32>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivationOperation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommandLineActivationOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandLineActivationOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandLineActivationOperationVtbl {
        unsafe extern "system" fn Arguments<Impl: ICommandLineActivationOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDirectoryPath<Impl: ICommandLineActivationOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDirectoryPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitCode<Impl: ICommandLineActivationOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitCode(value).into()
        }
        unsafe extern "system" fn ExitCode<Impl: ICommandLineActivationOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ICommandLineActivationOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandLineActivationOperation, BASE_OFFSET>(),
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
            CurrentDirectoryPath: CurrentDirectoryPath::<Impl, IMPL_OFFSET>,
            SetExitCode: SetExitCode::<Impl, IMPL_OFFSET>,
            ExitCode: ExitCode::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandLineActivationOperation as ::windows::core::Interface>::IID
    }
}
pub trait IContactActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
}
impl IContactActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactActivatedEventArgsVtbl {
        unsafe extern "system" fn Verb<Impl: IContactActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContactActivatedEventArgs, BASE_OFFSET>(), Verb: Verb::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactCallActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCallActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactCallActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactCallActivatedEventArgs, BASE_OFFSET>(),
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            ServiceUserId: ServiceUserId::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactCallActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMapActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn Address(&mut self) -> ::windows::core::Result<super::Contacts::ContactAddress>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMapActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactMapActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactMapActivatedEventArgsVtbl {
        unsafe extern "system" fn Address<Impl: IContactMapActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Contact<Impl: IContactMapActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactMapActivatedEventArgs, BASE_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactMapActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMessageActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMessageActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactMessageActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactMessageActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactMessageActivatedEventArgs, BASE_OFFSET>(),
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            ServiceUserId: ServiceUserId::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactMessageActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPanelActivatedEventArgsImpl: Sized {
    fn ContactPanel(&mut self) -> ::windows::core::Result<super::Contacts::ContactPanel>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPanelActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPanelActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPanelActivatedEventArgsVtbl {
        unsafe extern "system" fn ContactPanel<Impl: IContactPanelActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactPanel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactPanelActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPanelActivatedEventArgs, BASE_OFFSET>(),
            ContactPanel: ContactPanel::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPanelActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub trait IContactPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContactPickerUI(&mut self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI>;
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl ::windows::core::RuntimeName for IContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl IContactPickerActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPickerActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPickerActivatedEventArgsVtbl {
        unsafe extern "system" fn ContactPickerUI<Impl: IContactPickerActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactPickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPickerActivatedEventArgs, BASE_OFFSET>(),
            ContactPickerUI: ContactPickerUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPickerActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPostActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPostActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPostActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPostActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPostActivatedEventArgs, BASE_OFFSET>(),
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            ServiceUserId: ServiceUserId::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPostActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactVideoCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactVideoCallActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactVideoCallActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactVideoCallActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactVideoCallActivatedEventArgs, BASE_OFFSET>(),
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            ServiceUserId: ServiceUserId::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactVideoCallActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IContactsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
}
impl IContactsProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactsProviderActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactsProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Verb<Impl: IContactsProviderActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContactsProviderActivatedEventArgs, BASE_OFFSET>(), Verb: Verb::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactsProviderActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IContinuationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContinuationData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IContinuationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
impl IContinuationActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinuationActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinuationActivatedEventArgsVtbl {
        unsafe extern "system" fn ContinuationData<Impl: IContinuationActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContinuationActivatedEventArgs, BASE_OFFSET>(),
            ContinuationData: ContinuationData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinuationActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDeviceActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformationId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Verb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
}
impl IDeviceActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceActivatedEventArgsVtbl {
        unsafe extern "system" fn DeviceInformationId<Impl: IDeviceActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Impl: IDeviceActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceActivatedEventArgs, BASE_OFFSET>(),
            DeviceInformationId: DeviceInformationId::<Impl, IMPL_OFFSET>,
            Verb: Verb::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Enumeration")]
pub trait IDevicePairingActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformation(&mut self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(feature = "Devices_Enumeration")]
impl ::windows::core::RuntimeName for IDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
}
#[cfg(feature = "Devices_Enumeration")]
impl IDevicePairingActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePairingActivatedEventArgsVtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IDevicePairingActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePairingActivatedEventArgs, BASE_OFFSET>(),
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDialReceiverActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn AppName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
}
impl IDialReceiverActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialReceiverActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialReceiverActivatedEventArgsVtbl {
        unsafe extern "system" fn AppName<Impl: IDialReceiverActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDialReceiverActivatedEventArgs, BASE_OFFSET>(), AppName: AppName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialReceiverActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
pub trait IFileActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Files(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl ::windows::core::RuntimeName for IFileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl IFileActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileActivatedEventArgsVtbl {
        unsafe extern "system" fn Files<Impl: IFileActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Impl: IFileActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileActivatedEventArgs, BASE_OFFSET>(),
            Files: Files::<Impl, IMPL_OFFSET>,
            Verb: Verb::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyNameImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
}
impl IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileActivatedEventArgsWithCallerPackageFamilyNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IFileActivatedEventArgsWithCallerPackageFamilyNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileActivatedEventArgsWithCallerPackageFamilyName, BASE_OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithCallerPackageFamilyName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Search"))]
pub trait IFileActivatedEventArgsWithNeighboringFilesImpl: Sized + IActivatedEventArgsImpl + IFileActivatedEventArgsImpl {
    fn NeighboringFilesQuery(&mut self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Search"))]
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithNeighboringFiles {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Search"))]
impl IFileActivatedEventArgsWithNeighboringFilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileActivatedEventArgsWithNeighboringFilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileActivatedEventArgsWithNeighboringFilesVtbl {
        unsafe extern "system" fn NeighboringFilesQuery<Impl: IFileActivatedEventArgsWithNeighboringFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringFilesQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileActivatedEventArgsWithNeighboringFiles, BASE_OFFSET>(),
            NeighboringFilesQuery: NeighboringFilesQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithNeighboringFiles as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileOpenPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileOpenPickerUI(&mut self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileOpenPickerActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerActivatedEventArgsVtbl {
        unsafe extern "system" fn FileOpenPickerUI<Impl: IFileOpenPickerActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileOpenPickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPickerActivatedEventArgs, BASE_OFFSET>(),
            FileOpenPickerUI: FileOpenPickerUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IFileOpenPickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
}
impl IFileOpenPickerActivatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerActivatedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerActivatedEventArgs2Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IFileOpenPickerActivatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPickerActivatedEventArgs2, BASE_OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileOpenPickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn Files(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFileOpenPickerContinuationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerContinuationEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerContinuationEventArgsVtbl {
        unsafe extern "system" fn Files<Impl: IFileOpenPickerContinuationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPickerContinuationEventArgs, BASE_OFFSET>(), Files: Files::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerContinuationEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileSavePickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileSavePickerUI(&mut self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileSavePickerActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePickerActivatedEventArgsVtbl {
        unsafe extern "system" fn FileSavePickerUI<Impl: IFileSavePickerActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSavePickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePickerActivatedEventArgs, BASE_OFFSET>(),
            FileSavePickerUI: FileSavePickerUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IFileSavePickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnterpriseId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
}
impl IFileSavePickerActivatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerActivatedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePickerActivatedEventArgs2Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IFileSavePickerActivatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseId<Impl: IFileSavePickerActivatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePickerActivatedEventArgs2, BASE_OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Impl, IMPL_OFFSET>,
            EnterpriseId: EnterpriseId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileSavePickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn File(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFileSavePickerContinuationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerContinuationEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePickerContinuationEventArgsVtbl {
        unsafe extern "system" fn File<Impl: IFileSavePickerContinuationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePickerContinuationEventArgs, BASE_OFFSET>(), File: File::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerContinuationEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFolderPickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn Folder(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFolderPickerContinuationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPickerContinuationEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPickerContinuationEventArgsVtbl {
        unsafe extern "system" fn Folder<Impl: IFolderPickerContinuationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFolderPickerContinuationEventArgs, BASE_OFFSET>(), Folder: Folder::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPickerContinuationEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ILaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Arguments(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TileId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
}
impl ILaunchActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILaunchActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILaunchActivatedEventArgsVtbl {
        unsafe extern "system" fn Arguments<Impl: ILaunchActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileId<Impl: ILaunchActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILaunchActivatedEventArgs, BASE_OFFSET>(),
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
            TileId: TileId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ILaunchActivatedEventArgs2Impl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn TileActivatedInfo(&mut self) -> ::windows::core::Result<TileActivatedInfo>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
impl ILaunchActivatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILaunchActivatedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILaunchActivatedEventArgs2Vtbl {
        unsafe extern "system" fn TileActivatedInfo<Impl: ILaunchActivatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileActivatedInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILaunchActivatedEventArgs2, BASE_OFFSET>(),
            TileActivatedInfo: TileActivatedInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs2 as ::windows::core::Interface>::IID
    }
}
pub trait ILockScreenActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Info(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
}
impl ILockScreenActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenActivatedEventArgsVtbl {
        unsafe extern "system" fn Info<Impl: ILockScreenActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenActivatedEventArgs, BASE_OFFSET>(), Info: Info::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Calls")]
pub trait ILockScreenCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn CallUI(&mut self) -> ::windows::core::Result<super::Calls::LockScreenCallUI>;
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ::windows::core::RuntimeName for ILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ILockScreenCallActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenCallActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenCallActivatedEventArgsVtbl {
        unsafe extern "system" fn CallUI<Impl: ILockScreenCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenCallActivatedEventArgs, BASE_OFFSET>(), CallUI: CallUI::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenCallActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IPhoneCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
}
impl IPhoneCallActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallActivatedEventArgsVtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneCallActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallActivatedEventArgs, BASE_OFFSET>(), LineId: LineId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IPickerReturnedActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PickerOperationId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IPickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
}
impl IPickerReturnedActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerReturnedActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerReturnedActivatedEventArgsVtbl {
        unsafe extern "system" fn PickerOperationId<Impl: IPickerReturnedActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickerOperationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerReturnedActivatedEventArgs, BASE_OFFSET>(),
            PickerOperationId: PickerOperationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerReturnedActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IPrelaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PrelaunchActivated(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
impl IPrelaunchActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrelaunchActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrelaunchActivatedEventArgsVtbl {
        unsafe extern "system" fn PrelaunchActivated<Impl: IPrelaunchActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrelaunchActivated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrelaunchActivatedEventArgs, BASE_OFFSET>(),
            PrelaunchActivated: PrelaunchActivated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrelaunchActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrint3DWorkflowActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Workflow(&mut self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl ::windows::core::RuntimeName for IPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrint3DWorkflowActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DWorkflowActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DWorkflowActivatedEventArgsVtbl {
        unsafe extern "system" fn Workflow<Impl: IPrint3DWorkflowActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Workflow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DWorkflowActivatedEventArgs, BASE_OFFSET>(),
            Workflow: Workflow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DWorkflowActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrintTaskSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Configuration(&mut self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl ::windows::core::RuntimeName for IPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrintTaskSettingsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskSettingsActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskSettingsActivatedEventArgsVtbl {
        unsafe extern "system" fn Configuration<Impl: IPrintTaskSettingsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskSettingsActivatedEventArgs, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskSettingsActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IProtocolActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
}
#[cfg(feature = "Foundation")]
impl IProtocolActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtocolActivatedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IProtocolActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProtocolActivatedEventArgs, BASE_OFFSET>(), Uri: Uri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
}
#[cfg(feature = "Foundation_Collections")]
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, BASE_OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IProtocolForResultsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ProtocolForResultsOperation(&mut self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "System")]
impl IProtocolForResultsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolForResultsActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtocolForResultsActivatedEventArgsVtbl {
        unsafe extern "system" fn ProtocolForResultsOperation<Impl: IProtocolForResultsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolForResultsOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtocolForResultsActivatedEventArgs, BASE_OFFSET>(),
            ProtocolForResultsOperation: ProtocolForResultsOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolForResultsActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IRestrictedLaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn SharedContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
}
impl IRestrictedLaunchActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRestrictedLaunchActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRestrictedLaunchActivatedEventArgsVtbl {
        unsafe extern "system" fn SharedContext<Impl: IRestrictedLaunchActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharedContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRestrictedLaunchActivatedEventArgs, BASE_OFFSET>(),
            SharedContext: SharedContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRestrictedLaunchActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ISearchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn QueryText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
}
impl ISearchActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchActivatedEventArgsVtbl {
        unsafe extern "system" fn QueryText<Impl: ISearchActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchActivatedEventArgs, BASE_OFFSET>(),
            QueryText: QueryText::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Search")]
pub trait ISearchActivatedEventArgsWithLinguisticDetailsImpl: Sized {
    fn LinguisticDetails(&mut self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "ApplicationModel_Search")]
impl ::windows::core::RuntimeName for ISearchActivatedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
}
#[cfg(feature = "ApplicationModel_Search")]
impl ISearchActivatedEventArgsWithLinguisticDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchActivatedEventArgsWithLinguisticDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchActivatedEventArgsWithLinguisticDetailsVtbl {
        unsafe extern "system" fn LinguisticDetails<Impl: ISearchActivatedEventArgsWithLinguisticDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchActivatedEventArgsWithLinguisticDetails, BASE_OFFSET>(),
            LinguisticDetails: LinguisticDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgsWithLinguisticDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub trait IShareTargetActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ShareOperation(&mut self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl ::windows::core::RuntimeName for IShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl IShareTargetActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareTargetActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareTargetActivatedEventArgsVtbl {
        unsafe extern "system" fn ShareOperation<Impl: IShareTargetActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareTargetActivatedEventArgs, BASE_OFFSET>(),
            ShareOperation: ShareOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareTargetActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISplashScreenImpl: Sized {
    fn ImageLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Dismissed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDismissed(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISplashScreen";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISplashScreenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplashScreenImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplashScreenVtbl {
        unsafe extern "system" fn ImageLocation<Impl: ISplashScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dismissed<Impl: ISplashScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDismissed<Impl: ISplashScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDismissed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplashScreen, BASE_OFFSET>(),
            ImageLocation: ImageLocation::<Impl, IMPL_OFFSET>,
            Dismissed: Dismissed::<Impl, IMPL_OFFSET>,
            RemoveDismissed: RemoveDismissed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplashScreen as ::windows::core::Interface>::IID
    }
}
pub trait IStartupTaskActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn TaskId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
}
impl IStartupTaskActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartupTaskActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStartupTaskActivatedEventArgsVtbl {
        unsafe extern "system" fn TaskId<Impl: IStartupTaskActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStartupTaskActivatedEventArgs, BASE_OFFSET>(), TaskId: TaskId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartupTaskActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait ITileActivatedInfoImpl: Sized {
    fn RecentlyShownNotifications(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ITileActivatedInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ITileActivatedInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileActivatedInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileActivatedInfoVtbl {
        unsafe extern "system" fn RecentlyShownNotifications<Impl: ITileActivatedInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentlyShownNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileActivatedInfo, BASE_OFFSET>(),
            RecentlyShownNotifications: RecentlyShownNotifications::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileActivatedInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IToastNotificationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Argument(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
impl IToastNotificationActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationActivatedEventArgsVtbl {
        unsafe extern "system" fn Argument<Impl: IToastNotificationActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Argument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserInput<Impl: IToastNotificationActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationActivatedEventArgs, BASE_OFFSET>(),
            Argument: Argument::<Impl, IMPL_OFFSET>,
            UserInput: UserInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub trait IUserDataAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&mut self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl ::windows::core::RuntimeName for IUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl IUserDataAccountProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountProviderActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataAccountProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IUserDataAccountProviderActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataAccountProviderActivatedEventArgs, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataAccountProviderActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_ViewManagement")]
pub trait IViewSwitcherProviderImpl: Sized + IActivatedEventArgsImpl {
    fn ViewSwitcher(&mut self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows::core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
#[cfg(feature = "UI_ViewManagement")]
impl IViewSwitcherProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewSwitcherProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewSwitcherProviderVtbl {
        unsafe extern "system" fn ViewSwitcher<Impl: IViewSwitcherProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewSwitcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IViewSwitcherProvider, BASE_OFFSET>(), ViewSwitcher: ViewSwitcher::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewSwitcherProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Media_SpeechRecognition")]
pub trait IVoiceCommandActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Result(&mut self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "Media_SpeechRecognition")]
impl ::windows::core::RuntimeName for IVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "Media_SpeechRecognition")]
impl IVoiceCommandActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandActivatedEventArgsVtbl {
        unsafe extern "system" fn Result<Impl: IVoiceCommandActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandActivatedEventArgs, BASE_OFFSET>(), Result: Result::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Wallet")]
pub trait IWalletActionActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ItemId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActionKind(&mut self) -> ::windows::core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "ApplicationModel_Wallet")]
impl ::windows::core::RuntimeName for IWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Wallet")]
impl IWalletActionActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletActionActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletActionActivatedEventArgsVtbl {
        unsafe extern "system" fn ItemId<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionKind<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionId<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletActionActivatedEventArgs, BASE_OFFSET>(),
            ItemId: ItemId::<Impl, IMPL_OFFSET>,
            ActionKind: ActionKind::<Impl, IMPL_OFFSET>,
            ActionId: ActionId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletActionActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub trait IWebAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&mut self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl ::windows::core::RuntimeName for IWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl IWebAccountProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IWebAccountProviderActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderActivatedEventArgs, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
pub trait IWebAuthenticationBrokerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn WebAuthenticationResult(&mut self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl IWebAuthenticationBrokerContinuationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationBrokerContinuationEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationBrokerContinuationEventArgsVtbl {
        unsafe extern "system" fn WebAuthenticationResult<Impl: IWebAuthenticationBrokerContinuationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationBrokerContinuationEventArgs, BASE_OFFSET>(),
            WebAuthenticationResult: WebAuthenticationResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationBrokerContinuationEventArgs as ::windows::core::Interface>::IID
    }
}

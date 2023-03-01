#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IActivatedEventArgs_Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ActivationKind>;
    fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState>;
    fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
impl IActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: isize>() -> IActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Kind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousExecutionState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreviousExecutionState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SplashScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SplashScreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgs, OFFSET>(),
            Kind: Kind::<Identity, Impl, OFFSET>,
            PreviousExecutionState: PreviousExecutionState::<Identity, Impl, OFFSET>,
            SplashScreen: SplashScreen::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`, `\"implement\"`*"]
#[cfg(feature = "System")]
pub trait IActivatedEventArgsWithUser_Impl: Sized + IActivatedEventArgs_Impl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
#[cfg(feature = "System")]
impl IActivatedEventArgsWithUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgsWithUser_Impl, const OFFSET: isize>() -> IActivatedEventArgsWithUser_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgsWithUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgsWithUser, OFFSET>(), User: User::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedEventArgsWithUser as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IApplicationViewActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
impl IApplicationViewActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IApplicationViewActivatedEventArgs_Impl, const OFFSET: isize>() -> IApplicationViewActivatedEventArgs_Vtbl {
        unsafe extern "system" fn CurrentlyShownApplicationViewId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IApplicationViewActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentlyShownApplicationViewId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IApplicationViewActivatedEventArgs, OFFSET>(),
            CurrentlyShownApplicationViewId: CurrentlyShownApplicationViewId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IAppointmentsProviderActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
}
impl IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Verb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Verb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderActivatedEventArgs, OFFSET>(),
            Verb: Verb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows::core::RuntimeName for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
        unsafe extern "system" fn AddAppointmentOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderAddAppointmentActivatedEventArgs, OFFSET>(),
            AddAppointmentOperation: AddAppointmentOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows::core::RuntimeName for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
        unsafe extern "system" fn RemoveAppointmentOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoveAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderRemoveAppointmentActivatedEventArgs, OFFSET>(),
            RemoveAppointmentOperation: RemoveAppointmentOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows::core::RuntimeName for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ReplaceAppointmentOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReplaceAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderReplaceAppointmentActivatedEventArgs, OFFSET>(),
            ReplaceAppointmentOperation: ReplaceAppointmentOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "Foundation")]
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn InstanceStartDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstanceStartDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoamingId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, OFFSET>(),
            InstanceStartDate: InstanceStartDate::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            RoamingId: RoamingId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
#[cfg(feature = "Foundation")]
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
        unsafe extern "system" fn TimeToShow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TimeToShow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Duration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderShowTimeFrameActivatedEventArgs, OFFSET>(),
            TimeToShow: TimeToShow::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Background\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Background")]
pub trait IBackgroundActivatedEventArgs_Impl: Sized {
    fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance>;
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::RuntimeName for IBackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Background")]
impl IBackgroundActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundActivatedEventArgs_Impl, const OFFSET: isize>() -> IBackgroundActivatedEventArgs_Vtbl {
        unsafe extern "system" fn TaskInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TaskInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IBackgroundActivatedEventArgs, OFFSET>(),
            TaskInstance: TaskInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IBarcodeScannerPreviewActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
}
impl IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: isize>() -> IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ConnectionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IBarcodeScannerPreviewActivatedEventArgs, OFFSET>(),
            ConnectionId: ConnectionId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerPreviewActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Provider\"`, `\"implement\"`*"]
#[cfg(feature = "Storage_Provider")]
pub trait ICachedFileUpdaterActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
#[cfg(feature = "Storage_Provider")]
impl ::windows::core::RuntimeName for ICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "Storage_Provider")]
impl ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: isize>() -> ICachedFileUpdaterActivatedEventArgs_Vtbl {
        unsafe extern "system" fn CachedFileUpdaterUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFileUpdaterUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICachedFileUpdaterActivatedEventArgs, OFFSET>(),
            CachedFileUpdaterUI: CachedFileUpdaterUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait ICameraSettingsActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
}
impl ICameraSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>() -> ICameraSettingsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn VideoDeviceController<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VideoDeviceExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICameraSettingsActivatedEventArgs, OFFSET>(),
            VideoDeviceController: VideoDeviceController::<Identity, Impl, OFFSET>,
            VideoDeviceExtension: VideoDeviceExtension::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraSettingsActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait ICommandLineActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation>;
}
impl ::windows::core::RuntimeName for ICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
}
impl ICommandLineActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandLineActivatedEventArgs_Impl, const OFFSET: isize>() -> ICommandLineActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Operation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandLineActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICommandLineActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandLineActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IContactActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
}
impl IContactActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Verb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Verb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactCallActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactCallActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactCallActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMapActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMapActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactMapActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Address() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactMapActivatedEventArgs, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactMapActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMessageActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMessageActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactMessageActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactMessageActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactMessageActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPanelActivatedEventArgs_Impl: Sized {
    fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPanelActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactPanelActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ContactPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContactPanel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactPanelActivatedEventArgs, OFFSET>(),
            ContactPanel: ContactPanel::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPanelActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts_Provider\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub trait IContactPickerActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI>;
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl ::windows::core::RuntimeName for IContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl IContactPickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPickerActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactPickerActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ContactPickerUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContactPickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactPickerActivatedEventArgs, OFFSET>(),
            ContactPickerUI: ContactPickerUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPickerActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPostActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPostActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactPostActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactPostActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPostActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactVideoCallActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows::core::RuntimeName for IContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactVideoCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactVideoCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactVideoCallActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactVideoCallActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IContactsProviderActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
}
impl IContactsProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactsProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactsProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Verb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactsProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Verb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContactsProviderActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactsProviderActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IContinuationActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IContinuationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
impl IContinuationActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContinuationActivatedEventArgs_Impl, const OFFSET: isize>() -> IContinuationActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ContinuationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContinuationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContinuationActivatedEventArgs, OFFSET>(),
            ContinuationData: ContinuationData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinuationActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IDeviceActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
}
impl IDeviceActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>() -> IDeviceActivatedEventArgs_Vtbl {
        unsafe extern "system" fn DeviceInformationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceInformationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Verb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDeviceActivatedEventArgs, OFFSET>(),
            DeviceInformationId: DeviceInformationId::<Identity, Impl, OFFSET>,
            Verb: Verb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Enumeration\"`, `\"implement\"`*"]
#[cfg(feature = "Devices_Enumeration")]
pub trait IDevicePairingActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(feature = "Devices_Enumeration")]
impl ::windows::core::RuntimeName for IDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
}
#[cfg(feature = "Devices_Enumeration")]
impl IDevicePairingActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDevicePairingActivatedEventArgs_Impl, const OFFSET: isize>() -> IDevicePairingActivatedEventArgs_Vtbl {
        unsafe extern "system" fn DeviceInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDevicePairingActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDevicePairingActivatedEventArgs, OFFSET>(),
            DeviceInformation: DeviceInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IDialReceiverActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
}
impl IDialReceiverActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDialReceiverActivatedEventArgs_Impl, const OFFSET: isize>() -> IDialReceiverActivatedEventArgs_Vtbl {
        unsafe extern "system" fn AppName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDialReceiverActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDialReceiverActivatedEventArgs, OFFSET>(),
            AppName: AppName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialReceiverActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
pub trait IFileActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl ::windows::core::RuntimeName for IFileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl IFileActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgs_Impl, const OFFSET: isize>() -> IFileActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Files<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Files() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Verb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgs, OFFSET>(),
            Files: Files::<Identity, Impl, OFFSET>,
            Verb: Verb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IFileActivatedEventArgsWithCallerPackageFamilyName_Impl: Sized + IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
}
impl IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: isize>() -> IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgsWithCallerPackageFamilyName, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithCallerPackageFamilyName as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
pub trait IFileActivatedEventArgsWithNeighboringFiles_Impl: Sized + IActivatedEventArgs_Impl + IFileActivatedEventArgs_Impl {
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithNeighboringFiles {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: isize>() -> IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
        unsafe extern "system" fn NeighboringFilesQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NeighboringFilesQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgsWithNeighboringFiles, OFFSET>(),
            NeighboringFilesQuery: NeighboringFilesQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithNeighboringFiles as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Pickers_Provider\"`, `\"implement\"`*"]
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileOpenPickerActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileOpenPickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: isize>() -> IFileOpenPickerActivatedEventArgs_Vtbl {
        unsafe extern "system" fn FileOpenPickerUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileOpenPickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerActivatedEventArgs, OFFSET>(),
            FileOpenPickerUI: FileOpenPickerUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IFileOpenPickerActivatedEventArgs2_Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
}
impl IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: isize>() -> IFileOpenPickerActivatedEventArgs2_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerActivatedEventArgs2, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileOpenPickerContinuationEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFileOpenPickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: isize>() -> IFileOpenPickerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn Files<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Files() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerContinuationEventArgs, OFFSET>(),
            Files: Files::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerContinuationEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Pickers_Provider\"`, `\"implement\"`*"]
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileSavePickerActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileSavePickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: isize>() -> IFileSavePickerActivatedEventArgs_Vtbl {
        unsafe extern "system" fn FileSavePickerUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSavePickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileSavePickerActivatedEventArgs, OFFSET>(),
            FileSavePickerUI: FileSavePickerUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IFileSavePickerActivatedEventArgs2_Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
}
impl IFileSavePickerActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>() -> IFileSavePickerActivatedEventArgs2_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileSavePickerActivatedEventArgs2, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
            EnterpriseId: EnterpriseId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileSavePickerContinuationEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFileSavePickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: isize>() -> IFileSavePickerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn File<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.File() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFileSavePickerContinuationEventArgs, OFFSET>(),
            File: File::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerContinuationEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFolderPickerContinuationEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFolderPickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFolderPickerContinuationEventArgs_Impl, const OFFSET: isize>() -> IFolderPickerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn Folder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFolderPickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Folder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFolderPickerContinuationEventArgs, OFFSET>(),
            Folder: Folder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPickerContinuationEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait ILaunchActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
}
impl ILaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> ILaunchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Arguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TileId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs, OFFSET>(),
            Arguments: Arguments::<Identity, Impl, OFFSET>,
            TileId: TileId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait ILaunchActivatedEventArgs2_Impl: Sized + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
impl ILaunchActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILaunchActivatedEventArgs2_Impl, const OFFSET: isize>() -> ILaunchActivatedEventArgs2_Vtbl {
        unsafe extern "system" fn TileActivatedInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILaunchActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TileActivatedInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs2, OFFSET>(),
            TileActivatedInfo: TileActivatedInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait ILockScreenActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
}
impl ILockScreenActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILockScreenActivatedEventArgs_Impl, const OFFSET: isize>() -> ILockScreenActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Info<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILockScreenActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Info() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ILockScreenActivatedEventArgs, OFFSET>(), Info: Info::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Calls\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Calls")]
pub trait ILockScreenCallActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI>;
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ::windows::core::RuntimeName for ILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ILockScreenCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: isize>() -> ILockScreenCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn CallUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ILockScreenCallActivatedEventArgs, OFFSET>(),
            CallUI: CallUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenCallActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IPhoneCallActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
}
impl IPhoneCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhoneCallActivatedEventArgs_Impl, const OFFSET: isize>() -> IPhoneCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn LineId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhoneCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPhoneCallActivatedEventArgs, OFFSET>(), LineId: LineId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IPickerReturnedActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IPickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
}
impl IPickerReturnedActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: isize>() -> IPickerReturnedActivatedEventArgs_Vtbl {
        unsafe extern "system" fn PickerOperationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PickerOperationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPickerReturnedActivatedEventArgs, OFFSET>(),
            PickerOperationId: PickerOperationId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerReturnedActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IPrelaunchActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn PrelaunchActivated(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
impl IPrelaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrelaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> IPrelaunchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn PrelaunchActivated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrelaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrelaunchActivated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrelaunchActivatedEventArgs, OFFSET>(),
            PrelaunchActivated: PrelaunchActivated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrelaunchActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Printers_Extensions\"`, `\"implement\"`*"]
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrint3DWorkflowActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl ::windows::core::RuntimeName for IPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: isize>() -> IPrint3DWorkflowActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Workflow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Workflow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrint3DWorkflowActivatedEventArgs, OFFSET>(),
            Workflow: Workflow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DWorkflowActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Printers_Extensions\"`, `\"implement\"`*"]
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrintTaskSettingsActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl ::windows::core::RuntimeName for IPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: isize>() -> IPrintTaskSettingsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Configuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrintTaskSettingsActivatedEventArgs, OFFSET>(),
            Configuration: Configuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskSettingsActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IProtocolActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
}
#[cfg(feature = "Foundation")]
impl IProtocolActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolActivatedEventArgs_Impl, const OFFSET: isize>() -> IProtocolActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Uri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Uri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IProtocolActivatedEventArgs, OFFSET>(), Uri: Uri::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl: Sized + IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
}
#[cfg(feature = "Foundation_Collections")]
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>() -> IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`, `\"implement\"`*"]
#[cfg(feature = "System")]
pub trait IProtocolForResultsActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "System")]
impl IProtocolForResultsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: isize>() -> IProtocolForResultsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ProtocolForResultsOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProtocolForResultsOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IProtocolForResultsActivatedEventArgs, OFFSET>(),
            ProtocolForResultsOperation: ProtocolForResultsOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolForResultsActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IRestrictedLaunchActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
}
impl IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> IRestrictedLaunchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn SharedContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SharedContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IRestrictedLaunchActivatedEventArgs, OFFSET>(),
            SharedContext: SharedContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRestrictedLaunchActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait ISearchActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
}
impl ISearchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchActivatedEventArgs_Impl, const OFFSET: isize>() -> ISearchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn QueryText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISearchActivatedEventArgs, OFFSET>(),
            QueryText: QueryText::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Search\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_Search")]
pub trait ISearchActivatedEventArgsWithLinguisticDetails_Impl: Sized {
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "ApplicationModel_Search")]
impl ::windows::core::RuntimeName for ISearchActivatedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
}
#[cfg(feature = "ApplicationModel_Search")]
impl ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: isize>() -> ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
        unsafe extern "system" fn LinguisticDetails<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISearchActivatedEventArgsWithLinguisticDetails, OFFSET>(),
            LinguisticDetails: LinguisticDetails::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgsWithLinguisticDetails as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_DataTransfer_ShareTarget\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub trait IShareTargetActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl ::windows::core::RuntimeName for IShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl IShareTargetActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShareTargetActivatedEventArgs_Impl, const OFFSET: isize>() -> IShareTargetActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ShareOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShareTargetActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShareOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IShareTargetActivatedEventArgs, OFFSET>(),
            ShareOperation: ShareOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareTargetActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"implement\"`*"]
pub trait IStartupTaskActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
}
impl IStartupTaskActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStartupTaskActivatedEventArgs_Impl, const OFFSET: isize>() -> IStartupTaskActivatedEventArgs_Vtbl {
        unsafe extern "system" fn TaskId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStartupTaskActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStartupTaskActivatedEventArgs, OFFSET>(), TaskId: TaskId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartupTaskActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IToastNotificationActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
impl IToastNotificationActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>() -> IToastNotificationActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Argument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Argument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IToastNotificationActivatedEventArgs, OFFSET>(),
            Argument: Argument::<Identity, Impl, OFFSET>,
            UserInput: UserInput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_UserDataAccounts_Provider\"`, `\"implement\"`*"]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub trait IUserDataAccountProviderActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl ::windows::core::RuntimeName for IUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IUserDataAccountProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Operation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUserDataAccountProviderActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataAccountProviderActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"UI_ViewManagement\"`, `\"implement\"`*"]
#[cfg(feature = "UI_ViewManagement")]
pub trait IViewSwitcherProvider_Impl: Sized + IActivatedEventArgs_Impl {
    fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows::core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
#[cfg(feature = "UI_ViewManagement")]
impl IViewSwitcherProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewSwitcherProvider_Impl, const OFFSET: isize>() -> IViewSwitcherProvider_Vtbl {
        unsafe extern "system" fn ViewSwitcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewSwitcherProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ViewSwitcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IViewSwitcherProvider, OFFSET>(),
            ViewSwitcher: ViewSwitcher::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewSwitcherProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Media_SpeechRecognition\"`, `\"implement\"`*"]
#[cfg(feature = "Media_SpeechRecognition")]
pub trait IVoiceCommandActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "Media_SpeechRecognition")]
impl ::windows::core::RuntimeName for IVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "Media_SpeechRecognition")]
impl IVoiceCommandActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: isize>() -> IVoiceCommandActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Result<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Result() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IVoiceCommandActivatedEventArgs, OFFSET>(), Result: Result::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Wallet\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
pub trait IWalletActionActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
impl IWalletActionActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>() -> IWalletActionActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ItemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWalletActionActivatedEventArgs, OFFSET>(),
            ItemId: ItemId::<Identity, Impl, OFFSET>,
            ActionKind: ActionKind::<Identity, Impl, OFFSET>,
            ActionId: ActionId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletActionActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Security_Authentication_Web_Provider\"`, `\"implement\"`*"]
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub trait IWebAccountProviderActivatedEventArgs_Impl: Sized + IActivatedEventArgs_Impl {
    fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl ::windows::core::RuntimeName for IWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl IWebAccountProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IWebAccountProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Operation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderActivatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Security_Authentication_Web\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
pub trait IWebAuthenticationBrokerContinuationEventArgs_Impl: Sized + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: isize>() -> IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn WebAuthenticationResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WebAuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationBrokerContinuationEventArgs, OFFSET>(),
            WebAuthenticationResult: WebAuthenticationResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationBrokerContinuationEventArgs as ::windows::core::ComInterface>::IID
    }
}

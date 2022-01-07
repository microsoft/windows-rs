pub trait IActivatedEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ActivationKind>;
    fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState>;
    fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
impl IActivatedEventArgsVtbl {
    pub const fn new<Impl: IActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivatedEventArgsVtbl {
        unsafe extern "system" fn Kind<Impl: IActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousExecutionState<Impl: IActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviousExecutionState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SplashScreen<Impl: IActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SplashScreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivatedEventArgs>, base.5, Kind::<Impl, OFFSET>, PreviousExecutionState::<Impl, OFFSET>, SplashScreen::<Impl, OFFSET>)
    }
}
pub trait IActivatedEventArgsWithUserImpl: Sized + IActivatedEventArgsImpl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
impl IActivatedEventArgsWithUserVtbl {
    pub const fn new<Impl: IActivatedEventArgsWithUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivatedEventArgsWithUserVtbl {
        unsafe extern "system" fn User<Impl: IActivatedEventArgsWithUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivatedEventArgsWithUser>, base.5, User::<Impl, OFFSET>)
    }
}
pub trait IApplicationViewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
impl IApplicationViewActivatedEventArgsVtbl {
    pub const fn new<Impl: IApplicationViewActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationViewActivatedEventArgsVtbl {
        unsafe extern "system" fn CurrentlyShownApplicationViewId<Impl: IApplicationViewActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentlyShownApplicationViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationViewActivatedEventArgs>, base.5, CurrentlyShownApplicationViewId::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
}
impl IAppointmentsProviderActivatedEventArgsVtbl {
    pub const fn new<Impl: IAppointmentsProviderActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentsProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Verb<Impl: IAppointmentsProviderActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderActivatedEventArgs>, base.5, Verb::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl {
    pub const fn new<Impl: IAppointmentsProviderAddAppointmentActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl {
        unsafe extern "system" fn AddAppointmentOperation<Impl: IAppointmentsProviderAddAppointmentActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderAddAppointmentActivatedEventArgs>, base.5, AddAppointmentOperation::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl {
    pub const fn new<Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl {
        unsafe extern "system" fn RemoveAppointmentOperation<Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderRemoveAppointmentActivatedEventArgs>, base.5, RemoveAppointmentOperation::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl {
    pub const fn new<Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl {
        unsafe extern "system" fn ReplaceAppointmentOperation<Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReplaceAppointmentOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderReplaceAppointmentActivatedEventArgs>, base.5, ReplaceAppointmentOperation::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl {
    pub const fn new<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl {
        unsafe extern "system" fn InstanceStartDate<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceStartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingId<Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoamingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>, base.5, InstanceStartDate::<Impl, OFFSET>, LocalId::<Impl, OFFSET>, RoamingId::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl {
    pub const fn new<Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl {
        unsafe extern "system" fn TimeToShow<Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimeToShow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderShowTimeFrameActivatedEventArgs>, base.5, TimeToShow::<Impl, OFFSET>, Duration::<Impl, OFFSET>)
    }
}
pub trait IBackgroundActivatedEventArgsImpl: Sized {
    fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance>;
}
impl ::windows::core::RuntimeName for IBackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
}
impl IBackgroundActivatedEventArgsVtbl {
    pub const fn new<Impl: IBackgroundActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundActivatedEventArgsVtbl {
        unsafe extern "system" fn TaskInstance<Impl: IBackgroundActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundActivatedEventArgs>, base.5, TaskInstance::<Impl, OFFSET>)
    }
}
pub trait IBarcodeScannerPreviewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
}
impl IBarcodeScannerPreviewActivatedEventArgsVtbl {
    pub const fn new<Impl: IBarcodeScannerPreviewActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerPreviewActivatedEventArgsVtbl {
        unsafe extern "system" fn ConnectionId<Impl: IBarcodeScannerPreviewActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerPreviewActivatedEventArgs>, base.5, ConnectionId::<Impl, OFFSET>)
    }
}
pub trait ICachedFileUpdaterActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
impl ::windows::core::RuntimeName for ICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
}
impl ICachedFileUpdaterActivatedEventArgsVtbl {
    pub const fn new<Impl: ICachedFileUpdaterActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICachedFileUpdaterActivatedEventArgsVtbl {
        unsafe extern "system" fn CachedFileUpdaterUI<Impl: ICachedFileUpdaterActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CachedFileUpdaterUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICachedFileUpdaterActivatedEventArgs>, base.5, CachedFileUpdaterUI::<Impl, OFFSET>)
    }
}
pub trait ICameraSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
}
impl ICameraSettingsActivatedEventArgsVtbl {
    pub const fn new<Impl: ICameraSettingsActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraSettingsActivatedEventArgsVtbl {
        unsafe extern "system" fn VideoDeviceController<Impl: ICameraSettingsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceExtension<Impl: ICameraSettingsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraSettingsActivatedEventArgs>, base.5, VideoDeviceController::<Impl, OFFSET>, VideoDeviceExtension::<Impl, OFFSET>)
    }
}
pub trait ICommandLineActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation>;
}
impl ::windows::core::RuntimeName for ICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
}
impl ICommandLineActivatedEventArgsVtbl {
    pub const fn new<Impl: ICommandLineActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandLineActivatedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: ICommandLineActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandLineActivatedEventArgs>, base.5, Operation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandLineActivationOperationImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentDirectoryPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitCode(&self, value: i32) -> ::windows::core::Result<()>;
    fn ExitCode(&self) -> ::windows::core::Result<i32>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivationOperation";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandLineActivationOperationVtbl {
    pub const fn new<Impl: ICommandLineActivationOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandLineActivationOperationVtbl {
        unsafe extern "system" fn Arguments<Impl: ICommandLineActivationOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDirectoryPath<Impl: ICommandLineActivationOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentDirectoryPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitCode<Impl: ICommandLineActivationOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitCode(value).into()
        }
        unsafe extern "system" fn ExitCode<Impl: ICommandLineActivationOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ICommandLineActivationOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandLineActivationOperation>, base.5, Arguments::<Impl, OFFSET>, CurrentDirectoryPath::<Impl, OFFSET>, SetExitCode::<Impl, OFFSET>, ExitCode::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
pub trait IContactActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
}
impl IContactActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactActivatedEventArgsVtbl {
        unsafe extern "system" fn Verb<Impl: IContactActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactActivatedEventArgs>, base.5, Verb::<Impl, OFFSET>)
    }
}
pub trait IContactCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
impl ::windows::core::RuntimeName for IContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
}
impl IContactCallActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactCallActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactCallActivatedEventArgs>, base.5, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactMapActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
impl ::windows::core::RuntimeName for IContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
}
impl IContactMapActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactMapActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactMapActivatedEventArgsVtbl {
        unsafe extern "system" fn Address<Impl: IContactMapActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactMapActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactMapActivatedEventArgs>, base.5, Address::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactMessageActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
impl ::windows::core::RuntimeName for IContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
}
impl IContactMessageActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactMessageActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactMessageActivatedEventArgs>, base.5, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactPanelActivatedEventArgsImpl: Sized {
    fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
impl ::windows::core::RuntimeName for IContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
}
impl IContactPanelActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactPanelActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactPanelActivatedEventArgsVtbl {
        unsafe extern "system" fn ContactPanel<Impl: IContactPanelActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactPanel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactPanelActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactPanelActivatedEventArgs>, base.5, ContactPanel::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI>;
}
impl ::windows::core::RuntimeName for IContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
}
impl IContactPickerActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactPickerActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactPickerActivatedEventArgsVtbl {
        unsafe extern "system" fn ContactPickerUI<Impl: IContactPickerActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactPickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactPickerActivatedEventArgs>, base.5, ContactPickerUI::<Impl, OFFSET>)
    }
}
pub trait IContactPostActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
impl ::windows::core::RuntimeName for IContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
}
impl IContactPostActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactPostActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactPostActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactPostActivatedEventArgs>, base.5, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactVideoCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
impl ::windows::core::RuntimeName for IContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
}
impl IContactVideoCallActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactVideoCallActivatedEventArgsVtbl {
        unsafe extern "system" fn ServiceId<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceUserId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactVideoCallActivatedEventArgs>, base.5, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
}
impl IContactsProviderActivatedEventArgsVtbl {
    pub const fn new<Impl: IContactsProviderActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactsProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Verb<Impl: IContactsProviderActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactsProviderActivatedEventArgs>, base.5, Verb::<Impl, OFFSET>)
    }
}
pub trait IContinuationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
impl ::windows::core::RuntimeName for IContinuationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
}
impl IContinuationActivatedEventArgsVtbl {
    pub const fn new<Impl: IContinuationActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContinuationActivatedEventArgsVtbl {
        unsafe extern "system" fn ContinuationData<Impl: IContinuationActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContinuationActivatedEventArgs>, base.5, ContinuationData::<Impl, OFFSET>)
    }
}
pub trait IDeviceActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
}
impl IDeviceActivatedEventArgsVtbl {
    pub const fn new<Impl: IDeviceActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceActivatedEventArgsVtbl {
        unsafe extern "system" fn DeviceInformationId<Impl: IDeviceActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceInformationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Impl: IDeviceActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceActivatedEventArgs>, base.5, DeviceInformationId::<Impl, OFFSET>, Verb::<Impl, OFFSET>)
    }
}
pub trait IDevicePairingActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
impl ::windows::core::RuntimeName for IDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
}
impl IDevicePairingActivatedEventArgsVtbl {
    pub const fn new<Impl: IDevicePairingActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDevicePairingActivatedEventArgsVtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IDevicePairingActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDevicePairingActivatedEventArgs>, base.5, DeviceInformation::<Impl, OFFSET>)
    }
}
pub trait IDialReceiverActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
}
impl IDialReceiverActivatedEventArgsVtbl {
    pub const fn new<Impl: IDialReceiverActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDialReceiverActivatedEventArgsVtbl {
        unsafe extern "system" fn AppName<Impl: IDialReceiverActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDialReceiverActivatedEventArgs>, base.5, AppName::<Impl, OFFSET>)
    }
}
pub trait IFileActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
}
impl IFileActivatedEventArgsVtbl {
    pub const fn new<Impl: IFileActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileActivatedEventArgsVtbl {
        unsafe extern "system" fn Files<Impl: IFileActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Impl: IFileActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Verb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileActivatedEventArgs>, base.5, Files::<Impl, OFFSET>, Verb::<Impl, OFFSET>)
    }
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyNameImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
}
impl IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl {
    pub const fn new<Impl: IFileActivatedEventArgsWithCallerPackageFamilyNameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IFileActivatedEventArgsWithCallerPackageFamilyNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileActivatedEventArgsWithCallerPackageFamilyName>, base.5, CallerPackageFamilyName::<Impl, OFFSET>)
    }
}
pub trait IFileActivatedEventArgsWithNeighboringFilesImpl: Sized + IActivatedEventArgsImpl + IFileActivatedEventArgsImpl {
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithNeighboringFiles {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
}
impl IFileActivatedEventArgsWithNeighboringFilesVtbl {
    pub const fn new<Impl: IFileActivatedEventArgsWithNeighboringFilesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileActivatedEventArgsWithNeighboringFilesVtbl {
        unsafe extern "system" fn NeighboringFilesQuery<Impl: IFileActivatedEventArgsWithNeighboringFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NeighboringFilesQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileActivatedEventArgsWithNeighboringFiles>, base.5, NeighboringFilesQuery::<Impl, OFFSET>)
    }
}
pub trait IFileOpenPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
}
impl IFileOpenPickerActivatedEventArgsVtbl {
    pub const fn new<Impl: IFileOpenPickerActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerActivatedEventArgsVtbl {
        unsafe extern "system" fn FileOpenPickerUI<Impl: IFileOpenPickerActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileOpenPickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPickerActivatedEventArgs>, base.5, FileOpenPickerUI::<Impl, OFFSET>)
    }
}
pub trait IFileOpenPickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
}
impl IFileOpenPickerActivatedEventArgs2Vtbl {
    pub const fn new<Impl: IFileOpenPickerActivatedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerActivatedEventArgs2Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IFileOpenPickerActivatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPickerActivatedEventArgs2>, base.5, CallerPackageFamilyName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait IFileOpenPickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl IFileOpenPickerContinuationEventArgsVtbl {
    pub const fn new<Impl: IFileOpenPickerContinuationEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerContinuationEventArgsVtbl {
        unsafe extern "system" fn Files<Impl: IFileOpenPickerContinuationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPickerContinuationEventArgs>, base.5, Files::<Impl, OFFSET>)
    }
}
pub trait IFileSavePickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
}
impl IFileSavePickerActivatedEventArgsVtbl {
    pub const fn new<Impl: IFileSavePickerActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePickerActivatedEventArgsVtbl {
        unsafe extern "system" fn FileSavePickerUI<Impl: IFileSavePickerActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileSavePickerUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePickerActivatedEventArgs>, base.5, FileSavePickerUI::<Impl, OFFSET>)
    }
}
pub trait IFileSavePickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
}
impl IFileSavePickerActivatedEventArgs2Vtbl {
    pub const fn new<Impl: IFileSavePickerActivatedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePickerActivatedEventArgs2Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IFileSavePickerActivatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseId<Impl: IFileSavePickerActivatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePickerActivatedEventArgs2>, base.5, CallerPackageFamilyName::<Impl, OFFSET>, EnterpriseId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait IFileSavePickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl IFileSavePickerContinuationEventArgsVtbl {
    pub const fn new<Impl: IFileSavePickerContinuationEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePickerContinuationEventArgsVtbl {
        unsafe extern "system" fn File<Impl: IFileSavePickerContinuationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePickerContinuationEventArgs>, base.5, File::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait IFolderPickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl IFolderPickerContinuationEventArgsVtbl {
    pub const fn new<Impl: IFolderPickerContinuationEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFolderPickerContinuationEventArgsVtbl {
        unsafe extern "system" fn Folder<Impl: IFolderPickerContinuationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFolderPickerContinuationEventArgs>, base.5, Folder::<Impl, OFFSET>)
    }
}
pub trait ILaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
}
impl ILaunchActivatedEventArgsVtbl {
    pub const fn new<Impl: ILaunchActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILaunchActivatedEventArgsVtbl {
        unsafe extern "system" fn Arguments<Impl: ILaunchActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileId<Impl: ILaunchActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILaunchActivatedEventArgs>, base.5, Arguments::<Impl, OFFSET>, TileId::<Impl, OFFSET>)
    }
}
pub trait ILaunchActivatedEventArgs2Impl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
impl ILaunchActivatedEventArgs2Vtbl {
    pub const fn new<Impl: ILaunchActivatedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILaunchActivatedEventArgs2Vtbl {
        unsafe extern "system" fn TileActivatedInfo<Impl: ILaunchActivatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileActivatedInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILaunchActivatedEventArgs2>, base.5, TileActivatedInfo::<Impl, OFFSET>)
    }
}
pub trait ILockScreenActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
}
impl ILockScreenActivatedEventArgsVtbl {
    pub const fn new<Impl: ILockScreenActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILockScreenActivatedEventArgsVtbl {
        unsafe extern "system" fn Info<Impl: ILockScreenActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILockScreenActivatedEventArgs>, base.5, Info::<Impl, OFFSET>)
    }
}
pub trait ILockScreenCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI>;
}
impl ::windows::core::RuntimeName for ILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
}
impl ILockScreenCallActivatedEventArgsVtbl {
    pub const fn new<Impl: ILockScreenCallActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILockScreenCallActivatedEventArgsVtbl {
        unsafe extern "system" fn CallUI<Impl: ILockScreenCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILockScreenCallActivatedEventArgs>, base.5, CallUI::<Impl, OFFSET>)
    }
}
pub trait IPhoneCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
}
impl IPhoneCallActivatedEventArgsVtbl {
    pub const fn new<Impl: IPhoneCallActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallActivatedEventArgsVtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneCallActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LineId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallActivatedEventArgs>, base.5, LineId::<Impl, OFFSET>)
    }
}
pub trait IPickerReturnedActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IPickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
}
impl IPickerReturnedActivatedEventArgsVtbl {
    pub const fn new<Impl: IPickerReturnedActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPickerReturnedActivatedEventArgsVtbl {
        unsafe extern "system" fn PickerOperationId<Impl: IPickerReturnedActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PickerOperationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPickerReturnedActivatedEventArgs>, base.5, PickerOperationId::<Impl, OFFSET>)
    }
}
pub trait IPrelaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PrelaunchActivated(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
impl IPrelaunchActivatedEventArgsVtbl {
    pub const fn new<Impl: IPrelaunchActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrelaunchActivatedEventArgsVtbl {
        unsafe extern "system" fn PrelaunchActivated<Impl: IPrelaunchActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrelaunchActivated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrelaunchActivatedEventArgs>, base.5, PrelaunchActivated::<Impl, OFFSET>)
    }
}
pub trait IPrint3DWorkflowActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
impl ::windows::core::RuntimeName for IPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
}
impl IPrint3DWorkflowActivatedEventArgsVtbl {
    pub const fn new<Impl: IPrint3DWorkflowActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrint3DWorkflowActivatedEventArgsVtbl {
        unsafe extern "system" fn Workflow<Impl: IPrint3DWorkflowActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Workflow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrint3DWorkflowActivatedEventArgs>, base.5, Workflow::<Impl, OFFSET>)
    }
}
pub trait IPrintTaskSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
impl ::windows::core::RuntimeName for IPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
}
impl IPrintTaskSettingsActivatedEventArgsVtbl {
    pub const fn new<Impl: IPrintTaskSettingsActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintTaskSettingsActivatedEventArgsVtbl {
        unsafe extern "system" fn Configuration<Impl: IPrintTaskSettingsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintTaskSettingsActivatedEventArgs>, base.5, Configuration::<Impl, OFFSET>)
    }
}
pub trait IProtocolActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
}
impl IProtocolActivatedEventArgsVtbl {
    pub const fn new<Impl: IProtocolActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProtocolActivatedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IProtocolActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProtocolActivatedEventArgs>, base.5, Uri::<Impl, OFFSET>)
    }
}
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
}
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl {
    pub const fn new<Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>, base.5, CallerPackageFamilyName::<Impl, OFFSET>, Data::<Impl, OFFSET>)
    }
}
pub trait IProtocolForResultsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation>;
}
impl ::windows::core::RuntimeName for IProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
}
impl IProtocolForResultsActivatedEventArgsVtbl {
    pub const fn new<Impl: IProtocolForResultsActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProtocolForResultsActivatedEventArgsVtbl {
        unsafe extern "system" fn ProtocolForResultsOperation<Impl: IProtocolForResultsActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProtocolForResultsOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProtocolForResultsActivatedEventArgs>, base.5, ProtocolForResultsOperation::<Impl, OFFSET>)
    }
}
pub trait IRestrictedLaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
}
impl IRestrictedLaunchActivatedEventArgsVtbl {
    pub const fn new<Impl: IRestrictedLaunchActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRestrictedLaunchActivatedEventArgsVtbl {
        unsafe extern "system" fn SharedContext<Impl: IRestrictedLaunchActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SharedContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRestrictedLaunchActivatedEventArgs>, base.5, SharedContext::<Impl, OFFSET>)
    }
}
pub trait ISearchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
}
impl ISearchActivatedEventArgsVtbl {
    pub const fn new<Impl: ISearchActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchActivatedEventArgsVtbl {
        unsafe extern "system" fn QueryText<Impl: ISearchActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchActivatedEventArgs>, base.5, QueryText::<Impl, OFFSET>, Language::<Impl, OFFSET>)
    }
}
pub trait ISearchActivatedEventArgsWithLinguisticDetailsImpl: Sized {
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
impl ::windows::core::RuntimeName for ISearchActivatedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
}
impl ISearchActivatedEventArgsWithLinguisticDetailsVtbl {
    pub const fn new<Impl: ISearchActivatedEventArgsWithLinguisticDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchActivatedEventArgsWithLinguisticDetailsVtbl {
        unsafe extern "system" fn LinguisticDetails<Impl: ISearchActivatedEventArgsWithLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchActivatedEventArgsWithLinguisticDetails>, base.5, LinguisticDetails::<Impl, OFFSET>)
    }
}
pub trait IShareTargetActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
impl ::windows::core::RuntimeName for IShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
}
impl IShareTargetActivatedEventArgsVtbl {
    pub const fn new<Impl: IShareTargetActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShareTargetActivatedEventArgsVtbl {
        unsafe extern "system" fn ShareOperation<Impl: IShareTargetActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShareOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShareTargetActivatedEventArgs>, base.5, ShareOperation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplashScreenImpl: Sized {
    fn ImageLocation(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Dismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDismissed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISplashScreen";
}
#[cfg(feature = "implement_exclusive")]
impl ISplashScreenVtbl {
    pub const fn new<Impl: ISplashScreenImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplashScreenVtbl {
        unsafe extern "system" fn ImageLocation<Impl: ISplashScreenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dismissed<Impl: ISplashScreenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDismissed<Impl: ISplashScreenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDismissed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplashScreen>, base.5, ImageLocation::<Impl, OFFSET>, Dismissed::<Impl, OFFSET>, RemoveDismissed::<Impl, OFFSET>)
    }
}
pub trait IStartupTaskActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
}
impl IStartupTaskActivatedEventArgsVtbl {
    pub const fn new<Impl: IStartupTaskActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStartupTaskActivatedEventArgsVtbl {
        unsafe extern "system" fn TaskId<Impl: IStartupTaskActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStartupTaskActivatedEventArgs>, base.5, TaskId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileActivatedInfoImpl: Sized {
    fn RecentlyShownNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ITileActivatedInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ITileActivatedInfoVtbl {
    pub const fn new<Impl: ITileActivatedInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileActivatedInfoVtbl {
        unsafe extern "system" fn RecentlyShownNotifications<Impl: ITileActivatedInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecentlyShownNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileActivatedInfo>, base.5, RecentlyShownNotifications::<Impl, OFFSET>)
    }
}
pub trait IToastNotificationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
impl ::windows::core::RuntimeName for IToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
}
impl IToastNotificationActivatedEventArgsVtbl {
    pub const fn new<Impl: IToastNotificationActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationActivatedEventArgsVtbl {
        unsafe extern "system" fn Argument<Impl: IToastNotificationActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Argument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserInput<Impl: IToastNotificationActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationActivatedEventArgs>, base.5, Argument::<Impl, OFFSET>, UserInput::<Impl, OFFSET>)
    }
}
pub trait IUserDataAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
}
impl IUserDataAccountProviderActivatedEventArgsVtbl {
    pub const fn new<Impl: IUserDataAccountProviderActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAccountProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IUserDataAccountProviderActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAccountProviderActivatedEventArgs>, base.5, Operation::<Impl, OFFSET>)
    }
}
pub trait IViewSwitcherProviderImpl: Sized + IActivatedEventArgsImpl {
    fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
impl ::windows::core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
impl IViewSwitcherProviderVtbl {
    pub const fn new<Impl: IViewSwitcherProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IViewSwitcherProviderVtbl {
        unsafe extern "system" fn ViewSwitcher<Impl: IViewSwitcherProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewSwitcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IViewSwitcherProvider>, base.5, ViewSwitcher::<Impl, OFFSET>)
    }
}
pub trait IVoiceCommandActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
impl ::windows::core::RuntimeName for IVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
}
impl IVoiceCommandActivatedEventArgsVtbl {
    pub const fn new<Impl: IVoiceCommandActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandActivatedEventArgsVtbl {
        unsafe extern "system" fn Result<Impl: IVoiceCommandActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandActivatedEventArgs>, base.5, Result::<Impl, OFFSET>)
    }
}
pub trait IWalletActionActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
}
impl IWalletActionActivatedEventArgsVtbl {
    pub const fn new<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWalletActionActivatedEventArgsVtbl {
        unsafe extern "system" fn ItemId<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionKind<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActionKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionId<Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWalletActionActivatedEventArgs>, base.5, ItemId::<Impl, OFFSET>, ActionKind::<Impl, OFFSET>, ActionId::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
}
impl IWebAccountProviderActivatedEventArgsVtbl {
    pub const fn new<Impl: IWebAccountProviderActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IWebAccountProviderActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderActivatedEventArgs>, base.5, Operation::<Impl, OFFSET>)
    }
}
pub trait IWebAuthenticationBrokerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
}
impl IWebAuthenticationBrokerContinuationEventArgsVtbl {
    pub const fn new<Impl: IWebAuthenticationBrokerContinuationEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAuthenticationBrokerContinuationEventArgsVtbl {
        unsafe extern "system" fn WebAuthenticationResult<Impl: IWebAuthenticationBrokerContinuationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAuthenticationBrokerContinuationEventArgs>, base.5, WebAuthenticationResult::<Impl, OFFSET>)
    }
}

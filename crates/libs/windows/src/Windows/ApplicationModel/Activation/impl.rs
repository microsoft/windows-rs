pub trait IActivatedEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ActivationKind>;
    fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState>;
    fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
impl IActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedEventArgsImpl, const OFFSET: isize>() -> IActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivatedEventArgs>, ::windows::core::GetTrustLevel, Kind::<Impl, OFFSET>, PreviousExecutionState::<Impl, OFFSET>, SplashScreen::<Impl, OFFSET>)
    }
}
pub trait IActivatedEventArgsWithUserImpl: Sized + IActivatedEventArgsImpl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
impl IActivatedEventArgsWithUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedEventArgsWithUserImpl, const OFFSET: isize>() -> IActivatedEventArgsWithUserVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivatedEventArgsWithUser>, ::windows::core::GetTrustLevel, User::<Impl, OFFSET>)
    }
}
pub trait IApplicationViewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
impl IApplicationViewActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewActivatedEventArgsImpl, const OFFSET: isize>() -> IApplicationViewActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationViewActivatedEventArgs>, ::windows::core::GetTrustLevel, CurrentlyShownApplicationViewId::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
}
impl IAppointmentsProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderActivatedEventArgsImpl, const OFFSET: isize>() -> IAppointmentsProviderActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderActivatedEventArgs>, ::windows::core::GetTrustLevel, Verb::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderAddAppointmentActivatedEventArgsImpl, const OFFSET: isize>() -> IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderAddAppointmentActivatedEventArgs>, ::windows::core::GetTrustLevel, AddAppointmentOperation::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl, const OFFSET: isize>() -> IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderRemoveAppointmentActivatedEventArgs>, ::windows::core::GetTrustLevel, RemoveAppointmentOperation::<Impl, OFFSET>)
    }
}
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
impl ::windows::core::RuntimeName for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl, const OFFSET: isize>() -> IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderReplaceAppointmentActivatedEventArgs>, ::windows::core::GetTrustLevel, ReplaceAppointmentOperation::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl, const OFFSET: isize>() -> IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>, ::windows::core::GetTrustLevel, InstanceStartDate::<Impl, OFFSET>, LocalId::<Impl, OFFSET>, RoamingId::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl, const OFFSET: isize>() -> IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppointmentsProviderShowTimeFrameActivatedEventArgs>, ::windows::core::GetTrustLevel, TimeToShow::<Impl, OFFSET>, Duration::<Impl, OFFSET>)
    }
}
pub trait IBackgroundActivatedEventArgsImpl: Sized {
    fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance>;
}
impl ::windows::core::RuntimeName for IBackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
}
impl IBackgroundActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundActivatedEventArgsImpl, const OFFSET: isize>() -> IBackgroundActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundActivatedEventArgs>, ::windows::core::GetTrustLevel, TaskInstance::<Impl, OFFSET>)
    }
}
pub trait IBarcodeScannerPreviewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
}
impl IBarcodeScannerPreviewActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerPreviewActivatedEventArgsImpl, const OFFSET: isize>() -> IBarcodeScannerPreviewActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerPreviewActivatedEventArgs>, ::windows::core::GetTrustLevel, ConnectionId::<Impl, OFFSET>)
    }
}
pub trait ICachedFileUpdaterActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
impl ::windows::core::RuntimeName for ICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
}
impl ICachedFileUpdaterActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterActivatedEventArgsImpl, const OFFSET: isize>() -> ICachedFileUpdaterActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICachedFileUpdaterActivatedEventArgs>, ::windows::core::GetTrustLevel, CachedFileUpdaterUI::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraSettingsActivatedEventArgsImpl, const OFFSET: isize>() -> ICameraSettingsActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICameraSettingsActivatedEventArgs>, ::windows::core::GetTrustLevel, VideoDeviceController::<Impl, OFFSET>, VideoDeviceExtension::<Impl, OFFSET>)
    }
}
pub trait ICommandLineActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation>;
}
impl ::windows::core::RuntimeName for ICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
}
impl ICommandLineActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandLineActivatedEventArgsImpl, const OFFSET: isize>() -> ICommandLineActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICommandLineActivatedEventArgs>, ::windows::core::GetTrustLevel, Operation::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandLineActivationOperationImpl, const OFFSET: isize>() -> ICommandLineActivationOperationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICommandLineActivationOperation>, ::windows::core::GetTrustLevel, Arguments::<Impl, OFFSET>, CurrentDirectoryPath::<Impl, OFFSET>, SetExitCode::<Impl, OFFSET>, ExitCode::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
pub trait IContactActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
}
impl IContactActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactActivatedEventArgsImpl, const OFFSET: isize>() -> IContactActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactActivatedEventArgs>, ::windows::core::GetTrustLevel, Verb::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCallActivatedEventArgsImpl, const OFFSET: isize>() -> IContactCallActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactCallActivatedEventArgs>, ::windows::core::GetTrustLevel, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactMapActivatedEventArgsImpl, const OFFSET: isize>() -> IContactMapActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactMapActivatedEventArgs>, ::windows::core::GetTrustLevel, Address::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactMessageActivatedEventArgsImpl, const OFFSET: isize>() -> IContactMessageActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactMessageActivatedEventArgs>, ::windows::core::GetTrustLevel, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPanelActivatedEventArgsImpl, const OFFSET: isize>() -> IContactPanelActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPanelActivatedEventArgs>, ::windows::core::GetTrustLevel, ContactPanel::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI>;
}
impl ::windows::core::RuntimeName for IContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
}
impl IContactPickerActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPickerActivatedEventArgsImpl, const OFFSET: isize>() -> IContactPickerActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPickerActivatedEventArgs>, ::windows::core::GetTrustLevel, ContactPickerUI::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPostActivatedEventArgsImpl, const OFFSET: isize>() -> IContactPostActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPostActivatedEventArgs>, ::windows::core::GetTrustLevel, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactVideoCallActivatedEventArgsImpl, const OFFSET: isize>() -> IContactVideoCallActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactVideoCallActivatedEventArgs>, ::windows::core::GetTrustLevel, ServiceId::<Impl, OFFSET>, ServiceUserId::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
pub trait IContactsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
}
impl IContactsProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactsProviderActivatedEventArgsImpl, const OFFSET: isize>() -> IContactsProviderActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactsProviderActivatedEventArgs>, ::windows::core::GetTrustLevel, Verb::<Impl, OFFSET>)
    }
}
pub trait IContinuationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
impl ::windows::core::RuntimeName for IContinuationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
}
impl IContinuationActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinuationActivatedEventArgsImpl, const OFFSET: isize>() -> IContinuationActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContinuationActivatedEventArgs>, ::windows::core::GetTrustLevel, ContinuationData::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceActivatedEventArgsImpl, const OFFSET: isize>() -> IDeviceActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceActivatedEventArgs>, ::windows::core::GetTrustLevel, DeviceInformationId::<Impl, OFFSET>, Verb::<Impl, OFFSET>)
    }
}
pub trait IDevicePairingActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
impl ::windows::core::RuntimeName for IDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
}
impl IDevicePairingActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingActivatedEventArgsImpl, const OFFSET: isize>() -> IDevicePairingActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDevicePairingActivatedEventArgs>, ::windows::core::GetTrustLevel, DeviceInformation::<Impl, OFFSET>)
    }
}
pub trait IDialReceiverActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
}
impl IDialReceiverActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialReceiverActivatedEventArgsImpl, const OFFSET: isize>() -> IDialReceiverActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDialReceiverActivatedEventArgs>, ::windows::core::GetTrustLevel, AppName::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileActivatedEventArgsImpl, const OFFSET: isize>() -> IFileActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileActivatedEventArgs>, ::windows::core::GetTrustLevel, Files::<Impl, OFFSET>, Verb::<Impl, OFFSET>)
    }
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyNameImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
}
impl IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileActivatedEventArgsWithCallerPackageFamilyNameImpl, const OFFSET: isize>() -> IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileActivatedEventArgsWithCallerPackageFamilyName>, ::windows::core::GetTrustLevel, CallerPackageFamilyName::<Impl, OFFSET>)
    }
}
pub trait IFileActivatedEventArgsWithNeighboringFilesImpl: Sized + IActivatedEventArgsImpl + IFileActivatedEventArgsImpl {
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
impl ::windows::core::RuntimeName for IFileActivatedEventArgsWithNeighboringFiles {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
}
impl IFileActivatedEventArgsWithNeighboringFilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileActivatedEventArgsWithNeighboringFilesImpl, const OFFSET: isize>() -> IFileActivatedEventArgsWithNeighboringFilesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileActivatedEventArgsWithNeighboringFiles>, ::windows::core::GetTrustLevel, NeighboringFilesQuery::<Impl, OFFSET>)
    }
}
pub trait IFileOpenPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
}
impl IFileOpenPickerActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerActivatedEventArgsImpl, const OFFSET: isize>() -> IFileOpenPickerActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPickerActivatedEventArgs>, ::windows::core::GetTrustLevel, FileOpenPickerUI::<Impl, OFFSET>)
    }
}
pub trait IFileOpenPickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IFileOpenPickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
}
impl IFileOpenPickerActivatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerActivatedEventArgs2Impl, const OFFSET: isize>() -> IFileOpenPickerActivatedEventArgs2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPickerActivatedEventArgs2>, ::windows::core::GetTrustLevel, CallerPackageFamilyName::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerContinuationEventArgsImpl, const OFFSET: isize>() -> IFileOpenPickerContinuationEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPickerContinuationEventArgs>, ::windows::core::GetTrustLevel, Files::<Impl, OFFSET>)
    }
}
pub trait IFileSavePickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
impl ::windows::core::RuntimeName for IFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
}
impl IFileSavePickerActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerActivatedEventArgsImpl, const OFFSET: isize>() -> IFileSavePickerActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePickerActivatedEventArgs>, ::windows::core::GetTrustLevel, FileSavePickerUI::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerActivatedEventArgs2Impl, const OFFSET: isize>() -> IFileSavePickerActivatedEventArgs2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePickerActivatedEventArgs2>, ::windows::core::GetTrustLevel, CallerPackageFamilyName::<Impl, OFFSET>, EnterpriseId::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerContinuationEventArgsImpl, const OFFSET: isize>() -> IFileSavePickerContinuationEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePickerContinuationEventArgs>, ::windows::core::GetTrustLevel, File::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPickerContinuationEventArgsImpl, const OFFSET: isize>() -> IFolderPickerContinuationEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFolderPickerContinuationEventArgs>, ::windows::core::GetTrustLevel, Folder::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILaunchActivatedEventArgsImpl, const OFFSET: isize>() -> ILaunchActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILaunchActivatedEventArgs>, ::windows::core::GetTrustLevel, Arguments::<Impl, OFFSET>, TileId::<Impl, OFFSET>)
    }
}
pub trait ILaunchActivatedEventArgs2Impl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo>;
}
impl ::windows::core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
impl ILaunchActivatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILaunchActivatedEventArgs2Impl, const OFFSET: isize>() -> ILaunchActivatedEventArgs2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILaunchActivatedEventArgs2>, ::windows::core::GetTrustLevel, TileActivatedInfo::<Impl, OFFSET>)
    }
}
pub trait ILockScreenActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
}
impl ILockScreenActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenActivatedEventArgsImpl, const OFFSET: isize>() -> ILockScreenActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenActivatedEventArgs>, ::windows::core::GetTrustLevel, Info::<Impl, OFFSET>)
    }
}
pub trait ILockScreenCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI>;
}
impl ::windows::core::RuntimeName for ILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
}
impl ILockScreenCallActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenCallActivatedEventArgsImpl, const OFFSET: isize>() -> ILockScreenCallActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenCallActivatedEventArgs>, ::windows::core::GetTrustLevel, CallUI::<Impl, OFFSET>)
    }
}
pub trait IPhoneCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
}
impl IPhoneCallActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallActivatedEventArgsImpl, const OFFSET: isize>() -> IPhoneCallActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhoneCallActivatedEventArgs>, ::windows::core::GetTrustLevel, LineId::<Impl, OFFSET>)
    }
}
pub trait IPickerReturnedActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IPickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
}
impl IPickerReturnedActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerReturnedActivatedEventArgsImpl, const OFFSET: isize>() -> IPickerReturnedActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPickerReturnedActivatedEventArgs>, ::windows::core::GetTrustLevel, PickerOperationId::<Impl, OFFSET>)
    }
}
pub trait IPrelaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PrelaunchActivated(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
impl IPrelaunchActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrelaunchActivatedEventArgsImpl, const OFFSET: isize>() -> IPrelaunchActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrelaunchActivatedEventArgs>, ::windows::core::GetTrustLevel, PrelaunchActivated::<Impl, OFFSET>)
    }
}
pub trait IPrint3DWorkflowActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
impl ::windows::core::RuntimeName for IPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
}
impl IPrint3DWorkflowActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DWorkflowActivatedEventArgsImpl, const OFFSET: isize>() -> IPrint3DWorkflowActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DWorkflowActivatedEventArgs>, ::windows::core::GetTrustLevel, Workflow::<Impl, OFFSET>)
    }
}
pub trait IPrintTaskSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
impl ::windows::core::RuntimeName for IPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
}
impl IPrintTaskSettingsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskSettingsActivatedEventArgsImpl, const OFFSET: isize>() -> IPrintTaskSettingsActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskSettingsActivatedEventArgs>, ::windows::core::GetTrustLevel, Configuration::<Impl, OFFSET>)
    }
}
pub trait IProtocolActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
impl ::windows::core::RuntimeName for IProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
}
impl IProtocolActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolActivatedEventArgsImpl, const OFFSET: isize>() -> IProtocolActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtocolActivatedEventArgs>, ::windows::core::GetTrustLevel, Uri::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl, const OFFSET: isize>() -> IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>, ::windows::core::GetTrustLevel, CallerPackageFamilyName::<Impl, OFFSET>, Data::<Impl, OFFSET>)
    }
}
pub trait IProtocolForResultsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation>;
}
impl ::windows::core::RuntimeName for IProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
}
impl IProtocolForResultsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolForResultsActivatedEventArgsImpl, const OFFSET: isize>() -> IProtocolForResultsActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtocolForResultsActivatedEventArgs>, ::windows::core::GetTrustLevel, ProtocolForResultsOperation::<Impl, OFFSET>)
    }
}
pub trait IRestrictedLaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
}
impl IRestrictedLaunchActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRestrictedLaunchActivatedEventArgsImpl, const OFFSET: isize>() -> IRestrictedLaunchActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRestrictedLaunchActivatedEventArgs>, ::windows::core::GetTrustLevel, SharedContext::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchActivatedEventArgsImpl, const OFFSET: isize>() -> ISearchActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISearchActivatedEventArgs>, ::windows::core::GetTrustLevel, QueryText::<Impl, OFFSET>, Language::<Impl, OFFSET>)
    }
}
pub trait ISearchActivatedEventArgsWithLinguisticDetailsImpl: Sized {
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
impl ::windows::core::RuntimeName for ISearchActivatedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
}
impl ISearchActivatedEventArgsWithLinguisticDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchActivatedEventArgsWithLinguisticDetailsImpl, const OFFSET: isize>() -> ISearchActivatedEventArgsWithLinguisticDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISearchActivatedEventArgsWithLinguisticDetails>, ::windows::core::GetTrustLevel, LinguisticDetails::<Impl, OFFSET>)
    }
}
pub trait IShareTargetActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
impl ::windows::core::RuntimeName for IShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
}
impl IShareTargetActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareTargetActivatedEventArgsImpl, const OFFSET: isize>() -> IShareTargetActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareTargetActivatedEventArgs>, ::windows::core::GetTrustLevel, ShareOperation::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplashScreenImpl, const OFFSET: isize>() -> ISplashScreenVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplashScreen>, ::windows::core::GetTrustLevel, ImageLocation::<Impl, OFFSET>, Dismissed::<Impl, OFFSET>, RemoveDismissed::<Impl, OFFSET>)
    }
}
pub trait IStartupTaskActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
}
impl IStartupTaskActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartupTaskActivatedEventArgsImpl, const OFFSET: isize>() -> IStartupTaskActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStartupTaskActivatedEventArgs>, ::windows::core::GetTrustLevel, TaskId::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileActivatedInfoImpl, const OFFSET: isize>() -> ITileActivatedInfoVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITileActivatedInfo>, ::windows::core::GetTrustLevel, RecentlyShownNotifications::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationActivatedEventArgsImpl, const OFFSET: isize>() -> IToastNotificationActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToastNotificationActivatedEventArgs>, ::windows::core::GetTrustLevel, Argument::<Impl, OFFSET>, UserInput::<Impl, OFFSET>)
    }
}
pub trait IUserDataAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
}
impl IUserDataAccountProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountProviderActivatedEventArgsImpl, const OFFSET: isize>() -> IUserDataAccountProviderActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDataAccountProviderActivatedEventArgs>, ::windows::core::GetTrustLevel, Operation::<Impl, OFFSET>)
    }
}
pub trait IViewSwitcherProviderImpl: Sized + IActivatedEventArgsImpl {
    fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
impl ::windows::core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
impl IViewSwitcherProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewSwitcherProviderImpl, const OFFSET: isize>() -> IViewSwitcherProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IViewSwitcherProvider>, ::windows::core::GetTrustLevel, ViewSwitcher::<Impl, OFFSET>)
    }
}
pub trait IVoiceCommandActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
impl ::windows::core::RuntimeName for IVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
}
impl IVoiceCommandActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandActivatedEventArgsImpl, const OFFSET: isize>() -> IVoiceCommandActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandActivatedEventArgs>, ::windows::core::GetTrustLevel, Result::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletActionActivatedEventArgsImpl, const OFFSET: isize>() -> IWalletActionActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWalletActionActivatedEventArgs>, ::windows::core::GetTrustLevel, ItemId::<Impl, OFFSET>, ActionKind::<Impl, OFFSET>, ActionId::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
}
impl IWebAccountProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderActivatedEventArgsImpl, const OFFSET: isize>() -> IWebAccountProviderActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAccountProviderActivatedEventArgs>, ::windows::core::GetTrustLevel, Operation::<Impl, OFFSET>)
    }
}
pub trait IWebAuthenticationBrokerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
}
impl IWebAuthenticationBrokerContinuationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationBrokerContinuationEventArgsImpl, const OFFSET: isize>() -> IWebAuthenticationBrokerContinuationEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAuthenticationBrokerContinuationEventArgs>, ::windows::core::GetTrustLevel, WebAuthenticationResult::<Impl, OFFSET>)
    }
}

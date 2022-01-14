#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerForUser_Impl: Sized {
    fn AdvertisingId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&mut self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.IAdvertisingManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvertisingManagerForUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvertisingManagerForUser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvertisingManagerForUser_Vtbl {
        unsafe extern "system" fn AdvertisingId<Impl: IAdvertisingManagerForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAdvertisingManagerForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvertisingManagerForUser, BASE_OFFSET>(),
            AdvertisingId: AdvertisingId::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvertisingManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerStatics_Impl: Sized {
    fn AdvertisingId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvertisingManagerStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IAdvertisingManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvertisingManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvertisingManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvertisingManagerStatics_Vtbl {
        unsafe extern "system" fn AdvertisingId<Impl: IAdvertisingManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvertisingManagerStatics, BASE_OFFSET>(),
            AdvertisingId: AdvertisingId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvertisingManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerStatics2_Impl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<AdvertisingManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvertisingManagerStatics2 {
    const NAME: &'static str = "Windows.System.UserProfile.IAdvertisingManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvertisingManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvertisingManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvertisingManagerStatics2_Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IAdvertisingManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvertisingManagerStatics2, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvertisingManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAssignedAccessSettings_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsSingleAppKioskMode(&mut self) -> ::windows::core::Result<bool>;
    fn User(&mut self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IAssignedAccessSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAssignedAccessSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssignedAccessSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssignedAccessSettings_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAssignedAccessSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSingleAppKioskMode<Impl: IAssignedAccessSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSingleAppKioskMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAssignedAccessSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAssignedAccessSettings, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            IsSingleAppKioskMode: IsSingleAppKioskMode::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssignedAccessSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAssignedAccessSettingsStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<AssignedAccessSettings>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<AssignedAccessSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAssignedAccessSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IAssignedAccessSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAssignedAccessSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssignedAccessSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssignedAccessSettingsStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAssignedAccessSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IAssignedAccessSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAssignedAccessSettingsStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssignedAccessSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticsSettings_Impl: Sized {
    fn CanUseDiagnosticsToTailorExperiences(&mut self) -> ::windows::core::Result<bool>;
    fn User(&mut self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IDiagnosticsSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticsSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticsSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticsSettings_Vtbl {
        unsafe extern "system" fn CanUseDiagnosticsToTailorExperiences<Impl: IDiagnosticsSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanUseDiagnosticsToTailorExperiences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IDiagnosticsSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDiagnosticsSettings, BASE_OFFSET>(),
            CanUseDiagnosticsToTailorExperiences: CanUseDiagnosticsToTailorExperiences::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticsSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticsSettingsStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<DiagnosticsSettings>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<DiagnosticsSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticsSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IDiagnosticsSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticsSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticsSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticsSettingsStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IDiagnosticsSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IDiagnosticsSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDiagnosticsSettingsStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticsSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFirstSignInSettings_Impl: Sized + super::super::Foundation::Collections::IIterable_Impl<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + super::super::Foundation::Collections::IMapView_Impl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IFirstSignInSettings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFirstSignInSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFirstSignInSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFirstSignInSettings_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFirstSignInSettings, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFirstSignInSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFirstSignInSettingsStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<FirstSignInSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFirstSignInSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IFirstSignInSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFirstSignInSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFirstSignInSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFirstSignInSettingsStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IFirstSignInSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFirstSignInSettingsStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFirstSignInSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IGlobalizationPreferencesForUser_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::User>;
    fn Calendars(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&mut self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesForUser";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl IGlobalizationPreferencesForUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesForUser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesForUser_Vtbl {
        unsafe extern "system" fn User<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Calendars<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calendars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clocks<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clocks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Currencies<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Currencies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeGeographicRegion<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeGeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekStartsOn<Impl: IGlobalizationPreferencesForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekStartsOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGlobalizationPreferencesForUser, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            Calendars: Calendars::<Impl, IMPL_OFFSET>,
            Clocks: Clocks::<Impl, IMPL_OFFSET>,
            Currencies: Currencies::<Impl, IMPL_OFFSET>,
            Languages: Languages::<Impl, IMPL_OFFSET>,
            HomeGeographicRegion: HomeGeographicRegion::<Impl, IMPL_OFFSET>,
            WeekStartsOn: WeekStartsOn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IGlobalizationPreferencesStatics_Impl: Sized {
    fn Calendars(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&mut self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl IGlobalizationPreferencesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesStatics_Vtbl {
        unsafe extern "system" fn Calendars<Impl: IGlobalizationPreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calendars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clocks<Impl: IGlobalizationPreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clocks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Currencies<Impl: IGlobalizationPreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Currencies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IGlobalizationPreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeGeographicRegion<Impl: IGlobalizationPreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeGeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekStartsOn<Impl: IGlobalizationPreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekStartsOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGlobalizationPreferencesStatics, BASE_OFFSET>(),
            Calendars: Calendars::<Impl, IMPL_OFFSET>,
            Clocks: Clocks::<Impl, IMPL_OFFSET>,
            Currencies: Currencies::<Impl, IMPL_OFFSET>,
            Languages: Languages::<Impl, IMPL_OFFSET>,
            HomeGeographicRegion: HomeGeographicRegion::<Impl, IMPL_OFFSET>,
            WeekStartsOn: WeekStartsOn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGlobalizationPreferencesStatics2_Impl: Sized {
    fn TrySetHomeGeographicRegion(&mut self, region: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TrySetLanguages(&mut self, languagetags: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics2 {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGlobalizationPreferencesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesStatics2_Vtbl {
        unsafe extern "system" fn TrySetHomeGeographicRegion<Impl: IGlobalizationPreferencesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetHomeGeographicRegion(&*(&region as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetLanguages<Impl: IGlobalizationPreferencesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetags: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetLanguages(&*(&languagetags as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGlobalizationPreferencesStatics2, BASE_OFFSET>(),
            TrySetHomeGeographicRegion: TrySetHomeGeographicRegion::<Impl, IMPL_OFFSET>,
            TrySetLanguages: TrySetLanguages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStatics3_Impl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<GlobalizationPreferencesForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics3 {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IGlobalizationPreferencesStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesStatics3_Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IGlobalizationPreferencesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGlobalizationPreferencesStatics3, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILockScreenImageFeedStatics_Impl: Sized {
    fn RequestSetImageFeedAsync(&mut self, syndicationfeeduri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>>;
    fn TryRemoveImageFeed(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenImageFeedStatics {
    const NAME: &'static str = "Windows.System.UserProfile.ILockScreenImageFeedStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILockScreenImageFeedStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenImageFeedStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenImageFeedStatics_Vtbl {
        unsafe extern "system" fn RequestSetImageFeedAsync<Impl: ILockScreenImageFeedStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syndicationfeeduri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetImageFeedAsync(&*(&syndicationfeeduri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRemoveImageFeed<Impl: ILockScreenImageFeedStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRemoveImageFeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenImageFeedStatics, BASE_OFFSET>(),
            RequestSetImageFeedAsync: RequestSetImageFeedAsync::<Impl, IMPL_OFFSET>,
            TryRemoveImageFeed: TryRemoveImageFeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenImageFeedStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILockScreenStatics_Impl: Sized {
    fn OriginalImageFile(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetImageStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetImageFileAsync(&mut self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetImageStreamAsync(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenStatics {
    const NAME: &'static str = "Windows.System.UserProfile.ILockScreenStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILockScreenStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenStatics_Vtbl {
        unsafe extern "system" fn OriginalImageFile<Impl: ILockScreenStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalImageFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageStream<Impl: ILockScreenStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageFileAsync<Impl: ILockScreenStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImageFileAsync(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageStreamAsync<Impl: ILockScreenStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImageStreamAsync(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenStatics, BASE_OFFSET>(),
            OriginalImageFile: OriginalImageFile::<Impl, IMPL_OFFSET>,
            GetImageStream: GetImageStream::<Impl, IMPL_OFFSET>,
            SetImageFileAsync: SetImageFileAsync::<Impl, IMPL_OFFSET>,
            SetImageStreamAsync: SetImageStreamAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IUserInformationStatics_Impl: Sized {
    fn AccountPictureChangeEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn NameAccessAllowed(&mut self) -> ::windows::core::Result<bool>;
    fn GetAccountPicture(&mut self, kind: AccountPictureKind) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn SetAccountPictureAsync(&mut self, image: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPicturesAsync(&mut self, smallimage: &::core::option::Option<super::super::Storage::IStorageFile>, largeimage: &::core::option::Option<super::super::Storage::IStorageFile>, video: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPictureFromStreamAsync(&mut self, image: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPicturesFromStreamsAsync(&mut self, smallimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, largeimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, video: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn AccountPictureChanged(&mut self, changehandler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountPictureChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetDisplayNameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetFirstNameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetLastNameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetPrincipalNameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetSessionInitiationProtocolUriAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetDomainNameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserInformationStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IUserInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl IUserInformationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserInformationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserInformationStatics_Vtbl {
        unsafe extern "system" fn AccountPictureChangeEnabled<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountPictureChangeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameAccessAllowed<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameAccessAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccountPicture<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: AccountPictureKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccountPicture(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPictureAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAccountPictureAsync(&*(&image as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPicturesAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smallimage: ::windows::core::RawPtr, largeimage: ::windows::core::RawPtr, video: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAccountPicturesAsync(
                &*(&smallimage as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&largeimage as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&video as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPictureFromStreamAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAccountPictureFromStreamAsync(&*(&image as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPicturesFromStreamsAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smallimage: ::windows::core::RawPtr, largeimage: ::windows::core::RawPtr, video: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAccountPicturesFromStreamsAsync(
                &*(&smallimage as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&largeimage as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&video as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountPictureChanged<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountPictureChanged(&*(&changehandler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountPictureChanged<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountPictureChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDisplayNameAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstNameAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastNameAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrincipalNameAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrincipalNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionInitiationProtocolUriAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionInitiationProtocolUriAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainNameAsync<Impl: IUserInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDomainNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserInformationStatics, BASE_OFFSET>(),
            AccountPictureChangeEnabled: AccountPictureChangeEnabled::<Impl, IMPL_OFFSET>,
            NameAccessAllowed: NameAccessAllowed::<Impl, IMPL_OFFSET>,
            GetAccountPicture: GetAccountPicture::<Impl, IMPL_OFFSET>,
            SetAccountPictureAsync: SetAccountPictureAsync::<Impl, IMPL_OFFSET>,
            SetAccountPicturesAsync: SetAccountPicturesAsync::<Impl, IMPL_OFFSET>,
            SetAccountPictureFromStreamAsync: SetAccountPictureFromStreamAsync::<Impl, IMPL_OFFSET>,
            SetAccountPicturesFromStreamsAsync: SetAccountPicturesFromStreamsAsync::<Impl, IMPL_OFFSET>,
            AccountPictureChanged: AccountPictureChanged::<Impl, IMPL_OFFSET>,
            RemoveAccountPictureChanged: RemoveAccountPictureChanged::<Impl, IMPL_OFFSET>,
            GetDisplayNameAsync: GetDisplayNameAsync::<Impl, IMPL_OFFSET>,
            GetFirstNameAsync: GetFirstNameAsync::<Impl, IMPL_OFFSET>,
            GetLastNameAsync: GetLastNameAsync::<Impl, IMPL_OFFSET>,
            GetPrincipalNameAsync: GetPrincipalNameAsync::<Impl, IMPL_OFFSET>,
            GetSessionInitiationProtocolUriAsync: GetSessionInitiationProtocolUriAsync::<Impl, IMPL_OFFSET>,
            GetDomainNameAsync: GetDomainNameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IUserProfilePersonalizationSettings_Impl: Sized {
    fn TrySetLockScreenImageAsync(&mut self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetWallpaperImageAsync(&mut self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IUserProfilePersonalizationSettings";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IUserProfilePersonalizationSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserProfilePersonalizationSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserProfilePersonalizationSettings_Vtbl {
        unsafe extern "system" fn TrySetLockScreenImageAsync<Impl: IUserProfilePersonalizationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetLockScreenImageAsync(&*(&imagefile as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetWallpaperImageAsync<Impl: IUserProfilePersonalizationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetWallpaperImageAsync(&*(&imagefile as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserProfilePersonalizationSettings, BASE_OFFSET>(),
            TrySetLockScreenImageAsync: TrySetLockScreenImageAsync::<Impl, IMPL_OFFSET>,
            TrySetWallpaperImageAsync: TrySetWallpaperImageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserProfilePersonalizationSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserProfilePersonalizationSettingsStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<UserProfilePersonalizationSettings>;
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserProfilePersonalizationSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserProfilePersonalizationSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserProfilePersonalizationSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserProfilePersonalizationSettingsStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IUserProfilePersonalizationSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IUserProfilePersonalizationSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserProfilePersonalizationSettingsStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserProfilePersonalizationSettingsStatics as ::windows::core::Interface>::IID
    }
}

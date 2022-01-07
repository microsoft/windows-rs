#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerForUserImpl: Sized {
    fn AdvertisingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.IAdvertisingManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvertisingManagerForUserVtbl {
    pub const fn new<Impl: IAdvertisingManagerForUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvertisingManagerForUserVtbl {
        unsafe extern "system" fn AdvertisingId<Impl: IAdvertisingManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvertisingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAdvertisingManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvertisingManagerForUser>, base.5, AdvertisingId::<Impl, OFFSET>, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerStaticsImpl: Sized {
    fn AdvertisingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvertisingManagerStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IAdvertisingManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvertisingManagerStaticsVtbl {
    pub const fn new<Impl: IAdvertisingManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvertisingManagerStaticsVtbl {
        unsafe extern "system" fn AdvertisingId<Impl: IAdvertisingManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvertisingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvertisingManagerStatics>, base.5, AdvertisingId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<AdvertisingManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvertisingManagerStatics2 {
    const NAME: &'static str = "Windows.System.UserProfile.IAdvertisingManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvertisingManagerStatics2Vtbl {
    pub const fn new<Impl: IAdvertisingManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvertisingManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IAdvertisingManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvertisingManagerStatics2>, base.5, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAssignedAccessSettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsSingleAppKioskMode(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IAssignedAccessSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAssignedAccessSettingsVtbl {
    pub const fn new<Impl: IAssignedAccessSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAssignedAccessSettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAssignedAccessSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSingleAppKioskMode<Impl: IAssignedAccessSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSingleAppKioskMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAssignedAccessSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAssignedAccessSettings>, base.5, IsEnabled::<Impl, OFFSET>, IsSingleAppKioskMode::<Impl, OFFSET>, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAssignedAccessSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AssignedAccessSettings>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<AssignedAccessSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAssignedAccessSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IAssignedAccessSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAssignedAccessSettingsStaticsVtbl {
    pub const fn new<Impl: IAssignedAccessSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAssignedAccessSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAssignedAccessSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IAssignedAccessSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAssignedAccessSettingsStatics>, base.5, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticsSettingsImpl: Sized {
    fn CanUseDiagnosticsToTailorExperiences(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IDiagnosticsSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticsSettingsVtbl {
    pub const fn new<Impl: IDiagnosticsSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiagnosticsSettingsVtbl {
        unsafe extern "system" fn CanUseDiagnosticsToTailorExperiences<Impl: IDiagnosticsSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanUseDiagnosticsToTailorExperiences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IDiagnosticsSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiagnosticsSettings>, base.5, CanUseDiagnosticsToTailorExperiences::<Impl, OFFSET>, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticsSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<DiagnosticsSettings>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<DiagnosticsSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticsSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IDiagnosticsSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticsSettingsStaticsVtbl {
    pub const fn new<Impl: IDiagnosticsSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiagnosticsSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IDiagnosticsSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IDiagnosticsSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiagnosticsSettingsStatics>, base.5, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFirstSignInSettingsImpl: Sized + IIterableImpl<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapViewImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IFirstSignInSettings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFirstSignInSettingsVtbl {
    pub const fn new<Impl: IFirstSignInSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFirstSignInSettingsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFirstSignInSettings>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFirstSignInSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<FirstSignInSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFirstSignInSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IFirstSignInSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFirstSignInSettingsStaticsVtbl {
    pub const fn new<Impl: IFirstSignInSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFirstSignInSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IFirstSignInSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFirstSignInSettingsStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesForUserImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IGlobalizationPreferencesForUserVtbl {
    pub const fn new<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGlobalizationPreferencesForUserVtbl {
        unsafe extern "system" fn User<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Calendars<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Calendars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clocks<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clocks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Currencies<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Currencies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeGeographicRegion<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HomeGeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekStartsOn<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekStartsOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesForUser>, base.5, User::<Impl, OFFSET>, Calendars::<Impl, OFFSET>, Clocks::<Impl, OFFSET>, Currencies::<Impl, OFFSET>, Languages::<Impl, OFFSET>, HomeGeographicRegion::<Impl, OFFSET>, WeekStartsOn::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStaticsImpl: Sized {
    fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGlobalizationPreferencesStaticsVtbl {
    pub const fn new<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGlobalizationPreferencesStaticsVtbl {
        unsafe extern "system" fn Calendars<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Calendars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clocks<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clocks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Currencies<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Currencies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeGeographicRegion<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HomeGeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekStartsOn<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekStartsOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesStatics>, base.5, Calendars::<Impl, OFFSET>, Clocks::<Impl, OFFSET>, Currencies::<Impl, OFFSET>, Languages::<Impl, OFFSET>, HomeGeographicRegion::<Impl, OFFSET>, WeekStartsOn::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStatics2Impl: Sized {
    fn TrySetHomeGeographicRegion(&self, region: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TrySetLanguages(&self, languagetags: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics2 {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGlobalizationPreferencesStatics2Vtbl {
    pub const fn new<Impl: IGlobalizationPreferencesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGlobalizationPreferencesStatics2Vtbl {
        unsafe extern "system" fn TrySetHomeGeographicRegion<Impl: IGlobalizationPreferencesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, region: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetHomeGeographicRegion(&*(&region as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetLanguages<Impl: IGlobalizationPreferencesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languagetags: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetLanguages(&*(&languagetags as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesStatics2>, base.5, TrySetHomeGeographicRegion::<Impl, OFFSET>, TrySetLanguages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<GlobalizationPreferencesForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics3 {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IGlobalizationPreferencesStatics3Vtbl {
    pub const fn new<Impl: IGlobalizationPreferencesStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGlobalizationPreferencesStatics3Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IGlobalizationPreferencesStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesStatics3>, base.5, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenImageFeedStaticsImpl: Sized {
    fn RequestSetImageFeedAsync(&self, syndicationfeeduri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>>;
    fn TryRemoveImageFeed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenImageFeedStatics {
    const NAME: &'static str = "Windows.System.UserProfile.ILockScreenImageFeedStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenImageFeedStaticsVtbl {
    pub const fn new<Impl: ILockScreenImageFeedStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILockScreenImageFeedStaticsVtbl {
        unsafe extern "system" fn RequestSetImageFeedAsync<Impl: ILockScreenImageFeedStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syndicationfeeduri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestSetImageFeedAsync(&*(&syndicationfeeduri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRemoveImageFeed<Impl: ILockScreenImageFeedStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryRemoveImageFeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILockScreenImageFeedStatics>, base.5, RequestSetImageFeedAsync::<Impl, OFFSET>, TryRemoveImageFeed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenStaticsImpl: Sized {
    fn OriginalImageFile(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetImageFileAsync(&self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetImageStreamAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenStatics {
    const NAME: &'static str = "Windows.System.UserProfile.ILockScreenStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenStaticsVtbl {
    pub const fn new<Impl: ILockScreenStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILockScreenStaticsVtbl {
        unsafe extern "system" fn OriginalImageFile<Impl: ILockScreenStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OriginalImageFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageStream<Impl: ILockScreenStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageFileAsync<Impl: ILockScreenStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetImageFileAsync(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageStreamAsync<Impl: ILockScreenStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetImageStreamAsync(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILockScreenStatics>, base.5, OriginalImageFile::<Impl, OFFSET>, GetImageStream::<Impl, OFFSET>, SetImageFileAsync::<Impl, OFFSET>, SetImageStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IUserInformationStaticsImpl: Sized {
    fn AccountPictureChangeEnabled(&self) -> ::windows::core::Result<bool>;
    fn NameAccessAllowed(&self) -> ::windows::core::Result<bool>;
    fn GetAccountPicture(&self, kind: AccountPictureKind) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn SetAccountPictureAsync(&self, image: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPicturesAsync(&self, smallimage: &::core::option::Option<super::super::Storage::IStorageFile>, largeimage: &::core::option::Option<super::super::Storage::IStorageFile>, video: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPictureFromStreamAsync(&self, image: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPicturesFromStreamsAsync(&self, smallimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, largeimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, video: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn AccountPictureChanged(&self, changehandler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountPictureChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetDisplayNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetFirstNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetLastNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetPrincipalNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetSessionInitiationProtocolUriAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetDomainNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserInformationStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IUserInformationStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IUserInformationStaticsVtbl {
    pub const fn new<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserInformationStaticsVtbl {
        unsafe extern "system" fn AccountPictureChangeEnabled<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountPictureChangeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameAccessAllowed<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NameAccessAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccountPicture<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: AccountPictureKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccountPicture(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPictureAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccountPictureAsync(&*(&image as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPicturesAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smallimage: ::windows::core::RawPtr, largeimage: ::windows::core::RawPtr, video: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetAccountPictureFromStreamAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccountPictureFromStreamAsync(&*(&image as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountPicturesFromStreamsAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smallimage: ::windows::core::RawPtr, largeimage: ::windows::core::RawPtr, video: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn AccountPictureChanged<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountPictureChanged(&*(&changehandler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountPictureChanged<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccountPictureChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDisplayNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFirstNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrincipalNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrincipalNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionInitiationProtocolUriAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSessionInitiationProtocolUriAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDomainNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUserInformationStatics>,
            base.5,
            AccountPictureChangeEnabled::<Impl, OFFSET>,
            NameAccessAllowed::<Impl, OFFSET>,
            GetAccountPicture::<Impl, OFFSET>,
            SetAccountPictureAsync::<Impl, OFFSET>,
            SetAccountPicturesAsync::<Impl, OFFSET>,
            SetAccountPictureFromStreamAsync::<Impl, OFFSET>,
            SetAccountPicturesFromStreamsAsync::<Impl, OFFSET>,
            AccountPictureChanged::<Impl, OFFSET>,
            RemoveAccountPictureChanged::<Impl, OFFSET>,
            GetDisplayNameAsync::<Impl, OFFSET>,
            GetFirstNameAsync::<Impl, OFFSET>,
            GetLastNameAsync::<Impl, OFFSET>,
            GetPrincipalNameAsync::<Impl, OFFSET>,
            GetSessionInitiationProtocolUriAsync::<Impl, OFFSET>,
            GetDomainNameAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserProfilePersonalizationSettingsImpl: Sized {
    fn TrySetLockScreenImageAsync(&self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetWallpaperImageAsync(&self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IUserProfilePersonalizationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IUserProfilePersonalizationSettingsVtbl {
    pub const fn new<Impl: IUserProfilePersonalizationSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserProfilePersonalizationSettingsVtbl {
        unsafe extern "system" fn TrySetLockScreenImageAsync<Impl: IUserProfilePersonalizationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetLockScreenImageAsync(&*(&imagefile as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetWallpaperImageAsync<Impl: IUserProfilePersonalizationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetWallpaperImageAsync(&*(&imagefile as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserProfilePersonalizationSettings>, base.5, TrySetLockScreenImageAsync::<Impl, OFFSET>, TrySetWallpaperImageAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserProfilePersonalizationSettingsStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<UserProfilePersonalizationSettings>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserProfilePersonalizationSettingsStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserProfilePersonalizationSettingsStaticsVtbl {
    pub const fn new<Impl: IUserProfilePersonalizationSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserProfilePersonalizationSettingsStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IUserProfilePersonalizationSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IUserProfilePersonalizationSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserProfilePersonalizationSettingsStatics>, base.5, Current::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
    }
}

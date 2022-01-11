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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvertisingManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvertisingManagerForUserVtbl {
        unsafe extern "system" fn AdvertisingId<Impl: IAdvertisingManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IAdvertisingManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdvertisingManagerForUser>, ::windows::core::GetTrustLevel, AdvertisingId::<Impl, IMPL_OFFSET>, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvertisingManagerForUser as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvertisingManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvertisingManagerStaticsVtbl {
        unsafe extern "system" fn AdvertisingId<Impl: IAdvertisingManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdvertisingManagerStatics>, ::windows::core::GetTrustLevel, AdvertisingId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvertisingManagerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvertisingManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvertisingManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IAdvertisingManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdvertisingManagerStatics2>, ::windows::core::GetTrustLevel, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvertisingManagerStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssignedAccessSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssignedAccessSettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAssignedAccessSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSingleAppKioskMode<Impl: IAssignedAccessSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IAssignedAccessSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAssignedAccessSettings>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, IMPL_OFFSET>, IsSingleAppKioskMode::<Impl, IMPL_OFFSET>, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssignedAccessSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssignedAccessSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssignedAccessSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAssignedAccessSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IAssignedAccessSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAssignedAccessSettingsStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssignedAccessSettingsStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticsSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticsSettingsVtbl {
        unsafe extern "system" fn CanUseDiagnosticsToTailorExperiences<Impl: IDiagnosticsSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IDiagnosticsSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiagnosticsSettings>, ::windows::core::GetTrustLevel, CanUseDiagnosticsToTailorExperiences::<Impl, IMPL_OFFSET>, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticsSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticsSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticsSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IDiagnosticsSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IDiagnosticsSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiagnosticsSettingsStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticsSettingsStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFirstSignInSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFirstSignInSettingsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFirstSignInSettings>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFirstSignInSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFirstSignInSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFirstSignInSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IFirstSignInSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFirstSignInSettingsStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFirstSignInSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IGlobalizationPreferencesForUserImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesForUser";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl IGlobalizationPreferencesForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesForUserVtbl {
        unsafe extern "system" fn User<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Calendars<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clocks<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Currencies<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Languages<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HomeGeographicRegion<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekStartsOn<Impl: IGlobalizationPreferencesForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesForUser>,
            ::windows::core::GetTrustLevel,
            User::<Impl, IMPL_OFFSET>,
            Calendars::<Impl, IMPL_OFFSET>,
            Clocks::<Impl, IMPL_OFFSET>,
            Currencies::<Impl, IMPL_OFFSET>,
            Languages::<Impl, IMPL_OFFSET>,
            HomeGeographicRegion::<Impl, IMPL_OFFSET>,
            WeekStartsOn::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IGlobalizationPreferencesStaticsImpl: Sized {
    fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl IGlobalizationPreferencesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesStaticsVtbl {
        unsafe extern "system" fn Calendars<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clocks<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Currencies<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Languages<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HomeGeographicRegion<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekStartsOn<Impl: IGlobalizationPreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesStatics>,
            ::windows::core::GetTrustLevel,
            Calendars::<Impl, IMPL_OFFSET>,
            Clocks::<Impl, IMPL_OFFSET>,
            Currencies::<Impl, IMPL_OFFSET>,
            Languages::<Impl, IMPL_OFFSET>,
            HomeGeographicRegion::<Impl, IMPL_OFFSET>,
            WeekStartsOn::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGlobalizationPreferencesStatics2Impl: Sized {
    fn TrySetHomeGeographicRegion(&self, region: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TrySetLanguages(&self, languagetags: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGlobalizationPreferencesStatics2 {
    const NAME: &'static str = "Windows.System.UserProfile.IGlobalizationPreferencesStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGlobalizationPreferencesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesStatics2Vtbl {
        unsafe extern "system" fn TrySetHomeGeographicRegion<Impl: IGlobalizationPreferencesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetLanguages<Impl: IGlobalizationPreferencesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetags: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesStatics2>, ::windows::core::GetTrustLevel, TrySetHomeGeographicRegion::<Impl, IMPL_OFFSET>, TrySetLanguages::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalizationPreferencesStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalizationPreferencesStatics3Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IGlobalizationPreferencesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGlobalizationPreferencesStatics3>, ::windows::core::GetTrustLevel, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalizationPreferencesStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILockScreenImageFeedStaticsImpl: Sized {
    fn RequestSetImageFeedAsync(&self, syndicationfeeduri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>>;
    fn TryRemoveImageFeed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenImageFeedStatics {
    const NAME: &'static str = "Windows.System.UserProfile.ILockScreenImageFeedStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILockScreenImageFeedStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenImageFeedStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenImageFeedStaticsVtbl {
        unsafe extern "system" fn RequestSetImageFeedAsync<Impl: ILockScreenImageFeedStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syndicationfeeduri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryRemoveImageFeed<Impl: ILockScreenImageFeedStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenImageFeedStatics>, ::windows::core::GetTrustLevel, RequestSetImageFeedAsync::<Impl, IMPL_OFFSET>, TryRemoveImageFeed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenImageFeedStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILockScreenStaticsImpl: Sized {
    fn OriginalImageFile(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetImageFileAsync(&self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetImageStreamAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenStatics {
    const NAME: &'static str = "Windows.System.UserProfile.ILockScreenStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILockScreenStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenStaticsVtbl {
        unsafe extern "system" fn OriginalImageFile<Impl: ILockScreenStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetImageStream<Impl: ILockScreenStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImageFileAsync<Impl: ILockScreenStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImageStreamAsync<Impl: ILockScreenStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenStatics>, ::windows::core::GetTrustLevel, OriginalImageFile::<Impl, IMPL_OFFSET>, GetImageStream::<Impl, IMPL_OFFSET>, SetImageFileAsync::<Impl, IMPL_OFFSET>, SetImageStreamAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserInformationStatics {
    const NAME: &'static str = "Windows.System.UserProfile.IUserInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl IUserInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserInformationStaticsVtbl {
        unsafe extern "system" fn AccountPictureChangeEnabled<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NameAccessAllowed<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAccountPicture<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: AccountPictureKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccountPictureAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccountPicturesAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smallimage: ::windows::core::RawPtr, largeimage: ::windows::core::RawPtr, video: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccountPictureFromStreamAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccountPicturesFromStreamsAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smallimage: ::windows::core::RawPtr, largeimage: ::windows::core::RawPtr, video: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccountPictureChanged<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAccountPictureChanged<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountPictureChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDisplayNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFirstNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLastNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPrincipalNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSessionInitiationProtocolUriAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDomainNameAsync<Impl: IUserInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserInformationStatics>,
            ::windows::core::GetTrustLevel,
            AccountPictureChangeEnabled::<Impl, IMPL_OFFSET>,
            NameAccessAllowed::<Impl, IMPL_OFFSET>,
            GetAccountPicture::<Impl, IMPL_OFFSET>,
            SetAccountPictureAsync::<Impl, IMPL_OFFSET>,
            SetAccountPicturesAsync::<Impl, IMPL_OFFSET>,
            SetAccountPictureFromStreamAsync::<Impl, IMPL_OFFSET>,
            SetAccountPicturesFromStreamsAsync::<Impl, IMPL_OFFSET>,
            AccountPictureChanged::<Impl, IMPL_OFFSET>,
            RemoveAccountPictureChanged::<Impl, IMPL_OFFSET>,
            GetDisplayNameAsync::<Impl, IMPL_OFFSET>,
            GetFirstNameAsync::<Impl, IMPL_OFFSET>,
            GetLastNameAsync::<Impl, IMPL_OFFSET>,
            GetPrincipalNameAsync::<Impl, IMPL_OFFSET>,
            GetSessionInitiationProtocolUriAsync::<Impl, IMPL_OFFSET>,
            GetDomainNameAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IUserProfilePersonalizationSettingsImpl: Sized {
    fn TrySetLockScreenImageAsync(&self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetWallpaperImageAsync(&self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.IUserProfilePersonalizationSettings";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IUserProfilePersonalizationSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserProfilePersonalizationSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserProfilePersonalizationSettingsVtbl {
        unsafe extern "system" fn TrySetLockScreenImageAsync<Impl: IUserProfilePersonalizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetWallpaperImageAsync<Impl: IUserProfilePersonalizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserProfilePersonalizationSettings>, ::windows::core::GetTrustLevel, TrySetLockScreenImageAsync::<Impl, IMPL_OFFSET>, TrySetWallpaperImageAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserProfilePersonalizationSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserProfilePersonalizationSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserProfilePersonalizationSettingsStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IUserProfilePersonalizationSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSupported<Impl: IUserProfilePersonalizationSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserProfilePersonalizationSettingsStatics>, ::windows::core::GetTrustLevel, Current::<Impl, IMPL_OFFSET>, IsSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserProfilePersonalizationSettingsStatics as ::windows::core::Interface>::IID
    }
}

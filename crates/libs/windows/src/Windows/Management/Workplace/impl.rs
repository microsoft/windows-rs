#[cfg(feature = "implement_exclusive")]
pub trait IMdmAllowPolicyStaticsImpl: Sized {
    fn IsBrowserAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsCameraAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsMicrosoftAccountAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsStoreAllowed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMdmAllowPolicyStatics {
    const NAME: &'static str = "Windows.Management.Workplace.IMdmAllowPolicyStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMdmAllowPolicyStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMdmAllowPolicyStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMdmAllowPolicyStaticsVtbl {
        unsafe extern "system" fn IsBrowserAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBrowserAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCameraAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCameraAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMicrosoftAccountAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMicrosoftAccountAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStoreAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMdmAllowPolicyStatics>, ::windows::core::GetTrustLevel, IsBrowserAllowed::<Impl, IMPL_OFFSET>, IsCameraAllowed::<Impl, IMPL_OFFSET>, IsMicrosoftAccountAllowed::<Impl, IMPL_OFFSET>, IsStoreAllowed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMdmAllowPolicyStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMdmPolicyStatics2Impl: Sized {
    fn GetMessagingSyncPolicy(&self) -> ::windows::core::Result<MessagingSyncPolicy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMdmPolicyStatics2 {
    const NAME: &'static str = "Windows.Management.Workplace.IMdmPolicyStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMdmPolicyStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMdmPolicyStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMdmPolicyStatics2Vtbl {
        unsafe extern "system" fn GetMessagingSyncPolicy<Impl: IMdmPolicyStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MessagingSyncPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessagingSyncPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMdmPolicyStatics2>, ::windows::core::GetTrustLevel, GetMessagingSyncPolicy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMdmPolicyStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWorkplaceSettingsStaticsImpl: Sized {
    fn IsMicrosoftAccountOptional(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWorkplaceSettingsStatics {
    const NAME: &'static str = "Windows.Management.Workplace.IWorkplaceSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWorkplaceSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkplaceSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkplaceSettingsStaticsVtbl {
        unsafe extern "system" fn IsMicrosoftAccountOptional<Impl: IWorkplaceSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMicrosoftAccountOptional() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkplaceSettingsStatics>, ::windows::core::GetTrustLevel, IsMicrosoftAccountOptional::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkplaceSettingsStatics as ::windows::core::Interface>::IID
    }
}

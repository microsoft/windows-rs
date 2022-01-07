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
    pub const fn new<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMdmAllowPolicyStaticsVtbl {
        unsafe extern "system" fn IsBrowserAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBrowserAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCameraAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCameraAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMicrosoftAccountAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMicrosoftAccountAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStoreAllowed<Impl: IMdmAllowPolicyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStoreAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMdmAllowPolicyStatics>, base.5, IsBrowserAllowed::<Impl, OFFSET>, IsCameraAllowed::<Impl, OFFSET>, IsMicrosoftAccountAllowed::<Impl, OFFSET>, IsStoreAllowed::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMdmPolicyStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMdmPolicyStatics2Vtbl {
        unsafe extern "system" fn GetMessagingSyncPolicy<Impl: IMdmPolicyStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MessagingSyncPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMessagingSyncPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMdmPolicyStatics2>, base.5, GetMessagingSyncPolicy::<Impl, OFFSET>)
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
    pub const fn new<Impl: IWorkplaceSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWorkplaceSettingsStaticsVtbl {
        unsafe extern "system" fn IsMicrosoftAccountOptional<Impl: IWorkplaceSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMicrosoftAccountOptional() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWorkplaceSettingsStatics>, base.5, IsMicrosoftAccountOptional::<Impl, OFFSET>)
    }
}

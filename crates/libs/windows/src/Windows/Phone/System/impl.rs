#[cfg(feature = "implement_exclusive")]
pub trait ISystemProtectionStatics_Impl: Sized {
    fn ScreenLocked(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemProtectionStatics {
    const NAME: &'static str = "Windows.Phone.System.ISystemProtectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemProtectionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemProtectionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemProtectionStatics_Vtbl {
        unsafe extern "system" fn ScreenLocked<Impl: ISystemProtectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemProtectionStatics, BASE_OFFSET>(),
            ScreenLocked: ScreenLocked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemProtectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemProtectionUnlockStatics_Impl: Sized {
    fn RequestScreenUnlock(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemProtectionUnlockStatics {
    const NAME: &'static str = "Windows.Phone.System.ISystemProtectionUnlockStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemProtectionUnlockStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemProtectionUnlockStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemProtectionUnlockStatics_Vtbl {
        unsafe extern "system" fn RequestScreenUnlock<Impl: ISystemProtectionUnlockStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestScreenUnlock().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemProtectionUnlockStatics, BASE_OFFSET>(),
            RequestScreenUnlock: RequestScreenUnlock::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemProtectionUnlockStatics as ::windows::core::Interface>::IID
    }
}

#[cfg(feature = "implement_exclusive")]
pub trait ISystemProtectionStaticsImpl: Sized {
    fn ScreenLocked(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemProtectionStatics {
    const NAME: &'static str = "Windows.Phone.System.ISystemProtectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemProtectionStaticsVtbl {
    pub const fn new<Impl: ISystemProtectionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemProtectionStaticsVtbl {
        unsafe extern "system" fn ScreenLocked<Impl: ISystemProtectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemProtectionStatics>, base.5, ScreenLocked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemProtectionUnlockStaticsImpl: Sized {
    fn RequestScreenUnlock(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemProtectionUnlockStatics {
    const NAME: &'static str = "Windows.Phone.System.ISystemProtectionUnlockStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemProtectionUnlockStaticsVtbl {
    pub const fn new<Impl: ISystemProtectionUnlockStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemProtectionUnlockStaticsVtbl {
        unsafe extern "system" fn RequestScreenUnlock<Impl: ISystemProtectionUnlockStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestScreenUnlock().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemProtectionUnlockStatics>, base.5, RequestScreenUnlock::<Impl, OFFSET>)
    }
}

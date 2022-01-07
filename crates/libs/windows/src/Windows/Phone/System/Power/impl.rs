#[cfg(feature = "implement_exclusive")]
pub trait IPowerManagerStaticsImpl: Sized {
    fn PowerSavingMode(&self) -> ::windows::core::Result<PowerSavingMode>;
    fn PowerSavingModeChanged(&self, changehandler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePowerSavingModeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerManagerStatics {
    const NAME: &'static str = "Windows.Phone.System.Power.IPowerManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerManagerStaticsVtbl {
    pub const fn new<Impl: IPowerManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPowerManagerStaticsVtbl {
        unsafe extern "system" fn PowerSavingMode<Impl: IPowerManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PowerSavingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerSavingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSavingModeChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerSavingModeChanged(&*(&changehandler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePowerSavingModeChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePowerSavingModeChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPowerManagerStatics>, base.5, PowerSavingMode::<Impl, OFFSET>, PowerSavingModeChanged::<Impl, OFFSET>, RemovePowerSavingModeChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerManagerStatics2Impl: Sized {
    fn PowerSavingModeEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerManagerStatics2 {
    const NAME: &'static str = "Windows.Phone.System.Power.IPowerManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerManagerStatics2Vtbl {
    pub const fn new<Impl: IPowerManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPowerManagerStatics2Vtbl {
        unsafe extern "system" fn PowerSavingModeEnabled<Impl: IPowerManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerSavingModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPowerManagerStatics2>, base.5, PowerSavingModeEnabled::<Impl, OFFSET>)
    }
}

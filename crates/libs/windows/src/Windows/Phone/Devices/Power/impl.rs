#[cfg(feature = "implement_exclusive")]
pub trait IBatteryImpl: Sized {
    fn RemainingChargePercent(&self) -> ::windows::core::Result<i32>;
    fn RemainingDischargeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn RemainingChargePercentChanged(&self, changehandler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingChargePercentChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBattery {
    const NAME: &'static str = "Windows.Phone.Devices.Power.IBattery";
}
#[cfg(feature = "implement_exclusive")]
impl IBatteryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBatteryImpl, const OFFSET: isize>() -> IBatteryVtbl {
        unsafe extern "system" fn RemainingChargePercent<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingChargePercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingDischargeTime<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingDischargeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingChargePercentChanged<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingChargePercentChanged(&*(&changehandler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemainingChargePercentChanged<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemainingChargePercentChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBattery>, ::windows::core::GetTrustLevel, RemainingChargePercent::<Impl, OFFSET>, RemainingDischargeTime::<Impl, OFFSET>, RemainingChargePercentChanged::<Impl, OFFSET>, RemoveRemainingChargePercentChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBatteryStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Battery>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBatteryStatics {
    const NAME: &'static str = "Windows.Phone.Devices.Power.IBatteryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBatteryStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBatteryStaticsImpl, const OFFSET: isize>() -> IBatteryStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IBatteryStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBatteryStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}

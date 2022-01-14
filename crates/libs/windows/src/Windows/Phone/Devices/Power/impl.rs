#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBattery_Impl: Sized {
    fn RemainingChargePercent(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingDischargeTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn RemainingChargePercentChanged(&mut self, changehandler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingChargePercentChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBattery {
    const NAME: &'static str = "Windows.Phone.Devices.Power.IBattery";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBattery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBattery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBattery_Vtbl {
        unsafe extern "system" fn RemainingChargePercent<Impl: IBattery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemainingDischargeTime<Impl: IBattery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemainingChargePercentChanged<Impl: IBattery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRemainingChargePercentChanged<Impl: IBattery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemainingChargePercentChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBattery, BASE_OFFSET>(),
            RemainingChargePercent: RemainingChargePercent::<Impl, IMPL_OFFSET>,
            RemainingDischargeTime: RemainingDischargeTime::<Impl, IMPL_OFFSET>,
            RemainingChargePercentChanged: RemainingChargePercentChanged::<Impl, IMPL_OFFSET>,
            RemoveRemainingChargePercentChanged: RemoveRemainingChargePercentChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBattery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBatteryStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Battery>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBatteryStatics {
    const NAME: &'static str = "Windows.Phone.Devices.Power.IBatteryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBatteryStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBatteryStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBatteryStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IBatteryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBatteryStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBatteryStatics as ::windows::core::Interface>::IID
    }
}

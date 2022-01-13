#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundEnergyManagerStaticsImpl: Sized {
    fn LowUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn NearMaxAcceptableUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn MaxAcceptableUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn ExcessiveUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn NearTerminationUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn TerminationUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsage(&mut self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageIncreased(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageIncreased(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecentEnergyUsageReturnedToLow(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageReturnedToLow(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundEnergyManagerStatics {
    const NAME: &'static str = "Windows.System.Power.IBackgroundEnergyManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundEnergyManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundEnergyManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundEnergyManagerStaticsVtbl {
        unsafe extern "system" fn LowUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearMaxAcceptableUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NearMaxAcceptableUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAcceptableUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAcceptableUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcessiveUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcessiveUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearTerminationUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NearTerminationUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminationUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminationUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecentEnergyUsage<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecentEnergyUsageLevel<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecentEnergyUsageIncreased<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsageIncreased(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecentEnergyUsageIncreased<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecentEnergyUsageIncreased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecentEnergyUsageReturnedToLow<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsageReturnedToLow(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecentEnergyUsageReturnedToLow<Impl: IBackgroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecentEnergyUsageReturnedToLow(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundEnergyManagerStatics, BASE_OFFSET>(),
            LowUsageLevel: LowUsageLevel::<Impl, IMPL_OFFSET>,
            NearMaxAcceptableUsageLevel: NearMaxAcceptableUsageLevel::<Impl, IMPL_OFFSET>,
            MaxAcceptableUsageLevel: MaxAcceptableUsageLevel::<Impl, IMPL_OFFSET>,
            ExcessiveUsageLevel: ExcessiveUsageLevel::<Impl, IMPL_OFFSET>,
            NearTerminationUsageLevel: NearTerminationUsageLevel::<Impl, IMPL_OFFSET>,
            TerminationUsageLevel: TerminationUsageLevel::<Impl, IMPL_OFFSET>,
            RecentEnergyUsage: RecentEnergyUsage::<Impl, IMPL_OFFSET>,
            RecentEnergyUsageLevel: RecentEnergyUsageLevel::<Impl, IMPL_OFFSET>,
            RecentEnergyUsageIncreased: RecentEnergyUsageIncreased::<Impl, IMPL_OFFSET>,
            RemoveRecentEnergyUsageIncreased: RemoveRecentEnergyUsageIncreased::<Impl, IMPL_OFFSET>,
            RecentEnergyUsageReturnedToLow: RecentEnergyUsageReturnedToLow::<Impl, IMPL_OFFSET>,
            RemoveRecentEnergyUsageReturnedToLow: RemoveRecentEnergyUsageReturnedToLow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundEnergyManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IForegroundEnergyManagerStaticsImpl: Sized {
    fn LowUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn NearMaxAcceptableUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn MaxAcceptableUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn ExcessiveUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsage(&mut self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageLevel(&mut self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageIncreased(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageIncreased(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecentEnergyUsageReturnedToLow(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageReturnedToLow(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IForegroundEnergyManagerStatics {
    const NAME: &'static str = "Windows.System.Power.IForegroundEnergyManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IForegroundEnergyManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForegroundEnergyManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForegroundEnergyManagerStaticsVtbl {
        unsafe extern "system" fn LowUsageLevel<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearMaxAcceptableUsageLevel<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NearMaxAcceptableUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAcceptableUsageLevel<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAcceptableUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcessiveUsageLevel<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcessiveUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecentEnergyUsage<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecentEnergyUsageLevel<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecentEnergyUsageIncreased<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsageIncreased(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecentEnergyUsageIncreased<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecentEnergyUsageIncreased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecentEnergyUsageReturnedToLow<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecentEnergyUsageReturnedToLow(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecentEnergyUsageReturnedToLow<Impl: IForegroundEnergyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecentEnergyUsageReturnedToLow(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IForegroundEnergyManagerStatics, BASE_OFFSET>(),
            LowUsageLevel: LowUsageLevel::<Impl, IMPL_OFFSET>,
            NearMaxAcceptableUsageLevel: NearMaxAcceptableUsageLevel::<Impl, IMPL_OFFSET>,
            MaxAcceptableUsageLevel: MaxAcceptableUsageLevel::<Impl, IMPL_OFFSET>,
            ExcessiveUsageLevel: ExcessiveUsageLevel::<Impl, IMPL_OFFSET>,
            RecentEnergyUsage: RecentEnergyUsage::<Impl, IMPL_OFFSET>,
            RecentEnergyUsageLevel: RecentEnergyUsageLevel::<Impl, IMPL_OFFSET>,
            RecentEnergyUsageIncreased: RecentEnergyUsageIncreased::<Impl, IMPL_OFFSET>,
            RemoveRecentEnergyUsageIncreased: RemoveRecentEnergyUsageIncreased::<Impl, IMPL_OFFSET>,
            RecentEnergyUsageReturnedToLow: RecentEnergyUsageReturnedToLow::<Impl, IMPL_OFFSET>,
            RemoveRecentEnergyUsageReturnedToLow: RemoveRecentEnergyUsageReturnedToLow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForegroundEnergyManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPowerManagerStaticsImpl: Sized {
    fn EnergySaverStatus(&mut self) -> ::windows::core::Result<EnergySaverStatus>;
    fn EnergySaverStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnergySaverStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BatteryStatus(&mut self) -> ::windows::core::Result<BatteryStatus>;
    fn BatteryStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBatteryStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PowerSupplyStatus(&mut self) -> ::windows::core::Result<PowerSupplyStatus>;
    fn PowerSupplyStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePowerSupplyStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemainingChargePercent(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingChargePercentChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingChargePercentChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemainingDischargeTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RemainingDischargeTimeChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingDischargeTimeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPowerManagerStatics {
    const NAME: &'static str = "Windows.System.Power.IPowerManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPowerManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerManagerStaticsVtbl {
        unsafe extern "system" fn EnergySaverStatus<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EnergySaverStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnergySaverStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnergySaverStatusChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnergySaverStatusChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnergySaverStatusChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnergySaverStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BatteryStatus<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BatteryStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatteryStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatteryStatusChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatteryStatusChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBatteryStatusChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBatteryStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PowerSupplyStatus<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PowerSupplyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerSupplyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSupplyStatusChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerSupplyStatusChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePowerSupplyStatusChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePowerSupplyStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemainingChargePercent<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemainingChargePercentChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingChargePercentChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemainingChargePercentChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemainingChargePercentChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemainingDischargeTime<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemainingDischargeTimeChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingDischargeTimeChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemainingDischargeTimeChanged<Impl: IPowerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemainingDischargeTimeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPowerManagerStatics, BASE_OFFSET>(),
            EnergySaverStatus: EnergySaverStatus::<Impl, IMPL_OFFSET>,
            EnergySaverStatusChanged: EnergySaverStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveEnergySaverStatusChanged: RemoveEnergySaverStatusChanged::<Impl, IMPL_OFFSET>,
            BatteryStatus: BatteryStatus::<Impl, IMPL_OFFSET>,
            BatteryStatusChanged: BatteryStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveBatteryStatusChanged: RemoveBatteryStatusChanged::<Impl, IMPL_OFFSET>,
            PowerSupplyStatus: PowerSupplyStatus::<Impl, IMPL_OFFSET>,
            PowerSupplyStatusChanged: PowerSupplyStatusChanged::<Impl, IMPL_OFFSET>,
            RemovePowerSupplyStatusChanged: RemovePowerSupplyStatusChanged::<Impl, IMPL_OFFSET>,
            RemainingChargePercent: RemainingChargePercent::<Impl, IMPL_OFFSET>,
            RemainingChargePercentChanged: RemainingChargePercentChanged::<Impl, IMPL_OFFSET>,
            RemoveRemainingChargePercentChanged: RemoveRemainingChargePercentChanged::<Impl, IMPL_OFFSET>,
            RemainingDischargeTime: RemainingDischargeTime::<Impl, IMPL_OFFSET>,
            RemainingDischargeTimeChanged: RemainingDischargeTimeChanged::<Impl, IMPL_OFFSET>,
            RemoveRemainingDischargeTimeChanged: RemoveRemainingDischargeTimeChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerManagerStatics as ::windows::core::Interface>::IID
    }
}

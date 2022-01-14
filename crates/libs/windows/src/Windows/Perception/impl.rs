#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPerceptionTimestamp_Impl: Sized {
    fn TargetTime(&mut self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn PredictionAmount(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionTimestamp {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestamp";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPerceptionTimestamp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionTimestamp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionTimestamp_Vtbl {
        unsafe extern "system" fn TargetTime<Impl: IPerceptionTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PredictionAmount<Impl: IPerceptionTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PredictionAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionTimestamp, BASE_OFFSET>(),
            TargetTime: TargetTime::<Impl, IMPL_OFFSET>,
            PredictionAmount: PredictionAmount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionTimestamp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPerceptionTimestamp2_Impl: Sized {
    fn SystemRelativeTargetTime(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionTimestamp2 {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestamp2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPerceptionTimestamp2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionTimestamp2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionTimestamp2_Vtbl {
        unsafe extern "system" fn SystemRelativeTargetTime<Impl: IPerceptionTimestamp2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTargetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionTimestamp2, BASE_OFFSET>(),
            SystemRelativeTargetTime: SystemRelativeTargetTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionTimestamp2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPerceptionTimestampHelperStatics_Impl: Sized {
    fn FromHistoricalTargetTime(&mut self, targettime: &super::Foundation::DateTime) -> ::windows::core::Result<PerceptionTimestamp>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionTimestampHelperStatics {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestampHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPerceptionTimestampHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionTimestampHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionTimestampHelperStatics_Vtbl {
        unsafe extern "system" fn FromHistoricalTargetTime<Impl: IPerceptionTimestampHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targettime: super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHistoricalTargetTime(&*(&targettime as *const <super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionTimestampHelperStatics, BASE_OFFSET>(),
            FromHistoricalTargetTime: FromHistoricalTargetTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionTimestampHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPerceptionTimestampHelperStatics2_Impl: Sized {
    fn FromSystemRelativeTargetTime(&mut self, targettime: &super::Foundation::TimeSpan) -> ::windows::core::Result<PerceptionTimestamp>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionTimestampHelperStatics2 {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestampHelperStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPerceptionTimestampHelperStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionTimestampHelperStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionTimestampHelperStatics2_Vtbl {
        unsafe extern "system" fn FromSystemRelativeTargetTime<Impl: IPerceptionTimestampHelperStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targettime: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromSystemRelativeTargetTime(&*(&targettime as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionTimestampHelperStatics2, BASE_OFFSET>(),
            FromSystemRelativeTargetTime: FromSystemRelativeTargetTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionTimestampHelperStatics2 as ::windows::core::Interface>::IID
    }
}

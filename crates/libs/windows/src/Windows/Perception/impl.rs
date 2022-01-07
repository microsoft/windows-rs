#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestampImpl: Sized {
    fn TargetTime(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn PredictionAmount(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPerceptionTimestamp {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestamp";
}
#[cfg(feature = "implement_exclusive")]
impl IPerceptionTimestampVtbl {
    pub const fn new<Impl: IPerceptionTimestampImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionTimestampVtbl {
        unsafe extern "system" fn TargetTime<Impl: IPerceptionTimestampImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PredictionAmount<Impl: IPerceptionTimestampImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PredictionAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionTimestamp>, base.5, TargetTime::<Impl, OFFSET>, PredictionAmount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestamp2Impl: Sized {
    fn SystemRelativeTargetTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPerceptionTimestamp2 {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestamp2";
}
#[cfg(feature = "implement_exclusive")]
impl IPerceptionTimestamp2Vtbl {
    pub const fn new<Impl: IPerceptionTimestamp2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionTimestamp2Vtbl {
        unsafe extern "system" fn SystemRelativeTargetTime<Impl: IPerceptionTimestamp2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTargetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionTimestamp2>, base.5, SystemRelativeTargetTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestampHelperStaticsImpl: Sized {
    fn FromHistoricalTargetTime(&self, targettime: &super::Foundation::DateTime) -> ::windows::core::Result<PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPerceptionTimestampHelperStatics {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestampHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPerceptionTimestampHelperStaticsVtbl {
    pub const fn new<Impl: IPerceptionTimestampHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionTimestampHelperStaticsVtbl {
        unsafe extern "system" fn FromHistoricalTargetTime<Impl: IPerceptionTimestampHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targettime: super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHistoricalTargetTime(&*(&targettime as *const <super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionTimestampHelperStatics>, base.5, FromHistoricalTargetTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestampHelperStatics2Impl: Sized {
    fn FromSystemRelativeTargetTime(&self, targettime: &super::Foundation::TimeSpan) -> ::windows::core::Result<PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPerceptionTimestampHelperStatics2 {
    const NAME: &'static str = "Windows.Perception.IPerceptionTimestampHelperStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPerceptionTimestampHelperStatics2Vtbl {
    pub const fn new<Impl: IPerceptionTimestampHelperStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPerceptionTimestampHelperStatics2Vtbl {
        unsafe extern "system" fn FromSystemRelativeTargetTime<Impl: IPerceptionTimestampHelperStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targettime: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromSystemRelativeTargetTime(&*(&targettime as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPerceptionTimestampHelperStatics2>, base.5, FromSystemRelativeTargetTime::<Impl, OFFSET>)
    }
}

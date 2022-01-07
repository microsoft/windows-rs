#[cfg(feature = "implement_exclusive")]
pub trait ICoreUserActivityManagerStaticsImpl: Sized {
    fn CreateUserActivitySessionInBackground(&self, activity: &::core::option::Option<super::UserActivity>) -> ::windows::core::Result<super::UserActivitySession>;
    fn DeleteUserActivitySessionsInTimeRangeAsync(&self, channel: &::core::option::Option<super::UserActivityChannel>, starttime: &super::super::super::Foundation::DateTime, endtime: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreUserActivityManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreUserActivityManagerStaticsVtbl {
    pub const fn new<Impl: ICoreUserActivityManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreUserActivityManagerStaticsVtbl {
        unsafe extern "system" fn CreateUserActivitySessionInBackground<Impl: ICoreUserActivityManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUserActivitySessionInBackground(&*(&activity as *const <super::UserActivity as ::windows::core::Abi>::Abi as *const <super::UserActivity as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUserActivitySessionsInTimeRangeAsync<Impl: ICoreUserActivityManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteUserActivitySessionsInTimeRangeAsync(
                &*(&channel as *const <super::UserActivityChannel as ::windows::core::Abi>::Abi as *const <super::UserActivityChannel as ::windows::core::DefaultType>::DefaultType),
                &*(&starttime as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&endtime as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreUserActivityManagerStatics>, base.5, CreateUserActivitySessionInBackground::<Impl, OFFSET>, DeleteUserActivitySessionsInTimeRangeAsync::<Impl, OFFSET>)
    }
}

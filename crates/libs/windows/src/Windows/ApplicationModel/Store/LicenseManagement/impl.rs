#[cfg(feature = "implement_exclusive")]
pub trait ILicenseManagerStaticsImpl: Sized {
    fn AddLicenseAsync(&self, license: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GetSatisfactionInfosAsync(&self, contentids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, keyids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILicenseManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILicenseManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILicenseManagerStaticsImpl, const OFFSET: isize>() -> ILicenseManagerStaticsVtbl {
        unsafe extern "system" fn AddLicenseAsync<Impl: ILicenseManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, license: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLicenseAsync(&*(&license as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSatisfactionInfosAsync<Impl: ILicenseManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentids: ::windows::core::RawPtr, keyids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSatisfactionInfosAsync(
                &*(&contentids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&keyids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILicenseManagerStatics>, ::windows::core::GetTrustLevel, AddLicenseAsync::<Impl, OFFSET>, GetSatisfactionInfosAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseManagerStatics2Impl: Sized {
    fn RefreshLicensesAsync(&self, refreshoption: LicenseRefreshOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILicenseManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ILicenseManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILicenseManagerStatics2Impl, const OFFSET: isize>() -> ILicenseManagerStatics2Vtbl {
        unsafe extern "system" fn RefreshLicensesAsync<Impl: ILicenseManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refreshoption: LicenseRefreshOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshLicensesAsync(refreshoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILicenseManagerStatics2>, ::windows::core::GetTrustLevel, RefreshLicensesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseSatisfactionInfoImpl: Sized {
    fn SatisfiedByDevice(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByOpenLicense(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByTrial(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByPass(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByInstallMedia(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedBySignedInUser(&self) -> ::windows::core::Result<bool>;
    fn IsSatisfied(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILicenseSatisfactionInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ILicenseSatisfactionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>() -> ILicenseSatisfactionInfoVtbl {
        unsafe extern "system" fn SatisfiedByDevice<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatisfiedByDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatisfiedByOpenLicense<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatisfiedByOpenLicense() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatisfiedByTrial<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatisfiedByTrial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatisfiedByPass<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatisfiedByPass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatisfiedByInstallMedia<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatisfiedByInstallMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatisfiedBySignedInUser<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatisfiedBySignedInUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSatisfied<Impl: ILicenseSatisfactionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSatisfied() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILicenseSatisfactionInfo>,
            ::windows::core::GetTrustLevel,
            SatisfiedByDevice::<Impl, OFFSET>,
            SatisfiedByOpenLicense::<Impl, OFFSET>,
            SatisfiedByTrial::<Impl, OFFSET>,
            SatisfiedByPass::<Impl, OFFSET>,
            SatisfiedByInstallMedia::<Impl, OFFSET>,
            SatisfiedBySignedInUser::<Impl, OFFSET>,
            IsSatisfied::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseSatisfactionResultImpl: Sized {
    fn LicenseSatisfactionInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, LicenseSatisfactionInfo>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILicenseSatisfactionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult";
}
#[cfg(feature = "implement_exclusive")]
impl ILicenseSatisfactionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILicenseSatisfactionResultImpl, const OFFSET: isize>() -> ILicenseSatisfactionResultVtbl {
        unsafe extern "system" fn LicenseSatisfactionInfos<Impl: ILicenseSatisfactionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseSatisfactionInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: ILicenseSatisfactionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILicenseSatisfactionResult>, ::windows::core::GetTrustLevel, LicenseSatisfactionInfos::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}

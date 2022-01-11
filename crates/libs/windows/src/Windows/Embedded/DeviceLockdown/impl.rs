#[cfg(feature = "implement_exclusive")]
pub trait IDeviceLockdownProfileInformationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceLockdownProfileInformation {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.IDeviceLockdownProfileInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceLockdownProfileInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceLockdownProfileInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceLockdownProfileInformationVtbl {
        unsafe extern "system" fn Name<Impl: IDeviceLockdownProfileInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceLockdownProfileInformation>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceLockdownProfileInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDeviceLockdownProfileStaticsImpl: Sized {
    fn GetSupportedLockdownProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
    fn GetCurrentLockdownProfile(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ApplyLockdownProfileAsync(&self, profileid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetLockdownProfileInformation(&self, profileid: &::windows::core::GUID) -> ::windows::core::Result<DeviceLockdownProfileInformation>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceLockdownProfileStatics {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.IDeviceLockdownProfileStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDeviceLockdownProfileStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceLockdownProfileStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceLockdownProfileStaticsVtbl {
        unsafe extern "system" fn GetSupportedLockdownProfiles<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedLockdownProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLockdownProfile<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLockdownProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyLockdownProfileAsync<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyLockdownProfileAsync(&*(&profileid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLockdownProfileInformation<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLockdownProfileInformation(&*(&profileid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceLockdownProfileStatics>,
            ::windows::core::GetTrustLevel,
            GetSupportedLockdownProfiles::<Impl, IMPL_OFFSET>,
            GetCurrentLockdownProfile::<Impl, IMPL_OFFSET>,
            ApplyLockdownProfileAsync::<Impl, IMPL_OFFSET>,
            GetLockdownProfileInformation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceLockdownProfileStatics as ::windows::core::Interface>::IID
    }
}

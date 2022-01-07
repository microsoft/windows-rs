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
    pub const fn new<Impl: IDeviceLockdownProfileInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceLockdownProfileInformationVtbl {
        unsafe extern "system" fn Name<Impl: IDeviceLockdownProfileInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceLockdownProfileInformation>, base.5, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceLockdownProfileStaticsImpl: Sized {
    fn GetSupportedLockdownProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
    fn GetCurrentLockdownProfile(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ApplyLockdownProfileAsync(&self, profileid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetLockdownProfileInformation(&self, profileid: &::windows::core::GUID) -> ::windows::core::Result<DeviceLockdownProfileInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceLockdownProfileStatics {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.IDeviceLockdownProfileStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceLockdownProfileStaticsVtbl {
    pub const fn new<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceLockdownProfileStaticsVtbl {
        unsafe extern "system" fn GetSupportedLockdownProfiles<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedLockdownProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLockdownProfile<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentLockdownProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyLockdownProfileAsync<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplyLockdownProfileAsync(&*(&profileid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLockdownProfileInformation<Impl: IDeviceLockdownProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLockdownProfileInformation(&*(&profileid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceLockdownProfileStatics>, base.5, GetSupportedLockdownProfiles::<Impl, OFFSET>, GetCurrentLockdownProfile::<Impl, OFFSET>, ApplyLockdownProfileAsync::<Impl, OFFSET>, GetLockdownProfileInformation::<Impl, OFFSET>)
    }
}

#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationInfoImpl: Sized {
    fn TenantId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TenantName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseKeyCredentialRegistrationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseKeyCredentialRegistrationInfoImpl, const OFFSET: isize>() -> IEnterpriseKeyCredentialRegistrationInfoVtbl {
        unsafe extern "system" fn TenantId<Impl: IEnterpriseKeyCredentialRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TenantId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TenantName<Impl: IEnterpriseKeyCredentialRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TenantName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IEnterpriseKeyCredentialRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyId<Impl: IEnterpriseKeyCredentialRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyName<Impl: IEnterpriseKeyCredentialRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterpriseKeyCredentialRegistrationInfo>, ::windows::core::GetTrustLevel, TenantId::<Impl, OFFSET>, TenantName::<Impl, OFFSET>, Subject::<Impl, OFFSET>, KeyId::<Impl, OFFSET>, KeyName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationManagerImpl: Sized {
    fn GetRegistrationsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseKeyCredentialRegistrationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseKeyCredentialRegistrationManagerImpl, const OFFSET: isize>() -> IEnterpriseKeyCredentialRegistrationManagerVtbl {
        unsafe extern "system" fn GetRegistrationsAsync<Impl: IEnterpriseKeyCredentialRegistrationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistrationsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterpriseKeyCredentialRegistrationManager>, ::windows::core::GetTrustLevel, GetRegistrationsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationManagerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<EnterpriseKeyCredentialRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseKeyCredentialRegistrationManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseKeyCredentialRegistrationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseKeyCredentialRegistrationManagerStaticsImpl, const OFFSET: isize>() -> IEnterpriseKeyCredentialRegistrationManagerStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IEnterpriseKeyCredentialRegistrationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnterpriseKeyCredentialRegistrationManagerStatics>, ::windows::core::GetTrustLevel, Current::<Impl, OFFSET>)
    }
}

#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationInfo_Impl: Sized {
    fn TenantId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TenantName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subject(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseKeyCredentialRegistrationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseKeyCredentialRegistrationInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnterpriseKeyCredentialRegistrationInfo_Vtbl {
        unsafe extern "system" fn TenantId<Impl: IEnterpriseKeyCredentialRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TenantName<Impl: IEnterpriseKeyCredentialRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Subject<Impl: IEnterpriseKeyCredentialRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyId<Impl: IEnterpriseKeyCredentialRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyName<Impl: IEnterpriseKeyCredentialRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnterpriseKeyCredentialRegistrationInfo, BASE_OFFSET>(),
            TenantId: TenantId::<Impl, IMPL_OFFSET>,
            TenantName: TenantName::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            KeyId: KeyId::<Impl, IMPL_OFFSET>,
            KeyName: KeyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnterpriseKeyCredentialRegistrationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEnterpriseKeyCredentialRegistrationManager_Impl: Sized {
    fn GetRegistrationsAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEnterpriseKeyCredentialRegistrationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseKeyCredentialRegistrationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnterpriseKeyCredentialRegistrationManager_Vtbl {
        unsafe extern "system" fn GetRegistrationsAsync<Impl: IEnterpriseKeyCredentialRegistrationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnterpriseKeyCredentialRegistrationManager, BASE_OFFSET>(),
            GetRegistrationsAsync: GetRegistrationsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnterpriseKeyCredentialRegistrationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationManagerStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<EnterpriseKeyCredentialRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnterpriseKeyCredentialRegistrationManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnterpriseKeyCredentialRegistrationManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IEnterpriseKeyCredentialRegistrationManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnterpriseKeyCredentialRegistrationManagerStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnterpriseKeyCredentialRegistrationManagerStatics as ::windows::core::Interface>::IID
    }
}

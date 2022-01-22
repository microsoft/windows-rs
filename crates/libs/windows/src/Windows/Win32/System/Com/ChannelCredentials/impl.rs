#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IChannelCredentials_Impl: Sized + super::IDispatch_Impl {
    fn SetWindowsCredential(&mut self, domain: &super::super::super::Foundation::BSTR, username: &super::super::super::Foundation::BSTR, password: &super::super::super::Foundation::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetUserNameCredential(&mut self, username: &super::super::super::Foundation::BSTR, password: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStore(&mut self, storelocation: &super::super::super::Foundation::BSTR, storename: &super::super::super::Foundation::BSTR, findyype: &super::super::super::Foundation::BSTR, findvalue: &super::VARIANT) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStoreByName(&mut self, subjectname: &super::super::super::Foundation::BSTR, storelocation: &super::super::super::Foundation::BSTR, storename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromFile(&mut self, filename: &super::super::super::Foundation::BSTR, password: &super::super::super::Foundation::BSTR, keystorageflags: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&mut self, storelocation: &super::super::super::Foundation::BSTR, storename: &super::super::super::Foundation::BSTR, findtype: &super::super::super::Foundation::BSTR, findvalue: &super::VARIANT) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&mut self, subjectname: &super::super::super::Foundation::BSTR, storelocation: &super::super::super::Foundation::BSTR, storename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&mut self, filename: &super::super::super::Foundation::BSTR, password: &super::super::super::Foundation::BSTR, keystorageflags: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetServiceCertificateAuthentication(&mut self, storelocation: &super::super::super::Foundation::BSTR, revocationmode: &super::super::super::Foundation::BSTR, certificatevalidationmode: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetIssuedToken(&mut self, localissueraddres: &super::super::super::Foundation::BSTR, localissuerbindingtype: &super::super::super::Foundation::BSTR, localissuerbinding: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>() -> IChannelCredentials_Vtbl {
        unsafe extern "system" fn SetWindowsCredential<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowsCredential(::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&impersonationlevel), ::core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserNameCredential(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientCertificateFromStore(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&findyype), ::core::mem::transmute_copy(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientCertificateFromStoreByName(::core::mem::transmute_copy(&subjectname), ::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientCertificateFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultServiceCertificateFromStore(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&findtype), ::core::mem::transmute_copy(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultServiceCertificateFromStoreByName(::core::mem::transmute_copy(&subjectname), ::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultServiceCertificateFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceCertificateAuthentication(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&revocationmode), ::core::mem::transmute_copy(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localissueraddres: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIssuedToken(::core::mem::transmute_copy(&localissueraddres), ::core::mem::transmute_copy(&localissuerbindingtype), ::core::mem::transmute_copy(&localissuerbinding)).into()
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Identity, Impl, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, Impl, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, Impl, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelCredentials as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}

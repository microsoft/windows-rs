#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IChannelCredentialsImpl: Sized + IDispatchImpl {
    fn SetWindowsCredential(&mut self, domain: super::super::super::Foundation::BSTR, username: super::super::super::Foundation::BSTR, password: super::super::super::Foundation::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetUserNameCredential(&mut self, username: super::super::super::Foundation::BSTR, password: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStore(&mut self, storelocation: super::super::super::Foundation::BSTR, storename: super::super::super::Foundation::BSTR, findyype: super::super::super::Foundation::BSTR, findvalue: super::VARIANT) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStoreByName(&mut self, subjectname: super::super::super::Foundation::BSTR, storelocation: super::super::super::Foundation::BSTR, storename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromFile(&mut self, filename: super::super::super::Foundation::BSTR, password: super::super::super::Foundation::BSTR, keystorageflags: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&mut self, storelocation: super::super::super::Foundation::BSTR, storename: super::super::super::Foundation::BSTR, findtype: super::super::super::Foundation::BSTR, findvalue: super::VARIANT) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&mut self, subjectname: super::super::super::Foundation::BSTR, storelocation: super::super::super::Foundation::BSTR, storename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&mut self, filename: super::super::super::Foundation::BSTR, password: super::super::super::Foundation::BSTR, keystorageflags: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetServiceCertificateAuthentication(&mut self, storelocation: super::super::super::Foundation::BSTR, revocationmode: super::super::super::Foundation::BSTR, certificatevalidationmode: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetIssuedToken(&mut self, localissueraddres: super::super::super::Foundation::BSTR, localissuerbindingtype: super::super::super::Foundation::BSTR, localissuerbinding: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IChannelCredentialsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentialsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChannelCredentialsVtbl {
        unsafe extern "system" fn SetWindowsCredential<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWindowsCredential(::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&impersonationlevel), ::core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserNameCredential(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificateFromStore(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&findyype), ::core::mem::transmute_copy(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificateFromStoreByName(::core::mem::transmute_copy(&subjectname), ::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificateFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultServiceCertificateFromStore(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&findtype), ::core::mem::transmute_copy(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultServiceCertificateFromStoreByName(::core::mem::transmute_copy(&subjectname), ::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultServiceCertificateFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceCertificateAuthentication(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&revocationmode), ::core::mem::transmute_copy(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localissueraddres: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIssuedToken(::core::mem::transmute_copy(&localissueraddres), ::core::mem::transmute_copy(&localissuerbindingtype), ::core::mem::transmute_copy(&localissuerbinding)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Impl, IMPL_OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Impl, IMPL_OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Impl, IMPL_OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Impl, IMPL_OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Impl, IMPL_OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Impl, IMPL_OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Impl, IMPL_OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Impl, IMPL_OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Impl, IMPL_OFFSET>,
            SetIssuedToken: SetIssuedToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelCredentials as ::windows::core::Interface>::IID
    }
}

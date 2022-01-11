#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IChannelCredentialsImpl: Sized + IDispatchImpl {
    fn SetWindowsCredential();
    fn SetUserNameCredential();
    fn SetClientCertificateFromStore();
    fn SetClientCertificateFromStoreByName();
    fn SetClientCertificateFromFile();
    fn SetDefaultServiceCertificateFromStore();
    fn SetDefaultServiceCertificateFromStoreByName();
    fn SetDefaultServiceCertificateFromFile();
    fn SetServiceCertificateAuthentication();
    fn SetIssuedToken();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IChannelCredentialsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelCredentialsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChannelCredentialsVtbl {
        unsafe extern "system" fn SetWindowsCredential<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserNameCredential<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIssuedToken<Impl: IChannelCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localissueraddres: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            SetWindowsCredential::<Impl, IMPL_OFFSET>,
            SetUserNameCredential::<Impl, IMPL_OFFSET>,
            SetClientCertificateFromStore::<Impl, IMPL_OFFSET>,
            SetClientCertificateFromStoreByName::<Impl, IMPL_OFFSET>,
            SetClientCertificateFromFile::<Impl, IMPL_OFFSET>,
            SetDefaultServiceCertificateFromStore::<Impl, IMPL_OFFSET>,
            SetDefaultServiceCertificateFromStoreByName::<Impl, IMPL_OFFSET>,
            SetDefaultServiceCertificateFromFile::<Impl, IMPL_OFFSET>,
            SetServiceCertificateAuthentication::<Impl, IMPL_OFFSET>,
            SetIssuedToken::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelCredentials as ::windows::core::Interface>::IID
    }
}

#[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IChannelCredentials_Impl: Sized + super::IDispatch_Impl {
    fn SetWindowsCredential(&self, domain: &::windows::core::BSTR, username: &::windows::core::BSTR, password: &::windows::core::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetUserNameCredential(&self, username: &::windows::core::BSTR, password: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStore(&self, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR, findyype: &::windows::core::BSTR, findvalue: &super::VARIANT) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStoreByName(&self, subjectname: &::windows::core::BSTR, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromFile(&self, filename: &::windows::core::BSTR, password: &::windows::core::BSTR, keystorageflags: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&self, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR, findtype: &::windows::core::BSTR, findvalue: &super::VARIANT) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &::windows::core::BSTR, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&self, filename: &::windows::core::BSTR, password: &::windows::core::BSTR, keystorageflags: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetServiceCertificateAuthentication(&self, storelocation: &::windows::core::BSTR, revocationmode: &::windows::core::BSTR, certificatevalidationmode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetIssuedToken(&self, localissueraddres: &::windows::core::BSTR, localissuerbindingtype: &::windows::core::BSTR, localissuerbinding: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IChannelCredentials {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>() -> IChannelCredentials_Vtbl {
        unsafe extern "system" fn SetWindowsCredential<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: ::std::mem::MaybeUninit<::windows::core::BSTR>, username: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowsCredential(::core::mem::transmute(&domain), ::core::mem::transmute(&username), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&impersonationlevel), ::core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserNameCredential(::core::mem::transmute(&username), ::core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>, findyype: ::std::mem::MaybeUninit<::windows::core::BSTR>, findvalue: super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificateFromStore(::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&findyype), ::core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows::core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificateFromStoreByName(::core::mem::transmute(&subjectname), ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificateFromFile(::core::mem::transmute(&filename), ::core::mem::transmute(&password), ::core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>, findtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, findvalue: super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultServiceCertificateFromStore(::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&findtype), ::core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows::core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultServiceCertificateFromStoreByName(::core::mem::transmute(&subjectname), ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultServiceCertificateFromFile(::core::mem::transmute(&filename), ::core::mem::transmute(&password), ::core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, revocationmode: ::std::mem::MaybeUninit<::windows::core::BSTR>, certificatevalidationmode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceCertificateAuthentication(::core::mem::transmute(&storelocation), ::core::mem::transmute(&revocationmode), ::core::mem::transmute(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localissueraddres: ::std::mem::MaybeUninit<::windows::core::BSTR>, localissuerbindingtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, localissuerbinding: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIssuedToken(::core::mem::transmute(&localissueraddres), ::core::mem::transmute(&localissuerbindingtype), ::core::mem::transmute(&localissuerbinding)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IChannelCredentials as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}

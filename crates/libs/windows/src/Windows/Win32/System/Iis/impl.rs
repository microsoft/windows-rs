#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpAuthenticationProviderImpl: Sized {
    fn Begin_AuthenticateUser(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_AuthenticateUser(&mut self, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpAuthenticationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthenticationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpAuthenticationProviderVtbl {
        unsafe extern "system" fn Begin_AuthenticateUser<Impl: AsyncIFtpAuthenticationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_AuthenticateUser(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        unsafe extern "system" fn Finish_AuthenticateUser<Impl: AsyncIFtpAuthenticationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_AuthenticateUser(::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_AuthenticateUser: Begin_AuthenticateUser::<Impl, IMPL_OFFSET>,
            Finish_AuthenticateUser: Finish_AuthenticateUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpAuthenticationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpAuthorizationProviderImpl: Sized {
    fn Begin_GetUserAccessPermission(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_GetUserAccessPermission(&mut self) -> ::windows::core::Result<FTP_ACCESS>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpAuthorizationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthorizationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpAuthorizationProviderVtbl {
        unsafe extern "system" fn Begin_GetUserAccessPermission<Impl: AsyncIFtpAuthorizationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetUserAccessPermission(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszvirtualpath), ::core::mem::transmute_copy(&pszusername)).into()
        }
        unsafe extern "system" fn Finish_GetUserAccessPermission<Impl: AsyncIFtpAuthorizationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetUserAccessPermission() {
                ::core::result::Result::Ok(ok__) => {
                    *pftpaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_GetUserAccessPermission: Begin_GetUserAccessPermission::<Impl, IMPL_OFFSET>,
            Finish_GetUserAccessPermission: Finish_GetUserAccessPermission::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpAuthorizationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpHomeDirectoryProviderImpl: Sized {
    fn Begin_GetUserHomeDirectoryData(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_GetUserHomeDirectoryData(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpHomeDirectoryProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpHomeDirectoryProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpHomeDirectoryProviderVtbl {
        unsafe extern "system" fn Begin_GetUserHomeDirectoryData<Impl: AsyncIFtpHomeDirectoryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetUserHomeDirectoryData(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername)).into()
        }
        unsafe extern "system" fn Finish_GetUserHomeDirectoryData<Impl: AsyncIFtpHomeDirectoryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomedirectorydata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetUserHomeDirectoryData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszhomedirectorydata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_GetUserHomeDirectoryData: Begin_GetUserHomeDirectoryData::<Impl, IMPL_OFFSET>,
            Finish_GetUserHomeDirectoryData: Finish_GetUserHomeDirectoryData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpHomeDirectoryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpLogProviderImpl: Sized {
    fn Begin_Log(&mut self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()>;
    fn Finish_Log(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpLogProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpLogProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpLogProviderVtbl {
        unsafe extern "system" fn Begin_Log<Impl: AsyncIFtpLogProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_Log(::core::mem::transmute_copy(&ploggingparameters)).into()
        }
        unsafe extern "system" fn Finish_Log<Impl: AsyncIFtpLogProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_Log().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_Log: Begin_Log::<Impl, IMPL_OFFSET>,
            Finish_Log: Finish_Log::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpLogProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpPostprocessProviderImpl: Sized {
    fn Begin_HandlePostprocess(&mut self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<()>;
    fn Finish_HandlePostprocess(&mut self) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpPostprocessProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPostprocessProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpPostprocessProviderVtbl {
        unsafe extern "system" fn Begin_HandlePostprocess<Impl: AsyncIFtpPostprocessProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_HandlePostprocess(::core::mem::transmute_copy(&ppostprocessparameters)).into()
        }
        unsafe extern "system" fn Finish_HandlePostprocess<Impl: AsyncIFtpPostprocessProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_HandlePostprocess() {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_HandlePostprocess: Begin_HandlePostprocess::<Impl, IMPL_OFFSET>,
            Finish_HandlePostprocess: Finish_HandlePostprocess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpPostprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpPreprocessProviderImpl: Sized {
    fn Begin_HandlePreprocess(&mut self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<()>;
    fn Finish_HandlePreprocess(&mut self) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpPreprocessProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPreprocessProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpPreprocessProviderVtbl {
        unsafe extern "system" fn Begin_HandlePreprocess<Impl: AsyncIFtpPreprocessProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_HandlePreprocess(::core::mem::transmute_copy(&ppreprocessparameters)).into()
        }
        unsafe extern "system" fn Finish_HandlePreprocess<Impl: AsyncIFtpPreprocessProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_HandlePreprocess() {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_HandlePreprocess: Begin_HandlePreprocess::<Impl, IMPL_OFFSET>,
            Finish_HandlePreprocess: Finish_HandlePreprocess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpPreprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpRoleProviderImpl: Sized {
    fn Begin_IsUserInRole(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_IsUserInRole(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpRoleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpRoleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIFtpRoleProviderVtbl {
        unsafe extern "system" fn Begin_IsUserInRole<Impl: AsyncIFtpRoleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_IsUserInRole(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszrole)).into()
        }
        unsafe extern "system" fn Finish_IsUserInRole<Impl: AsyncIFtpRoleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_IsUserInRole() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_IsUserInRole: Begin_IsUserInRole::<Impl, IMPL_OFFSET>,
            Finish_IsUserInRole: Finish_IsUserInRole::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpRoleProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIMSAdminBaseSinkWImpl: Sized {
    fn Begin_SinkNotify(&mut self, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::Result<()>;
    fn Finish_SinkNotify(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ShutdownNotify(&mut self) -> ::windows::core::Result<()>;
    fn Finish_ShutdownNotify(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIMSAdminBaseSinkWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMSAdminBaseSinkWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIMSAdminBaseSinkWVtbl {
        unsafe extern "system" fn Begin_SinkNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_SinkNotify(::core::mem::transmute_copy(&dwmdnumelements), ::core::mem::transmute_copy(&pcochangelist)).into()
        }
        unsafe extern "system" fn Finish_SinkNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_SinkNotify().into()
        }
        unsafe extern "system" fn Begin_ShutdownNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_ShutdownNotify().into()
        }
        unsafe extern "system" fn Finish_ShutdownNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_ShutdownNotify().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_SinkNotify: Begin_SinkNotify::<Impl, IMPL_OFFSET>,
            Finish_SinkNotify: Finish_SinkNotify::<Impl, IMPL_OFFSET>,
            Begin_ShutdownNotify: Begin_ShutdownNotify::<Impl, IMPL_OFFSET>,
            Finish_ShutdownNotify: Finish_ShutdownNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIMSAdminBaseSinkW as ::windows::core::Interface>::IID
    }
}
pub trait IADMEXTImpl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn EnumDcomCLSIDs(&mut self, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::Result<()>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
}
impl IADMEXTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADMEXTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADMEXTVtbl {
        unsafe extern "system" fn Initialize<Impl: IADMEXTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn EnumDcomCLSIDs<Impl: IADMEXTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDcomCLSIDs(::core::mem::transmute_copy(&pclsiddcom), ::core::mem::transmute_copy(&dwenumindex)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IADMEXTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            EnumDcomCLSIDs: EnumDcomCLSIDs::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADMEXT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpAuthenticationProviderImpl: Sized {
    fn AuthenticateUser(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpAuthenticationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpAuthenticationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpAuthenticationProviderVtbl {
        unsafe extern "system" fn AuthenticateUser<Impl: IFtpAuthenticationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthenticateUser(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), ::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AuthenticateUser: AuthenticateUser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpAuthenticationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpAuthorizationProviderImpl: Sized {
    fn GetUserAccessPermission(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<FTP_ACCESS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpAuthorizationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpAuthorizationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpAuthorizationProviderVtbl {
        unsafe extern "system" fn GetUserAccessPermission<Impl: IFtpAuthorizationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserAccessPermission(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszvirtualpath), ::core::mem::transmute_copy(&pszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pftpaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUserAccessPermission: GetUserAccessPermission::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpAuthorizationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpHomeDirectoryProviderImpl: Sized {
    fn GetUserHomeDirectoryData(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpHomeDirectoryProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpHomeDirectoryProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpHomeDirectoryProviderVtbl {
        unsafe extern "system" fn GetUserHomeDirectoryData<Impl: IFtpHomeDirectoryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, ppszhomedirectorydata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserHomeDirectoryData(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszhomedirectorydata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUserHomeDirectoryData: GetUserHomeDirectoryData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpHomeDirectoryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpLogProviderImpl: Sized {
    fn Log(&mut self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpLogProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpLogProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpLogProviderVtbl {
        unsafe extern "system" fn Log<Impl: IFtpLogProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Log(::core::mem::transmute_copy(&ploggingparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Log: Log::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpLogProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpPostprocessProviderImpl: Sized {
    fn HandlePostprocess(&mut self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpPostprocessProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpPostprocessProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpPostprocessProviderVtbl {
        unsafe extern "system" fn HandlePostprocess<Impl: IFtpPostprocessProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandlePostprocess(::core::mem::transmute_copy(&ppostprocessparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandlePostprocess: HandlePostprocess::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpPostprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpPreprocessProviderImpl: Sized {
    fn HandlePreprocess(&mut self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpPreprocessProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpPreprocessProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpPreprocessProviderVtbl {
        unsafe extern "system" fn HandlePreprocess<Impl: IFtpPreprocessProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandlePreprocess(::core::mem::transmute_copy(&ppreprocessparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandlePreprocess: HandlePreprocess::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpPreprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFtpProviderConstructImpl: Sized {
    fn Construct(&mut self, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFtpProviderConstructVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpProviderConstructImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpProviderConstructVtbl {
        unsafe extern "system" fn Construct<Impl: IFtpProviderConstructImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Construct(::core::mem::transmute_copy(&configurationentries)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Construct: Construct::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpProviderConstruct as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpRoleProviderImpl: Sized {
    fn IsUserInRole(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpRoleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpRoleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFtpRoleProviderVtbl {
        unsafe extern "system" fn IsUserInRole<Impl: IFtpRoleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserInRole(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsUserInRole: IsUserInRole::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpRoleProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBase2WImpl: Sized + IMSAdminBaseWImpl {
    fn BackupWithPasswd(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RestoreWithPasswd(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Export(&mut self, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn Import(&mut self, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, pszdestpath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn RestoreHistory(&mut self, pszmdhistorylocation: super::super::Foundation::PWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn EnumHistory(&mut self, pszmdhistorylocation: super::super::Foundation::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBase2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSAdminBase2WVtbl {
        unsafe extern "system" fn BackupWithPasswd<Impl: IMSAdminBase2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackupWithPasswd(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags), ::core::mem::transmute_copy(&pszpasswd)).into()
        }
        unsafe extern "system" fn RestoreWithPasswd<Impl: IMSAdminBase2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreWithPasswd(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags), ::core::mem::transmute_copy(&pszpasswd)).into()
        }
        unsafe extern "system" fn Export<Impl: IMSAdminBase2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Export(::core::mem::transmute_copy(&pszpasswd), ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&pszsourcepath), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn Import<Impl: IMSAdminBase2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, pszdestpath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&pszpasswd), ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&pszsourcepath), ::core::mem::transmute_copy(&pszdestpath), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn RestoreHistory<Impl: IMSAdminBase2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: super::super::Foundation::PWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreHistory(::core::mem::transmute_copy(&pszmdhistorylocation), ::core::mem::transmute_copy(&dwmdmajorversion), ::core::mem::transmute_copy(&dwmdminorversion), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn EnumHistory<Impl: IMSAdminBase2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: super::super::Foundation::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumHistory(::core::mem::transmute_copy(&pszmdhistorylocation), ::core::mem::transmute_copy(&pdwmdmajorversion), ::core::mem::transmute_copy(&pdwmdminorversion), ::core::mem::transmute_copy(&pftmdhistorytime), ::core::mem::transmute_copy(&dwmdenumindex)).into()
        }
        Self {
            base: IMSAdminBaseWVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BackupWithPasswd: BackupWithPasswd::<Impl, IMPL_OFFSET>,
            RestoreWithPasswd: RestoreWithPasswd::<Impl, IMPL_OFFSET>,
            Export: Export::<Impl, IMPL_OFFSET>,
            Import: Import::<Impl, IMPL_OFFSET>,
            RestoreHistory: RestoreHistory::<Impl, IMPL_OFFSET>,
            EnumHistory: EnumHistory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBase2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBase3WImpl: Sized + IMSAdminBaseWImpl + IMSAdminBase2WImpl {
    fn GetChildPaths(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, cchmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBase3WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase3WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSAdminBase3WVtbl {
        unsafe extern "system" fn GetChildPaths<Impl: IMSAdminBase3WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, cchmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChildPaths(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&cchmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pcchmdrequiredbuffersize)).into()
        }
        Self { base: IMSAdminBase2WVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetChildPaths: GetChildPaths::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBase3W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBaseSinkWImpl: Sized {
    fn SinkNotify(&mut self, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::Result<()>;
    fn ShutdownNotify(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBaseSinkWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseSinkWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSAdminBaseSinkWVtbl {
        unsafe extern "system" fn SinkNotify<Impl: IMSAdminBaseSinkWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SinkNotify(::core::mem::transmute_copy(&dwmdnumelements), ::core::mem::transmute_copy(&pcochangelist)).into()
        }
        unsafe extern "system" fn ShutdownNotify<Impl: IMSAdminBaseSinkWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownNotify().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SinkNotify: SinkNotify::<Impl, IMPL_OFFSET>,
            ShutdownNotify: ShutdownNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBaseSinkW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBaseWImpl: Sized {
    fn AddKey(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeleteKey(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeleteChildKeys(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumKeys(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdname: super::super::Foundation::PWSTR, dwmdenumobjectindex: u32) -> ::windows::core::Result<()>;
    fn CopyKey(&mut self, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RenameKey(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdnewname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetData(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()>;
    fn GetData(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>;
    fn DeleteData(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()>;
    fn EnumData(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>;
    fn GetAllData(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn DeleteAllData(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()>;
    fn CopyData(&mut self, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDataPaths(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn OpenKey(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32>;
    fn CloseKey(&mut self, hmdhandle: u32) -> ::windows::core::Result<()>;
    fn ChangePermissions(&mut self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()>;
    fn SaveData(&mut self) -> ::windows::core::Result<()>;
    fn GetHandleInfo(&mut self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO>;
    fn GetSystemChangeNumber(&mut self) -> ::windows::core::Result<u32>;
    fn GetDataSetNumber(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn SetLastChangeTime(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLastChangeTime(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn KeyExchangePhase1(&mut self) -> ::windows::core::Result<()>;
    fn KeyExchangePhase2(&mut self) -> ::windows::core::Result<()>;
    fn Backup(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn Restore(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn EnumBackups(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()>;
    fn DeleteBackup(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32) -> ::windows::core::Result<()>;
    fn UnmarshalInterface(&mut self) -> ::windows::core::Result<IMSAdminBaseW>;
    fn GetServerGuid(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBaseWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSAdminBaseWVtbl {
        unsafe extern "system" fn AddKey<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)).into()
        }
        unsafe extern "system" fn DeleteKey<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)).into()
        }
        unsafe extern "system" fn DeleteChildKeys<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteChildKeys(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)).into()
        }
        unsafe extern "system" fn EnumKeys<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdname: super::super::Foundation::PWSTR, dwmdenumobjectindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumKeys(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pszmdname), ::core::mem::transmute_copy(&dwmdenumobjectindex)).into()
        }
        unsafe extern "system" fn CopyKey<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyKey(::core::mem::transmute_copy(&hmdsourcehandle), ::core::mem::transmute_copy(&pszmdsourcepath), ::core::mem::transmute_copy(&hmddesthandle), ::core::mem::transmute_copy(&pszmddestpath), ::core::mem::transmute_copy(&bmdoverwriteflag), ::core::mem::transmute_copy(&bmdcopyflag)).into()
        }
        unsafe extern "system" fn RenameKey<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pszmdnewname)).into()
        }
        unsafe extern "system" fn SetData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata)).into()
        }
        unsafe extern "system" fn GetData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata), ::core::mem::transmute_copy(&pdwmdrequireddatalen)).into()
        }
        unsafe extern "system" fn DeleteData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdidentifier), ::core::mem::transmute_copy(&dwmddatatype)).into()
        }
        unsafe extern "system" fn EnumData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata), ::core::mem::transmute_copy(&dwmdenumdataindex), ::core::mem::transmute_copy(&pdwmdrequireddatalen)).into()
        }
        unsafe extern "system" fn GetAllData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .GetAllData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdattributes), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&pdwmdnumdataentries), ::core::mem::transmute_copy(&pdwmddatasetnumber), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pbmdbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize))
                .into()
        }
        unsafe extern "system" fn DeleteAllData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAllData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype)).into()
        }
        unsafe extern "system" fn CopyData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyData(::core::mem::transmute_copy(&hmdsourcehandle), ::core::mem::transmute_copy(&pszmdsourcepath), ::core::mem::transmute_copy(&hmddesthandle), ::core::mem::transmute_copy(&pszmddestpath), ::core::mem::transmute_copy(&dwmdattributes), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&bmdcopyflag)).into()
        }
        unsafe extern "system" fn GetDataPaths<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataPaths(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdidentifier), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        unsafe extern "system" fn OpenKey<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32, phmdnewhandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdaccessrequested), ::core::mem::transmute_copy(&dwmdtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *phmdnewhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseKey<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseKey(::core::mem::transmute_copy(&hmdhandle)).into()
        }
        unsafe extern "system" fn ChangePermissions<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangePermissions(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&dwmdtimeout), ::core::mem::transmute_copy(&dwmdaccessrequested)).into()
        }
        unsafe extern "system" fn SaveData<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveData().into()
        }
        unsafe extern "system" fn GetHandleInfo<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pmdhiinfo: *mut METADATA_HANDLE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHandleInfo(::core::mem::transmute_copy(&hmdhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmdhiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemChangeNumber<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsystemchangenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsystemchangenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataSetNumber<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pdwmddatasetnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataSetNumber(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmddatasetnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastChangeTime<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastChangeTime(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pftmdlastchangetime), ::core::mem::transmute_copy(&blocaltime)).into()
        }
        unsafe extern "system" fn GetLastChangeTime<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLastChangeTime(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pftmdlastchangetime), ::core::mem::transmute_copy(&blocaltime)).into()
        }
        unsafe extern "system" fn KeyExchangePhase1<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyExchangePhase1().into()
        }
        unsafe extern "system" fn KeyExchangePhase2<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyExchangePhase2().into()
        }
        unsafe extern "system" fn Backup<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn Restore<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn EnumBackups<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumBackups(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&pdwmdversion), ::core::mem::transmute_copy(&pftmdbackuptime), ::core::mem::transmute_copy(&dwmdenumindex)).into()
        }
        unsafe extern "system" fn DeleteBackup<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteBackup(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion)).into()
        }
        unsafe extern "system" fn UnmarshalInterface<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadmbwinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnmarshalInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *piadmbwinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerGuid<Impl: IMSAdminBaseWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetServerGuid().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddKey: AddKey::<Impl, IMPL_OFFSET>,
            DeleteKey: DeleteKey::<Impl, IMPL_OFFSET>,
            DeleteChildKeys: DeleteChildKeys::<Impl, IMPL_OFFSET>,
            EnumKeys: EnumKeys::<Impl, IMPL_OFFSET>,
            CopyKey: CopyKey::<Impl, IMPL_OFFSET>,
            RenameKey: RenameKey::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            DeleteData: DeleteData::<Impl, IMPL_OFFSET>,
            EnumData: EnumData::<Impl, IMPL_OFFSET>,
            GetAllData: GetAllData::<Impl, IMPL_OFFSET>,
            DeleteAllData: DeleteAllData::<Impl, IMPL_OFFSET>,
            CopyData: CopyData::<Impl, IMPL_OFFSET>,
            GetDataPaths: GetDataPaths::<Impl, IMPL_OFFSET>,
            OpenKey: OpenKey::<Impl, IMPL_OFFSET>,
            CloseKey: CloseKey::<Impl, IMPL_OFFSET>,
            ChangePermissions: ChangePermissions::<Impl, IMPL_OFFSET>,
            SaveData: SaveData::<Impl, IMPL_OFFSET>,
            GetHandleInfo: GetHandleInfo::<Impl, IMPL_OFFSET>,
            GetSystemChangeNumber: GetSystemChangeNumber::<Impl, IMPL_OFFSET>,
            GetDataSetNumber: GetDataSetNumber::<Impl, IMPL_OFFSET>,
            SetLastChangeTime: SetLastChangeTime::<Impl, IMPL_OFFSET>,
            GetLastChangeTime: GetLastChangeTime::<Impl, IMPL_OFFSET>,
            KeyExchangePhase1: KeyExchangePhase1::<Impl, IMPL_OFFSET>,
            KeyExchangePhase2: KeyExchangePhase2::<Impl, IMPL_OFFSET>,
            Backup: Backup::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            EnumBackups: EnumBackups::<Impl, IMPL_OFFSET>,
            DeleteBackup: DeleteBackup::<Impl, IMPL_OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Impl, IMPL_OFFSET>,
            GetServerGuid: GetServerGuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBaseW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSImpExpHelpWImpl: Sized {
    fn EnumeratePathsInFile(&mut self, pszfilename: super::super::Foundation::PWSTR, pszkeytype: super::super::Foundation::PWSTR, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSImpExpHelpWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSImpExpHelpWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSImpExpHelpWVtbl {
        unsafe extern "system" fn EnumeratePathsInFile<Impl: IMSImpExpHelpWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, pszkeytype: super::super::Foundation::PWSTR, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumeratePathsInFile(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&pszkeytype), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EnumeratePathsInFile: EnumeratePathsInFile::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSImpExpHelpW as ::windows::core::Interface>::IID
    }
}

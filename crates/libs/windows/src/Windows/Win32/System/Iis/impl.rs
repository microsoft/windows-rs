#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpAuthenticationProvider_Impl: Sized {
    fn Begin_AuthenticateUser(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_AuthenticateUser(&mut self, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpAuthenticationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthenticationProvider_Impl, const OFFSET: isize>() -> AsyncIFtpAuthenticationProvider_Vtbl {
        unsafe extern "system" fn Begin_AuthenticateUser<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthenticationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_AuthenticateUser(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        unsafe extern "system" fn Finish_AuthenticateUser<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthenticationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_AuthenticateUser(::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_AuthenticateUser: Begin_AuthenticateUser::<Identity, Impl, OFFSET>,
            Finish_AuthenticateUser: Finish_AuthenticateUser::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpAuthenticationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpAuthorizationProvider_Impl: Sized {
    fn Begin_GetUserAccessPermission(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_GetUserAccessPermission(&mut self) -> ::windows::core::Result<FTP_ACCESS>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpAuthorizationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthorizationProvider_Impl, const OFFSET: isize>() -> AsyncIFtpAuthorizationProvider_Vtbl {
        unsafe extern "system" fn Begin_GetUserAccessPermission<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthorizationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetUserAccessPermission(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszvirtualpath), ::core::mem::transmute_copy(&pszusername)).into()
        }
        unsafe extern "system" fn Finish_GetUserAccessPermission<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpAuthorizationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_GetUserAccessPermission() {
                ::core::result::Result::Ok(ok__) => {
                    *pftpaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_GetUserAccessPermission: Begin_GetUserAccessPermission::<Identity, Impl, OFFSET>,
            Finish_GetUserAccessPermission: Finish_GetUserAccessPermission::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpAuthorizationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpHomeDirectoryProvider_Impl: Sized {
    fn Begin_GetUserHomeDirectoryData(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_GetUserHomeDirectoryData(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpHomeDirectoryProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpHomeDirectoryProvider_Impl, const OFFSET: isize>() -> AsyncIFtpHomeDirectoryProvider_Vtbl {
        unsafe extern "system" fn Begin_GetUserHomeDirectoryData<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpHomeDirectoryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetUserHomeDirectoryData(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername)).into()
        }
        unsafe extern "system" fn Finish_GetUserHomeDirectoryData<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpHomeDirectoryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomedirectorydata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_GetUserHomeDirectoryData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszhomedirectorydata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_GetUserHomeDirectoryData: Begin_GetUserHomeDirectoryData::<Identity, Impl, OFFSET>,
            Finish_GetUserHomeDirectoryData: Finish_GetUserHomeDirectoryData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpHomeDirectoryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpLogProvider_Impl: Sized {
    fn Begin_Log(&mut self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()>;
    fn Finish_Log(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpLogProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpLogProvider_Impl, const OFFSET: isize>() -> AsyncIFtpLogProvider_Vtbl {
        unsafe extern "system" fn Begin_Log<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpLogProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_Log(::core::mem::transmute_copy(&ploggingparameters)).into()
        }
        unsafe extern "system" fn Finish_Log<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpLogProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_Log().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_Log: Begin_Log::<Identity, Impl, OFFSET>,
            Finish_Log: Finish_Log::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpLogProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpPostprocessProvider_Impl: Sized {
    fn Begin_HandlePostprocess(&mut self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<()>;
    fn Finish_HandlePostprocess(&mut self) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpPostprocessProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPostprocessProvider_Impl, const OFFSET: isize>() -> AsyncIFtpPostprocessProvider_Vtbl {
        unsafe extern "system" fn Begin_HandlePostprocess<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPostprocessProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_HandlePostprocess(::core::mem::transmute_copy(&ppostprocessparameters)).into()
        }
        unsafe extern "system" fn Finish_HandlePostprocess<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPostprocessProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_HandlePostprocess() {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_HandlePostprocess: Begin_HandlePostprocess::<Identity, Impl, OFFSET>,
            Finish_HandlePostprocess: Finish_HandlePostprocess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpPostprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpPreprocessProvider_Impl: Sized {
    fn Begin_HandlePreprocess(&mut self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<()>;
    fn Finish_HandlePreprocess(&mut self) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpPreprocessProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPreprocessProvider_Impl, const OFFSET: isize>() -> AsyncIFtpPreprocessProvider_Vtbl {
        unsafe extern "system" fn Begin_HandlePreprocess<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPreprocessProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_HandlePreprocess(::core::mem::transmute_copy(&ppreprocessparameters)).into()
        }
        unsafe extern "system" fn Finish_HandlePreprocess<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpPreprocessProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_HandlePreprocess() {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_HandlePreprocess: Begin_HandlePreprocess::<Identity, Impl, OFFSET>,
            Finish_HandlePreprocess: Finish_HandlePreprocess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpPreprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpRoleProvider_Impl: Sized {
    fn Begin_IsUserInRole(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_IsUserInRole(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIFtpRoleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpRoleProvider_Impl, const OFFSET: isize>() -> AsyncIFtpRoleProvider_Vtbl {
        unsafe extern "system" fn Begin_IsUserInRole<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpRoleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_IsUserInRole(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszrole)).into()
        }
        unsafe extern "system" fn Finish_IsUserInRole<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIFtpRoleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_IsUserInRole() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_IsUserInRole: Begin_IsUserInRole::<Identity, Impl, OFFSET>,
            Finish_IsUserInRole: Finish_IsUserInRole::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIFtpRoleProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIMSAdminBaseSinkW_Impl: Sized {
    fn Begin_SinkNotify(&mut self, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::Result<()>;
    fn Finish_SinkNotify(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ShutdownNotify(&mut self) -> ::windows::core::Result<()>;
    fn Finish_ShutdownNotify(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIMSAdminBaseSinkW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: isize>() -> AsyncIMSAdminBaseSinkW_Vtbl {
        unsafe extern "system" fn Begin_SinkNotify<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_SinkNotify(::core::mem::transmute_copy(&dwmdnumelements), ::core::mem::transmute_copy(&pcochangelist)).into()
        }
        unsafe extern "system" fn Finish_SinkNotify<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_SinkNotify().into()
        }
        unsafe extern "system" fn Begin_ShutdownNotify<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_ShutdownNotify().into()
        }
        unsafe extern "system" fn Finish_ShutdownNotify<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_ShutdownNotify().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_SinkNotify: Begin_SinkNotify::<Identity, Impl, OFFSET>,
            Finish_SinkNotify: Finish_SinkNotify::<Identity, Impl, OFFSET>,
            Begin_ShutdownNotify: Begin_ShutdownNotify::<Identity, Impl, OFFSET>,
            Finish_ShutdownNotify: Finish_ShutdownNotify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIMSAdminBaseSinkW as ::windows::core::Interface>::IID
    }
}
pub trait IADMEXT_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn EnumDcomCLSIDs(&mut self, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::Result<()>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
}
impl IADMEXT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADMEXT_Impl, const OFFSET: isize>() -> IADMEXT_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IADMEXT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn EnumDcomCLSIDs<Identity: ::windows::core::IUnknownImpl, Impl: IADMEXT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDcomCLSIDs(::core::mem::transmute_copy(&pclsiddcom), ::core::mem::transmute_copy(&dwenumindex)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IADMEXT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            EnumDcomCLSIDs: EnumDcomCLSIDs::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADMEXT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpAuthenticationProvider_Impl: Sized {
    fn AuthenticateUser(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpAuthenticationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpAuthenticationProvider_Impl, const OFFSET: isize>() -> IFtpAuthenticationProvider_Vtbl {
        unsafe extern "system" fn AuthenticateUser<Identity: ::windows::core::IUnknownImpl, Impl: IFtpAuthenticationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AuthenticateUser(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), ::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AuthenticateUser: AuthenticateUser::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpAuthenticationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpAuthorizationProvider_Impl: Sized {
    fn GetUserAccessPermission(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<FTP_ACCESS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpAuthorizationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpAuthorizationProvider_Impl, const OFFSET: isize>() -> IFtpAuthorizationProvider_Vtbl {
        unsafe extern "system" fn GetUserAccessPermission<Identity: ::windows::core::IUnknownImpl, Impl: IFtpAuthorizationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUserAccessPermission(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszvirtualpath), ::core::mem::transmute_copy(&pszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pftpaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetUserAccessPermission: GetUserAccessPermission::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpAuthorizationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpHomeDirectoryProvider_Impl: Sized {
    fn GetUserHomeDirectoryData(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpHomeDirectoryProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpHomeDirectoryProvider_Impl, const OFFSET: isize>() -> IFtpHomeDirectoryProvider_Vtbl {
        unsafe extern "system" fn GetUserHomeDirectoryData<Identity: ::windows::core::IUnknownImpl, Impl: IFtpHomeDirectoryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, ppszhomedirectorydata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUserHomeDirectoryData(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszhomedirectorydata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetUserHomeDirectoryData: GetUserHomeDirectoryData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpHomeDirectoryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpLogProvider_Impl: Sized {
    fn Log(&mut self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpLogProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpLogProvider_Impl, const OFFSET: isize>() -> IFtpLogProvider_Vtbl {
        unsafe extern "system" fn Log<Identity: ::windows::core::IUnknownImpl, Impl: IFtpLogProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Log(::core::mem::transmute_copy(&ploggingparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Log: Log::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpLogProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpPostprocessProvider_Impl: Sized {
    fn HandlePostprocess(&mut self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpPostprocessProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpPostprocessProvider_Impl, const OFFSET: isize>() -> IFtpPostprocessProvider_Vtbl {
        unsafe extern "system" fn HandlePostprocess<Identity: ::windows::core::IUnknownImpl, Impl: IFtpPostprocessProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HandlePostprocess(::core::mem::transmute_copy(&ppostprocessparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandlePostprocess: HandlePostprocess::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpPostprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpPreprocessProvider_Impl: Sized {
    fn HandlePreprocess(&mut self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpPreprocessProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpPreprocessProvider_Impl, const OFFSET: isize>() -> IFtpPreprocessProvider_Vtbl {
        unsafe extern "system" fn HandlePreprocess<Identity: ::windows::core::IUnknownImpl, Impl: IFtpPreprocessProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HandlePreprocess(::core::mem::transmute_copy(&ppreprocessparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *pftpprocessstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandlePreprocess: HandlePreprocess::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpPreprocessProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFtpProviderConstruct_Impl: Sized {
    fn Construct(&mut self, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFtpProviderConstruct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpProviderConstruct_Impl, const OFFSET: isize>() -> IFtpProviderConstruct_Vtbl {
        unsafe extern "system" fn Construct<Identity: ::windows::core::IUnknownImpl, Impl: IFtpProviderConstruct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Construct(::core::mem::transmute_copy(&configurationentries)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpProviderConstruct as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpRoleProvider_Impl: Sized {
    fn IsUserInRole(&mut self, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFtpRoleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFtpRoleProvider_Impl, const OFFSET: isize>() -> IFtpRoleProvider_Vtbl {
        unsafe extern "system" fn IsUserInRole<Identity: ::windows::core::IUnknownImpl, Impl: IFtpRoleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUserInRole(::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pszsitename), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsUserInRole: IsUserInRole::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFtpRoleProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBase2W_Impl: Sized + IMSAdminBaseW_Impl {
    fn BackupWithPasswd(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RestoreWithPasswd(&mut self, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Export(&mut self, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn Import(&mut self, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, pszdestpath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn RestoreHistory(&mut self, pszmdhistorylocation: super::super::Foundation::PWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>;
    fn EnumHistory(&mut self, pszmdhistorylocation: super::super::Foundation::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBase2W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>() -> IMSAdminBase2W_Vtbl {
        unsafe extern "system" fn BackupWithPasswd<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackupWithPasswd(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags), ::core::mem::transmute_copy(&pszpasswd)).into()
        }
        unsafe extern "system" fn RestoreWithPasswd<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreWithPasswd(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags), ::core::mem::transmute_copy(&pszpasswd)).into()
        }
        unsafe extern "system" fn Export<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Export(::core::mem::transmute_copy(&pszpasswd), ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&pszsourcepath), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, pszdestpath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&pszpasswd), ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&pszsourcepath), ::core::mem::transmute_copy(&pszdestpath), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn RestoreHistory<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: super::super::Foundation::PWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreHistory(::core::mem::transmute_copy(&pszmdhistorylocation), ::core::mem::transmute_copy(&dwmdmajorversion), ::core::mem::transmute_copy(&dwmdminorversion), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn EnumHistory<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: super::super::Foundation::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumHistory(::core::mem::transmute_copy(&pszmdhistorylocation), ::core::mem::transmute_copy(&pdwmdmajorversion), ::core::mem::transmute_copy(&pdwmdminorversion), ::core::mem::transmute_copy(&pftmdhistorytime), ::core::mem::transmute_copy(&dwmdenumindex)).into()
        }
        Self {
            base: IMSAdminBaseW_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupWithPasswd: BackupWithPasswd::<Identity, Impl, OFFSET>,
            RestoreWithPasswd: RestoreWithPasswd::<Identity, Impl, OFFSET>,
            Export: Export::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            RestoreHistory: RestoreHistory::<Identity, Impl, OFFSET>,
            EnumHistory: EnumHistory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBase2W as ::windows::core::Interface>::IID || iid == &<IMSAdminBaseW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBase3W_Impl: Sized + IMSAdminBaseW_Impl + IMSAdminBase2W_Impl {
    fn GetChildPaths(&mut self, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, cchmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBase3W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase3W_Impl, const OFFSET: isize>() -> IMSAdminBase3W_Vtbl {
        unsafe extern "system" fn GetChildPaths<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBase3W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, cchmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChildPaths(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&cchmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pcchmdrequiredbuffersize)).into()
        }
        Self { base: IMSAdminBase2W_Vtbl::new::<Identity, Impl, OFFSET>(), GetChildPaths: GetChildPaths::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBase3W as ::windows::core::Interface>::IID || iid == &<IMSAdminBaseW as ::windows::core::Interface>::IID || iid == &<IMSAdminBase2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBaseSinkW_Impl: Sized {
    fn SinkNotify(&mut self, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::Result<()>;
    fn ShutdownNotify(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSAdminBaseSinkW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseSinkW_Impl, const OFFSET: isize>() -> IMSAdminBaseSinkW_Vtbl {
        unsafe extern "system" fn SinkNotify<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseSinkW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SinkNotify(::core::mem::transmute_copy(&dwmdnumelements), ::core::mem::transmute_copy(&pcochangelist)).into()
        }
        unsafe extern "system" fn ShutdownNotify<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseSinkW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShutdownNotify().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SinkNotify: SinkNotify::<Identity, Impl, OFFSET>,
            ShutdownNotify: ShutdownNotify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBaseSinkW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBaseW_Impl: Sized {
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
impl IMSAdminBaseW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>() -> IMSAdminBaseW_Vtbl {
        unsafe extern "system" fn AddKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)).into()
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)).into()
        }
        unsafe extern "system" fn DeleteChildKeys<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteChildKeys(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdname: super::super::Foundation::PWSTR, dwmdenumobjectindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumKeys(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pszmdname), ::core::mem::transmute_copy(&dwmdenumobjectindex)).into()
        }
        unsafe extern "system" fn CopyKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyKey(::core::mem::transmute_copy(&hmdsourcehandle), ::core::mem::transmute_copy(&pszmdsourcepath), ::core::mem::transmute_copy(&hmddesthandle), ::core::mem::transmute_copy(&pszmddestpath), ::core::mem::transmute_copy(&bmdoverwriteflag), ::core::mem::transmute_copy(&bmdcopyflag)).into()
        }
        unsafe extern "system" fn RenameKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenameKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pszmdnewname)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata), ::core::mem::transmute_copy(&pdwmdrequireddatalen)).into()
        }
        unsafe extern "system" fn DeleteData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdidentifier), ::core::mem::transmute_copy(&dwmddatatype)).into()
        }
        unsafe extern "system" fn EnumData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata), ::core::mem::transmute_copy(&dwmdenumdataindex), ::core::mem::transmute_copy(&pdwmdrequireddatalen)).into()
        }
        unsafe extern "system" fn GetAllData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .GetAllData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdattributes), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&pdwmdnumdataentries), ::core::mem::transmute_copy(&pdwmddatasetnumber), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pbmdbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize))
                .into()
        }
        unsafe extern "system" fn DeleteAllData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAllData(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype)).into()
        }
        unsafe extern "system" fn CopyData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyData(::core::mem::transmute_copy(&hmdsourcehandle), ::core::mem::transmute_copy(&pszmdsourcepath), ::core::mem::transmute_copy(&hmddesthandle), ::core::mem::transmute_copy(&pszmddestpath), ::core::mem::transmute_copy(&dwmdattributes), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&bmdcopyflag)).into()
        }
        unsafe extern "system" fn GetDataPaths<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataPaths(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdidentifier), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32, phmdnewhandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenKey(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&dwmdaccessrequested), ::core::mem::transmute_copy(&dwmdtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *phmdnewhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseKey(::core::mem::transmute_copy(&hmdhandle)).into()
        }
        unsafe extern "system" fn ChangePermissions<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangePermissions(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&dwmdtimeout), ::core::mem::transmute_copy(&dwmdaccessrequested)).into()
        }
        unsafe extern "system" fn SaveData<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveData().into()
        }
        unsafe extern "system" fn GetHandleInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pmdhiinfo: *mut METADATA_HANDLE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHandleInfo(::core::mem::transmute_copy(&hmdhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmdhiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemChangeNumber<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsystemchangenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSystemChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsystemchangenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataSetNumber<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pdwmddatasetnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataSetNumber(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmddatasetnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastChangeTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLastChangeTime(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pftmdlastchangetime), ::core::mem::transmute_copy(&blocaltime)).into()
        }
        unsafe extern "system" fn GetLastChangeTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastChangeTime(::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&pszmdpath), ::core::mem::transmute_copy(&pftmdlastchangetime), ::core::mem::transmute_copy(&blocaltime)).into()
        }
        unsafe extern "system" fn KeyExchangePhase1<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KeyExchangePhase1().into()
        }
        unsafe extern "system" fn KeyExchangePhase2<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KeyExchangePhase2().into()
        }
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restore(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn EnumBackups<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumBackups(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&pdwmdversion), ::core::mem::transmute_copy(&pftmdbackuptime), ::core::mem::transmute_copy(&dwmdenumindex)).into()
        }
        unsafe extern "system" fn DeleteBackup<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteBackup(::core::mem::transmute_copy(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion)).into()
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadmbwinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnmarshalInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *piadmbwinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSAdminBaseW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetServerGuid().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddKey: AddKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteChildKeys: DeleteChildKeys::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            CopyKey: CopyKey::<Identity, Impl, OFFSET>,
            RenameKey: RenameKey::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            DeleteData: DeleteData::<Identity, Impl, OFFSET>,
            EnumData: EnumData::<Identity, Impl, OFFSET>,
            GetAllData: GetAllData::<Identity, Impl, OFFSET>,
            DeleteAllData: DeleteAllData::<Identity, Impl, OFFSET>,
            CopyData: CopyData::<Identity, Impl, OFFSET>,
            GetDataPaths: GetDataPaths::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CloseKey: CloseKey::<Identity, Impl, OFFSET>,
            ChangePermissions: ChangePermissions::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
            GetHandleInfo: GetHandleInfo::<Identity, Impl, OFFSET>,
            GetSystemChangeNumber: GetSystemChangeNumber::<Identity, Impl, OFFSET>,
            GetDataSetNumber: GetDataSetNumber::<Identity, Impl, OFFSET>,
            SetLastChangeTime: SetLastChangeTime::<Identity, Impl, OFFSET>,
            GetLastChangeTime: GetLastChangeTime::<Identity, Impl, OFFSET>,
            KeyExchangePhase1: KeyExchangePhase1::<Identity, Impl, OFFSET>,
            KeyExchangePhase2: KeyExchangePhase2::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            EnumBackups: EnumBackups::<Identity, Impl, OFFSET>,
            DeleteBackup: DeleteBackup::<Identity, Impl, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, Impl, OFFSET>,
            GetServerGuid: GetServerGuid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSAdminBaseW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMSImpExpHelpW_Impl: Sized {
    fn EnumeratePathsInFile(&mut self, pszfilename: super::super::Foundation::PWSTR, pszkeytype: super::super::Foundation::PWSTR, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMSImpExpHelpW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSImpExpHelpW_Impl, const OFFSET: isize>() -> IMSImpExpHelpW_Vtbl {
        unsafe extern "system" fn EnumeratePathsInFile<Identity: ::windows::core::IUnknownImpl, Impl: IMSImpExpHelpW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, pszkeytype: super::super::Foundation::PWSTR, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumeratePathsInFile(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&pszkeytype), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), EnumeratePathsInFile: EnumeratePathsInFile::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSImpExpHelpW as ::windows::core::Interface>::IID
    }
}

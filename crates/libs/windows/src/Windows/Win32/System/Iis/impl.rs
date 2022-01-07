pub trait AsyncIFtpAuthenticationProviderImpl: Sized {
    fn Begin_AuthenticateUser();
    fn Finish_AuthenticateUser();
}
impl ::windows::core::RuntimeName for AsyncIFtpAuthenticationProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpAuthenticationProvider";
}
impl AsyncIFtpAuthenticationProviderVtbl {
    pub const fn new<Impl: AsyncIFtpAuthenticationProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpAuthenticationProviderVtbl {
        unsafe extern "system" fn Begin_AuthenticateUser<Impl: AsyncIFtpAuthenticationProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_AuthenticateUser(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszpassword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_AuthenticateUser<Impl: AsyncIFtpAuthenticationProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_AuthenticateUser(::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpAuthenticationProvider>, base.5, Begin_AuthenticateUser::<Impl, OFFSET>, Finish_AuthenticateUser::<Impl, OFFSET>)
    }
}
pub trait AsyncIFtpAuthorizationProviderImpl: Sized {
    fn Begin_GetUserAccessPermission();
    fn Finish_GetUserAccessPermission();
}
impl ::windows::core::RuntimeName for AsyncIFtpAuthorizationProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpAuthorizationProvider";
}
impl AsyncIFtpAuthorizationProviderVtbl {
    pub const fn new<Impl: AsyncIFtpAuthorizationProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpAuthorizationProviderVtbl {
        unsafe extern "system" fn Begin_GetUserAccessPermission<Impl: AsyncIFtpAuthorizationProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_GetUserAccessPermission(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszvirtualpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetUserAccessPermission<Impl: AsyncIFtpAuthorizationProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_GetUserAccessPermission(::core::mem::transmute_copy(&pftpaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpAuthorizationProvider>, base.5, Begin_GetUserAccessPermission::<Impl, OFFSET>, Finish_GetUserAccessPermission::<Impl, OFFSET>)
    }
}
pub trait AsyncIFtpHomeDirectoryProviderImpl: Sized {
    fn Begin_GetUserHomeDirectoryData();
    fn Finish_GetUserHomeDirectoryData();
}
impl ::windows::core::RuntimeName for AsyncIFtpHomeDirectoryProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpHomeDirectoryProvider";
}
impl AsyncIFtpHomeDirectoryProviderVtbl {
    pub const fn new<Impl: AsyncIFtpHomeDirectoryProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpHomeDirectoryProviderVtbl {
        unsafe extern "system" fn Begin_GetUserHomeDirectoryData<Impl: AsyncIFtpHomeDirectoryProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_GetUserHomeDirectoryData(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetUserHomeDirectoryData<Impl: AsyncIFtpHomeDirectoryProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszhomedirectorydata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_GetUserHomeDirectoryData(::core::mem::transmute_copy(&ppszhomedirectorydata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpHomeDirectoryProvider>, base.5, Begin_GetUserHomeDirectoryData::<Impl, OFFSET>, Finish_GetUserHomeDirectoryData::<Impl, OFFSET>)
    }
}
pub trait AsyncIFtpLogProviderImpl: Sized {
    fn Begin_Log();
    fn Finish_Log();
}
impl ::windows::core::RuntimeName for AsyncIFtpLogProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpLogProvider";
}
impl AsyncIFtpLogProviderVtbl {
    pub const fn new<Impl: AsyncIFtpLogProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpLogProviderVtbl {
        unsafe extern "system" fn Begin_Log<Impl: AsyncIFtpLogProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_Log(&*(&ploggingparameters as *const <LOGGING_PARAMETERS as ::windows::core::Abi>::Abi as *const <LOGGING_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Log<Impl: AsyncIFtpLogProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_Log() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpLogProvider>, base.5, Begin_Log::<Impl, OFFSET>, Finish_Log::<Impl, OFFSET>)
    }
}
pub trait AsyncIFtpPostprocessProviderImpl: Sized {
    fn Begin_HandlePostprocess();
    fn Finish_HandlePostprocess();
}
impl ::windows::core::RuntimeName for AsyncIFtpPostprocessProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpPostprocessProvider";
}
impl AsyncIFtpPostprocessProviderVtbl {
    pub const fn new<Impl: AsyncIFtpPostprocessProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpPostprocessProviderVtbl {
        unsafe extern "system" fn Begin_HandlePostprocess<Impl: AsyncIFtpPostprocessProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_HandlePostprocess(&*(&ppostprocessparameters as *const <POST_PROCESS_PARAMETERS as ::windows::core::Abi>::Abi as *const <POST_PROCESS_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_HandlePostprocess<Impl: AsyncIFtpPostprocessProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_HandlePostprocess(::core::mem::transmute_copy(&pftpprocessstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpPostprocessProvider>, base.5, Begin_HandlePostprocess::<Impl, OFFSET>, Finish_HandlePostprocess::<Impl, OFFSET>)
    }
}
pub trait AsyncIFtpPreprocessProviderImpl: Sized {
    fn Begin_HandlePreprocess();
    fn Finish_HandlePreprocess();
}
impl ::windows::core::RuntimeName for AsyncIFtpPreprocessProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpPreprocessProvider";
}
impl AsyncIFtpPreprocessProviderVtbl {
    pub const fn new<Impl: AsyncIFtpPreprocessProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpPreprocessProviderVtbl {
        unsafe extern "system" fn Begin_HandlePreprocess<Impl: AsyncIFtpPreprocessProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_HandlePreprocess(&*(&ppreprocessparameters as *const <PRE_PROCESS_PARAMETERS as ::windows::core::Abi>::Abi as *const <PRE_PROCESS_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_HandlePreprocess<Impl: AsyncIFtpPreprocessProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_HandlePreprocess(::core::mem::transmute_copy(&pftpprocessstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpPreprocessProvider>, base.5, Begin_HandlePreprocess::<Impl, OFFSET>, Finish_HandlePreprocess::<Impl, OFFSET>)
    }
}
pub trait AsyncIFtpRoleProviderImpl: Sized {
    fn Begin_IsUserInRole();
    fn Finish_IsUserInRole();
}
impl ::windows::core::RuntimeName for AsyncIFtpRoleProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIFtpRoleProvider";
}
impl AsyncIFtpRoleProviderVtbl {
    pub const fn new<Impl: AsyncIFtpRoleProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIFtpRoleProviderVtbl {
        unsafe extern "system" fn Begin_IsUserInRole<Impl: AsyncIFtpRoleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_IsUserInRole(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszrole as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_IsUserInRole<Impl: AsyncIFtpRoleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_IsUserInRole(::core::mem::transmute_copy(&pfisinrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIFtpRoleProvider>, base.5, Begin_IsUserInRole::<Impl, OFFSET>, Finish_IsUserInRole::<Impl, OFFSET>)
    }
}
pub trait AsyncIMSAdminBaseSinkWImpl: Sized {
    fn Begin_SinkNotify();
    fn Finish_SinkNotify();
    fn Begin_ShutdownNotify();
    fn Finish_ShutdownNotify();
}
impl ::windows::core::RuntimeName for AsyncIMSAdminBaseSinkW {
    const NAME: &'static str = "Windows.Win32.System.Iis.AsyncIMSAdminBaseSinkW";
}
impl AsyncIMSAdminBaseSinkWVtbl {
    pub const fn new<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIMSAdminBaseSinkWVtbl {
        unsafe extern "system" fn Begin_SinkNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_SinkNotify(dwmdnumelements, &*(&pcochangelist as *const <MD_CHANGE_OBJECT_W as ::windows::core::Abi>::Abi as *const <MD_CHANGE_OBJECT_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_SinkNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_SinkNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_ShutdownNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_ShutdownNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_ShutdownNotify<Impl: AsyncIMSAdminBaseSinkWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_ShutdownNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIMSAdminBaseSinkW>, base.5, Begin_SinkNotify::<Impl, OFFSET>, Finish_SinkNotify::<Impl, OFFSET>, Begin_ShutdownNotify::<Impl, OFFSET>, Finish_ShutdownNotify::<Impl, OFFSET>)
    }
}
pub trait IADMEXTImpl: Sized {
    fn Initialize();
    fn EnumDcomCLSIDs();
    fn Terminate();
}
impl ::windows::core::RuntimeName for IADMEXT {
    const NAME: &'static str = "Windows.Win32.System.Iis.IADMEXT";
}
impl IADMEXTVtbl {
    pub const fn new<Impl: IADMEXTImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IADMEXTVtbl {
        unsafe extern "system" fn Initialize<Impl: IADMEXTImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDcomCLSIDs<Impl: IADMEXTImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDcomCLSIDs(&*(&pclsiddcom as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwenumindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IADMEXTImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IADMEXT>, base.5, Initialize::<Impl, OFFSET>, EnumDcomCLSIDs::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
pub trait IFtpAuthenticationProviderImpl: Sized {
    fn AuthenticateUser();
}
impl ::windows::core::RuntimeName for IFtpAuthenticationProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpAuthenticationProvider";
}
impl IFtpAuthenticationProviderVtbl {
    pub const fn new<Impl: IFtpAuthenticationProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpAuthenticationProviderVtbl {
        unsafe extern "system" fn AuthenticateUser<Impl: IFtpAuthenticationProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppszcanonicalusername: *mut super::super::Foundation::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticateUser(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszpassword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszcanonicalusername),
                ::core::mem::transmute_copy(&pfauthenticated),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpAuthenticationProvider>, base.5, AuthenticateUser::<Impl, OFFSET>)
    }
}
pub trait IFtpAuthorizationProviderImpl: Sized {
    fn GetUserAccessPermission();
}
impl ::windows::core::RuntimeName for IFtpAuthorizationProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpAuthorizationProvider";
}
impl IFtpAuthorizationProviderVtbl {
    pub const fn new<Impl: IFtpAuthorizationProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpAuthorizationProviderVtbl {
        unsafe extern "system" fn GetUserAccessPermission<Impl: IFtpAuthorizationProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszvirtualpath: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserAccessPermission(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszvirtualpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pftpaccess),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpAuthorizationProvider>, base.5, GetUserAccessPermission::<Impl, OFFSET>)
    }
}
pub trait IFtpHomeDirectoryProviderImpl: Sized {
    fn GetUserHomeDirectoryData();
}
impl ::windows::core::RuntimeName for IFtpHomeDirectoryProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpHomeDirectoryProvider";
}
impl IFtpHomeDirectoryProviderVtbl {
    pub const fn new<Impl: IFtpHomeDirectoryProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpHomeDirectoryProviderVtbl {
        unsafe extern "system" fn GetUserHomeDirectoryData<Impl: IFtpHomeDirectoryProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, ppszhomedirectorydata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserHomeDirectoryData(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszhomedirectorydata),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpHomeDirectoryProvider>, base.5, GetUserHomeDirectoryData::<Impl, OFFSET>)
    }
}
pub trait IFtpLogProviderImpl: Sized {
    fn Log();
}
impl ::windows::core::RuntimeName for IFtpLogProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpLogProvider";
}
impl IFtpLogProviderVtbl {
    pub const fn new<Impl: IFtpLogProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpLogProviderVtbl {
        unsafe extern "system" fn Log<Impl: IFtpLogProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Log(&*(&ploggingparameters as *const <LOGGING_PARAMETERS as ::windows::core::Abi>::Abi as *const <LOGGING_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpLogProvider>, base.5, Log::<Impl, OFFSET>)
    }
}
pub trait IFtpPostprocessProviderImpl: Sized {
    fn HandlePostprocess();
}
impl ::windows::core::RuntimeName for IFtpPostprocessProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpPostprocessProvider";
}
impl IFtpPostprocessProviderVtbl {
    pub const fn new<Impl: IFtpPostprocessProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpPostprocessProviderVtbl {
        unsafe extern "system" fn HandlePostprocess<Impl: IFtpPostprocessProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandlePostprocess(&*(&ppostprocessparameters as *const <POST_PROCESS_PARAMETERS as ::windows::core::Abi>::Abi as *const <POST_PROCESS_PARAMETERS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pftpprocessstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpPostprocessProvider>, base.5, HandlePostprocess::<Impl, OFFSET>)
    }
}
pub trait IFtpPreprocessProviderImpl: Sized {
    fn HandlePreprocess();
}
impl ::windows::core::RuntimeName for IFtpPreprocessProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpPreprocessProvider";
}
impl IFtpPreprocessProviderVtbl {
    pub const fn new<Impl: IFtpPreprocessProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpPreprocessProviderVtbl {
        unsafe extern "system" fn HandlePreprocess<Impl: IFtpPreprocessProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandlePreprocess(&*(&ppreprocessparameters as *const <PRE_PROCESS_PARAMETERS as ::windows::core::Abi>::Abi as *const <PRE_PROCESS_PARAMETERS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pftpprocessstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpPreprocessProvider>, base.5, HandlePreprocess::<Impl, OFFSET>)
    }
}
pub trait IFtpProviderConstructImpl: Sized {
    fn Construct();
}
impl ::windows::core::RuntimeName for IFtpProviderConstruct {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpProviderConstruct";
}
impl IFtpProviderConstructVtbl {
    pub const fn new<Impl: IFtpProviderConstructImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpProviderConstructVtbl {
        unsafe extern "system" fn Construct<Impl: IFtpProviderConstructImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Construct(&*(&configurationentries as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpProviderConstruct>, base.5, Construct::<Impl, OFFSET>)
    }
}
pub trait IFtpRoleProviderImpl: Sized {
    fn IsUserInRole();
}
impl ::windows::core::RuntimeName for IFtpRoleProvider {
    const NAME: &'static str = "Windows.Win32.System.Iis.IFtpRoleProvider";
}
impl IFtpRoleProviderVtbl {
    pub const fn new<Impl: IFtpRoleProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFtpRoleProviderVtbl {
        unsafe extern "system" fn IsUserInRole<Impl: IFtpRoleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: super::super::Foundation::PWSTR, pszsitename: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszrole: super::super::Foundation::PWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUserInRole(
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsitename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszrole as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfisinrole),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFtpRoleProvider>, base.5, IsUserInRole::<Impl, OFFSET>)
    }
}
pub trait IMSAdminBase2WImpl: Sized + IMSAdminBaseWImpl {
    fn BackupWithPasswd();
    fn RestoreWithPasswd();
    fn Export();
    fn Import();
    fn RestoreHistory();
    fn EnumHistory();
}
impl ::windows::core::RuntimeName for IMSAdminBase2W {
    const NAME: &'static str = "Windows.Win32.System.Iis.IMSAdminBase2W";
}
impl IMSAdminBase2WVtbl {
    pub const fn new<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMSAdminBase2WVtbl {
        unsafe extern "system" fn BackupWithPasswd<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackupWithPasswd(&*(&pszmdbackuplocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdversion, dwmdflags, &*(&pszpasswd as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreWithPasswd<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreWithPasswd(&*(&pszmdbackuplocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdversion, dwmdflags, &*(&pszpasswd as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Export<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Export(
                &*(&pszpasswd as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsourcepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwmdflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Import<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpasswd: super::super::Foundation::PWSTR, pszfilename: super::super::Foundation::PWSTR, pszsourcepath: super::super::Foundation::PWSTR, pszdestpath: super::super::Foundation::PWSTR, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Import(
                &*(&pszpasswd as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsourcepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdestpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwmdflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreHistory<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: super::super::Foundation::PWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreHistory(&*(&pszmdhistorylocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdmajorversion, dwmdminorversion, dwmdflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumHistory<Impl: IMSAdminBase2WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: super::super::Foundation::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumHistory(&*(&pszmdhistorylocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwmdmajorversion), ::core::mem::transmute_copy(&pdwmdminorversion), ::core::mem::transmute_copy(&pftmdhistorytime), dwmdenumindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMSAdminBase2W>, base.5, BackupWithPasswd::<Impl, OFFSET>, RestoreWithPasswd::<Impl, OFFSET>, Export::<Impl, OFFSET>, Import::<Impl, OFFSET>, RestoreHistory::<Impl, OFFSET>, EnumHistory::<Impl, OFFSET>)
    }
}
pub trait IMSAdminBase3WImpl: Sized + IMSAdminBase2WImpl + IMSAdminBaseWImpl {
    fn GetChildPaths();
}
impl ::windows::core::RuntimeName for IMSAdminBase3W {
    const NAME: &'static str = "Windows.Win32.System.Iis.IMSAdminBase3W";
}
impl IMSAdminBase3WVtbl {
    pub const fn new<Impl: IMSAdminBase3WImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMSAdminBase3WVtbl {
        unsafe extern "system" fn GetChildPaths<Impl: IMSAdminBase3WImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, cchmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildPaths(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchmdbuffersize, &*(&pszbuffer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcchmdrequiredbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMSAdminBase3W>, base.5, GetChildPaths::<Impl, OFFSET>)
    }
}
pub trait IMSAdminBaseSinkWImpl: Sized {
    fn SinkNotify();
    fn ShutdownNotify();
}
impl ::windows::core::RuntimeName for IMSAdminBaseSinkW {
    const NAME: &'static str = "Windows.Win32.System.Iis.IMSAdminBaseSinkW";
}
impl IMSAdminBaseSinkWVtbl {
    pub const fn new<Impl: IMSAdminBaseSinkWImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMSAdminBaseSinkWVtbl {
        unsafe extern "system" fn SinkNotify<Impl: IMSAdminBaseSinkWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SinkNotify(dwmdnumelements, &*(&pcochangelist as *const <MD_CHANGE_OBJECT_W as ::windows::core::Abi>::Abi as *const <MD_CHANGE_OBJECT_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownNotify<Impl: IMSAdminBaseSinkWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMSAdminBaseSinkW>, base.5, SinkNotify::<Impl, OFFSET>, ShutdownNotify::<Impl, OFFSET>)
    }
}
pub trait IMSAdminBaseWImpl: Sized {
    fn AddKey();
    fn DeleteKey();
    fn DeleteChildKeys();
    fn EnumKeys();
    fn CopyKey();
    fn RenameKey();
    fn SetData();
    fn GetData();
    fn DeleteData();
    fn EnumData();
    fn GetAllData();
    fn DeleteAllData();
    fn CopyData();
    fn GetDataPaths();
    fn OpenKey();
    fn CloseKey();
    fn ChangePermissions();
    fn SaveData();
    fn GetHandleInfo();
    fn GetSystemChangeNumber();
    fn GetDataSetNumber();
    fn SetLastChangeTime();
    fn GetLastChangeTime();
    fn KeyExchangePhase1();
    fn KeyExchangePhase2();
    fn Backup();
    fn Restore();
    fn EnumBackups();
    fn DeleteBackup();
    fn UnmarshalInterface();
    fn GetServerGuid();
}
impl ::windows::core::RuntimeName for IMSAdminBaseW {
    const NAME: &'static str = "Windows.Win32.System.Iis.IMSAdminBaseW";
}
impl IMSAdminBaseWVtbl {
    pub const fn new<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMSAdminBaseWVtbl {
        unsafe extern "system" fn AddKey<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddKey(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteKey(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteChildKeys<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteChildKeys(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumKeys<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdname: super::super::Foundation::PWSTR, dwmdenumobjectindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumKeys(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszmdname), dwmdenumobjectindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyKey<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyKey(
                hmdsourcehandle,
                &*(&pszmdsourcepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                hmddesthandle,
                &*(&pszmddestpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bmdoverwriteflag as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bmdcopyflag as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameKey<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pszmdnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenameKey(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszmdnewname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetData(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pmdrmddata as *const <METADATA_RECORD as ::windows::core::Abi>::Abi as *const <METADATA_RECORD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pmdrmddata as *const <METADATA_RECORD as ::windows::core::Abi>::Abi as *const <METADATA_RECORD as ::windows::core::DefaultType>::DefaultType), pdwmdrequireddatalen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteData(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdidentifier, dwmddatatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumData(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pmdrmddata as *const <METADATA_RECORD as ::windows::core::Abi>::Abi as *const <METADATA_RECORD as ::windows::core::DefaultType>::DefaultType), dwmdenumdataindex, pdwmdrequireddatalen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllData(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAllData(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdusertype, dwmddatatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: super::super::Foundation::PWSTR, hmddesthandle: u32, pszmddestpath: super::super::Foundation::PWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyData(
                hmdsourcehandle,
                &*(&pszmdsourcepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                hmddesthandle,
                &*(&pszmddestpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwmdattributes,
                dwmdusertype,
                dwmddatatype,
                &*(&bmdcopyflag as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPaths<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataPaths(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdidentifier, dwmddatatype, dwmdbuffersize, ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32, phmdnewhandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenKey(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdaccessrequested, dwmdtimeout, ::core::mem::transmute_copy(&phmdnewhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseKey<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseKey(hmdhandle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangePermissions<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangePermissions(hmdhandle, dwmdtimeout, dwmdaccessrequested) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveData<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHandleInfo<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pmdhiinfo: *mut METADATA_HANDLE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHandleInfo(hmdhandle, ::core::mem::transmute_copy(&pmdhiinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemChangeNumber<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsystemchangenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSystemChangeNumber(::core::mem::transmute_copy(&pdwsystemchangenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataSetNumber<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pdwmddatasetnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataSetNumber(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwmddatasetnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastChangeTime<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLastChangeTime(
                hmdhandle,
                &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pftmdlastchangetime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&blocaltime as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChangeTime<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: super::super::Foundation::PWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastChangeTime(hmdhandle, &*(&pszmdpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pftmdlastchangetime), &*(&blocaltime as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyExchangePhase1<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyExchangePhase1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyExchangePhase2<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyExchangePhase2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Backup<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Backup(&*(&pszmdbackuplocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdversion, dwmdflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore(&*(&pszmdbackuplocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdversion, dwmdflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBackups<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumBackups(&*(&pszmdbackuplocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwmdversion), ::core::mem::transmute_copy(&pftmdbackuptime), dwmdenumindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteBackup<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: super::super::Foundation::PWSTR, dwmdversion: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteBackup(&*(&pszmdbackuplocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmdversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmarshalInterface<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piadmbwinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnmarshalInterface(::core::mem::transmute_copy(&piadmbwinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerGuid<Impl: IMSAdminBaseWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetServerGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMSAdminBaseW>,
            base.5,
            AddKey::<Impl, OFFSET>,
            DeleteKey::<Impl, OFFSET>,
            DeleteChildKeys::<Impl, OFFSET>,
            EnumKeys::<Impl, OFFSET>,
            CopyKey::<Impl, OFFSET>,
            RenameKey::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            GetData::<Impl, OFFSET>,
            DeleteData::<Impl, OFFSET>,
            EnumData::<Impl, OFFSET>,
            GetAllData::<Impl, OFFSET>,
            DeleteAllData::<Impl, OFFSET>,
            CopyData::<Impl, OFFSET>,
            GetDataPaths::<Impl, OFFSET>,
            OpenKey::<Impl, OFFSET>,
            CloseKey::<Impl, OFFSET>,
            ChangePermissions::<Impl, OFFSET>,
            SaveData::<Impl, OFFSET>,
            GetHandleInfo::<Impl, OFFSET>,
            GetSystemChangeNumber::<Impl, OFFSET>,
            GetDataSetNumber::<Impl, OFFSET>,
            SetLastChangeTime::<Impl, OFFSET>,
            GetLastChangeTime::<Impl, OFFSET>,
            KeyExchangePhase1::<Impl, OFFSET>,
            KeyExchangePhase2::<Impl, OFFSET>,
            Backup::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            EnumBackups::<Impl, OFFSET>,
            DeleteBackup::<Impl, OFFSET>,
            UnmarshalInterface::<Impl, OFFSET>,
            GetServerGuid::<Impl, OFFSET>,
        )
    }
}
pub trait IMSImpExpHelpWImpl: Sized {
    fn EnumeratePathsInFile();
}
impl ::windows::core::RuntimeName for IMSImpExpHelpW {
    const NAME: &'static str = "Windows.Win32.System.Iis.IMSImpExpHelpW";
}
impl IMSImpExpHelpWVtbl {
    pub const fn new<Impl: IMSImpExpHelpWImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMSImpExpHelpWVtbl {
        unsafe extern "system" fn EnumeratePathsInFile<Impl: IMSImpExpHelpWImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, pszkeytype: super::super::Foundation::PWSTR, dwmdbuffersize: u32, pszbuffer: super::super::Foundation::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePathsInFile(
                &*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszkeytype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwmdbuffersize,
                &*(&pszbuffer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwmdrequiredbuffersize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMSImpExpHelpW>, base.5, EnumeratePathsInFile::<Impl, OFFSET>)
    }
}

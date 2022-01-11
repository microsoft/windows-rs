#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorerImpl: Sized {
    fn AddGame();
    fn RemoveGame();
    fn UpdateGame();
    fn VerifyAccess();
}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameExplorerVtbl {
        unsafe extern "system" fn AddGame<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveGame<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateGame<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerifyAccess<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddGame::<Impl, IMPL_OFFSET>, RemoveGame::<Impl, IMPL_OFFSET>, UpdateGame::<Impl, IMPL_OFFSET>, VerifyAccess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer2Impl: Sized {
    fn InstallGame();
    fn UninstallGame();
    fn CheckAccess();
}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameExplorer2Vtbl {
        unsafe extern "system" fn InstallGame<Impl: IGameExplorer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UninstallGame<Impl: IGameExplorer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckAccess<Impl: IGameExplorer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InstallGame::<Impl, IMPL_OFFSET>, UninstallGame::<Impl, IMPL_OFFSET>, CheckAccess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatisticsImpl: Sized {
    fn GetMaxCategoryLength();
    fn GetMaxNameLength();
    fn GetMaxValueLength();
    fn GetMaxCategories();
    fn GetMaxStatsPerCategory();
    fn SetCategoryTitle();
    fn GetCategoryTitle();
    fn GetStatistic();
    fn SetStatistic();
    fn Save();
    fn SetLastPlayedCategory();
    fn GetLastPlayedCategory();
}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameStatisticsVtbl {
        unsafe extern "system" fn GetMaxCategoryLength<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxNameLength<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxValueLength<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxCategories<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCategoryTitle<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCategoryTitle<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistic<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatistic<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastPlayedCategory<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastPlayedCategory<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetMaxCategoryLength::<Impl, IMPL_OFFSET>,
            GetMaxNameLength::<Impl, IMPL_OFFSET>,
            GetMaxValueLength::<Impl, IMPL_OFFSET>,
            GetMaxCategories::<Impl, IMPL_OFFSET>,
            GetMaxStatsPerCategory::<Impl, IMPL_OFFSET>,
            SetCategoryTitle::<Impl, IMPL_OFFSET>,
            GetCategoryTitle::<Impl, IMPL_OFFSET>,
            GetStatistic::<Impl, IMPL_OFFSET>,
            SetStatistic::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            SetLastPlayedCategory::<Impl, IMPL_OFFSET>,
            GetLastPlayedCategory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatisticsMgrImpl: Sized {
    fn GetGameStatistics();
    fn RemoveGameStatistics();
}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatisticsMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameStatisticsMgrVtbl {
        unsafe extern "system" fn GetGameStatistics<Impl: IGameStatisticsMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveGameStatistics<Impl: IGameStatisticsMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGameStatistics::<Impl, IMPL_OFFSET>, RemoveGameStatistics::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthManagerImpl: Sized {
    fn SetGamerAccount();
    fn GetGamerAccount();
    fn SetAppViewInitialized();
    fn GetEnvironment();
    fn GetSandbox();
    fn GetTokenAndSignatureWithTokenResult();
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXblIdpAuthManagerVtbl {
        unsafe extern "system" fn SetGamerAccount<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGamerAccount<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppViewInitialized<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnvironment<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSandbox<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetGamerAccount::<Impl, IMPL_OFFSET>, GetGamerAccount::<Impl, IMPL_OFFSET>, SetAppViewInitialized::<Impl, IMPL_OFFSET>, GetEnvironment::<Impl, IMPL_OFFSET>, GetSandbox::<Impl, IMPL_OFFSET>, GetTokenAndSignatureWithTokenResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthTokenResultImpl: Sized {
    fn GetStatus();
    fn GetErrorCode();
    fn GetToken();
    fn GetSignature();
    fn GetSandbox();
    fn GetEnvironment();
    fn GetMsaAccountId();
    fn GetXuid();
    fn GetGamertag();
    fn GetAgeGroup();
    fn GetPrivileges();
    fn GetMsaTarget();
    fn GetMsaPolicy();
    fn GetMsaAppId();
    fn GetRedirect();
    fn GetMessage();
    fn GetHelpId();
    fn GetEnforcementBans();
    fn GetRestrictions();
    fn GetTitleRestrictions();
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthTokenResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXblIdpAuthTokenResultVtbl {
        unsafe extern "system" fn GetStatus<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorCode<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetToken<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignature<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSandbox<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnvironment<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMsaAccountId<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetXuid<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGamertag<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamertag: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAgeGroup<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agegroup: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivileges<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privileges: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMsaTarget<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msatarget: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMsaPolicy<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msapolicy: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMsaAppId<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaappid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRedirect<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redirect: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessage<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpId<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, helpid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnforcementBans<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enforcementbans: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestrictions<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTitleRestrictions<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titlerestrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            GetErrorCode::<Impl, IMPL_OFFSET>,
            GetToken::<Impl, IMPL_OFFSET>,
            GetSignature::<Impl, IMPL_OFFSET>,
            GetSandbox::<Impl, IMPL_OFFSET>,
            GetEnvironment::<Impl, IMPL_OFFSET>,
            GetMsaAccountId::<Impl, IMPL_OFFSET>,
            GetXuid::<Impl, IMPL_OFFSET>,
            GetGamertag::<Impl, IMPL_OFFSET>,
            GetAgeGroup::<Impl, IMPL_OFFSET>,
            GetPrivileges::<Impl, IMPL_OFFSET>,
            GetMsaTarget::<Impl, IMPL_OFFSET>,
            GetMsaPolicy::<Impl, IMPL_OFFSET>,
            GetMsaAppId::<Impl, IMPL_OFFSET>,
            GetRedirect::<Impl, IMPL_OFFSET>,
            GetMessage::<Impl, IMPL_OFFSET>,
            GetHelpId::<Impl, IMPL_OFFSET>,
            GetEnforcementBans::<Impl, IMPL_OFFSET>,
            GetRestrictions::<Impl, IMPL_OFFSET>,
            GetTitleRestrictions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthTokenResult2Impl: Sized {
    fn GetModernGamertag();
    fn GetModernGamertagSuffix();
    fn GetUniqueModernGamertag();
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthTokenResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXblIdpAuthTokenResult2Vtbl {
        unsafe extern "system" fn GetModernGamertag<Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetModernGamertag::<Impl, IMPL_OFFSET>, GetModernGamertagSuffix::<Impl, IMPL_OFFSET>, GetUniqueModernGamertag::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as ::windows::core::Interface>::IID
    }
}

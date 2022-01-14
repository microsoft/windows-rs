#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer_Impl: Sized {
    fn AddGame(&mut self, bstrgdfbinarypath: &super::Foundation::BSTR, bstrgameinstalldirectory: &super::Foundation::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RemoveGame(&mut self, guidinstanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UpdateGame(&mut self, guidinstanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn VerifyAccess(&mut self, bstrgdfbinarypath: &super::Foundation::BSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameExplorer_Vtbl {
        unsafe extern "system" fn AddGame<Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGame(::core::mem::transmute_copy(&bstrgdfbinarypath), ::core::mem::transmute_copy(&bstrgameinstalldirectory), ::core::mem::transmute_copy(&installscope), ::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn RemoveGame<Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGame(::core::mem::transmute_copy(&guidinstanceid)).into()
        }
        unsafe extern "system" fn UpdateGame<Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateGame(::core::mem::transmute_copy(&guidinstanceid)).into()
        }
        unsafe extern "system" fn VerifyAccess<Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyAccess(::core::mem::transmute_copy(&bstrgdfbinarypath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddGame: AddGame::<Impl, IMPL_OFFSET>,
            RemoveGame: RemoveGame::<Impl, IMPL_OFFSET>,
            UpdateGame: UpdateGame::<Impl, IMPL_OFFSET>,
            VerifyAccess: VerifyAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer2_Impl: Sized {
    fn InstallGame(&mut self, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::Result<()>;
    fn UninstallGame(&mut self, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CheckAccess(&mut self, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameExplorer2_Vtbl {
        unsafe extern "system" fn InstallGame<Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallGame(::core::mem::transmute_copy(&binarygdfpath), ::core::mem::transmute_copy(&installdirectory), ::core::mem::transmute_copy(&installscope)).into()
        }
        unsafe extern "system" fn UninstallGame<Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninstallGame(::core::mem::transmute_copy(&binarygdfpath)).into()
        }
        unsafe extern "system" fn CheckAccess<Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckAccess(::core::mem::transmute_copy(&binarygdfpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *phasaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InstallGame: InstallGame::<Impl, IMPL_OFFSET>,
            UninstallGame: UninstallGame::<Impl, IMPL_OFFSET>,
            CheckAccess: CheckAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatistics_Impl: Sized {
    fn GetMaxCategoryLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaxNameLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaxValueLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaxCategories(&mut self) -> ::windows::core::Result<u16>;
    fn GetMaxStatsPerCategory(&mut self) -> ::windows::core::Result<u16>;
    fn SetCategoryTitle(&mut self, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCategoryTitle(&mut self, categoryindex: u16) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetStatistic(&mut self, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetStatistic(&mut self, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Save(&mut self, trackchanges: super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetLastPlayedCategory(&mut self, categoryindex: u32) -> ::windows::core::Result<()>;
    fn GetLastPlayedCategory(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameStatistics_Vtbl {
        unsafe extern "system" fn GetMaxCategoryLength<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCategoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *cch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    *cch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxValueLength() {
                ::core::result::Result::Ok(ok__) => {
                    *cch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCategories<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *pmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxStatsPerCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategoryTitle(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn GetCategoryTitle<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategoryTitle(::core::mem::transmute_copy(&categoryindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistic<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistic(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetStatistic<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatistic(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Save<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&trackchanges)).into()
        }
        unsafe extern "system" fn SetLastPlayedCategory<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastPlayedCategory(::core::mem::transmute_copy(&categoryindex)).into()
        }
        unsafe extern "system" fn GetLastPlayedCategory<Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPlayedCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pcategoryindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMaxCategoryLength: GetMaxCategoryLength::<Impl, IMPL_OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Impl, IMPL_OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Impl, IMPL_OFFSET>,
            GetMaxCategories: GetMaxCategories::<Impl, IMPL_OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Impl, IMPL_OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Impl, IMPL_OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Impl, IMPL_OFFSET>,
            GetStatistic: GetStatistic::<Impl, IMPL_OFFSET>,
            SetStatistic: SetStatistic::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Impl, IMPL_OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatisticsMgr_Impl: Sized {
    fn GetGameStatistics(&mut self, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows::core::Result<()>;
    fn RemoveGameStatistics(&mut self, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatisticsMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameStatisticsMgr_Vtbl {
        unsafe extern "system" fn GetGameStatistics<Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGameStatistics(::core::mem::transmute_copy(&gdfbinarypath), ::core::mem::transmute_copy(&opentype), ::core::mem::transmute_copy(&popenresult), ::core::mem::transmute_copy(&ppistats)).into()
        }
        unsafe extern "system" fn RemoveGameStatistics<Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameStatistics(::core::mem::transmute_copy(&gdfbinarypath)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Impl, IMPL_OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthManager_Impl: Sized {
    fn SetGamerAccount(&mut self, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetGamerAccount(&mut self, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetAppViewInitialized(&mut self, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetEnvironment(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetSandbox(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetTokenAndSignatureWithTokenResult(&mut self, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> ::windows::core::Result<IXblIdpAuthTokenResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXblIdpAuthManager_Vtbl {
        unsafe extern "system" fn SetGamerAccount<Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGamerAccount(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn GetGamerAccount<Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGamerAccount(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn SetAppViewInitialized<Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppViewInitialized(::core::mem::transmute_copy(&appsid), ::core::mem::transmute_copy(&msaaccountid)).into()
        }
        unsafe extern "system" fn GetEnvironment<Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *environment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSandbox() {
                ::core::result::Result::Ok(ok__) => {
                    *sandbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokenAndSignatureWithTokenResult(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&appsid), ::core::mem::transmute_copy(&msatarget), ::core::mem::transmute_copy(&msapolicy), ::core::mem::transmute_copy(&httpmethod), ::core::mem::transmute_copy(&uri), ::core::mem::transmute_copy(&headers), ::core::mem::transmute_copy(&body), ::core::mem::transmute_copy(&bodysize), ::core::mem::transmute_copy(&forcerefresh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetGamerAccount: SetGamerAccount::<Impl, IMPL_OFFSET>,
            GetGamerAccount: GetGamerAccount::<Impl, IMPL_OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Impl, IMPL_OFFSET>,
            GetEnvironment: GetEnvironment::<Impl, IMPL_OFFSET>,
            GetSandbox: GetSandbox::<Impl, IMPL_OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthTokenResult_Impl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<XBL_IDP_AUTH_TOKEN_STATUS>;
    fn GetErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetToken(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetSignature(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetSandbox(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetEnvironment(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaAccountId(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetXuid(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetGamertag(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetAgeGroup(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetPrivileges(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaTarget(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaPolicy(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaAppId(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetRedirect(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMessage(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetHelpId(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetEnforcementBans(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetRestrictions(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetTitleRestrictions(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthTokenResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXblIdpAuthTokenResult_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *errorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToken() {
                ::core::result::Result::Ok(ok__) => {
                    *token = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSandbox() {
                ::core::result::Result::Ok(ok__) => {
                    *sandbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *environment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *msaaccountid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXuid<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXuid() {
                ::core::result::Result::Ok(ok__) => {
                    *xuid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamertag<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamertag: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    *gamertag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAgeGroup<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agegroup: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAgeGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *agegroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivileges<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privileges: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivileges() {
                ::core::result::Result::Ok(ok__) => {
                    *privileges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaTarget<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msatarget: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *msatarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msapolicy: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *msapolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAppId<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaappid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaAppId() {
                ::core::result::Result::Ok(ok__) => {
                    *msaappid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirect<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redirect: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    *redirect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpId<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, helpid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpId() {
                ::core::result::Result::Ok(ok__) => {
                    *helpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enforcementbans: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnforcementBans() {
                ::core::result::Result::Ok(ok__) => {
                    *enforcementbans = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *restrictions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titlerestrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitleRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *titlerestrictions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetErrorCode: GetErrorCode::<Impl, IMPL_OFFSET>,
            GetToken: GetToken::<Impl, IMPL_OFFSET>,
            GetSignature: GetSignature::<Impl, IMPL_OFFSET>,
            GetSandbox: GetSandbox::<Impl, IMPL_OFFSET>,
            GetEnvironment: GetEnvironment::<Impl, IMPL_OFFSET>,
            GetMsaAccountId: GetMsaAccountId::<Impl, IMPL_OFFSET>,
            GetXuid: GetXuid::<Impl, IMPL_OFFSET>,
            GetGamertag: GetGamertag::<Impl, IMPL_OFFSET>,
            GetAgeGroup: GetAgeGroup::<Impl, IMPL_OFFSET>,
            GetPrivileges: GetPrivileges::<Impl, IMPL_OFFSET>,
            GetMsaTarget: GetMsaTarget::<Impl, IMPL_OFFSET>,
            GetMsaPolicy: GetMsaPolicy::<Impl, IMPL_OFFSET>,
            GetMsaAppId: GetMsaAppId::<Impl, IMPL_OFFSET>,
            GetRedirect: GetRedirect::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
            GetHelpId: GetHelpId::<Impl, IMPL_OFFSET>,
            GetEnforcementBans: GetEnforcementBans::<Impl, IMPL_OFFSET>,
            GetRestrictions: GetRestrictions::<Impl, IMPL_OFFSET>,
            GetTitleRestrictions: GetTitleRestrictions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthTokenResult2_Impl: Sized {
    fn GetModernGamertag(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetModernGamertagSuffix(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetUniqueModernGamertag(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthTokenResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXblIdpAuthTokenResult2_Vtbl {
        unsafe extern "system" fn GetModernGamertag<Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModernGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModernGamertagSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUniqueModernGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetModernGamertag: GetModernGamertag::<Impl, IMPL_OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Impl, IMPL_OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as ::windows::core::Interface>::IID
    }
}

#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer_Impl: Sized {
    fn AddGame(&self, bstrgdfbinarypath: &super::Foundation::BSTR, bstrgameinstalldirectory: &super::Foundation::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RemoveGame(&self, guidinstanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UpdateGame(&self, guidinstanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn VerifyAccess(&self, bstrgdfbinarypath: &super::Foundation::BSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer_Impl, const OFFSET: isize>() -> IGameExplorer_Vtbl {
        unsafe extern "system" fn AddGame<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddGame(::core::mem::transmute_copy(&bstrgdfbinarypath), ::core::mem::transmute_copy(&bstrgameinstalldirectory), ::core::mem::transmute_copy(&installscope), ::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn RemoveGame<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveGame(::core::mem::transmute_copy(&guidinstanceid)).into()
        }
        unsafe extern "system" fn UpdateGame<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateGame(::core::mem::transmute_copy(&guidinstanceid)).into()
        }
        unsafe extern "system" fn VerifyAccess<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerifyAccess(::core::mem::transmute_copy(&bstrgdfbinarypath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddGame: AddGame::<Identity, Impl, OFFSET>,
            RemoveGame: RemoveGame::<Identity, Impl, OFFSET>,
            UpdateGame: UpdateGame::<Identity, Impl, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer2_Impl: Sized {
    fn InstallGame(&self, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::Result<()>;
    fn UninstallGame(&self, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CheckAccess(&self, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2_Impl, const OFFSET: isize>() -> IGameExplorer2_Vtbl {
        unsafe extern "system" fn InstallGame<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallGame(::core::mem::transmute_copy(&binarygdfpath), ::core::mem::transmute_copy(&installdirectory), ::core::mem::transmute_copy(&installscope)).into()
        }
        unsafe extern "system" fn UninstallGame<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UninstallGame(::core::mem::transmute_copy(&binarygdfpath)).into()
        }
        unsafe extern "system" fn CheckAccess<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckAccess(::core::mem::transmute_copy(&binarygdfpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *phasaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InstallGame: InstallGame::<Identity, Impl, OFFSET>,
            UninstallGame: UninstallGame::<Identity, Impl, OFFSET>,
            CheckAccess: CheckAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatistics_Impl: Sized {
    fn GetMaxCategoryLength(&self) -> ::windows::core::Result<u32>;
    fn GetMaxNameLength(&self) -> ::windows::core::Result<u32>;
    fn GetMaxValueLength(&self) -> ::windows::core::Result<u32>;
    fn GetMaxCategories(&self) -> ::windows::core::Result<u16>;
    fn GetMaxStatsPerCategory(&self) -> ::windows::core::Result<u16>;
    fn SetCategoryTitle(&self, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetStatistic(&self, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Save(&self, trackchanges: super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows::core::Result<()>;
    fn GetLastPlayedCategory(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>() -> IGameStatistics_Vtbl {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxCategoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *cch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    *cch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxValueLength() {
                ::core::result::Result::Ok(ok__) => {
                    *cch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCategories<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *pmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxStatsPerCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCategoryTitle(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCategoryTitle(::core::mem::transmute_copy(&categoryindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistic<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatistic(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetStatistic<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatistic(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&trackchanges)).into()
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLastPlayedCategory(::core::mem::transmute_copy(&categoryindex)).into()
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastPlayedCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pcategoryindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMaxCategoryLength: GetMaxCategoryLength::<Identity, Impl, OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Identity, Impl, OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Identity, Impl, OFFSET>,
            GetMaxCategories: GetMaxCategories::<Identity, Impl, OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Identity, Impl, OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Identity, Impl, OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Identity, Impl, OFFSET>,
            GetStatistic: GetStatistic::<Identity, Impl, OFFSET>,
            SetStatistic: SetStatistic::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Identity, Impl, OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatisticsMgr_Impl: Sized {
    fn GetGameStatistics(&self, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows::core::Result<()>;
    fn RemoveGameStatistics(&self, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatisticsMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>() -> IGameStatisticsMgr_Vtbl {
        unsafe extern "system" fn GetGameStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGameStatistics(::core::mem::transmute_copy(&gdfbinarypath), ::core::mem::transmute_copy(&opentype), ::core::mem::transmute_copy(&popenresult), ::core::mem::transmute_copy(&ppistats)).into()
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveGameStatistics(::core::mem::transmute_copy(&gdfbinarypath)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Identity, Impl, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthManager_Impl: Sized {
    fn SetGamerAccount(&self, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetGamerAccount(&self, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetAppViewInitialized(&self, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetEnvironment(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetSandbox(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetTokenAndSignatureWithTokenResult(&self, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> ::windows::core::Result<IXblIdpAuthTokenResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>() -> IXblIdpAuthManager_Vtbl {
        unsafe extern "system" fn SetGamerAccount<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGamerAccount(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn GetGamerAccount<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGamerAccount(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn SetAppViewInitialized<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppViewInitialized(::core::mem::transmute_copy(&appsid), ::core::mem::transmute_copy(&msaaccountid)).into()
        }
        unsafe extern "system" fn GetEnvironment<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *environment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSandbox() {
                ::core::result::Result::Ok(ok__) => {
                    *sandbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTokenAndSignatureWithTokenResult(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&appsid), ::core::mem::transmute_copy(&msatarget), ::core::mem::transmute_copy(&msapolicy), ::core::mem::transmute_copy(&httpmethod), ::core::mem::transmute_copy(&uri), ::core::mem::transmute_copy(&headers), ::core::mem::transmute_copy(&body), ::core::mem::transmute_copy(&bodysize), ::core::mem::transmute_copy(&forcerefresh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetGamerAccount: SetGamerAccount::<Identity, Impl, OFFSET>,
            GetGamerAccount: GetGamerAccount::<Identity, Impl, OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthTokenResult_Impl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<XBL_IDP_AUTH_TOKEN_STATUS>;
    fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetToken(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetSignature(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetSandbox(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetEnvironment(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaAccountId(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetXuid(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetGamertag(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetAgeGroup(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetPrivileges(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaTarget(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaPolicy(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMsaAppId(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetRedirect(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetMessage(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetHelpId(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetEnforcementBans(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetRestrictions(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetTitleRestrictions(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthTokenResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *errorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetToken() {
                ::core::result::Result::Ok(ok__) => {
                    *token = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSandbox() {
                ::core::result::Result::Ok(ok__) => {
                    *sandbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *environment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMsaAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *msaaccountid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXuid<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXuid() {
                ::core::result::Result::Ok(ok__) => {
                    *xuid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamertag<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamertag: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    *gamertag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAgeGroup<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agegroup: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAgeGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *agegroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivileges<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privileges: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrivileges() {
                ::core::result::Result::Ok(ok__) => {
                    *privileges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaTarget<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msatarget: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMsaTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *msatarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msapolicy: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMsaPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *msapolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAppId<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaappid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMsaAppId() {
                ::core::result::Result::Ok(ok__) => {
                    *msaappid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirect<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redirect: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    *redirect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpId<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, helpid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHelpId() {
                ::core::result::Result::Ok(ok__) => {
                    *helpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enforcementbans: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnforcementBans() {
                ::core::result::Result::Ok(ok__) => {
                    *enforcementbans = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *restrictions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titlerestrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTitleRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *titlerestrictions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, Impl, OFFSET>,
            GetToken: GetToken::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetMsaAccountId: GetMsaAccountId::<Identity, Impl, OFFSET>,
            GetXuid: GetXuid::<Identity, Impl, OFFSET>,
            GetGamertag: GetGamertag::<Identity, Impl, OFFSET>,
            GetAgeGroup: GetAgeGroup::<Identity, Impl, OFFSET>,
            GetPrivileges: GetPrivileges::<Identity, Impl, OFFSET>,
            GetMsaTarget: GetMsaTarget::<Identity, Impl, OFFSET>,
            GetMsaPolicy: GetMsaPolicy::<Identity, Impl, OFFSET>,
            GetMsaAppId: GetMsaAppId::<Identity, Impl, OFFSET>,
            GetRedirect: GetRedirect::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetHelpId: GetHelpId::<Identity, Impl, OFFSET>,
            GetEnforcementBans: GetEnforcementBans::<Identity, Impl, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, Impl, OFFSET>,
            GetTitleRestrictions: GetTitleRestrictions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthTokenResult2_Impl: Sized {
    fn GetModernGamertag(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetModernGamertagSuffix(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetUniqueModernGamertag(&self) -> ::windows::core::Result<super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthTokenResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult2_Vtbl {
        unsafe extern "system" fn GetModernGamertag<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetModernGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetModernGamertagSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUniqueModernGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetModernGamertag: GetModernGamertag::<Identity, Impl, OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Identity, Impl, OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as ::windows::core::Interface>::IID
    }
}

#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer_Impl: Sized {
    fn AddGame(&self, bstrgdfbinarypath: &::windows::core::BSTR, bstrgameinstalldirectory: &::windows::core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RemoveGame(&self, guidinstanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UpdateGame(&self, guidinstanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn VerifyAccess(&self, bstrgdfbinarypath: &::windows::core::BSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGameExplorer {}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>() -> IGameExplorer_Vtbl {
        unsafe extern "system" fn AddGame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrgameinstalldirectory: ::std::mem::MaybeUninit<::windows::core::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddGame(::core::mem::transmute(&bstrgdfbinarypath), ::core::mem::transmute(&bstrgameinstalldirectory), ::core::mem::transmute_copy(&installscope), ::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn RemoveGame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveGame(::core::mem::transmute(&guidinstanceid)).into()
        }
        unsafe extern "system" fn UpdateGame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateGame(::core::mem::transmute(&guidinstanceid)).into()
        }
        unsafe extern "system" fn VerifyAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerifyAccess(::core::mem::transmute(&bstrgdfbinarypath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddGame: AddGame::<Identity, Impl, OFFSET>,
            RemoveGame: RemoveGame::<Identity, Impl, OFFSET>,
            UpdateGame: UpdateGame::<Identity, Impl, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer2_Impl: Sized {
    fn InstallGame(&self, binarygdfpath: &::windows::core::PCWSTR, installdirectory: &::windows::core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::Result<()>;
    fn UninstallGame(&self, binarygdfpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn CheckAccess(&self, binarygdfpath: &::windows::core::PCWSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGameExplorer2 {}
#[cfg(feature = "Win32_Foundation")]
impl IGameExplorer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>() -> IGameExplorer2_Vtbl {
        unsafe extern "system" fn InstallGame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows::core::PCWSTR, installdirectory: ::windows::core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallGame(::core::mem::transmute(&binarygdfpath), ::core::mem::transmute(&installdirectory), ::core::mem::transmute_copy(&installscope)).into()
        }
        unsafe extern "system" fn UninstallGame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UninstallGame(::core::mem::transmute(&binarygdfpath)).into()
        }
        unsafe extern "system" fn CheckAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows::core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckAccess(::core::mem::transmute(&binarygdfpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phasaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallGame: InstallGame::<Identity, Impl, OFFSET>,
            UninstallGame: UninstallGame::<Identity, Impl, OFFSET>,
            CheckAccess: CheckAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameExplorer2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatistics_Impl: Sized {
    fn GetMaxCategoryLength(&self) -> ::windows::core::Result<u32>;
    fn GetMaxNameLength(&self) -> ::windows::core::Result<u32>;
    fn GetMaxValueLength(&self) -> ::windows::core::Result<u32>;
    fn GetMaxCategories(&self) -> ::windows::core::Result<u16>;
    fn GetMaxStatsPerCategory(&self) -> ::windows::core::Result<u16>;
    fn SetCategoryTitle(&self, categoryindex: u16, title: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetStatistic(&self, categoryindex: u16, statindex: u16, name: &::windows::core::PCWSTR, value: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Save(&self, trackchanges: super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows::core::Result<()>;
    fn GetLastPlayedCategory(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGameStatistics {}
#[cfg(feature = "Win32_Foundation")]
impl IGameStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>() -> IGameStatistics_Vtbl {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxCategoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxValueLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxCategories() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxStatsPerCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, title: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCategoryTitle(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategoryTitle(::core::mem::transmute_copy(&categoryindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptitle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatistic(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetStatistic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatistic(::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute_copy(&trackchanges)).into()
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastPlayedCategory(::core::mem::transmute_copy(&categoryindex)).into()
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastPlayedCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcategoryindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IGameStatistics as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"implement\"`*"]
pub trait IGameStatisticsMgr_Impl: Sized {
    fn GetGameStatistics(&self, gdfbinarypath: &::windows::core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows::core::Result<()>;
    fn RemoveGameStatistics(&self, gdfbinarypath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGameStatisticsMgr {}
impl IGameStatisticsMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>() -> IGameStatisticsMgr_Vtbl {
        unsafe extern "system" fn GetGameStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows::core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGameStatistics(::core::mem::transmute(&gdfbinarypath), ::core::mem::transmute_copy(&opentype), ::core::mem::transmute_copy(&popenresult), ::core::mem::transmute_copy(&ppistats)).into()
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveGameStatistics(::core::mem::transmute(&gdfbinarypath)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Identity, Impl, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthManager_Impl: Sized {
    fn SetGamerAccount(&self, msaaccountid: &::windows::core::PCWSTR, xuid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetGamerAccount(&self, msaaccountid: *mut ::windows::core::PWSTR, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetAppViewInitialized(&self, appsid: &::windows::core::PCWSTR, msaaccountid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetEnvironment(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSandbox(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetTokenAndSignatureWithTokenResult(&self, msaaccountid: &::windows::core::PCWSTR, appsid: &::windows::core::PCWSTR, msatarget: &::windows::core::PCWSTR, msapolicy: &::windows::core::PCWSTR, httpmethod: &::windows::core::PCWSTR, uri: &::windows::core::PCWSTR, headers: &::windows::core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> ::windows::core::Result<IXblIdpAuthTokenResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXblIdpAuthManager {}
#[cfg(feature = "Win32_Foundation")]
impl IXblIdpAuthManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>() -> IXblIdpAuthManager_Vtbl {
        unsafe extern "system" fn SetGamerAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: ::windows::core::PCWSTR, xuid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGamerAccount(::core::mem::transmute(&msaaccountid), ::core::mem::transmute(&xuid)).into()
        }
        unsafe extern "system" fn GetGamerAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows::core::PWSTR, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGamerAccount(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn SetAppViewInitialized<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appsid: ::windows::core::PCWSTR, msaaccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppViewInitialized(::core::mem::transmute(&appsid), ::core::mem::transmute(&msaaccountid)).into()
        }
        unsafe extern "system" fn GetEnvironment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(environment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSandbox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sandbox, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: ::windows::core::PCWSTR, appsid: ::windows::core::PCWSTR, msatarget: ::windows::core::PCWSTR, msapolicy: ::windows::core::PCWSTR, httpmethod: ::windows::core::PCWSTR, uri: ::windows::core::PCWSTR, headers: ::windows::core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTokenAndSignatureWithTokenResult(::core::mem::transmute(&msaaccountid), ::core::mem::transmute(&appsid), ::core::mem::transmute(&msatarget), ::core::mem::transmute(&msapolicy), ::core::mem::transmute(&httpmethod), ::core::mem::transmute(&uri), ::core::mem::transmute(&headers), ::core::mem::transmute_copy(&body), ::core::mem::transmute_copy(&bodysize), ::core::mem::transmute_copy(&forcerefresh)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGamerAccount: SetGamerAccount::<Identity, Impl, OFFSET>,
            GetGamerAccount: GetGamerAccount::<Identity, Impl, OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"implement\"`*"]
pub trait IXblIdpAuthTokenResult_Impl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<XBL_IDP_AUTH_TOKEN_STATUS>;
    fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetToken(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSignature(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSandbox(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetEnvironment(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMsaAccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetXuid(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetAgeGroup(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPrivileges(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMsaTarget(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMsaPolicy(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMsaAppId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetRedirect(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMessage(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetHelpId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetEnforcementBans(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetTitleRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IXblIdpAuthTokenResult {}
impl IXblIdpAuthTokenResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSandbox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sandbox, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(environment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMsaAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msaaccountid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xuid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamertag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamertag: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gamertag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAgeGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agegroup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAgeGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(agegroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivileges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privileges: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrivileges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(privileges, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msatarget: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMsaTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msatarget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msapolicy: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMsaPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msapolicy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAppId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaappid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMsaAppId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msaappid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redirect: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(redirect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, helpid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(helpid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enforcementbans: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnforcementBans() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enforcementbans, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titlerestrictions: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTitleRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(titlerestrictions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXblIdpAuthTokenResult as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"implement\"`*"]
pub trait IXblIdpAuthTokenResult2_Impl: Sized {
    fn GetModernGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetModernGamertagSuffix(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetUniqueModernGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IXblIdpAuthTokenResult2 {}
impl IXblIdpAuthTokenResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult2_Vtbl {
        unsafe extern "system" fn GetModernGamertag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModernGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModernGamertagSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUniqueModernGamertag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetModernGamertag: GetModernGamertag::<Identity, Impl, OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Identity, Impl, OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as ::windows::core::ComInterface>::IID
    }
}

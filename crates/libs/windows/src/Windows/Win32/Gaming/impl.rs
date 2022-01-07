pub trait IGameExplorerImpl: Sized {
    fn AddGame();
    fn RemoveGame();
    fn UpdateGame();
    fn VerifyAccess();
}
impl ::windows::core::RuntimeName for IGameExplorer {
    const NAME: &'static str = "Windows.Win32.Gaming.IGameExplorer";
}
impl IGameExplorerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorerImpl, const OFFSET: isize>() -> IGameExplorerVtbl {
        unsafe extern "system" fn AddGame<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddGame(
                &*(&bstrgdfbinarypath as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrgameinstalldirectory as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                installscope,
                &*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGame<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveGame(&*(&guidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateGame<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateGame(&*(&guidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyAccess<Impl: IGameExplorerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyAccess(&*(&bstrgdfbinarypath as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfhasaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameExplorer>, ::windows::core::GetTrustLevel, AddGame::<Impl, OFFSET>, RemoveGame::<Impl, OFFSET>, UpdateGame::<Impl, OFFSET>, VerifyAccess::<Impl, OFFSET>)
    }
}
pub trait IGameExplorer2Impl: Sized {
    fn InstallGame();
    fn UninstallGame();
    fn CheckAccess();
}
impl ::windows::core::RuntimeName for IGameExplorer2 {
    const NAME: &'static str = "Windows.Win32.Gaming.IGameExplorer2";
}
impl IGameExplorer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameExplorer2Impl, const OFFSET: isize>() -> IGameExplorer2Vtbl {
        unsafe extern "system" fn InstallGame<Impl: IGameExplorer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallGame(&*(&binarygdfpath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&installdirectory as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), installscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallGame<Impl: IGameExplorer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallGame(&*(&binarygdfpath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckAccess<Impl: IGameExplorer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binarygdfpath: super::Foundation::PWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckAccess(&*(&binarygdfpath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phasaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameExplorer2>, ::windows::core::GetTrustLevel, InstallGame::<Impl, OFFSET>, UninstallGame::<Impl, OFFSET>, CheckAccess::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IGameStatistics {
    const NAME: &'static str = "Windows.Win32.Gaming.IGameStatistics";
}
impl IGameStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsImpl, const OFFSET: isize>() -> IGameStatisticsVtbl {
        unsafe extern "system" fn GetMaxCategoryLength<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCategoryLength(::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxNameLength(::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxValueLength(::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCategories<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCategories(::core::mem::transmute_copy(&pmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxStatsPerCategory(::core::mem::transmute_copy(&pmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCategoryTitle(categoryindex, &*(&title as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategoryTitle<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategoryTitle(categoryindex, ::core::mem::transmute_copy(&ptitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistic<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatistic(categoryindex, statindex, &*(&pname as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatistic<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatistic(categoryindex, statindex, &*(&name as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(&*(&trackchanges as *const <super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPlayedCategory<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLastPlayedCategory(categoryindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayedCategory<Impl: IGameStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPlayedCategory(::core::mem::transmute_copy(&pcategoryindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGameStatistics>,
            ::windows::core::GetTrustLevel,
            GetMaxCategoryLength::<Impl, OFFSET>,
            GetMaxNameLength::<Impl, OFFSET>,
            GetMaxValueLength::<Impl, OFFSET>,
            GetMaxCategories::<Impl, OFFSET>,
            GetMaxStatsPerCategory::<Impl, OFFSET>,
            SetCategoryTitle::<Impl, OFFSET>,
            GetCategoryTitle::<Impl, OFFSET>,
            GetStatistic::<Impl, OFFSET>,
            SetStatistic::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            SetLastPlayedCategory::<Impl, OFFSET>,
            GetLastPlayedCategory::<Impl, OFFSET>,
        )
    }
}
pub trait IGameStatisticsMgrImpl: Sized {
    fn GetGameStatistics();
    fn RemoveGameStatistics();
}
impl ::windows::core::RuntimeName for IGameStatisticsMgr {
    const NAME: &'static str = "Windows.Win32.Gaming.IGameStatisticsMgr";
}
impl IGameStatisticsMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameStatisticsMgrImpl, const OFFSET: isize>() -> IGameStatisticsMgrVtbl {
        unsafe extern "system" fn GetGameStatistics<Impl: IGameStatisticsMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGameStatistics(&*(&gdfbinarypath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), opentype, ::core::mem::transmute_copy(&popenresult), ::core::mem::transmute_copy(&ppistats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGameStatistics<Impl: IGameStatisticsMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveGameStatistics(&*(&gdfbinarypath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameStatisticsMgr>, ::windows::core::GetTrustLevel, GetGameStatistics::<Impl, OFFSET>, RemoveGameStatistics::<Impl, OFFSET>)
    }
}
pub trait IXblIdpAuthManagerImpl: Sized {
    fn SetGamerAccount();
    fn GetGamerAccount();
    fn SetAppViewInitialized();
    fn GetEnvironment();
    fn GetSandbox();
    fn GetTokenAndSignatureWithTokenResult();
}
impl ::windows::core::RuntimeName for IXblIdpAuthManager {
    const NAME: &'static str = "Windows.Win32.Gaming.IXblIdpAuthManager";
}
impl IXblIdpAuthManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>() -> IXblIdpAuthManagerVtbl {
        unsafe extern "system" fn SetGamerAccount<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGamerAccount(&*(&msaaccountid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&xuid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamerAccount<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamerAccount(::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppViewInitialized<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppViewInitialized(&*(&appsid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&msaaccountid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnvironment(::core::mem::transmute_copy(&environment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSandbox(::core::mem::transmute_copy(&sandbox)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Impl: IXblIdpAuthManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokenAndSignatureWithTokenResult(
                &*(&msaaccountid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&appsid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&msatarget as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&msapolicy as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&httpmethod as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&headers as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                body,
                bodysize,
                &*(&forcerefresh as *const <super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&result),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXblIdpAuthManager>, ::windows::core::GetTrustLevel, SetGamerAccount::<Impl, OFFSET>, GetGamerAccount::<Impl, OFFSET>, SetAppViewInitialized::<Impl, OFFSET>, GetEnvironment::<Impl, OFFSET>, GetSandbox::<Impl, OFFSET>, GetTokenAndSignatureWithTokenResult::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXblIdpAuthTokenResult {
    const NAME: &'static str = "Windows.Win32.Gaming.IXblIdpAuthTokenResult";
}
impl IXblIdpAuthTokenResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>() -> IXblIdpAuthTokenResultVtbl {
        unsafe extern "system" fn GetStatus<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorCode(::core::mem::transmute_copy(&errorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToken(::core::mem::transmute_copy(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignature(::core::mem::transmute_copy(&signature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSandbox(::core::mem::transmute_copy(&sandbox)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnvironment(::core::mem::transmute_copy(&environment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaAccountId(::core::mem::transmute_copy(&msaaccountid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXuid<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXuid(::core::mem::transmute_copy(&xuid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamertag<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamertag: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamertag(::core::mem::transmute_copy(&gamertag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAgeGroup<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agegroup: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAgeGroup(::core::mem::transmute_copy(&agegroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivileges<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privileges: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivileges(::core::mem::transmute_copy(&privileges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaTarget<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msatarget: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaTarget(::core::mem::transmute_copy(&msatarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msapolicy: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaPolicy(::core::mem::transmute_copy(&msapolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAppId<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msaappid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMsaAppId(::core::mem::transmute_copy(&msaappid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirect<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redirect: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRedirect(::core::mem::transmute_copy(&redirect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpId<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, helpid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpId(::core::mem::transmute_copy(&helpid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enforcementbans: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnforcementBans(::core::mem::transmute_copy(&enforcementbans)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictions(::core::mem::transmute_copy(&restrictions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Impl: IXblIdpAuthTokenResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titlerestrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitleRestrictions(::core::mem::transmute_copy(&titlerestrictions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXblIdpAuthTokenResult>,
            ::windows::core::GetTrustLevel,
            GetStatus::<Impl, OFFSET>,
            GetErrorCode::<Impl, OFFSET>,
            GetToken::<Impl, OFFSET>,
            GetSignature::<Impl, OFFSET>,
            GetSandbox::<Impl, OFFSET>,
            GetEnvironment::<Impl, OFFSET>,
            GetMsaAccountId::<Impl, OFFSET>,
            GetXuid::<Impl, OFFSET>,
            GetGamertag::<Impl, OFFSET>,
            GetAgeGroup::<Impl, OFFSET>,
            GetPrivileges::<Impl, OFFSET>,
            GetMsaTarget::<Impl, OFFSET>,
            GetMsaPolicy::<Impl, OFFSET>,
            GetMsaAppId::<Impl, OFFSET>,
            GetRedirect::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
            GetHelpId::<Impl, OFFSET>,
            GetEnforcementBans::<Impl, OFFSET>,
            GetRestrictions::<Impl, OFFSET>,
            GetTitleRestrictions::<Impl, OFFSET>,
        )
    }
}
pub trait IXblIdpAuthTokenResult2Impl: Sized {
    fn GetModernGamertag();
    fn GetModernGamertagSuffix();
    fn GetUniqueModernGamertag();
}
impl ::windows::core::RuntimeName for IXblIdpAuthTokenResult2 {
    const NAME: &'static str = "Windows.Win32.Gaming.IXblIdpAuthTokenResult2";
}
impl IXblIdpAuthTokenResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult2Vtbl {
        unsafe extern "system" fn GetModernGamertag<Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModernGamertag(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModernGamertagSuffix(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Impl: IXblIdpAuthTokenResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUniqueModernGamertag(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXblIdpAuthTokenResult2>, ::windows::core::GetTrustLevel, GetModernGamertag::<Impl, OFFSET>, GetModernGamertagSuffix::<Impl, OFFSET>, GetUniqueModernGamertag::<Impl, OFFSET>)
    }
}

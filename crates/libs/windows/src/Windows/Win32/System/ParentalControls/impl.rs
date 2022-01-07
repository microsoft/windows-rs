pub trait IWPCGamesSettingsImpl: Sized + IWPCSettingsImpl {
    fn IsBlocked();
}
impl ::windows::core::RuntimeName for IWPCGamesSettings {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCGamesSettings";
}
impl IWPCGamesSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCGamesSettingsImpl, const OFFSET: isize>() -> IWPCGamesSettingsVtbl {
        unsafe extern "system" fn IsBlocked<Impl: IWPCGamesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidappid: ::windows::core::GUID, pdwreasons: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlocked(&*(&guidappid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwreasons)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWPCGamesSettings>, ::windows::core::GetTrustLevel, IsBlocked::<Impl, OFFSET>)
    }
}
pub trait IWPCProviderConfigImpl: Sized {
    fn GetUserSummary();
    fn Configure();
    fn RequestOverride();
}
impl ::windows::core::RuntimeName for IWPCProviderConfig {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCProviderConfig";
}
impl IWPCProviderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderConfigImpl, const OFFSET: isize>() -> IWPCProviderConfigVtbl {
        unsafe extern "system" fn GetUserSummary<Impl: IWPCProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrusersummary: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserSummary(&*(&bstrsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrusersummary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IWPCProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configure(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&bstrsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestOverride<Impl: IWPCProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestOverride(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&bstrpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWPCProviderConfig>, ::windows::core::GetTrustLevel, GetUserSummary::<Impl, OFFSET>, Configure::<Impl, OFFSET>, RequestOverride::<Impl, OFFSET>)
    }
}
pub trait IWPCProviderStateImpl: Sized {
    fn Enable();
    fn Disable();
}
impl ::windows::core::RuntimeName for IWPCProviderState {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCProviderState";
}
impl IWPCProviderStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderStateImpl, const OFFSET: isize>() -> IWPCProviderStateVtbl {
        unsafe extern "system" fn Enable<Impl: IWPCProviderStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IWPCProviderStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWPCProviderState>, ::windows::core::GetTrustLevel, Enable::<Impl, OFFSET>, Disable::<Impl, OFFSET>)
    }
}
pub trait IWPCProviderSupportImpl: Sized {
    fn GetCurrent();
}
impl ::windows::core::RuntimeName for IWPCProviderSupport {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCProviderSupport";
}
impl IWPCProviderSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderSupportImpl, const OFFSET: isize>() -> IWPCProviderSupportVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IWPCProviderSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&pguidprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWPCProviderSupport>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>)
    }
}
pub trait IWPCSettingsImpl: Sized {
    fn IsLoggingRequired();
    fn GetLastSettingsChangeTime();
    fn GetRestrictions();
}
impl ::windows::core::RuntimeName for IWPCSettings {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCSettings";
}
impl IWPCSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCSettingsImpl, const OFFSET: isize>() -> IWPCSettingsVtbl {
        unsafe extern "system" fn IsLoggingRequired<Impl: IWPCSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLoggingRequired(::core::mem::transmute_copy(&pfrequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Impl: IWPCSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastSettingsChangeTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Impl: IWPCSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictions(::core::mem::transmute_copy(&pdwrestrictions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWPCSettings>, ::windows::core::GetTrustLevel, IsLoggingRequired::<Impl, OFFSET>, GetLastSettingsChangeTime::<Impl, OFFSET>, GetRestrictions::<Impl, OFFSET>)
    }
}
pub trait IWPCWebSettingsImpl: Sized + IWPCSettingsImpl {
    fn GetSettings();
    fn RequestURLOverride();
}
impl ::windows::core::RuntimeName for IWPCWebSettings {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCWebSettings";
}
impl IWPCWebSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCWebSettingsImpl, const OFFSET: isize>() -> IWPCWebSettingsVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWPCWebSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettings(::core::mem::transmute_copy(&pdwsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestURLOverride<Impl: IWPCWebSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: super::super::Foundation::PWSTR, curls: u32, ppcszsuburls: *const super::super::Foundation::PWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestURLOverride(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pcszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                curls,
                &*(&ppcszsuburls as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfchanged),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWPCWebSettings>, ::windows::core::GetTrustLevel, GetSettings::<Impl, OFFSET>, RequestURLOverride::<Impl, OFFSET>)
    }
}
pub trait IWindowsParentalControlsImpl: Sized + IWindowsParentalControlsCoreImpl {
    fn GetGamesSettings();
}
impl ::windows::core::RuntimeName for IWindowsParentalControls {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWindowsParentalControls";
}
impl IWindowsParentalControlsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsParentalControlsImpl, const OFFSET: isize>() -> IWindowsParentalControlsVtbl {
        unsafe extern "system" fn GetGamesSettings<Impl: IWindowsParentalControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamesSettings(&*(&pcszsid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsParentalControls>, ::windows::core::GetTrustLevel, GetGamesSettings::<Impl, OFFSET>)
    }
}
pub trait IWindowsParentalControlsCoreImpl: Sized {
    fn GetVisibility();
    fn GetUserSettings();
    fn GetWebSettings();
    fn GetWebFilterInfo();
}
impl ::windows::core::RuntimeName for IWindowsParentalControlsCore {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWindowsParentalControlsCore";
}
impl IWindowsParentalControlsCoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>() -> IWindowsParentalControlsCoreVtbl {
        unsafe extern "system" fn GetVisibility<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibility(::core::mem::transmute_copy(&pevisibility)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSettings<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserSettings(&*(&pcszsid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebSettings<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWebSettings(&*(&pcszsid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebFilterInfo<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWebFilterInfo(::core::mem::transmute_copy(&pguidid), &*(&ppszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsParentalControlsCore>, ::windows::core::GetTrustLevel, GetVisibility::<Impl, OFFSET>, GetUserSettings::<Impl, OFFSET>, GetWebSettings::<Impl, OFFSET>, GetWebFilterInfo::<Impl, OFFSET>)
    }
}

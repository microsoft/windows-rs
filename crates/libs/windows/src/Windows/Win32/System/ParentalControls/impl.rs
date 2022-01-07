pub trait IWPCGamesSettingsImpl: Sized + IWPCSettingsImpl {
    fn IsBlocked();
}
impl ::windows::core::RuntimeName for IWPCGamesSettings {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCGamesSettings";
}
impl IWPCGamesSettingsVtbl {
    pub const fn new<Impl: IWPCGamesSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWPCGamesSettingsVtbl {
        unsafe extern "system" fn IsBlocked<Impl: IWPCGamesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidappid: ::windows::core::GUID, pdwreasons: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBlocked(&*(&guidappid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwreasons)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWPCGamesSettings>, base.5, IsBlocked::<Impl, OFFSET>)
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
    pub const fn new<Impl: IWPCProviderConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWPCProviderConfigVtbl {
        unsafe extern "system" fn GetUserSummary<Impl: IWPCProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrusersummary: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserSummary(&*(&bstrsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrusersummary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IWPCProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configure(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&bstrsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestOverride<Impl: IWPCProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestOverride(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&bstrpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWPCProviderConfig>, base.5, GetUserSummary::<Impl, OFFSET>, Configure::<Impl, OFFSET>, RequestOverride::<Impl, OFFSET>)
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
    pub const fn new<Impl: IWPCProviderStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWPCProviderStateVtbl {
        unsafe extern "system" fn Enable<Impl: IWPCProviderStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IWPCProviderStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWPCProviderState>, base.5, Enable::<Impl, OFFSET>, Disable::<Impl, OFFSET>)
    }
}
pub trait IWPCProviderSupportImpl: Sized {
    fn GetCurrent();
}
impl ::windows::core::RuntimeName for IWPCProviderSupport {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWPCProviderSupport";
}
impl IWPCProviderSupportVtbl {
    pub const fn new<Impl: IWPCProviderSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWPCProviderSupportVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IWPCProviderSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&pguidprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWPCProviderSupport>, base.5, GetCurrent::<Impl, OFFSET>)
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
    pub const fn new<Impl: IWPCSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWPCSettingsVtbl {
        unsafe extern "system" fn IsLoggingRequired<Impl: IWPCSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLoggingRequired(::core::mem::transmute_copy(&pfrequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Impl: IWPCSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastSettingsChangeTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Impl: IWPCSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRestrictions(::core::mem::transmute_copy(&pdwrestrictions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWPCSettings>, base.5, IsLoggingRequired::<Impl, OFFSET>, GetLastSettingsChangeTime::<Impl, OFFSET>, GetRestrictions::<Impl, OFFSET>)
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
    pub const fn new<Impl: IWPCWebSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWPCWebSettingsVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWPCWebSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSettings(::core::mem::transmute_copy(&pdwsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestURLOverride<Impl: IWPCWebSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: super::super::Foundation::PWSTR, curls: u32, ppcszsuburls: *const super::super::Foundation::PWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWPCWebSettings>, base.5, GetSettings::<Impl, OFFSET>, RequestURLOverride::<Impl, OFFSET>)
    }
}
pub trait IWindowsParentalControlsImpl: Sized + IWindowsParentalControlsCoreImpl {
    fn GetGamesSettings();
}
impl ::windows::core::RuntimeName for IWindowsParentalControls {
    const NAME: &'static str = "Windows.Win32.System.ParentalControls.IWindowsParentalControls";
}
impl IWindowsParentalControlsVtbl {
    pub const fn new<Impl: IWindowsParentalControlsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsParentalControlsVtbl {
        unsafe extern "system" fn GetGamesSettings<Impl: IWindowsParentalControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGamesSettings(&*(&pcszsid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsParentalControls>, base.5, GetGamesSettings::<Impl, OFFSET>)
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
    pub const fn new<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsParentalControlsCoreVtbl {
        unsafe extern "system" fn GetVisibility<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisibility(::core::mem::transmute_copy(&pevisibility)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSettings<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserSettings(&*(&pcszsid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebSettings<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWebSettings(&*(&pcszsid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebFilterInfo<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWebFilterInfo(::core::mem::transmute_copy(&pguidid), &*(&ppszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsParentalControlsCore>, base.5, GetVisibility::<Impl, OFFSET>, GetUserSettings::<Impl, OFFSET>, GetWebSettings::<Impl, OFFSET>, GetWebFilterInfo::<Impl, OFFSET>)
    }
}

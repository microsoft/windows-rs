#[cfg(feature = "Win32_Foundation")]
pub trait IWPCGamesSettings_Impl: Sized + IWPCSettings_Impl {
    fn IsBlocked(&mut self, guidappid: &::windows::core::GUID) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCGamesSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCGamesSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCGamesSettings_Vtbl {
        unsafe extern "system" fn IsBlocked<Impl: IWPCGamesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidappid: ::windows::core::GUID, pdwreasons: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlocked(::core::mem::transmute_copy(&guidappid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwreasons = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWPCSettings_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), IsBlocked: IsBlocked::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCGamesSettings as ::windows::core::Interface>::IID || iid == &<IWPCSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCProviderConfig_Impl: Sized {
    fn GetUserSummary(&mut self, bstrsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Configure(&mut self, hwnd: super::super::Foundation::HWND, bstrsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RequestOverride(&mut self, hwnd: super::super::Foundation::HWND, bstrpath: &super::super::Foundation::BSTR, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCProviderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCProviderConfig_Vtbl {
        unsafe extern "system" fn GetUserSummary<Impl: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrusersummary: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserSummary(::core::mem::transmute_copy(&bstrsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusersummary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&bstrsid)).into()
        }
        unsafe extern "system" fn RequestOverride<Impl: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestOverride(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&bstrpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetUserSummary: GetUserSummary::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
            RequestOverride: RequestOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCProviderConfig as ::windows::core::Interface>::IID
    }
}
pub trait IWPCProviderState_Impl: Sized {
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
}
impl IWPCProviderState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCProviderState_Vtbl {
        unsafe extern "system" fn Enable<Impl: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Disable<Impl: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Enable: Enable::<Impl, IMPL_OFFSET>, Disable: Disable::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCProviderState as ::windows::core::Interface>::IID
    }
}
pub trait IWPCProviderSupport_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IWPCProviderSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderSupport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCProviderSupport_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IWPCProviderSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCurrent: GetCurrent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCProviderSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCSettings_Impl: Sized {
    fn IsLoggingRequired(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetLastSettingsChangeTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetRestrictions(&mut self) -> ::windows::core::Result<WPCFLAG_RESTRICTION>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCSettings_Vtbl {
        unsafe extern "system" fn IsLoggingRequired<Impl: IWPCSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLoggingRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrequired = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Impl: IWPCSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastSettingsChangeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Impl: IWPCSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrestrictions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsLoggingRequired: IsLoggingRequired::<Impl, IMPL_OFFSET>,
            GetLastSettingsChangeTime: GetLastSettingsChangeTime::<Impl, IMPL_OFFSET>,
            GetRestrictions: GetRestrictions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCWebSettings_Impl: Sized + IWPCSettings_Impl {
    fn GetSettings(&mut self) -> ::windows::core::Result<WPCFLAG_WEB_SETTING>;
    fn RequestURLOverride(&mut self, hwnd: super::super::Foundation::HWND, pcszurl: super::super::Foundation::PWSTR, curls: u32, ppcszsuburls: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCWebSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCWebSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCWebSettings_Vtbl {
        unsafe extern "system" fn GetSettings<Impl: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestURLOverride<Impl: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: super::super::Foundation::PWSTR, curls: u32, ppcszsuburls: *const super::super::Foundation::PWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestURLOverride(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pcszurl), ::core::mem::transmute_copy(&curls), ::core::mem::transmute_copy(&ppcszsuburls)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfchanged = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWPCSettings_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSettings: GetSettings::<Impl, IMPL_OFFSET>,
            RequestURLOverride: RequestURLOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCWebSettings as ::windows::core::Interface>::IID || iid == &<IWPCSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsParentalControls_Impl: Sized + IWindowsParentalControlsCore_Impl {
    fn GetGamesSettings(&mut self, pcszsid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWPCGamesSettings>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowsParentalControls_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsParentalControls_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsParentalControls_Vtbl {
        unsafe extern "system" fn GetGamesSettings<Impl: IWindowsParentalControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamesSettings(::core::mem::transmute_copy(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWindowsParentalControlsCore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetGamesSettings: GetGamesSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsParentalControls as ::windows::core::Interface>::IID || iid == &<IWindowsParentalControlsCore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsParentalControlsCore_Impl: Sized {
    fn GetVisibility(&mut self) -> ::windows::core::Result<WPCFLAG_VISIBILITY>;
    fn GetUserSettings(&mut self, pcszsid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWPCSettings>;
    fn GetWebSettings(&mut self, pcszsid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWPCWebSettings>;
    fn GetWebFilterInfo(&mut self, pguidid: *mut ::windows::core::GUID, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowsParentalControlsCore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsParentalControlsCore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsParentalControlsCore_Vtbl {
        unsafe extern "system" fn GetVisibility<Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    *pevisibility = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSettings<Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserSettings(::core::mem::transmute_copy(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebSettings<Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWebSettings(::core::mem::transmute_copy(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebFilterInfo<Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWebFilterInfo(::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&ppszname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVisibility: GetVisibility::<Impl, IMPL_OFFSET>,
            GetUserSettings: GetUserSettings::<Impl, IMPL_OFFSET>,
            GetWebSettings: GetWebSettings::<Impl, IMPL_OFFSET>,
            GetWebFilterInfo: GetWebFilterInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsParentalControlsCore as ::windows::core::Interface>::IID
    }
}

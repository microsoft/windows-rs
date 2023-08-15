#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCGamesSettings_Impl: Sized + IWPCSettings_Impl {
    fn IsBlocked(&self, guidappid: &::windows_core::GUID) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWPCGamesSettings {}
#[cfg(feature = "Win32_Foundation")]
impl IWPCGamesSettings_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCGamesSettings_Impl, const OFFSET: isize>() -> IWPCGamesSettings_Vtbl {
        unsafe extern "system" fn IsBlocked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCGamesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidappid: ::windows_core::GUID, pdwreasons: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBlocked(::core::mem::transmute(&guidappid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwreasons, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWPCSettings_Vtbl::new::<Identity, Impl, OFFSET>(), IsBlocked: IsBlocked::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWPCGamesSettings as ::windows_core::ComInterface>::IID || iid == &<IWPCSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCProviderConfig_Impl: Sized {
    fn GetUserSummary(&self, bstrsid: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Configure(&self, hwnd: super::super::Foundation::HWND, bstrsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RequestOverride(&self, hwnd: super::super::Foundation::HWND, bstrpath: &::windows_core::BSTR, dwflags: &WPCFLAG_RESTRICTION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWPCProviderConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IWPCProviderConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: isize>() -> IWPCProviderConfig_Vtbl {
        unsafe extern "system" fn GetUserSummary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrusersummary: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserSummary(::core::mem::transmute(&bstrsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusersummary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Configure(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&bstrsid)).into()
        }
        unsafe extern "system" fn RequestOverride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestOverride(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&bstrpath), ::core::mem::transmute(&dwflags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserSummary: GetUserSummary::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            RequestOverride: RequestOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWPCProviderConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"implement\"`*"]
pub trait IWPCProviderState_Impl: Sized {
    fn Enable(&self) -> ::windows_core::Result<()>;
    fn Disable(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWPCProviderState {}
impl IWPCProviderState_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderState_Impl, const OFFSET: isize>() -> IWPCProviderState_Vtbl {
        unsafe extern "system" fn Enable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable().into()
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disable().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWPCProviderState as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"implement\"`*"]
pub trait IWPCProviderSupport_Impl: Sized {
    fn GetCurrent(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IWPCProviderSupport {}
impl IWPCProviderSupport_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderSupport_Impl, const OFFSET: isize>() -> IWPCProviderSupport_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCProviderSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCurrent: GetCurrent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWPCProviderSupport as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCSettings_Impl: Sized {
    fn IsLoggingRequired(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetLastSettingsChangeTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetRestrictions(&self) -> ::windows_core::Result<WPCFLAG_RESTRICTION>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWPCSettings {}
#[cfg(feature = "Win32_Foundation")]
impl IWPCSettings_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: isize>() -> IWPCSettings_Vtbl {
        unsafe extern "system" fn IsLoggingRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLoggingRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrequired, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastSettingsChangeTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrestrictions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsLoggingRequired: IsLoggingRequired::<Identity, Impl, OFFSET>,
            GetLastSettingsChangeTime: GetLastSettingsChangeTime::<Identity, Impl, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWPCSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCWebSettings_Impl: Sized + IWPCSettings_Impl {
    fn GetSettings(&self) -> ::windows_core::Result<WPCFLAG_WEB_SETTING>;
    fn RequestURLOverride(&self, hwnd: super::super::Foundation::HWND, pcszurl: &::windows_core::PCWSTR, curls: u32, ppcszsuburls: *const ::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWPCWebSettings {}
#[cfg(feature = "Win32_Foundation")]
impl IWPCWebSettings_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCWebSettings_Impl, const OFFSET: isize>() -> IWPCWebSettings_Vtbl {
        unsafe extern "system" fn GetSettings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestURLOverride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: ::windows_core::PCWSTR, curls: u32, ppcszsuburls: *const ::windows_core::PCWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestURLOverride(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pcszurl), ::core::mem::transmute_copy(&curls), ::core::mem::transmute_copy(&ppcszsuburls)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfchanged, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWPCSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSettings: GetSettings::<Identity, Impl, OFFSET>,
            RequestURLOverride: RequestURLOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWPCWebSettings as ::windows_core::ComInterface>::IID || iid == &<IWPCSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"implement\"`*"]
pub trait IWindowsParentalControls_Impl: Sized + IWindowsParentalControlsCore_Impl {
    fn GetGamesSettings(&self, pcszsid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWPCGamesSettings>;
}
impl ::windows_core::RuntimeName for IWindowsParentalControls {}
impl IWindowsParentalControls_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControls_Impl, const OFFSET: isize>() -> IWindowsParentalControls_Vtbl {
        unsafe extern "system" fn GetGamesSettings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: ::windows_core::PCWSTR, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGamesSettings(::core::mem::transmute(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWindowsParentalControlsCore_Vtbl::new::<Identity, Impl, OFFSET>(), GetGamesSettings: GetGamesSettings::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsParentalControls as ::windows_core::ComInterface>::IID || iid == &<IWindowsParentalControlsCore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ParentalControls\"`, `\"implement\"`*"]
pub trait IWindowsParentalControlsCore_Impl: Sized {
    fn GetVisibility(&self) -> ::windows_core::Result<WPCFLAG_VISIBILITY>;
    fn GetUserSettings(&self, pcszsid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWPCSettings>;
    fn GetWebSettings(&self, pcszsid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWPCWebSettings>;
    fn GetWebFilterInfo(&self, pguidid: *mut ::windows_core::GUID, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWindowsParentalControlsCore {}
impl IWindowsParentalControlsCore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>() -> IWindowsParentalControlsCore_Vtbl {
        unsafe extern "system" fn GetVisibility<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevisibility, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSettings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: ::windows_core::PCWSTR, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserSettings(::core::mem::transmute(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebSettings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: ::windows_core::PCWSTR, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWebSettings(::core::mem::transmute(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebFilterInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows_core::GUID, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWebFilterInfo(::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&ppszname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVisibility: GetVisibility::<Identity, Impl, OFFSET>,
            GetUserSettings: GetUserSettings::<Identity, Impl, OFFSET>,
            GetWebSettings: GetWebSettings::<Identity, Impl, OFFSET>,
            GetWebFilterInfo: GetWebFilterInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsParentalControlsCore as ::windows_core::ComInterface>::IID
    }
}

#[cfg(feature = "Win32_Foundation")]
pub trait IWPCGamesSettingsImpl: Sized + IWPCSettingsImpl {
    fn IsBlocked();
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCGamesSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCGamesSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCGamesSettingsVtbl {
        unsafe extern "system" fn IsBlocked<Impl: IWPCGamesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidappid: ::windows::core::GUID, pdwreasons: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsLoggingRequired::<Impl, IMPL_OFFSET>, GetLastSettingsChangeTime::<Impl, IMPL_OFFSET>, GetRestrictions::<Impl, IMPL_OFFSET>, IsBlocked::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCGamesSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCProviderConfigImpl: Sized {
    fn GetUserSummary();
    fn Configure();
    fn RequestOverride();
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCProviderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCProviderConfigVtbl {
        unsafe extern "system" fn GetUserSummary<Impl: IWPCProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrusersummary: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Configure<Impl: IWPCProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestOverride<Impl: IWPCProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetUserSummary::<Impl, IMPL_OFFSET>, Configure::<Impl, IMPL_OFFSET>, RequestOverride::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCProviderConfig as ::windows::core::Interface>::IID
    }
}
pub trait IWPCProviderStateImpl: Sized {
    fn Enable();
    fn Disable();
}
impl IWPCProviderStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCProviderStateVtbl {
        unsafe extern "system" fn Enable<Impl: IWPCProviderStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disable<Impl: IWPCProviderStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Enable::<Impl, IMPL_OFFSET>, Disable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCProviderState as ::windows::core::Interface>::IID
    }
}
pub trait IWPCProviderSupportImpl: Sized {
    fn GetCurrent();
}
impl IWPCProviderSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCProviderSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCProviderSupportVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IWPCProviderSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCurrent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCProviderSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCSettingsImpl: Sized {
    fn IsLoggingRequired();
    fn GetLastSettingsChangeTime();
    fn GetRestrictions();
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCSettingsVtbl {
        unsafe extern "system" fn IsLoggingRequired<Impl: IWPCSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Impl: IWPCSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestrictions<Impl: IWPCSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsLoggingRequired::<Impl, IMPL_OFFSET>, GetLastSettingsChangeTime::<Impl, IMPL_OFFSET>, GetRestrictions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCWebSettingsImpl: Sized + IWPCSettingsImpl {
    fn GetSettings();
    fn RequestURLOverride();
}
#[cfg(feature = "Win32_Foundation")]
impl IWPCWebSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWPCWebSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWPCWebSettingsVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWPCWebSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestURLOverride<Impl: IWPCWebSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: super::super::Foundation::PWSTR, curls: u32, ppcszsuburls: *const super::super::Foundation::PWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsLoggingRequired::<Impl, IMPL_OFFSET>, GetLastSettingsChangeTime::<Impl, IMPL_OFFSET>, GetRestrictions::<Impl, IMPL_OFFSET>, GetSettings::<Impl, IMPL_OFFSET>, RequestURLOverride::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWPCWebSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsParentalControlsImpl: Sized + IWindowsParentalControlsCoreImpl {
    fn GetGamesSettings();
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowsParentalControlsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsParentalControlsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsParentalControlsVtbl {
        unsafe extern "system" fn GetGamesSettings<Impl: IWindowsParentalControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetVisibility::<Impl, IMPL_OFFSET>, GetUserSettings::<Impl, IMPL_OFFSET>, GetWebSettings::<Impl, IMPL_OFFSET>, GetWebFilterInfo::<Impl, IMPL_OFFSET>, GetGamesSettings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsParentalControls as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsParentalControlsCoreImpl: Sized {
    fn GetVisibility();
    fn GetUserSettings();
    fn GetWebSettings();
    fn GetWebFilterInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowsParentalControlsCoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsParentalControlsCoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsParentalControlsCoreVtbl {
        unsafe extern "system" fn GetVisibility<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserSettings<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWebSettings<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszsid: super::super::Foundation::PWSTR, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWebFilterInfo<Impl: IWindowsParentalControlsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetVisibility::<Impl, IMPL_OFFSET>, GetUserSettings::<Impl, IMPL_OFFSET>, GetWebSettings::<Impl, IMPL_OFFSET>, GetWebFilterInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsParentalControlsCore as ::windows::core::Interface>::IID
    }
}

windows_core::imp::define_interface!(IWPCGamesSettings, IWPCGamesSettings_Vtbl, 0x95e87780_e158_489e_b452_bbb850790715);
impl core::ops::Deref for IWPCGamesSettings {
    type Target = IWPCSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWPCGamesSettings, windows_core::IUnknown, IWPCSettings);
impl IWPCGamesSettings {
    pub unsafe fn IsBlocked(&self, guidappid: windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBlocked)(windows_core::Interface::as_raw(self), core::mem::transmute(guidappid), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCGamesSettings_Vtbl {
    pub base__: IWPCSettings_Vtbl,
    pub IsBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_minwinbase")]
pub trait IWPCGamesSettings_Impl: IWPCSettings_Impl {
    fn IsBlocked(&self, guidappid: &windows_core::GUID) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_minwinbase")]
impl IWPCGamesSettings_Vtbl {
    pub const fn new<Identity: IWPCGamesSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsBlocked<Identity: IWPCGamesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidappid: windows_core::GUID, pdwreasons: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCGamesSettings_Impl::IsBlocked(this, core::mem::transmute(&guidappid)) {
                    Ok(ok__) => {
                        pdwreasons.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWPCSettings_Vtbl::new::<Identity, OFFSET>(), IsBlocked: IsBlocked::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCGamesSettings as windows_core::Interface>::IID || iid == &<IWPCSettings as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_minwinbase")]
impl windows_core::RuntimeName for IWPCGamesSettings {}
windows_core::imp::define_interface!(IWPCProviderConfig, IWPCProviderConfig_Vtbl, 0xbef54196_2d02_4a26_b6e5_d65af295d0f1);
windows_core::imp::interface_hierarchy!(IWPCProviderConfig, windows_core::IUnknown);
impl IWPCProviderConfig {
    pub unsafe fn GetUserSummary(&self, bstrsid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserSummary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn Configure(&self, hwnd: super::windef::HWND, bstrsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), hwnd, core::mem::transmute_copy(bstrsid)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RequestOverride(&self, hwnd: super::windef::HWND, bstrpath: &windows_core::BSTR, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestOverride)(windows_core::Interface::as_raw(self), hwnd, core::mem::transmute_copy(bstrpath), dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserSummary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    Configure: usize,
    #[cfg(feature = "Win32_windef")]
    pub RequestOverride: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RequestOverride: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait IWPCProviderConfig_Impl: windows_core::IUnknownImpl {
    fn GetUserSummary(&self, bstrsid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn Configure(&self, hwnd: super::windef::HWND, bstrsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RequestOverride(&self, hwnd: super::windef::HWND, bstrpath: &windows_core::BSTR, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IWPCProviderConfig_Vtbl {
    pub const fn new<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUserSummary<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsid: *mut core::ffi::c_void, pbstrusersummary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCProviderConfig_Impl::GetUserSummary(this, core::mem::transmute(&bstrsid)) {
                    Ok(ok__) => {
                        pbstrusersummary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Configure<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, bstrsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderConfig_Impl::Configure(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&bstrsid)).into()
            }
        }
        unsafe extern "system" fn RequestOverride<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, bstrpath: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderConfig_Impl::RequestOverride(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&bstrpath), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserSummary: GetUserSummary::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
            RequestOverride: RequestOverride::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCProviderConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWPCProviderConfig {}
windows_core::imp::define_interface!(IWPCProviderState, IWPCProviderState_Vtbl, 0x50b6a267_c4bd_450b_adb5_759073837c9e);
windows_core::imp::interface_hierarchy!(IWPCProviderState, windows_core::IUnknown);
impl IWPCProviderState {
    pub unsafe fn Enable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Disable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWPCProviderState_Impl: windows_core::IUnknownImpl {
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
}
impl IWPCProviderState_Vtbl {
    pub const fn new<Identity: IWPCProviderState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enable<Identity: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderState_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn Disable<Identity: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderState_Impl::Disable(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Enable: Enable::<Identity, OFFSET>, Disable: Disable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCProviderState as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCProviderState {}
windows_core::imp::define_interface!(IWPCProviderSupport, IWPCProviderSupport_Vtbl, 0x41eba572_23ed_4779_bec1_8df96206c44c);
windows_core::imp::interface_hierarchy!(IWPCProviderSupport, windows_core::IUnknown);
impl IWPCProviderSupport {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IWPCProviderSupport_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IWPCProviderSupport_Vtbl {
    pub const fn new<Identity: IWPCProviderSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IWPCProviderSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidprovider: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCProviderSupport_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        pguidprovider.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCurrent: GetCurrent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCProviderSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCProviderSupport {}
windows_core::imp::define_interface!(IWPCSettings, IWPCSettings_Vtbl, 0x8fdf6ca1_0189_47e4_b670_1a8a4636e340);
windows_core::imp::interface_hierarchy!(IWPCSettings, windows_core::IUnknown);
impl IWPCSettings {
    pub unsafe fn IsLoggingRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLoggingRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwinbase")]
    pub unsafe fn GetLastSettingsChangeTime(&self) -> windows_core::Result<super::minwinbase::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastSettingsChangeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRestrictions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsLoggingRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwinbase")]
    pub GetLastSettingsChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwinbase"))]
    GetLastSettingsChangeTime: usize,
    pub GetRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_minwinbase")]
pub trait IWPCSettings_Impl: windows_core::IUnknownImpl {
    fn IsLoggingRequired(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetLastSettingsChangeTime(&self) -> windows_core::Result<super::minwinbase::SYSTEMTIME>;
    fn GetRestrictions(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_minwinbase")]
impl IWPCSettings_Vtbl {
    pub const fn new<Identity: IWPCSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsLoggingRequired<Identity: IWPCSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrequired: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCSettings_Impl::IsLoggingRequired(this) {
                    Ok(ok__) => {
                        pfrequired.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Identity: IWPCSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCSettings_Impl::GetLastSettingsChangeTime(this) {
                    Ok(ok__) => {
                        ptime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: IWPCSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrestrictions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCSettings_Impl::GetRestrictions(this) {
                    Ok(ok__) => {
                        pdwrestrictions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsLoggingRequired: IsLoggingRequired::<Identity, OFFSET>,
            GetLastSettingsChangeTime: GetLastSettingsChangeTime::<Identity, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCSettings as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_minwinbase")]
impl windows_core::RuntimeName for IWPCSettings {}
windows_core::imp::define_interface!(IWPCWebSettings, IWPCWebSettings_Vtbl, 0xffccbdb8_0992_4c30_b0f1_1cbb09c240aa);
impl core::ops::Deref for IWPCWebSettings {
    type Target = IWPCSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWPCWebSettings, windows_core::IUnknown, IWPCSettings);
impl IWPCWebSettings {
    pub unsafe fn GetSettings(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettings)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RequestURLOverride<P1>(&self, hwnd: super::windef::HWND, pcszurl: P1, curls: u32, ppcszsuburls: *const windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestURLOverride)(windows_core::Interface::as_raw(self), hwnd, pcszurl.param().abi(), curls, ppcszsuburls, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCWebSettings_Vtbl {
    pub base__: IWPCSettings_Vtbl,
    pub GetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub RequestURLOverride: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RequestURLOverride: usize,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef"))]
pub trait IWPCWebSettings_Impl: IWPCSettings_Impl {
    fn GetSettings(&self) -> windows_core::Result<u32>;
    fn RequestURLOverride(&self, hwnd: super::windef::HWND, pcszurl: &windows_core::PCWSTR, curls: u32, ppcszsuburls: *const windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef"))]
impl IWPCWebSettings_Vtbl {
    pub const fn new<Identity: IWPCWebSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSettings<Identity: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsettings: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCWebSettings_Impl::GetSettings(this) {
                    Ok(ok__) => {
                        pdwsettings.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestURLOverride<Identity: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, pcszurl: windows_core::PCWSTR, curls: u32, ppcszsuburls: *const windows_core::PCWSTR, pfchanged: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCWebSettings_Impl::RequestURLOverride(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&pcszurl), core::mem::transmute_copy(&curls), core::mem::transmute_copy(&ppcszsuburls)) {
                    Ok(ok__) => {
                        pfchanged.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWPCSettings_Vtbl::new::<Identity, OFFSET>(),
            GetSettings: GetSettings::<Identity, OFFSET>,
            RequestURLOverride: RequestURLOverride::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCWebSettings as windows_core::Interface>::IID || iid == &<IWPCSettings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IWPCWebSettings {}
windows_core::imp::define_interface!(IWindowsParentalControls, IWindowsParentalControls_Vtbl, 0x28b4d88b_e072_49e6_804d_26edbe21a7b9);
impl core::ops::Deref for IWindowsParentalControls {
    type Target = IWindowsParentalControlsCore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowsParentalControls, windows_core::IUnknown, IWindowsParentalControlsCore);
impl IWindowsParentalControls {
    pub unsafe fn GetGamesSettings<P0>(&self, pcszsid: P0) -> windows_core::Result<IWPCGamesSettings>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGamesSettings)(windows_core::Interface::as_raw(self), pcszsid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsParentalControls_Vtbl {
    pub base__: IWindowsParentalControlsCore_Vtbl,
    pub GetGamesSettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWindowsParentalControls_Impl: IWindowsParentalControlsCore_Impl {
    fn GetGamesSettings(&self, pcszsid: &windows_core::PCWSTR) -> windows_core::Result<IWPCGamesSettings>;
}
impl IWindowsParentalControls_Vtbl {
    pub const fn new<Identity: IWindowsParentalControls_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGamesSettings<Identity: IWindowsParentalControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszsid: windows_core::PCWSTR, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControls_Impl::GetGamesSettings(this, core::mem::transmute(&pcszsid)) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWindowsParentalControlsCore_Vtbl::new::<Identity, OFFSET>(), GetGamesSettings: GetGamesSettings::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsParentalControls as windows_core::Interface>::IID || iid == &<IWindowsParentalControlsCore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowsParentalControls {}
windows_core::imp::define_interface!(IWindowsParentalControlsCore, IWindowsParentalControlsCore_Vtbl, 0x4ff40a0f_3f3b_4d7c_a41b_4f39d7b44d05);
windows_core::imp::interface_hierarchy!(IWindowsParentalControlsCore, windows_core::IUnknown);
impl IWindowsParentalControlsCore {
    pub unsafe fn GetVisibility(&self) -> windows_core::Result<WPCFLAG_VISIBILITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibility)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUserSettings<P0>(&self, pcszsid: P0) -> windows_core::Result<IWPCSettings>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserSettings)(windows_core::Interface::as_raw(self), pcszsid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWebSettings<P0>(&self, pcszsid: P0) -> windows_core::Result<IWPCWebSettings>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWebSettings)(windows_core::Interface::as_raw(self), pcszsid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWebFilterInfo(&self, pguidid: *mut windows_core::GUID, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWebFilterInfo)(windows_core::Interface::as_raw(self), pguidid as _, ppszname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsParentalControlsCore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WPCFLAG_VISIBILITY) -> windows_core::HRESULT,
    pub GetUserSettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWebSettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWebFilterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IWindowsParentalControlsCore_Impl: windows_core::IUnknownImpl {
    fn GetVisibility(&self) -> windows_core::Result<WPCFLAG_VISIBILITY>;
    fn GetUserSettings(&self, pcszsid: &windows_core::PCWSTR) -> windows_core::Result<IWPCSettings>;
    fn GetWebSettings(&self, pcszsid: &windows_core::PCWSTR) -> windows_core::Result<IWPCWebSettings>;
    fn GetWebFilterInfo(&self, pguidid: *mut windows_core::GUID, ppszname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IWindowsParentalControlsCore_Vtbl {
    pub const fn new<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVisibility<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControlsCore_Impl::GetVisibility(this) {
                    Ok(ok__) => {
                        pevisibility.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserSettings<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszsid: windows_core::PCWSTR, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControlsCore_Impl::GetUserSettings(this, core::mem::transmute(&pcszsid)) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWebSettings<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszsid: windows_core::PCWSTR, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControlsCore_Impl::GetWebSettings(this, core::mem::transmute(&pcszsid)) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWebFilterInfo<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidid: *mut windows_core::GUID, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsParentalControlsCore_Impl::GetWebFilterInfo(this, core::mem::transmute_copy(&pguidid), core::mem::transmute_copy(&ppszname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVisibility: GetVisibility::<Identity, OFFSET>,
            GetUserSettings: GetUserSettings::<Identity, OFFSET>,
            GetWebSettings: GetWebSettings::<Identity, OFFSET>,
            GetWebFilterInfo: GetWebFilterInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsParentalControlsCore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowsParentalControlsCore {}
pub const WPCFLAG_APPLICATION: WPCFLAG_OVERRIDE = 1;
pub const WPCFLAG_APPS_RESTRICTED: WPCFLAG_RESTRICTION = 16;
pub const WPCFLAG_GAMES_BLOCKED: WPCFLAG_RESTRICTION = 8;
pub const WPCFLAG_GAMES_RESTRICTED: WPCFLAG_RESTRICTION = 64;
pub const WPCFLAG_HOURS_RESTRICTED: WPCFLAG_RESTRICTION = 4;
pub const WPCFLAG_LOGGING_REQUIRED: WPCFLAG_RESTRICTION = 1;
pub const WPCFLAG_NO_RESTRICTION: WPCFLAG_RESTRICTION = 0;
pub type WPCFLAG_OVERRIDE = i32;
pub type WPCFLAG_RESTRICTION = i32;
pub const WPCFLAG_TIME_ALLOWANCE_RESTRICTED: WPCFLAG_RESTRICTION = 32;
pub type WPCFLAG_VISIBILITY = i32;
pub const WPCFLAG_WEB_FILTERED: WPCFLAG_RESTRICTION = 2;
pub type WPCFLAG_WEB_SETTING = i32;
pub const WPCFLAG_WEB_SETTING_DOWNLOADSBLOCKED: WPCFLAG_WEB_SETTING = 1;
pub const WPCFLAG_WEB_SETTING_NOTBLOCKED: WPCFLAG_WEB_SETTING = 0;
pub const WPCFLAG_WPC_HIDDEN: WPCFLAG_VISIBILITY = 1;
pub const WPCFLAG_WPC_VISIBLE: WPCFLAG_VISIBILITY = 0;
pub const WindowsParentalControls: windows_core::GUID = windows_core::GUID::from_u128(0xe77cc89b_7401_4c04_8ced_149db35add04);
pub const WpcProviderSupport: windows_core::GUID = windows_core::GUID::from_u128(0xbb18c7a0_2186_4be0_97d8_04847b628e02);
pub const WpcSettingsProvider: windows_core::GUID = windows_core::GUID::from_u128(0x355dffaa_3b9f_435c_b428_5d44290bc5f2);

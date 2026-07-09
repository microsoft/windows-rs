windows_core::imp::define_interface!(ICoreInputInterop, ICoreInputInterop_Vtbl, 0x40bfe3e3_b75a_4479_ac96_475365749bb8);
windows_core::imp::interface_hierarchy!(ICoreInputInterop, windows_core::IUnknown);
impl ICoreInputInterop {
    pub unsafe fn SetInputSource<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputSource)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn SetMessageHandled(&self, value: super::rpc::boolean) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageHandled)(windows_core::Interface::as_raw(self), value) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInputSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "rpc")]
    pub SetMessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, super::rpc::boolean) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    SetMessageHandled: usize,
}
#[cfg(feature = "rpc")]
pub trait ICoreInputInterop_Impl: windows_core::IUnknownImpl {
    fn SetInputSource(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetMessageHandled(&self, value: super::rpc::boolean) -> windows_core::Result<()>;
}
#[cfg(feature = "rpc")]
impl ICoreInputInterop_Vtbl {
    pub const fn new<Identity: ICoreInputInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInputSource<Identity: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop_Impl::SetInputSource(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::rpc::boolean) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop_Impl::SetMessageHandled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInputSource: SetInputSource::<Identity, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "rpc")]
impl windows_core::RuntimeName for ICoreInputInterop {}
windows_core::imp::define_interface!(ICoreInputInterop2, ICoreInputInterop2_Vtbl, 0xb8a2acd7_a0f0_40ee_8ee7_c82f59cc5cd4);
windows_core::imp::interface_hierarchy!(ICoreInputInterop2, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreInputInterop2 {
    #[cfg(feature = "windef")]
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ChangeHostingContext(&self, newparentwindow: super::windef::HWND, newviewinstanceid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeHostingContext)(windows_core::Interface::as_raw(self), newparentwindow, newviewinstanceid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    WindowHandle: usize,
    #[cfg(feature = "windef")]
    pub ChangeHostingContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ChangeHostingContext: usize,
}
#[cfg(feature = "windef")]
pub trait ICoreInputInterop2_Impl: windows_core::IUnknownImpl {
    fn WindowHandle(&self) -> windows_core::Result<super::windef::HWND>;
    fn ChangeHostingContext(&self, newparentwindow: super::windef::HWND, newviewinstanceid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl ICoreInputInterop2_Vtbl {
    pub const fn new<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowHandle<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreInputInterop2_Impl::WindowHandle(this) {
                    Ok(ok__) => {
                        window.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeHostingContext<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newparentwindow: super::windef::HWND, newviewinstanceid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop2_Impl::ChangeHostingContext(this, core::mem::transmute_copy(&newparentwindow), core::mem::transmute_copy(&newviewinstanceid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreInputInterop2, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            ChangeHostingContext: ChangeHostingContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ICoreInputInterop2 {}
windows_core::imp::define_interface!(ICoreWindowAdapterInterop, ICoreWindowAdapterInterop_Vtbl, 0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
windows_core::imp::interface_hierarchy!(ICoreWindowAdapterInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreWindowAdapterInterop {
    pub unsafe fn AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppActivationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoreApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoloViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PositionerClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemNavigationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TitleBarClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetWindowClientAdapter<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWindowClientAdapter)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowAdapterInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppActivationClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoreApplicationViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HoloViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PositionerClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemNavigationClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TitleBarClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreWindowAdapterInterop_Impl: windows_core::IUnknownImpl {
    fn AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetWindowClientAdapter(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ICoreWindowAdapterInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AppActivationClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::AppActivationClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::ApplicationViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CoreApplicationViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::CoreApplicationViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HoloViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::HoloViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PositionerClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::PositionerClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SystemNavigationClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::SystemNavigationClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TitleBarClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::TitleBarClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWindowClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowAdapterInterop_Impl::SetWindowClientAdapter(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindowAdapterInterop, OFFSET>(),
            AppActivationClientAdapter: AppActivationClientAdapter::<Identity, OFFSET>,
            ApplicationViewClientAdapter: ApplicationViewClientAdapter::<Identity, OFFSET>,
            CoreApplicationViewClientAdapter: CoreApplicationViewClientAdapter::<Identity, OFFSET>,
            HoloViewClientAdapter: HoloViewClientAdapter::<Identity, OFFSET>,
            PositionerClientAdapter: PositionerClientAdapter::<Identity, OFFSET>,
            SystemNavigationClientAdapter: SystemNavigationClientAdapter::<Identity, OFFSET>,
            TitleBarClientAdapter: TitleBarClientAdapter::<Identity, OFFSET>,
            SetWindowClientAdapter: SetWindowClientAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowAdapterInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWindowAdapterInterop {}
windows_core::imp::define_interface!(ICoreWindowComponentInterop, ICoreWindowComponentInterop_Vtbl, 0x0576ab31_a310_4c40_ba31_fd37e0298dfa);
windows_core::imp::interface_hierarchy!(ICoreWindowComponentInterop, windows_core::IUnknown);
impl ICoreWindowComponentInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn ConfigureComponentInput<P2>(&self, hostviewinstanceid: u32, hwndhost: super::windef::HWND, inputsourcevisual: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConfigureComponentInput)(windows_core::Interface::as_raw(self), hostviewinstanceid, hwndhost, inputsourcevisual.param().abi()) }
    }
    pub unsafe fn GetViewInstanceId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowComponentInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub ConfigureComponentInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::windef::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ConfigureComponentInput: usize,
    pub GetViewInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ICoreWindowComponentInterop_Impl: windows_core::IUnknownImpl {
    fn ConfigureComponentInput(&self, hostviewinstanceid: u32, hwndhost: super::windef::HWND, inputsourcevisual: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetViewInstanceId(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "windef")]
impl ICoreWindowComponentInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureComponentInput<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::windef::HWND, inputsourcevisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowComponentInterop_Impl::ConfigureComponentInput(this, core::mem::transmute_copy(&hostviewinstanceid), core::mem::transmute_copy(&hwndhost), core::mem::transmute_copy(&inputsourcevisual)).into()
            }
        }
        unsafe extern "system" fn GetViewInstanceId<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, componentviewinstanceid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowComponentInterop_Impl::GetViewInstanceId(this) {
                    Ok(ok__) => {
                        componentviewinstanceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureComponentInput: ConfigureComponentInput::<Identity, OFFSET>,
            GetViewInstanceId: GetViewInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowComponentInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ICoreWindowComponentInterop {}
windows_core::imp::define_interface!(ICoreWindowInterop, ICoreWindowInterop_Vtbl, 0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
windows_core::imp::interface_hierarchy!(ICoreWindowInterop, windows_core::IUnknown);
impl ICoreWindowInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn SetMessageHandled(&self, value: super::rpc::boolean) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageHandled)(windows_core::Interface::as_raw(self), value) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    WindowHandle: usize,
    #[cfg(feature = "rpc")]
    pub SetMessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, super::rpc::boolean) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    SetMessageHandled: usize,
}
#[cfg(all(feature = "rpc", feature = "windef"))]
pub trait ICoreWindowInterop_Impl: windows_core::IUnknownImpl {
    fn WindowHandle(&self) -> windows_core::Result<super::windef::HWND>;
    fn SetMessageHandled(&self, value: super::rpc::boolean) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "windef"))]
impl ICoreWindowInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowHandle<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowInterop_Impl::WindowHandle(this) {
                    Ok(ok__) => {
                        hwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::rpc::boolean) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowInterop_Impl::SetMessageHandled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "windef"))]
impl windows_core::RuntimeName for ICoreWindowInterop {}

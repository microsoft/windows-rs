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
    pub unsafe fn put_MessageHandled(&self, value: super::boolean) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MessageHandled)(windows_core::Interface::as_raw(self), value) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInputSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "rpc")]
    pub put_MessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, super::boolean) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    put_MessageHandled: usize,
}
#[cfg(feature = "rpc")]
pub trait ICoreInputInterop_Impl: windows_core::IUnknownImpl {
    fn SetInputSource(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn put_MessageHandled(&self, value: super::boolean) -> windows_core::Result<()>;
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
        unsafe extern "system" fn put_MessageHandled<Identity: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::boolean) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop_Impl::put_MessageHandled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInputSource: SetInputSource::<Identity, OFFSET>,
            put_MessageHandled: put_MessageHandled::<Identity, OFFSET>,
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
    pub unsafe fn get_WindowHandle(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ChangeHostingContext(&self, newparentwindow: super::HWND, newviewinstanceid: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeHostingContext)(windows_core::Interface::as_raw(self), newparentwindow, newviewinstanceid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub get_WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_WindowHandle: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ChangeHostingContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ChangeHostingContext: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait ICoreInputInterop2_Impl: windows_core::IUnknownImpl {
    fn get_WindowHandle(&self) -> windows_core::Result<super::HWND>;
    fn ChangeHostingContext(&self, newparentwindow: super::HWND, newviewinstanceid: super::UINT32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl ICoreInputInterop2_Vtbl {
    pub const fn new<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WindowHandle<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreInputInterop2_Impl::get_WindowHandle(this) {
                    Ok(ok__) => {
                        window.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeHostingContext<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newparentwindow: super::HWND, newviewinstanceid: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop2_Impl::ChangeHostingContext(this, core::mem::transmute_copy(&newparentwindow), core::mem::transmute_copy(&newviewinstanceid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreInputInterop2, OFFSET>(),
            get_WindowHandle: get_WindowHandle::<Identity, OFFSET>,
            ChangeHostingContext: ChangeHostingContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ICoreInputInterop2 {}
windows_core::imp::define_interface!(ICoreWindowAdapterInterop, ICoreWindowAdapterInterop_Vtbl, 0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
windows_core::imp::interface_hierarchy!(ICoreWindowAdapterInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreWindowAdapterInterop {
    pub unsafe fn get_AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AppActivationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CoreApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HoloViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PositionerClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SystemNavigationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TitleBarClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub get_AppActivationClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ApplicationViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CoreApplicationViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HoloViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PositionerClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SystemNavigationClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TitleBarClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreWindowAdapterInterop_Impl: windows_core::IUnknownImpl {
    fn get_AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetWindowClientAdapter(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ICoreWindowAdapterInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AppActivationClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_AppActivationClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ApplicationViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_ApplicationViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CoreApplicationViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_CoreApplicationViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HoloViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_HoloViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PositionerClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_PositionerClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SystemNavigationClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_SystemNavigationClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TitleBarClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::get_TitleBarClientAdapter(this) {
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
            get_AppActivationClientAdapter: get_AppActivationClientAdapter::<Identity, OFFSET>,
            get_ApplicationViewClientAdapter: get_ApplicationViewClientAdapter::<Identity, OFFSET>,
            get_CoreApplicationViewClientAdapter: get_CoreApplicationViewClientAdapter::<Identity, OFFSET>,
            get_HoloViewClientAdapter: get_HoloViewClientAdapter::<Identity, OFFSET>,
            get_PositionerClientAdapter: get_PositionerClientAdapter::<Identity, OFFSET>,
            get_SystemNavigationClientAdapter: get_SystemNavigationClientAdapter::<Identity, OFFSET>,
            get_TitleBarClientAdapter: get_TitleBarClientAdapter::<Identity, OFFSET>,
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ConfigureComponentInput<P2>(&self, hostviewinstanceid: super::UINT32, hwndhost: super::HWND, inputsourcevisual: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConfigureComponentInput)(windows_core::Interface::as_raw(self), hostviewinstanceid, hwndhost, inputsourcevisual.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetViewInstanceId(&self) -> windows_core::Result<super::UINT32> {
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ConfigureComponentInput: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, super::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ConfigureComponentInput: usize,
    #[cfg(feature = "shared")]
    pub GetViewInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetViewInstanceId: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait ICoreWindowComponentInterop_Impl: windows_core::IUnknownImpl {
    fn ConfigureComponentInput(&self, hostviewinstanceid: super::UINT32, hwndhost: super::HWND, inputsourcevisual: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetViewInstanceId(&self) -> windows_core::Result<super::UINT32>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl ICoreWindowComponentInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureComponentInput<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostviewinstanceid: super::UINT32, hwndhost: super::HWND, inputsourcevisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowComponentInterop_Impl::ConfigureComponentInput(this, core::mem::transmute_copy(&hostviewinstanceid), core::mem::transmute_copy(&hwndhost), core::mem::transmute_copy(&inputsourcevisual)).into()
            }
        }
        unsafe extern "system" fn GetViewInstanceId<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, componentviewinstanceid: *mut super::UINT32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ICoreWindowComponentInterop {}
windows_core::imp::define_interface!(ICoreWindowInterop, ICoreWindowInterop_Vtbl, 0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
windows_core::imp::interface_hierarchy!(ICoreWindowInterop, windows_core::IUnknown);
impl ICoreWindowInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn get_WindowHandle(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn put_MessageHandled(&self, value: super::boolean) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MessageHandled)(windows_core::Interface::as_raw(self), value) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub get_WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_WindowHandle: usize,
    #[cfg(feature = "rpc")]
    pub put_MessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, super::boolean) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    put_MessageHandled: usize,
}
#[cfg(all(feature = "rpc", feature = "windef"))]
pub trait ICoreWindowInterop_Impl: windows_core::IUnknownImpl {
    fn get_WindowHandle(&self) -> windows_core::Result<super::HWND>;
    fn put_MessageHandled(&self, value: super::boolean) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "windef"))]
impl ICoreWindowInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WindowHandle<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowInterop_Impl::get_WindowHandle(this) {
                    Ok(ok__) => {
                        hwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MessageHandled<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::boolean) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowInterop_Impl::put_MessageHandled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_WindowHandle: get_WindowHandle::<Identity, OFFSET>,
            put_MessageHandled: put_MessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "windef"))]
impl windows_core::RuntimeName for ICoreWindowInterop {}

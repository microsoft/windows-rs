windows_core::imp::define_interface!(IDesktopWindowXamlSourceNative, IDesktopWindowXamlSourceNative_Vtbl, 0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554);
windows_core::imp::interface_hierarchy!(IDesktopWindowXamlSourceNative, windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[cfg(feature = "windef")]
    pub unsafe fn AttachToWindow(&self, parentwnd: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AttachToWindow)(windows_core::Interface::as_raw(self), parentwnd) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub AttachToWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AttachToWindow: usize,
    #[cfg(feature = "windef")]
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    WindowHandle: usize,
}
#[cfg(feature = "windef")]
pub trait IDesktopWindowXamlSourceNative_Impl: windows_core::IUnknownImpl {
    fn AttachToWindow(&self, parentwnd: super::HWND) -> windows_core::Result<()>;
    fn WindowHandle(&self) -> windows_core::Result<super::HWND>;
}
#[cfg(feature = "windef")]
impl IDesktopWindowXamlSourceNative_Vtbl {
    pub const fn new<Identity: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachToWindow<Identity: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwnd: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopWindowXamlSourceNative_Impl::AttachToWindow(this, core::mem::transmute_copy(&parentwnd)).into()
            }
        }
        unsafe extern "system" fn WindowHandle<Identity: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopWindowXamlSourceNative_Impl::WindowHandle(this) {
                    Ok(ok__) => {
                        hwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachToWindow: AttachToWindow::<Identity, OFFSET>,
            WindowHandle: WindowHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDesktopWindowXamlSourceNative {}
windows_core::imp::define_interface!(IDesktopWindowXamlSourceNative2, IDesktopWindowXamlSourceNative2_Vtbl, 0xe3dcd8c7_3057_4692_99c3_7b7720afda31);
impl core::ops::Deref for IDesktopWindowXamlSourceNative2 {
    type Target = IDesktopWindowXamlSourceNative;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDesktopWindowXamlSourceNative2, windows_core::IUnknown, IDesktopWindowXamlSourceNative);
impl IDesktopWindowXamlSourceNative2 {
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn PreTranslateMessage(&self, message: *const super::MSG) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreTranslateMessage)(windows_core::Interface::as_raw(self), message, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2_Vtbl {
    pub base__: IDesktopWindowXamlSourceNative_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub PreTranslateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MSG, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    PreTranslateMessage: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait IDesktopWindowXamlSourceNative2_Impl: IDesktopWindowXamlSourceNative_Impl {
    fn PreTranslateMessage(&self, message: *const super::MSG) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl IDesktopWindowXamlSourceNative2_Vtbl {
    pub const fn new<Identity: IDesktopWindowXamlSourceNative2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreTranslateMessage<Identity: IDesktopWindowXamlSourceNative2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *const super::MSG, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopWindowXamlSourceNative2_Impl::PreTranslateMessage(this, core::mem::transmute_copy(&message)) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDesktopWindowXamlSourceNative_Vtbl::new::<Identity, OFFSET>(), PreTranslateMessage: PreTranslateMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative2 as windows_core::Interface>::IID || iid == &<IDesktopWindowXamlSourceNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IDesktopWindowXamlSourceNative2 {}

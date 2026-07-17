windows_core::imp::define_interface!(IHolographicSpaceInterop, IHolographicSpaceInterop_Vtbl, 0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
windows_core::imp::interface_hierarchy!(IHolographicSpaceInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicSpaceInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn CreateForWindow<T>(&self, window: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IHolographicSpaceInterop_Impl: windows_core::IUnknownImpl {
    fn CreateForWindow(&self, window: super::windef::HWND, riid: *const windows_core::GUID, holographicspace: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IHolographicSpaceInterop_Vtbl {
    pub const fn new<Identity: IHolographicSpaceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForWindow<Identity: IHolographicSpaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::windef::HWND, riid: *const windows_core::GUID, holographicspace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicSpaceInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&holographicspace)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicSpaceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicSpaceInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IHolographicSpaceInterop {}

pub const CLSID_AudioFrameNativeFactory: windows_core::GUID = windows_core::GUID::from_u128(0x16a0a3b9_9f65_4102_9367_2cda3a4f372a);
pub const CLSID_VideoFrameNativeFactory: windows_core::GUID = windows_core::GUID::from_u128(0xd194386a_04e3_4814_8100_b2b0ae6d78c7);
windows_core::imp::define_interface!(IAudioFrameNative, IAudioFrameNative_Vtbl, 0x20be1e2e_930f_4746_9335_3c332f255093);
windows_core::imp::interface_hierarchy!(IAudioFrameNative, windows_core::IUnknown, windows_core::IInspectable);
impl IAudioFrameNative {
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetData(&self, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetData: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub trait IAudioFrameNative_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl IAudioFrameNative_Vtbl {
    pub const fn new<Identity: IAudioFrameNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IAudioFrameNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioFrameNative_Impl::GetData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioFrameNative, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFrameNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl windows_core::RuntimeName for IAudioFrameNative {}
windows_core::imp::define_interface!(IAudioFrameNativeFactory, IAudioFrameNativeFactory_Vtbl, 0x7bd67cf8_bf7d_43e6_af8d_b170ee0c0110);
windows_core::imp::interface_hierarchy!(IAudioFrameNativeFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IAudioFrameNativeFactory {
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
    pub unsafe fn CreateFromMFSample<P0>(&self, data: P0, forcereadonly: bool, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateFromMFSample)(windows_core::Interface::as_raw(self), data.param().abi(), forcereadonly.into(), riid, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
    pub CreateFromMFSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef")))]
    CreateFromMFSample: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
pub trait IAudioFrameNativeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateFromMFSample(&self, data: windows_core::Ref<super::IMFSample>, forcereadonly: windows_core::BOOL, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
impl IAudioFrameNativeFactory_Vtbl {
    pub const fn new<Identity: IAudioFrameNativeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromMFSample<Identity: IAudioFrameNativeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, forcereadonly: windows_core::BOOL, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioFrameNativeFactory_Impl::CreateFromMFSample(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioFrameNativeFactory, OFFSET>(),
            CreateFromMFSample: CreateFromMFSample::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFrameNativeFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
impl windows_core::RuntimeName for IAudioFrameNativeFactory {}
windows_core::imp::define_interface!(IVideoFrameNative, IVideoFrameNative_Vtbl, 0x26ba702b_314a_4620_aaf6_7a51aa58fa18);
windows_core::imp::interface_hierarchy!(IVideoFrameNative, windows_core::IUnknown, windows_core::IInspectable);
impl IVideoFrameNative {
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetData(&self, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetDevice(&self, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetData: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetDevice: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub trait IVideoFrameNative_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetDevice(&self, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl IVideoFrameNative_Vtbl {
    pub const fn new<Identity: IVideoFrameNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IVideoFrameNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoFrameNative_Impl::GetData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IVideoFrameNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoFrameNative_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoFrameNative, OFFSET>(),
            GetData: GetData::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoFrameNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl windows_core::RuntimeName for IVideoFrameNative {}
windows_core::imp::define_interface!(IVideoFrameNativeFactory, IVideoFrameNativeFactory_Vtbl, 0x69e3693e_8e1e_4e63_ac4c_7fdc21d9731d);
windows_core::imp::interface_hierarchy!(IVideoFrameNativeFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVideoFrameNativeFactory {
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn CreateFromMFSample<P0, P6>(&self, data: P0, subtype: *const windows_core::GUID, width: super::UINT32, height: super::UINT32, forcereadonly: bool, mindisplayaperture: Option<*const super::MFVideoArea>, device: P6, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFSample>,
        P6: windows_core::Param<super::IMFDXGIDeviceManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateFromMFSample)(windows_core::Interface::as_raw(self), data.param().abi(), subtype, width, height, forcereadonly.into(), mindisplayaperture.unwrap_or(core::mem::zeroed()) as _, device.param().abi(), riid, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared", feature = "windef"))]
    pub CreateFromMFSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, super::UINT32, super::UINT32, windows_core::BOOL, *const super::MFVideoArea, *mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared", feature = "windef")))]
    CreateFromMFSample: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared", feature = "windef"))]
pub trait IVideoFrameNativeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateFromMFSample(&self, data: windows_core::Ref<super::IMFSample>, subtype: *const windows_core::GUID, width: super::UINT32, height: super::UINT32, forcereadonly: windows_core::BOOL, mindisplayaperture: *const super::MFVideoArea, device: windows_core::Ref<super::IMFDXGIDeviceManager>, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared", feature = "windef"))]
impl IVideoFrameNativeFactory_Vtbl {
    pub const fn new<Identity: IVideoFrameNativeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromMFSample<Identity: IVideoFrameNativeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, subtype: *const windows_core::GUID, width: super::UINT32, height: super::UINT32, forcereadonly: windows_core::BOOL, mindisplayaperture: *const super::MFVideoArea, device: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoFrameNativeFactory_Impl::CreateFromMFSample(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&subtype), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&mindisplayaperture), core::mem::transmute_copy(&device), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoFrameNativeFactory, OFFSET>(),
            CreateFromMFSample: CreateFromMFSample::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoFrameNativeFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IVideoFrameNativeFactory {}

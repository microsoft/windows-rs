#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3D9_Impl: Sized + windows_core::IUnknownImpl {
    fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::Result<()>;
    fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32;
    fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn CheckDeviceType(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::Result<()>;
    fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::Result<()>;
    fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::Result<()>;
    fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR;
    fn CreateDevice(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3D9 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3D9_Vtbl {
    pub const fn new<Identity: IDirect3D9_Impl, const OFFSET: isize>() -> IDirect3D9_Vtbl {
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::RegisterSoftwareDevice(this, core::mem::transmute_copy(&pinitializefunction)).into()
        }
        unsafe extern "system" fn GetAdapterCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::GetAdapterCount(this)
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::GetAdapterIdentifier(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pidentifier)).into()
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::GetAdapterModeCount(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::EnumAdapterModes(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::GetAdapterDisplayMode(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn CheckDeviceType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::CheckDeviceType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&backbufferformat), core::mem::transmute_copy(&bwindowed)).into()
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::CheckDeviceFormat(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&rtype), core::mem::transmute_copy(&checkformat)).into()
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::CheckDeviceMultiSampleType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&surfaceformat), core::mem::transmute_copy(&windowed), core::mem::transmute_copy(&multisampletype), core::mem::transmute_copy(&pqualitylevels)).into()
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::CheckDepthStencilMatch(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&depthstencilformat)).into()
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::CheckDeviceFormatConversion(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&sourceformat), core::mem::transmute_copy(&targetformat)).into()
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::GetAdapterMonitor(this, core::mem::transmute_copy(&adapter))
        }
        unsafe extern "system" fn CreateDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9_Impl::CreateDevice(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterSoftwareDevice: RegisterSoftwareDevice::<Identity, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, OFFSET>,
            GetAdapterIdentifier: GetAdapterIdentifier::<Identity, OFFSET>,
            GetAdapterModeCount: GetAdapterModeCount::<Identity, OFFSET>,
            EnumAdapterModes: EnumAdapterModes::<Identity, OFFSET>,
            GetAdapterDisplayMode: GetAdapterDisplayMode::<Identity, OFFSET>,
            CheckDeviceType: CheckDeviceType::<Identity, OFFSET>,
            CheckDeviceFormat: CheckDeviceFormat::<Identity, OFFSET>,
            CheckDeviceMultiSampleType: CheckDeviceMultiSampleType::<Identity, OFFSET>,
            CheckDepthStencilMatch: CheckDepthStencilMatch::<Identity, OFFSET>,
            CheckDeviceFormatConversion: CheckDeviceFormatConversion::<Identity, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, OFFSET>,
            GetAdapterMonitor: GetAdapterMonitor::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3D9Ex_Impl: Sized + IDirect3D9_Impl {
    fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32;
    fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
    fn CreateDeviceEx(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9Ex>) -> windows_core::Result<()>;
    fn GetAdapterLUID(&self, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3D9Ex {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3D9Ex_Vtbl {
    pub const fn new<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>() -> IDirect3D9Ex_Vtbl {
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9Ex_Impl::GetAdapterModeCountEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter))
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9Ex_Impl::EnumAdapterModesEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9Ex_Impl::GetAdapterDisplayModeEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9Ex_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3D9Ex_Impl::GetAdapterLUID(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pluid)).into()
        }
        Self {
            base__: IDirect3D9_Vtbl::new::<Identity, OFFSET>(),
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Identity, OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Identity, OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Identity, OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9Ex as windows_core::Interface>::IID || iid == &<IDirect3D9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DBaseTexture9_Impl: Sized + IDirect3DResource9_Impl {
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
}
impl windows_core::RuntimeName for IDirect3DBaseTexture9 {}
impl IDirect3DBaseTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>() -> IDirect3DBaseTexture9_Vtbl {
        unsafe extern "system" fn SetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: u32) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DBaseTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DBaseTexture9_Impl::GetLOD(this)
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DBaseTexture9_Impl::GetLevelCount(this)
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DBaseTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DBaseTexture9_Impl::GetAutoGenFilterType(this)
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DBaseTexture9_Impl::GenerateMipSubLevels(this)
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DCubeTexture9_Impl: Sized + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DCubeTexture9 {}
impl IDirect3DCubeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>() -> IDirect3DCubeTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DCubeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DCubeTexture9_Impl::GetCubeMapSurface(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)) {
                Ok(ok__) => {
                    ppcubemapsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DCubeTexture9_Impl::LockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DCubeTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DCubeTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DCubeTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9_Impl: Sized + windows_core::IUnknownImpl {
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetAvailableTextureMem(&self) -> u32;
    fn EvictManagedResources(&self) -> windows_core::Result<()>;
    fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9>;
    fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::Result<()>;
    fn SetCursorProperties(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: Option<&IDirect3DSurface9>) -> windows_core::Result<()>;
    fn SetCursorPosition(&self, x: i32, y: i32, flags: u32);
    fn ShowCursor(&self, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::Result<()>;
    fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(&self) -> u32;
    fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()>;
    fn SetDialogBoxMode(&self, benabledialogs: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP);
    fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP);
    fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateRenderTarget(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn UpdateSurface(&self, psourcesurface: Option<&IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: Option<&IDirect3DSurface9>, pdestpoint: *const super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn UpdateTexture(&self, psourcetexture: Option<&IDirect3DBaseTexture9>, pdestinationtexture: Option<&IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetRenderTargetData(&self, prendertarget: Option<&IDirect3DSurface9>, pdestsurface: Option<&IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, iswapchain: u32, pdestsurface: Option<&IDirect3DSurface9>) -> windows_core::Result<()>;
    fn StretchRect(&self, psourcesurface: Option<&IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestsurface: Option<&IDirect3DSurface9>, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn ColorFill(&self, psurface: Option<&IDirect3DSurface9>, prect: *const super::super::Foundation::RECT, color: u32) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetRenderTarget(&self, rendertargetindex: u32, prendertarget: Option<&IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(&self, pnewzstencil: Option<&IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9>;
    fn BeginScene(&self) -> windows_core::Result<()>;
    fn EndScene(&self) -> windows_core::Result<()>;
    fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> windows_core::Result<()>;
    fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()>;
    fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()>;
    fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows_core::Result<()>;
    fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows_core::Result<()>;
    fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows_core::Result<()>;
    fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows_core::Result<()>;
    fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> windows_core::Result<()>;
    fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> windows_core::Result<()>;
    fn LightEnable(&self, index: u32, enable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLightEnable(&self, index: u32, penable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::Result<()>;
    fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> windows_core::Result<()>;
    fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::Result<()>;
    fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(&self) -> windows_core::Result<()>;
    fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(&self, stage: u32, ptexture: Option<&IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::Result<()>;
    fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::Result<()>;
    fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn ValidateDevice(&self, pnumpasses: *mut u32) -> windows_core::Result<()>;
    fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::Result<()>;
    fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> windows_core::Result<()>;
    fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetSoftwareVertexProcessing(&self, bsoftware: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSoftwareVertexProcessing(&self) -> super::super::Foundation::BOOL;
    fn SetNPatchMode(&self, nsegments: f32) -> windows_core::Result<()>;
    fn GetNPatchMode(&self) -> f32;
    fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::Result<()>;
    fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn ProcessVertices(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: Option<&IDirect3DVertexBuffer9>, pvertexdecl: Option<&IDirect3DVertexDeclaration9>, flags: u32) -> windows_core::Result<()>;
    fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(&self, pdecl: Option<&IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(&self, fvf: u32) -> windows_core::Result<()>;
    fn GetFVF(&self, pfvf: *mut u32) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(&self, pshader: Option<&IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn SetStreamSource(&self, streamnumber: u32, pstreamdata: Option<&IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> windows_core::Result<()>;
    fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::Result<()>;
    fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::Result<()>;
    fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> windows_core::Result<()>;
    fn SetIndices(&self, pindexdata: Option<&IDirect3DIndexBuffer9>) -> windows_core::Result<()>;
    fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(&self, pshader: Option<&IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::Result<()>;
    fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::Result<()>;
    fn DeletePatch(&self, handle: u32) -> windows_core::Result<()>;
    fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDirect3DDevice9 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>() -> IDirect3DDevice9_Vtbl {
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::TestCooperativeLevel(this).into()
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetAvailableTextureMem(this)
        }
        unsafe extern "system" fn EvictManagedResources<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::EvictManagedResources(this).into()
        }
        unsafe extern "system" fn GetDirect3D<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppd3d9: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetDirect3D(this) {
                Ok(ok__) => {
                    ppd3d9.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn SetCursorProperties<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetCursorProperties(this, core::mem::transmute_copy(&xhotspot), core::mem::transmute_copy(&yhotspot), windows_core::from_raw_borrowed(&pcursorbitmap)).into()
        }
        unsafe extern "system" fn SetCursorPosition<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, flags: u32) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetCursorPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ShowCursor<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::ShowCursor(this, core::mem::transmute_copy(&bshow))
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateAdditionalSwapChain(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetSwapChain(this, core::mem::transmute_copy(&iswapchain)) {
                Ok(ok__) => {
                    pswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetNumberOfSwapChains(this)
        }
        unsafe extern "system" fn Reset<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::Reset(this, core::mem::transmute_copy(&ppresentationparameters)).into()
        }
        unsafe extern "system" fn Present<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion)).into()
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppbackbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&prasterstatus)).into()
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetDialogBoxMode(this, core::mem::transmute_copy(&benabledialogs)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pramp))
        }
        unsafe extern "system" fn GetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pramp))
        }
        unsafe extern "system" fn CreateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&pptexture), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateVolumeTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvolumetexture), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateCubeTexture(this, core::mem::transmute_copy(&edgelength), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppcubetexture), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateVertexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&fvf), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvertexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateIndexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppindexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateRenderTarget(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateDepthStencilSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn UpdateSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: *mut core::ffi::c_void, pdestpoint: *const super::super::Foundation::POINT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::UpdateSurface(this, windows_core::from_raw_borrowed(&psourcesurface), core::mem::transmute_copy(&psourcerect), windows_core::from_raw_borrowed(&pdestinationsurface), core::mem::transmute_copy(&pdestpoint)).into()
        }
        unsafe extern "system" fn UpdateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetexture: *mut core::ffi::c_void, pdestinationtexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::UpdateTexture(this, windows_core::from_raw_borrowed(&psourcetexture), windows_core::from_raw_borrowed(&pdestinationtexture)).into()
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetRenderTargetData(this, windows_core::from_raw_borrowed(&prendertarget), windows_core::from_raw_borrowed(&pdestsurface)).into()
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&iswapchain), windows_core::from_raw_borrowed(&pdestsurface)).into()
        }
        unsafe extern "system" fn StretchRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestsurface: *mut core::ffi::c_void, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::StretchRect(this, windows_core::from_raw_borrowed(&psourcesurface), core::mem::transmute_copy(&psourcerect), windows_core::from_raw_borrowed(&pdestsurface), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn ColorFill<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT, color: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::ColorFill(this, windows_core::from_raw_borrowed(&psurface), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::CreateOffscreenPlainSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn SetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, prendertarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex), windows_core::from_raw_borrowed(&prendertarget)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex)) {
                Ok(ok__) => {
                    pprendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewzstencil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetDepthStencilSurface(this, windows_core::from_raw_borrowed(&pnewzstencil)).into()
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppzstencilsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetDepthStencilSurface(this) {
                Ok(ok__) => {
                    ppzstencilsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::BeginScene(this).into()
        }
        unsafe extern "system" fn EndScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::EndScene(this).into()
        }
        unsafe extern "system" fn Clear<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::Clear(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&color), core::mem::transmute_copy(&z), core::mem::transmute_copy(&stencil)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn MultiplyTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::MultiplyTransform(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetViewport(this, core::mem::transmute_copy(&pviewport)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetViewport(this, core::mem::transmute_copy(&pviewport)).into()
        }
        unsafe extern "system" fn SetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
        }
        unsafe extern "system" fn GetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
        }
        unsafe extern "system" fn SetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn LightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::LightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn GetLightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetLightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&penable)).into()
        }
        unsafe extern "system" fn SetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *const f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
        }
        unsafe extern "system" fn GetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
        }
        unsafe extern "system" fn SetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn CreateStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::CreateStateBlock(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppsb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::BeginStateBlock(this).into()
        }
        unsafe extern "system" fn EndStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::EndStateBlock(this) {
                Ok(ok__) => {
                    ppsb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
        }
        unsafe extern "system" fn GetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
        }
        unsafe extern "system" fn GetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetTexture(this, core::mem::transmute_copy(&stage)) {
                Ok(ok__) => {
                    pptexture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, ptexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetTexture(this, core::mem::transmute_copy(&stage), windows_core::from_raw_borrowed(&ptexture)).into()
        }
        unsafe extern "system" fn GetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValidateDevice<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpasses: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::ValidateDevice(this, core::mem::transmute_copy(&pnumpasses)).into()
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
        }
        unsafe extern "system" fn SetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetScissorRect(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn GetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetScissorRect(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetSoftwareVertexProcessing(this, core::mem::transmute_copy(&bsoftware)).into()
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetSoftwareVertexProcessing(this)
        }
        unsafe extern "system" fn SetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsegments: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetNPatchMode(this, core::mem::transmute_copy(&nsegments)).into()
        }
        unsafe extern "system" fn GetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetNPatchMode(this)
        }
        unsafe extern "system" fn DrawPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DrawPrimitive(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&startvertex), core::mem::transmute_copy(&primitivecount)).into()
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DrawIndexedPrimitive(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&basevertexindex), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&primcount)).into()
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DrawPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DrawIndexedPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pindexdata), core::mem::transmute_copy(&indexdataformat), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
        }
        unsafe extern "system" fn ProcessVertices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: *mut core::ffi::c_void, pvertexdecl: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::ProcessVertices(this, core::mem::transmute_copy(&srcstartindex), core::mem::transmute_copy(&destindex), core::mem::transmute_copy(&vertexcount), windows_core::from_raw_borrowed(&pdestbuffer), windows_core::from_raw_borrowed(&pvertexdecl), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::CreateVertexDeclaration(this, core::mem::transmute_copy(&pvertexelements)) {
                Ok(ok__) => {
                    ppdecl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetVertexDeclaration(this, windows_core::from_raw_borrowed(&pdecl)).into()
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetVertexDeclaration(this) {
                Ok(ok__) => {
                    ppdecl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvf: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetFVF(this, core::mem::transmute_copy(&fvf)).into()
        }
        unsafe extern "system" fn GetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvf: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetFVF(this, core::mem::transmute_copy(&pfvf)).into()
        }
        unsafe extern "system" fn CreateVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pfunction)) {
                Ok(ok__) => {
                    ppshader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetVertexShader(this, windows_core::from_raw_borrowed(&pshader)).into()
        }
        unsafe extern "system" fn GetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetVertexShader(this) {
                Ok(ok__) => {
                    ppshader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn SetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, pstreamdata: *mut core::ffi::c_void, offsetinbytes: u32, stride: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetStreamSource(this, core::mem::transmute_copy(&streamnumber), windows_core::from_raw_borrowed(&pstreamdata), core::mem::transmute_copy(&offsetinbytes), core::mem::transmute_copy(&stride)).into()
        }
        unsafe extern "system" fn GetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut *mut core::ffi::c_void, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&ppstreamdata), core::mem::transmute_copy(&poffsetinbytes), core::mem::transmute_copy(&pstride)).into()
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, setting: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&psetting)).into()
        }
        unsafe extern "system" fn SetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetIndices(this, windows_core::from_raw_borrowed(&pindexdata)).into()
        }
        unsafe extern "system" fn GetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetIndices(this) {
                Ok(ok__) => {
                    ppindexdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pfunction)) {
                Ok(ok__) => {
                    ppshader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetPixelShader(this, windows_core::from_raw_borrowed(&pshader)).into()
        }
        unsafe extern "system" fn GetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::GetPixelShader(this) {
                Ok(ok__) => {
                    ppshader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::SetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::GetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn DrawRectPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DrawRectPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&prectpatchinfo)).into()
        }
        unsafe extern "system" fn DrawTriPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DrawTriPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&ptripatchinfo)).into()
        }
        unsafe extern "system" fn DeletePatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9_Impl::DeletePatch(this, core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn CreateQuery<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DDevice9_Impl::CreateQuery(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetAvailableTextureMem: GetAvailableTextureMem::<Identity, OFFSET>,
            EvictManagedResources: EvictManagedResources::<Identity, OFFSET>,
            GetDirect3D: GetDirect3D::<Identity, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, OFFSET>,
            SetCursorProperties: SetCursorProperties::<Identity, OFFSET>,
            SetCursorPosition: SetCursorPosition::<Identity, OFFSET>,
            ShowCursor: ShowCursor::<Identity, OFFSET>,
            CreateAdditionalSwapChain: CreateAdditionalSwapChain::<Identity, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, OFFSET>,
            GetNumberOfSwapChains: GetNumberOfSwapChains::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Present: Present::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            SetDialogBoxMode: SetDialogBoxMode::<Identity, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, OFFSET>,
            GetGammaRamp: GetGammaRamp::<Identity, OFFSET>,
            CreateTexture: CreateTexture::<Identity, OFFSET>,
            CreateVolumeTexture: CreateVolumeTexture::<Identity, OFFSET>,
            CreateCubeTexture: CreateCubeTexture::<Identity, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, OFFSET>,
            CreateIndexBuffer: CreateIndexBuffer::<Identity, OFFSET>,
            CreateRenderTarget: CreateRenderTarget::<Identity, OFFSET>,
            CreateDepthStencilSurface: CreateDepthStencilSurface::<Identity, OFFSET>,
            UpdateSurface: UpdateSurface::<Identity, OFFSET>,
            UpdateTexture: UpdateTexture::<Identity, OFFSET>,
            GetRenderTargetData: GetRenderTargetData::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            StretchRect: StretchRect::<Identity, OFFSET>,
            ColorFill: ColorFill::<Identity, OFFSET>,
            CreateOffscreenPlainSurface: CreateOffscreenPlainSurface::<Identity, OFFSET>,
            SetRenderTarget: SetRenderTarget::<Identity, OFFSET>,
            GetRenderTarget: GetRenderTarget::<Identity, OFFSET>,
            SetDepthStencilSurface: SetDepthStencilSurface::<Identity, OFFSET>,
            GetDepthStencilSurface: GetDepthStencilSurface::<Identity, OFFSET>,
            BeginScene: BeginScene::<Identity, OFFSET>,
            EndScene: EndScene::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            MultiplyTransform: MultiplyTransform::<Identity, OFFSET>,
            SetViewport: SetViewport::<Identity, OFFSET>,
            GetViewport: GetViewport::<Identity, OFFSET>,
            SetMaterial: SetMaterial::<Identity, OFFSET>,
            GetMaterial: GetMaterial::<Identity, OFFSET>,
            SetLight: SetLight::<Identity, OFFSET>,
            GetLight: GetLight::<Identity, OFFSET>,
            LightEnable: LightEnable::<Identity, OFFSET>,
            GetLightEnable: GetLightEnable::<Identity, OFFSET>,
            SetClipPlane: SetClipPlane::<Identity, OFFSET>,
            GetClipPlane: GetClipPlane::<Identity, OFFSET>,
            SetRenderState: SetRenderState::<Identity, OFFSET>,
            GetRenderState: GetRenderState::<Identity, OFFSET>,
            CreateStateBlock: CreateStateBlock::<Identity, OFFSET>,
            BeginStateBlock: BeginStateBlock::<Identity, OFFSET>,
            EndStateBlock: EndStateBlock::<Identity, OFFSET>,
            SetClipStatus: SetClipStatus::<Identity, OFFSET>,
            GetClipStatus: GetClipStatus::<Identity, OFFSET>,
            GetTexture: GetTexture::<Identity, OFFSET>,
            SetTexture: SetTexture::<Identity, OFFSET>,
            GetTextureStageState: GetTextureStageState::<Identity, OFFSET>,
            SetTextureStageState: SetTextureStageState::<Identity, OFFSET>,
            GetSamplerState: GetSamplerState::<Identity, OFFSET>,
            SetSamplerState: SetSamplerState::<Identity, OFFSET>,
            ValidateDevice: ValidateDevice::<Identity, OFFSET>,
            SetPaletteEntries: SetPaletteEntries::<Identity, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, OFFSET>,
            SetCurrentTexturePalette: SetCurrentTexturePalette::<Identity, OFFSET>,
            GetCurrentTexturePalette: GetCurrentTexturePalette::<Identity, OFFSET>,
            SetScissorRect: SetScissorRect::<Identity, OFFSET>,
            GetScissorRect: GetScissorRect::<Identity, OFFSET>,
            SetSoftwareVertexProcessing: SetSoftwareVertexProcessing::<Identity, OFFSET>,
            GetSoftwareVertexProcessing: GetSoftwareVertexProcessing::<Identity, OFFSET>,
            SetNPatchMode: SetNPatchMode::<Identity, OFFSET>,
            GetNPatchMode: GetNPatchMode::<Identity, OFFSET>,
            DrawPrimitive: DrawPrimitive::<Identity, OFFSET>,
            DrawIndexedPrimitive: DrawIndexedPrimitive::<Identity, OFFSET>,
            DrawPrimitiveUP: DrawPrimitiveUP::<Identity, OFFSET>,
            DrawIndexedPrimitiveUP: DrawIndexedPrimitiveUP::<Identity, OFFSET>,
            ProcessVertices: ProcessVertices::<Identity, OFFSET>,
            CreateVertexDeclaration: CreateVertexDeclaration::<Identity, OFFSET>,
            SetVertexDeclaration: SetVertexDeclaration::<Identity, OFFSET>,
            GetVertexDeclaration: GetVertexDeclaration::<Identity, OFFSET>,
            SetFVF: SetFVF::<Identity, OFFSET>,
            GetFVF: GetFVF::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            SetVertexShader: SetVertexShader::<Identity, OFFSET>,
            GetVertexShader: GetVertexShader::<Identity, OFFSET>,
            SetVertexShaderConstantF: SetVertexShaderConstantF::<Identity, OFFSET>,
            GetVertexShaderConstantF: GetVertexShaderConstantF::<Identity, OFFSET>,
            SetVertexShaderConstantI: SetVertexShaderConstantI::<Identity, OFFSET>,
            GetVertexShaderConstantI: GetVertexShaderConstantI::<Identity, OFFSET>,
            SetVertexShaderConstantB: SetVertexShaderConstantB::<Identity, OFFSET>,
            GetVertexShaderConstantB: GetVertexShaderConstantB::<Identity, OFFSET>,
            SetStreamSource: SetStreamSource::<Identity, OFFSET>,
            GetStreamSource: GetStreamSource::<Identity, OFFSET>,
            SetStreamSourceFreq: SetStreamSourceFreq::<Identity, OFFSET>,
            GetStreamSourceFreq: GetStreamSourceFreq::<Identity, OFFSET>,
            SetIndices: SetIndices::<Identity, OFFSET>,
            GetIndices: GetIndices::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, OFFSET>,
            GetPixelShader: GetPixelShader::<Identity, OFFSET>,
            SetPixelShaderConstantF: SetPixelShaderConstantF::<Identity, OFFSET>,
            GetPixelShaderConstantF: GetPixelShaderConstantF::<Identity, OFFSET>,
            SetPixelShaderConstantI: SetPixelShaderConstantI::<Identity, OFFSET>,
            GetPixelShaderConstantI: GetPixelShaderConstantI::<Identity, OFFSET>,
            SetPixelShaderConstantB: SetPixelShaderConstantB::<Identity, OFFSET>,
            GetPixelShaderConstantB: GetPixelShaderConstantB::<Identity, OFFSET>,
            DrawRectPatch: DrawRectPatch::<Identity, OFFSET>,
            DrawTriPatch: DrawTriPatch::<Identity, OFFSET>,
            DeletePatch: DeletePatch::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9Ex_Impl: Sized + IDirect3DDevice9_Impl {
    fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::Result<()>;
    fn ComposeRects(&self, psrc: Option<&IDirect3DSurface9>, pdst: Option<&IDirect3DSurface9>, psrcrectdescs: Option<&IDirect3DVertexBuffer9>, numrects: u32, pdstrectdescs: Option<&IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::Result<()>;
    fn PresentEx(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn WaitForVBlank(&self, iswapchain: u32) -> windows_core::Result<()>;
    fn CheckResourceResidency(&self, presourcearray: *mut Option<IDirect3DResource9>, numresources: u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> windows_core::Result<()>;
    fn CheckDeviceState(&self, hdestinationwindow: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn CreateRenderTargetEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn CreateDepthStencilSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDirect3DDevice9Ex {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>() -> IDirect3DDevice9Ex_Vtbl {
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::SetConvolutionMonoKernel(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&rows), core::mem::transmute_copy(&columns)).into()
        }
        unsafe extern "system" fn ComposeRects<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void, psrcrectdescs: *mut core::ffi::c_void, numrects: u32, pdstrectdescs: *mut core::ffi::c_void, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::ComposeRects(this, windows_core::from_raw_borrowed(&psrc), windows_core::from_raw_borrowed(&pdst), windows_core::from_raw_borrowed(&psrcrectdescs), core::mem::transmute_copy(&numrects), windows_core::from_raw_borrowed(&pdstrectdescs), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&xoffset), core::mem::transmute_copy(&yoffset)).into()
        }
        unsafe extern "system" fn PresentEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::PresentEx(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::GetGPUThreadPriority(this, core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::WaitForVBlank(this, core::mem::transmute_copy(&iswapchain)).into()
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcearray: *mut *mut core::ffi::c_void, numresources: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::CheckResourceResidency(this, core::mem::transmute_copy(&presourcearray), core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::GetMaximumFrameLatency(this, core::mem::transmute_copy(&pmaxlatency)).into()
        }
        unsafe extern "system" fn CheckDeviceState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::CheckDeviceState(this, core::mem::transmute_copy(&hdestinationwindow)).into()
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::CreateRenderTargetEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::CreateOffscreenPlainSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::CreateDepthStencilSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn ResetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::ResetEx(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode)).into()
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DDevice9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
        }
        Self {
            base__: IDirect3DDevice9_Vtbl::new::<Identity, OFFSET>(),
            SetConvolutionMonoKernel: SetConvolutionMonoKernel::<Identity, OFFSET>,
            ComposeRects: ComposeRects::<Identity, OFFSET>,
            PresentEx: PresentEx::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            CheckResourceResidency: CheckResourceResidency::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, OFFSET>,
            CreateRenderTargetEx: CreateRenderTargetEx::<Identity, OFFSET>,
            CreateOffscreenPlainSurfaceEx: CreateOffscreenPlainSurfaceEx::<Identity, OFFSET>,
            CreateDepthStencilSurfaceEx: CreateDepthStencilSurfaceEx::<Identity, OFFSET>,
            ResetEx: ResetEx::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9Ex as windows_core::Interface>::IID || iid == &<IDirect3DDevice9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DIndexBuffer9_Impl: Sized + IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DIndexBuffer9 {}
impl IDirect3DIndexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>() -> IDirect3DIndexBuffer9_Vtbl {
        unsafe extern "system" fn Lock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DIndexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DIndexBuffer9_Impl::Unlock(this).into()
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DIndexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DPixelShader9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DPixelShader9 {}
impl IDirect3DPixelShader9_Vtbl {
    pub const fn new<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>() -> IDirect3DPixelShader9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DPixelShader9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DPixelShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DQuery9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetType(&self) -> D3DQUERYTYPE;
    fn GetDataSize(&self) -> u32;
    fn Issue(&self, dwissueflags: u32) -> windows_core::Result<()>;
    fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DQuery9 {}
impl IDirect3DQuery9_Vtbl {
    pub const fn new<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>() -> IDirect3DQuery9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DQuery9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DQUERYTYPE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DQuery9_Impl::GetType(this)
        }
        unsafe extern "system" fn GetDataSize<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DQuery9_Impl::GetDataSize(this)
        }
        unsafe extern "system" fn Issue<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwissueflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DQuery9_Impl::Issue(this, core::mem::transmute_copy(&dwissueflags)).into()
        }
        unsafe extern "system" fn GetData<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DQuery9_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwgetdataflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
            Issue: Issue::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DQuery9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DResource9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> D3DRESOURCETYPE;
}
impl windows_core::RuntimeName for IDirect3DResource9 {}
impl IDirect3DResource9_Vtbl {
    pub const fn new<Identity: IDirect3DResource9_Impl, const OFFSET: isize>() -> IDirect3DResource9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DResource9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::GetPriority(this)
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::PreLoad(this)
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DRESOURCETYPE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DResource9_Impl::GetType(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DStateBlock9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DStateBlock9 {}
impl IDirect3DStateBlock9_Vtbl {
    pub const fn new<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>() -> IDirect3DStateBlock9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DStateBlock9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capture<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DStateBlock9_Impl::Capture(this).into()
        }
        unsafe extern "system" fn Apply<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DStateBlock9_Impl::Apply(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3DSurface9_Impl: Sized + IDirect3DResource9_Impl {
    fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self) -> windows_core::Result<()>;
    fn GetDC(&self, phdc: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn ReleaseDC(&self, hdc: super::Gdi::HDC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3DSurface9 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3DSurface9_Vtbl {
    pub const fn new<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>() -> IDirect3DSurface9_Vtbl {
        unsafe extern "system" fn GetContainer<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSurface9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSurface9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSurface9_Impl::LockRect(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSurface9_Impl::UnlockRect(this).into()
        }
        unsafe extern "system" fn GetDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSurface9_Impl::GetDC(this, core::mem::transmute_copy(&phdc)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Gdi::HDC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSurface9_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            GetContainer: GetContainer::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSurface9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3DSwapChain9_Impl: Sized + windows_core::IUnknownImpl {
    fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, pdestsurface: Option<&IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3DSwapChain9 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3DSwapChain9_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>() -> IDirect3DSwapChain9_Vtbl {
        unsafe extern "system" fn Present<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9_Impl::GetFrontBufferData(this, windows_core::from_raw_borrowed(&pdestsurface)).into()
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DSwapChain9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppbackbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&prasterstatus)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DSwapChain9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9_Impl::GetPresentParameters(this, core::mem::transmute_copy(&ppresentationparameters)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPresentParameters: GetPresentParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3DSwapChain9Ex_Impl: Sized + IDirect3DSwapChain9_Impl {
    fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> windows_core::Result<()>;
    fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3DSwapChain9Ex {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3DSwapChain9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>() -> IDirect3DSwapChain9Ex_Vtbl {
        unsafe extern "system" fn GetLastPresentCount<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9Ex_Impl::GetLastPresentCount(this, core::mem::transmute_copy(&plastpresentcount)).into()
        }
        unsafe extern "system" fn GetPresentStats<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9Ex_Impl::GetPresentStats(this, core::mem::transmute_copy(&ppresentationstatistics)).into()
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DSwapChain9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
        }
        Self {
            base__: IDirect3DSwapChain9_Vtbl::new::<Identity, OFFSET>(),
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as windows_core::Interface>::IID || iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DTexture9_Impl: Sized + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetSurfaceLevel(&self, level: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn LockRect(&self, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self, level: u32) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DTexture9 {}
impl IDirect3DTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>() -> IDirect3DTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, ppsurfacelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DTexture9_Impl::GetSurfaceLevel(this, core::mem::transmute_copy(&level)) {
                Ok(ok__) => {
                    ppsurfacelevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DTexture9_Impl::LockRect(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DVertexBuffer9_Impl: Sized + IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DVertexBuffer9 {}
impl IDirect3DVertexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>() -> IDirect3DVertexBuffer9_Vtbl {
        unsafe extern "system" fn Lock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVertexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVertexBuffer9_Impl::Unlock(this).into()
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVertexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DVertexDeclaration9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DVertexDeclaration9 {}
impl IDirect3DVertexDeclaration9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>() -> IDirect3DVertexDeclaration9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DVertexDeclaration9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeclaration<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVertexDeclaration9_Impl::GetDeclaration(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&pnumelements)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DVertexShader9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DVertexShader9 {}
impl IDirect3DVertexShader9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>() -> IDirect3DVertexShader9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DVertexShader9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVertexShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DVolume9_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::Result<()>;
    fn UnlockBox(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DVolume9 {}
impl IDirect3DVolume9_Vtbl {
    pub const fn new<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>() -> IDirect3DVolume9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DVolume9_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
        }
        unsafe extern "system" fn GetContainer<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::LockBox(this, core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolume9_Impl::UnlockBox(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetContainer: GetContainer::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            LockBox: LockBox::<Identity, OFFSET>,
            UnlockBox: UnlockBox::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVolume9 as windows_core::Interface>::IID
    }
}
pub trait IDirect3DVolumeTexture9_Impl: Sized + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn GetVolumeLevel(&self, level: u32) -> windows_core::Result<IDirect3DVolume9>;
    fn LockBox(&self, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::Result<()>;
    fn UnlockBox(&self, level: u32) -> windows_core::Result<()>;
    fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DVolumeTexture9 {}
impl IDirect3DVolumeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>() -> IDirect3DVolumeTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolumeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, ppvolumelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirect3DVolumeTexture9_Impl::GetVolumeLevel(this, core::mem::transmute_copy(&level)) {
                Ok(ok__) => {
                    ppvolumelevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolumeTexture9_Impl::LockBox(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolumeTexture9_Impl::UnlockBox(this, core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybox: *const D3DBOX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirect3DVolumeTexture9_Impl::AddDirtyBox(this, core::mem::transmute_copy(&pdirtybox)).into()
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, OFFSET>,
            LockBox: LockBox::<Identity, OFFSET>,
            UnlockBox: UnlockBox::<Identity, OFFSET>,
            AddDirtyBox: AddDirtyBox::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVolumeTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}

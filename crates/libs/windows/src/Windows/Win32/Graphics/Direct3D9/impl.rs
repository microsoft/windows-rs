#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9_Impl: Sized {
    fn RegisterSoftwareDevice(&mut self, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdapterCount(&mut self) -> u32;
    fn GetAdapterIdentifier(&mut self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::Result<()>;
    fn GetAdapterModeCount(&mut self, adapter: u32, format: D3DFORMAT) -> u32;
    fn EnumAdapterModes(&mut self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn GetAdapterDisplayMode(&mut self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn CheckDeviceType(&mut self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CheckDeviceFormat(&mut self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::Result<()>;
    fn CheckDeviceMultiSampleType(&mut self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::Result<()>;
    fn CheckDepthStencilMatch(&mut self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::Result<()>;
    fn CheckDeviceFormatConversion(&mut self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::Result<()>;
    fn GetDeviceCaps(&mut self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()>;
    fn GetAdapterMonitor(&mut self, adapter: u32) -> super::Gdi::HMONITOR;
    fn CreateDevice(&mut self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3D9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>() -> IDirect3D9_Vtbl {
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterSoftwareDevice(::core::mem::transmute_copy(&pinitializefunction)).into()
        }
        unsafe extern "system" fn GetAdapterCount<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterCount()
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterIdentifier(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pidentifier)).into()
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterModeCount(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAdapterModes(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterDisplayMode(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn CheckDeviceType<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDeviceType(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devtype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&backbufferformat), ::core::mem::transmute_copy(&bwindowed)).into()
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDeviceFormat(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&rtype), ::core::mem::transmute_copy(&checkformat)).into()
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDeviceMultiSampleType(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&surfaceformat), ::core::mem::transmute_copy(&windowed), ::core::mem::transmute_copy(&multisampletype), ::core::mem::transmute_copy(&pqualitylevels)).into()
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDepthStencilMatch(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&rendertargetformat), ::core::mem::transmute_copy(&depthstencilformat)).into()
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDeviceFormatConversion(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&sourceformat), ::core::mem::transmute_copy(&targetformat)).into()
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceCaps(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterMonitor(::core::mem::transmute_copy(&adapter))
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&hfocuswindow), ::core::mem::transmute_copy(&behaviorflags), ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterSoftwareDevice: RegisterSoftwareDevice::<Identity, Impl, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, Impl, OFFSET>,
            GetAdapterIdentifier: GetAdapterIdentifier::<Identity, Impl, OFFSET>,
            GetAdapterModeCount: GetAdapterModeCount::<Identity, Impl, OFFSET>,
            EnumAdapterModes: EnumAdapterModes::<Identity, Impl, OFFSET>,
            GetAdapterDisplayMode: GetAdapterDisplayMode::<Identity, Impl, OFFSET>,
            CheckDeviceType: CheckDeviceType::<Identity, Impl, OFFSET>,
            CheckDeviceFormat: CheckDeviceFormat::<Identity, Impl, OFFSET>,
            CheckDeviceMultiSampleType: CheckDeviceMultiSampleType::<Identity, Impl, OFFSET>,
            CheckDepthStencilMatch: CheckDepthStencilMatch::<Identity, Impl, OFFSET>,
            CheckDeviceFormatConversion: CheckDeviceFormatConversion::<Identity, Impl, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, Impl, OFFSET>,
            GetAdapterMonitor: GetAdapterMonitor::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9Ex_Impl: Sized + IDirect3D9_Impl {
    fn GetAdapterModeCountEx(&mut self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32;
    fn EnumAdapterModesEx(&mut self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::Result<()>;
    fn GetAdapterDisplayModeEx(&mut self, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()>;
    fn CreateDeviceEx(&mut self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9Ex>) -> ::windows::core::Result<()>;
    fn GetAdapterLUID(&mut self, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3D9Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>() -> IDirect3D9Ex_Vtbl {
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterModeCountEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pfilter))
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAdapterModesEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterDisplayModeEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into()
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&hfocuswindow), ::core::mem::transmute_copy(&behaviorflags), ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pfullscreendisplaymode), ::core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterLUID(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pluid)).into()
        }
        Self {
            base: IDirect3D9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Identity, Impl, OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Identity, Impl, OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Identity, Impl, OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D9Ex as ::windows::core::Interface>::IID || iid == &<IDirect3D9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DBaseTexture9_Impl: Sized + IDirect3DResource9_Impl {
    fn SetLOD(&mut self, lodnew: u32) -> u32;
    fn GetLOD(&mut self) -> u32;
    fn GetLevelCount(&mut self) -> u32;
    fn SetAutoGenFilterType(&mut self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()>;
    fn GetAutoGenFilterType(&mut self) -> D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&mut self);
}
impl IDirect3DBaseTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>() -> IDirect3DBaseTexture9_Vtbl {
        unsafe extern "system" fn SetLOD<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lodnew: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLOD(::core::mem::transmute_copy(&lodnew))
        }
        unsafe extern "system" fn GetLOD<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLOD()
        }
        unsafe extern "system" fn GetLevelCount<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLevelCount()
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoGenFilterType(::core::mem::transmute_copy(&filtertype)).into()
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAutoGenFilterType()
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateMipSubLevels()
        }
        Self {
            base: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetLOD: SetLOD::<Identity, Impl, OFFSET>,
            GetLOD: GetLOD::<Identity, Impl, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, Impl, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, Impl, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, Impl, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DBaseTexture9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DCubeTexture9_Impl: Sized + IDirect3DResource9_Impl + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&mut self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()>;
    fn GetCubeMapSurface(&mut self, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::Result<IDirect3DSurface9>;
    fn LockRect(&mut self, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockRect(&mut self, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::Result<()>;
    fn AddDirtyRect(&mut self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirect3DCubeTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>() -> IDirect3DCubeTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLevelDesc(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCubeMapSurface(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcubemapsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockRect(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockRect(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDirtyRect(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base: IDirect3DBaseTexture9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DCubeTexture9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9_Impl: Sized {
    fn TestCooperativeLevel(&mut self) -> ::windows::core::Result<()>;
    fn GetAvailableTextureMem(&mut self) -> u32;
    fn EvictManagedResources(&mut self) -> ::windows::core::Result<()>;
    fn GetDirect3D(&mut self) -> ::windows::core::Result<IDirect3D9>;
    fn GetDeviceCaps(&mut self, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&mut self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn GetCreationParameters(&mut self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::Result<()>;
    fn SetCursorProperties(&mut self, xhotspot: u32, yhotspot: u32, pcursorbitmap: &::core::option::Option<IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn SetCursorPosition(&mut self, x: i32, y: i32, flags: u32);
    fn ShowCursor(&mut self, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn CreateAdditionalSwapChain(&mut self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::core::option::Option<IDirect3DSwapChain9>) -> ::windows::core::Result<()>;
    fn GetSwapChain(&mut self, iswapchain: u32) -> ::windows::core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(&mut self) -> u32;
    fn Reset(&mut self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()>;
    fn Present(&mut self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::Result<()>;
    fn GetBackBuffer(&mut self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&mut self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()>;
    fn SetDialogBoxMode(&mut self, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGammaRamp(&mut self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP);
    fn GetGammaRamp(&mut self, iswapchain: u32, pramp: *mut D3DGAMMARAMP);
    fn CreateTexture(&mut self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::core::option::Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateVolumeTexture(&mut self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::core::option::Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateCubeTexture(&mut self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::core::option::Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateVertexBuffer(&mut self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::core::option::Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateIndexBuffer(&mut self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::core::option::Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateRenderTarget(&mut self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateDepthStencilSurface(&mut self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn UpdateSurface(&mut self, psourcesurface: &::core::option::Option<IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: &::core::option::Option<IDirect3DSurface9>, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn UpdateTexture(&mut self, psourcetexture: &::core::option::Option<IDirect3DBaseTexture9>, pdestinationtexture: &::core::option::Option<IDirect3DBaseTexture9>) -> ::windows::core::Result<()>;
    fn GetRenderTargetData(&mut self, prendertarget: &::core::option::Option<IDirect3DSurface9>, pdestsurface: &::core::option::Option<IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetFrontBufferData(&mut self, iswapchain: u32, pdestsurface: &::core::option::Option<IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn StretchRect(&mut self, psourcesurface: &::core::option::Option<IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestsurface: &::core::option::Option<IDirect3DSurface9>, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()>;
    fn ColorFill(&mut self, psurface: &::core::option::Option<IDirect3DSurface9>, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::Result<()>;
    fn CreateOffscreenPlainSurface(&mut self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetRenderTarget(&mut self, rendertargetindex: u32, prendertarget: &::core::option::Option<IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetRenderTarget(&mut self, rendertargetindex: u32) -> ::windows::core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(&mut self, pnewzstencil: &::core::option::Option<IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetDepthStencilSurface(&mut self) -> ::windows::core::Result<IDirect3DSurface9>;
    fn BeginScene(&mut self) -> ::windows::core::Result<()>;
    fn EndScene(&mut self) -> ::windows::core::Result<()>;
    fn Clear(&mut self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::Result<()>;
    fn SetTransform(&mut self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()>;
    fn GetTransform(&mut self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()>;
    fn MultiplyTransform(&mut self, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()>;
    fn SetViewport(&mut self, pviewport: *const D3DVIEWPORT9) -> ::windows::core::Result<()>;
    fn GetViewport(&mut self, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::Result<()>;
    fn SetMaterial(&mut self, pmaterial: *const D3DMATERIAL9) -> ::windows::core::Result<()>;
    fn GetMaterial(&mut self, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::Result<()>;
    fn SetLight(&mut self, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::Result<()>;
    fn GetLight(&mut self, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::Result<()>;
    fn LightEnable(&mut self, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLightEnable(&mut self, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClipPlane(&mut self, index: u32, pplane: *const f32) -> ::windows::core::Result<()>;
    fn GetClipPlane(&mut self, index: u32, pplane: *mut f32) -> ::windows::core::Result<()>;
    fn SetRenderState(&mut self, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::Result<()>;
    fn GetRenderState(&mut self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn CreateStateBlock(&mut self, r#type: D3DSTATEBLOCKTYPE) -> ::windows::core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(&mut self) -> ::windows::core::Result<()>;
    fn EndStateBlock(&mut self) -> ::windows::core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(&mut self, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::Result<()>;
    fn GetClipStatus(&mut self, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::Result<()>;
    fn GetTexture(&mut self, stage: u32) -> ::windows::core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(&mut self, stage: u32, ptexture: &::core::option::Option<IDirect3DBaseTexture9>) -> ::windows::core::Result<()>;
    fn GetTextureStageState(&mut self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetTextureStageState(&mut self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::Result<()>;
    fn GetSamplerState(&mut self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetSamplerState(&mut self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::Result<()>;
    fn ValidateDevice(&mut self, pnumpasses: *mut u32) -> ::windows::core::Result<()>;
    fn SetPaletteEntries(&mut self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
    fn GetPaletteEntries(&mut self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
    fn SetCurrentTexturePalette(&mut self, palettenumber: u32) -> ::windows::core::Result<()>;
    fn GetCurrentTexturePalette(&mut self, palettenumber: *mut u32) -> ::windows::core::Result<()>;
    fn SetScissorRect(&mut self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetScissorRect(&mut self, prect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetSoftwareVertexProcessing(&mut self, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSoftwareVertexProcessing(&mut self) -> super::super::Foundation::BOOL;
    fn SetNPatchMode(&mut self, nsegments: f32) -> ::windows::core::Result<()>;
    fn GetNPatchMode(&mut self) -> f32;
    fn DrawPrimitive(&mut self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::Result<()>;
    fn DrawIndexedPrimitive(&mut self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::Result<()>;
    fn DrawPrimitiveUP(&mut self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()>;
    fn DrawIndexedPrimitiveUP(&mut self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()>;
    fn ProcessVertices(&mut self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: &::core::option::Option<IDirect3DVertexBuffer9>, pvertexdecl: &::core::option::Option<IDirect3DVertexDeclaration9>, flags: u32) -> ::windows::core::Result<()>;
    fn CreateVertexDeclaration(&mut self, pvertexelements: *const D3DVERTEXELEMENT9) -> ::windows::core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(&mut self, pdecl: &::core::option::Option<IDirect3DVertexDeclaration9>) -> ::windows::core::Result<()>;
    fn GetVertexDeclaration(&mut self) -> ::windows::core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(&mut self, fvf: u32) -> ::windows::core::Result<()>;
    fn GetFVF(&mut self, pfvf: *mut u32) -> ::windows::core::Result<()>;
    fn CreateVertexShader(&mut self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(&mut self, pshader: &::core::option::Option<IDirect3DVertexShader9>) -> ::windows::core::Result<()>;
    fn GetVertexShader(&mut self) -> ::windows::core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(&mut self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn GetVertexShaderConstantF(&mut self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn SetVertexShaderConstantI(&mut self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn GetVertexShaderConstantI(&mut self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn SetVertexShaderConstantB(&mut self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn GetVertexShaderConstantB(&mut self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn SetStreamSource(&mut self, streamnumber: u32, pstreamdata: &::core::option::Option<IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> ::windows::core::Result<()>;
    fn GetStreamSource(&mut self, streamnumber: u32, ppstreamdata: *mut ::core::option::Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::Result<()>;
    fn SetStreamSourceFreq(&mut self, streamnumber: u32, setting: u32) -> ::windows::core::Result<()>;
    fn GetStreamSourceFreq(&mut self, streamnumber: u32, psetting: *mut u32) -> ::windows::core::Result<()>;
    fn SetIndices(&mut self, pindexdata: &::core::option::Option<IDirect3DIndexBuffer9>) -> ::windows::core::Result<()>;
    fn GetIndices(&mut self) -> ::windows::core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(&mut self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(&mut self, pshader: &::core::option::Option<IDirect3DPixelShader9>) -> ::windows::core::Result<()>;
    fn GetPixelShader(&mut self) -> ::windows::core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(&mut self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn GetPixelShaderConstantF(&mut self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn SetPixelShaderConstantI(&mut self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn GetPixelShaderConstantI(&mut self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn SetPixelShaderConstantB(&mut self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn GetPixelShaderConstantB(&mut self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn DrawRectPatch(&mut self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::Result<()>;
    fn DrawTriPatch(&mut self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::Result<()>;
    fn DeletePatch(&mut self, handle: u32) -> ::windows::core::Result<()>;
    fn CreateQuery(&mut self, r#type: D3DQUERYTYPE) -> ::windows::core::Result<IDirect3DQuery9>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>() -> IDirect3DDevice9_Vtbl {
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAvailableTextureMem()
        }
        unsafe extern "system" fn EvictManagedResources<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EvictManagedResources().into()
        }
        unsafe extern "system" fn GetDirect3D<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppd3d9: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDirect3D() {
                ::core::result::Result::Ok(ok__) => {
                    *ppd3d9 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceCaps(::core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetCreationParameters<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn SetCursorProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCursorProperties(::core::mem::transmute_copy(&xhotspot), ::core::mem::transmute_copy(&yhotspot), ::core::mem::transmute(&pcursorbitmap)).into()
        }
        unsafe extern "system" fn SetCursorPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCursorPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ShowCursor<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowCursor(::core::mem::transmute_copy(&bshow))
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAdditionalSwapChain(::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSwapChain(::core::mem::transmute_copy(&iswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *pswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumberOfSwapChains()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&ppresentationparameters)).into()
        }
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Present(::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion)).into()
        }
        unsafe extern "system" fn GetBackBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackBuffer(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&ibackbuffer), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbackbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRasterStatus(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&prasterstatus)).into()
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDialogBoxMode(::core::mem::transmute_copy(&benabledialogs)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGammaRamp(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pramp))
        }
        unsafe extern "system" fn GetGammaRamp<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGammaRamp(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pramp))
        }
        unsafe extern "system" fn CreateTexture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateTexture(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&pptexture), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateVolumeTexture(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppvolumetexture), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCubeTexture(::core::mem::transmute_copy(&edgelength), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppcubetexture), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateVertexBuffer(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&fvf), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppvertexbuffer), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateIndexBuffer(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppindexbuffer), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateRenderTarget(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&lockable), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDepthStencilSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&discard), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn UpdateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: ::windows::core::RawPtr, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateSurface(::core::mem::transmute(&psourcesurface), ::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute(&pdestinationsurface), ::core::mem::transmute_copy(&pdestpoint)).into()
        }
        unsafe extern "system" fn UpdateTexture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetexture: ::windows::core::RawPtr, pdestinationtexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateTexture(::core::mem::transmute(&psourcetexture), ::core::mem::transmute(&pdestinationtexture)).into()
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRenderTargetData(::core::mem::transmute(&prendertarget), ::core::mem::transmute(&pdestsurface)).into()
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrontBufferData(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute(&pdestsurface)).into()
        }
        unsafe extern "system" fn StretchRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestsurface: ::windows::core::RawPtr, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StretchRect(::core::mem::transmute(&psourcesurface), ::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute(&pdestsurface), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn ColorFill<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ColorFill(::core::mem::transmute(&psurface), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateOffscreenPlainSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn SetRenderTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, prendertarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRenderTarget(::core::mem::transmute_copy(&rendertargetindex), ::core::mem::transmute(&prendertarget)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRenderTarget(::core::mem::transmute_copy(&rendertargetindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprendertarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewzstencil: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDepthStencilSurface(::core::mem::transmute(&pnewzstencil)).into()
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppzstencilsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDepthStencilSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *ppzstencilsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginScene<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginScene().into()
        }
        unsafe extern "system" fn EndScene<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndScene().into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prects), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&stencil)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn MultiplyTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MultiplyTransform(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetViewport<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewport(::core::mem::transmute_copy(&pviewport)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetViewport(::core::mem::transmute_copy(&pviewport)).into()
        }
        unsafe extern "system" fn SetMaterial<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaterial(::core::mem::transmute_copy(&pmaterial)).into()
        }
        unsafe extern "system" fn GetMaterial<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMaterial(::core::mem::transmute_copy(&pmaterial)).into()
        }
        unsafe extern "system" fn SetLight<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLight(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetLight<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLight(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn LightEnable<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LightEnable(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn GetLightEnable<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLightEnable(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&penable)).into()
        }
        unsafe extern "system" fn SetClipPlane<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipPlane(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pplane)).into()
        }
        unsafe extern "system" fn GetClipPlane<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClipPlane(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pplane)).into()
        }
        unsafe extern "system" fn SetRenderState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRenderState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetRenderState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRenderState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn CreateStateBlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStateBlock(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginStateBlock().into()
        }
        unsafe extern "system" fn EndStateBlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndStateBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipStatus(::core::mem::transmute_copy(&pclipstatus)).into()
        }
        unsafe extern "system" fn GetClipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClipStatus(::core::mem::transmute_copy(&pclipstatus)).into()
        }
        unsafe extern "system" fn GetTexture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, pptexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTexture(::core::mem::transmute_copy(&stage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTexture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, ptexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTexture(::core::mem::transmute_copy(&stage), ::core::mem::transmute(&ptexture)).into()
        }
        unsafe extern "system" fn GetTextureStageState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTextureStageState(::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetTextureStageState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextureStageState(::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSamplerState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSamplerState(::core::mem::transmute_copy(&sampler), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetSamplerState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSamplerState(::core::mem::transmute_copy(&sampler), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValidateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateDevice(::core::mem::transmute_copy(&pnumpasses)).into()
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPaletteEntries(::core::mem::transmute_copy(&palettenumber), ::core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPaletteEntries(::core::mem::transmute_copy(&palettenumber), ::core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentTexturePalette(::core::mem::transmute_copy(&palettenumber)).into()
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentTexturePalette(::core::mem::transmute_copy(&palettenumber)).into()
        }
        unsafe extern "system" fn SetScissorRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScissorRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn GetScissorRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScissorRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSoftwareVertexProcessing(::core::mem::transmute_copy(&bsoftware)).into()
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSoftwareVertexProcessing()
        }
        unsafe extern "system" fn SetNPatchMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsegments: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNPatchMode(::core::mem::transmute_copy(&nsegments)).into()
        }
        unsafe extern "system" fn GetNPatchMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNPatchMode()
        }
        unsafe extern "system" fn DrawPrimitive<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawPrimitive(::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&startvertex), ::core::mem::transmute_copy(&primitivecount)).into()
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawIndexedPrimitive(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&basevertexindex), ::core::mem::transmute_copy(&minvertexindex), ::core::mem::transmute_copy(&numvertices), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&primcount)).into()
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawPrimitiveUP(::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&primitivecount), ::core::mem::transmute_copy(&pvertexstreamzerodata), ::core::mem::transmute_copy(&vertexstreamzerostride)).into()
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawIndexedPrimitiveUP(::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&minvertexindex), ::core::mem::transmute_copy(&numvertices), ::core::mem::transmute_copy(&primitivecount), ::core::mem::transmute_copy(&pindexdata), ::core::mem::transmute_copy(&indexdataformat), ::core::mem::transmute_copy(&pvertexstreamzerodata), ::core::mem::transmute_copy(&vertexstreamzerostride)).into()
        }
        unsafe extern "system" fn ProcessVertices<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: ::windows::core::RawPtr, pvertexdecl: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessVertices(::core::mem::transmute_copy(&srcstartindex), ::core::mem::transmute_copy(&destindex), ::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute(&pdestbuffer), ::core::mem::transmute(&pvertexdecl), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVertexDeclaration(::core::mem::transmute_copy(&pvertexelements)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdecl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVertexDeclaration(::core::mem::transmute(&pdecl)).into()
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVertexDeclaration() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdecl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFVF<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFVF(::core::mem::transmute_copy(&fvf)).into()
        }
        unsafe extern "system" fn GetFVF<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFVF(::core::mem::transmute_copy(&pfvf)).into()
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVertexShader(::core::mem::transmute_copy(&pfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVertexShader(::core::mem::transmute(&pshader)).into()
        }
        unsafe extern "system" fn GetVertexShader<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVertexShader() {
                ::core::result::Result::Ok(ok__) => {
                    *ppshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVertexShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVertexShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVertexShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVertexShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVertexShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVertexShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn SetStreamSource<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, pstreamdata: ::windows::core::RawPtr, offsetinbytes: u32, stride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamSource(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute(&pstreamdata), ::core::mem::transmute_copy(&offsetinbytes), ::core::mem::transmute_copy(&stride)).into()
        }
        unsafe extern "system" fn GetStreamSource<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut ::windows::core::RawPtr, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamSource(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&ppstreamdata), ::core::mem::transmute_copy(&poffsetinbytes), ::core::mem::transmute_copy(&pstride)).into()
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, setting: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamSourceFreq(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamSourceFreq(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&psetting)).into()
        }
        unsafe extern "system" fn SetIndices<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndices(::core::mem::transmute(&pindexdata)).into()
        }
        unsafe extern "system" fn GetIndices<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppindexdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppindexdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePixelShader(::core::mem::transmute_copy(&pfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPixelShader(::core::mem::transmute(&pshader)).into()
        }
        unsafe extern "system" fn GetPixelShader<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPixelShader() {
                ::core::result::Result::Ok(ok__) => {
                    *ppshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPixelShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPixelShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPixelShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn DrawRectPatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawRectPatch(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&pnumsegs), ::core::mem::transmute_copy(&prectpatchinfo)).into()
        }
        unsafe extern "system" fn DrawTriPatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawTriPatch(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&pnumsegs), ::core::mem::transmute_copy(&ptripatchinfo)).into()
        }
        unsafe extern "system" fn DeletePatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePatch(::core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQuery(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetAvailableTextureMem: GetAvailableTextureMem::<Identity, Impl, OFFSET>,
            EvictManagedResources: EvictManagedResources::<Identity, Impl, OFFSET>,
            GetDirect3D: GetDirect3D::<Identity, Impl, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, Impl, OFFSET>,
            SetCursorProperties: SetCursorProperties::<Identity, Impl, OFFSET>,
            SetCursorPosition: SetCursorPosition::<Identity, Impl, OFFSET>,
            ShowCursor: ShowCursor::<Identity, Impl, OFFSET>,
            CreateAdditionalSwapChain: CreateAdditionalSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            GetNumberOfSwapChains: GetNumberOfSwapChains::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Present: Present::<Identity, Impl, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, Impl, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, Impl, OFFSET>,
            SetDialogBoxMode: SetDialogBoxMode::<Identity, Impl, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, Impl, OFFSET>,
            GetGammaRamp: GetGammaRamp::<Identity, Impl, OFFSET>,
            CreateTexture: CreateTexture::<Identity, Impl, OFFSET>,
            CreateVolumeTexture: CreateVolumeTexture::<Identity, Impl, OFFSET>,
            CreateCubeTexture: CreateCubeTexture::<Identity, Impl, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, Impl, OFFSET>,
            CreateIndexBuffer: CreateIndexBuffer::<Identity, Impl, OFFSET>,
            CreateRenderTarget: CreateRenderTarget::<Identity, Impl, OFFSET>,
            CreateDepthStencilSurface: CreateDepthStencilSurface::<Identity, Impl, OFFSET>,
            UpdateSurface: UpdateSurface::<Identity, Impl, OFFSET>,
            UpdateTexture: UpdateTexture::<Identity, Impl, OFFSET>,
            GetRenderTargetData: GetRenderTargetData::<Identity, Impl, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, Impl, OFFSET>,
            StretchRect: StretchRect::<Identity, Impl, OFFSET>,
            ColorFill: ColorFill::<Identity, Impl, OFFSET>,
            CreateOffscreenPlainSurface: CreateOffscreenPlainSurface::<Identity, Impl, OFFSET>,
            SetRenderTarget: SetRenderTarget::<Identity, Impl, OFFSET>,
            GetRenderTarget: GetRenderTarget::<Identity, Impl, OFFSET>,
            SetDepthStencilSurface: SetDepthStencilSurface::<Identity, Impl, OFFSET>,
            GetDepthStencilSurface: GetDepthStencilSurface::<Identity, Impl, OFFSET>,
            BeginScene: BeginScene::<Identity, Impl, OFFSET>,
            EndScene: EndScene::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            MultiplyTransform: MultiplyTransform::<Identity, Impl, OFFSET>,
            SetViewport: SetViewport::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            SetMaterial: SetMaterial::<Identity, Impl, OFFSET>,
            GetMaterial: GetMaterial::<Identity, Impl, OFFSET>,
            SetLight: SetLight::<Identity, Impl, OFFSET>,
            GetLight: GetLight::<Identity, Impl, OFFSET>,
            LightEnable: LightEnable::<Identity, Impl, OFFSET>,
            GetLightEnable: GetLightEnable::<Identity, Impl, OFFSET>,
            SetClipPlane: SetClipPlane::<Identity, Impl, OFFSET>,
            GetClipPlane: GetClipPlane::<Identity, Impl, OFFSET>,
            SetRenderState: SetRenderState::<Identity, Impl, OFFSET>,
            GetRenderState: GetRenderState::<Identity, Impl, OFFSET>,
            CreateStateBlock: CreateStateBlock::<Identity, Impl, OFFSET>,
            BeginStateBlock: BeginStateBlock::<Identity, Impl, OFFSET>,
            EndStateBlock: EndStateBlock::<Identity, Impl, OFFSET>,
            SetClipStatus: SetClipStatus::<Identity, Impl, OFFSET>,
            GetClipStatus: GetClipStatus::<Identity, Impl, OFFSET>,
            GetTexture: GetTexture::<Identity, Impl, OFFSET>,
            SetTexture: SetTexture::<Identity, Impl, OFFSET>,
            GetTextureStageState: GetTextureStageState::<Identity, Impl, OFFSET>,
            SetTextureStageState: SetTextureStageState::<Identity, Impl, OFFSET>,
            GetSamplerState: GetSamplerState::<Identity, Impl, OFFSET>,
            SetSamplerState: SetSamplerState::<Identity, Impl, OFFSET>,
            ValidateDevice: ValidateDevice::<Identity, Impl, OFFSET>,
            SetPaletteEntries: SetPaletteEntries::<Identity, Impl, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, Impl, OFFSET>,
            SetCurrentTexturePalette: SetCurrentTexturePalette::<Identity, Impl, OFFSET>,
            GetCurrentTexturePalette: GetCurrentTexturePalette::<Identity, Impl, OFFSET>,
            SetScissorRect: SetScissorRect::<Identity, Impl, OFFSET>,
            GetScissorRect: GetScissorRect::<Identity, Impl, OFFSET>,
            SetSoftwareVertexProcessing: SetSoftwareVertexProcessing::<Identity, Impl, OFFSET>,
            GetSoftwareVertexProcessing: GetSoftwareVertexProcessing::<Identity, Impl, OFFSET>,
            SetNPatchMode: SetNPatchMode::<Identity, Impl, OFFSET>,
            GetNPatchMode: GetNPatchMode::<Identity, Impl, OFFSET>,
            DrawPrimitive: DrawPrimitive::<Identity, Impl, OFFSET>,
            DrawIndexedPrimitive: DrawIndexedPrimitive::<Identity, Impl, OFFSET>,
            DrawPrimitiveUP: DrawPrimitiveUP::<Identity, Impl, OFFSET>,
            DrawIndexedPrimitiveUP: DrawIndexedPrimitiveUP::<Identity, Impl, OFFSET>,
            ProcessVertices: ProcessVertices::<Identity, Impl, OFFSET>,
            CreateVertexDeclaration: CreateVertexDeclaration::<Identity, Impl, OFFSET>,
            SetVertexDeclaration: SetVertexDeclaration::<Identity, Impl, OFFSET>,
            GetVertexDeclaration: GetVertexDeclaration::<Identity, Impl, OFFSET>,
            SetFVF: SetFVF::<Identity, Impl, OFFSET>,
            GetFVF: GetFVF::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            SetVertexShader: SetVertexShader::<Identity, Impl, OFFSET>,
            GetVertexShader: GetVertexShader::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantF: SetVertexShaderConstantF::<Identity, Impl, OFFSET>,
            GetVertexShaderConstantF: GetVertexShaderConstantF::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantI: SetVertexShaderConstantI::<Identity, Impl, OFFSET>,
            GetVertexShaderConstantI: GetVertexShaderConstantI::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantB: SetVertexShaderConstantB::<Identity, Impl, OFFSET>,
            GetVertexShaderConstantB: GetVertexShaderConstantB::<Identity, Impl, OFFSET>,
            SetStreamSource: SetStreamSource::<Identity, Impl, OFFSET>,
            GetStreamSource: GetStreamSource::<Identity, Impl, OFFSET>,
            SetStreamSourceFreq: SetStreamSourceFreq::<Identity, Impl, OFFSET>,
            GetStreamSourceFreq: GetStreamSourceFreq::<Identity, Impl, OFFSET>,
            SetIndices: SetIndices::<Identity, Impl, OFFSET>,
            GetIndices: GetIndices::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, Impl, OFFSET>,
            GetPixelShader: GetPixelShader::<Identity, Impl, OFFSET>,
            SetPixelShaderConstantF: SetPixelShaderConstantF::<Identity, Impl, OFFSET>,
            GetPixelShaderConstantF: GetPixelShaderConstantF::<Identity, Impl, OFFSET>,
            SetPixelShaderConstantI: SetPixelShaderConstantI::<Identity, Impl, OFFSET>,
            GetPixelShaderConstantI: GetPixelShaderConstantI::<Identity, Impl, OFFSET>,
            SetPixelShaderConstantB: SetPixelShaderConstantB::<Identity, Impl, OFFSET>,
            GetPixelShaderConstantB: GetPixelShaderConstantB::<Identity, Impl, OFFSET>,
            DrawRectPatch: DrawRectPatch::<Identity, Impl, OFFSET>,
            DrawTriPatch: DrawTriPatch::<Identity, Impl, OFFSET>,
            DeletePatch: DeletePatch::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9Ex_Impl: Sized + IDirect3DDevice9_Impl {
    fn SetConvolutionMonoKernel(&mut self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::Result<()>;
    fn ComposeRects(&mut self, psrc: &::core::option::Option<IDirect3DSurface9>, pdst: &::core::option::Option<IDirect3DSurface9>, psrcrectdescs: &::core::option::Option<IDirect3DVertexBuffer9>, numrects: u32, pdstrectdescs: &::core::option::Option<IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::Result<()>;
    fn PresentEx(&mut self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetGPUThreadPriority(&mut self, ppriority: *mut i32) -> ::windows::core::Result<()>;
    fn SetGPUThreadPriority(&mut self, priority: i32) -> ::windows::core::Result<()>;
    fn WaitForVBlank(&mut self, iswapchain: u32) -> ::windows::core::Result<()>;
    fn CheckResourceResidency(&mut self, presourcearray: *mut ::core::option::Option<IDirect3DResource9>, numresources: u32) -> ::windows::core::Result<()>;
    fn SetMaximumFrameLatency(&mut self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&mut self, pmaxlatency: *mut u32) -> ::windows::core::Result<()>;
    fn CheckDeviceState(&mut self, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CreateRenderTargetEx(&mut self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(&mut self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()>;
    fn CreateDepthStencilSurfaceEx(&mut self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()>;
    fn ResetEx(&mut self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::Result<()>;
    fn GetDisplayModeEx(&mut self, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>() -> IDirect3DDevice9Ex_Vtbl {
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConvolutionMonoKernel(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&rows), ::core::mem::transmute_copy(&columns)).into()
        }
        unsafe extern "system" fn ComposeRects<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr, psrcrectdescs: ::windows::core::RawPtr, numrects: u32, pdstrectdescs: ::windows::core::RawPtr, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ComposeRects(::core::mem::transmute(&psrc), ::core::mem::transmute(&pdst), ::core::mem::transmute(&psrcrectdescs), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute(&pdstrectdescs), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&xoffset), ::core::mem::transmute_copy(&yoffset)).into()
        }
        unsafe extern "system" fn PresentEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PresentEx(::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGPUThreadPriority(::core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGPUThreadPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForVBlank(::core::mem::transmute_copy(&iswapchain)).into()
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcearray: *mut ::windows::core::RawPtr, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckResourceResidency(::core::mem::transmute_copy(&presourcearray), ::core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMaximumFrameLatency(::core::mem::transmute_copy(&pmaxlatency)).into()
        }
        unsafe extern "system" fn CheckDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckDeviceState(::core::mem::transmute_copy(&hdestinationwindow)).into()
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateRenderTargetEx(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&lockable), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateOffscreenPlainSurfaceEx(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDepthStencilSurfaceEx(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&discard), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn ResetEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetEx(::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pfullscreendisplaymode)).into()
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayModeEx(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into()
        }
        Self {
            base: IDirect3DDevice9_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetConvolutionMonoKernel: SetConvolutionMonoKernel::<Identity, Impl, OFFSET>,
            ComposeRects: ComposeRects::<Identity, Impl, OFFSET>,
            PresentEx: PresentEx::<Identity, Impl, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, Impl, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, Impl, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, Impl, OFFSET>,
            CheckResourceResidency: CheckResourceResidency::<Identity, Impl, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, Impl, OFFSET>,
            CreateRenderTargetEx: CreateRenderTargetEx::<Identity, Impl, OFFSET>,
            CreateOffscreenPlainSurfaceEx: CreateOffscreenPlainSurfaceEx::<Identity, Impl, OFFSET>,
            CreateDepthStencilSurfaceEx: CreateDepthStencilSurfaceEx::<Identity, Impl, OFFSET>,
            ResetEx: ResetEx::<Identity, Impl, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice9Ex as ::windows::core::Interface>::IID || iid == &<IDirect3DDevice9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DIndexBuffer9_Impl: Sized + IDirect3DResource9_Impl {
    fn Lock(&mut self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()>;
    fn Unlock(&mut self) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::Result<()>;
}
impl IDirect3DIndexBuffer9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>() -> IDirect3DIndexBuffer9_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&offsettolock), ::core::mem::transmute_copy(&sizetolock), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock().into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DPixelShader9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetFunction(&mut self, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirect3DPixelShader9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DPixelShader9_Impl, const OFFSET: isize>() -> IDirect3DPixelShader9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFunction(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DQuery9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetType(&mut self) -> D3DQUERYTYPE;
    fn GetDataSize(&mut self) -> u32;
    fn Issue(&mut self, dwissueflags: u32) -> ::windows::core::Result<()>;
    fn GetData(&mut self, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::Result<()>;
}
impl IDirect3DQuery9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>() -> IDirect3DQuery9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DQUERYTYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataSize()
        }
        unsafe extern "system" fn Issue<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwissueflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Issue(::core::mem::transmute_copy(&dwissueflags)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwgetdataflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
            Issue: Issue::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DQuery9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DResource9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&mut self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
    fn FreePrivateData(&mut self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetPriority(&mut self, prioritynew: u32) -> u32;
    fn GetPriority(&mut self) -> u32;
    fn PreLoad(&mut self);
    fn GetType(&mut self) -> D3DRESOURCETYPE;
}
impl IDirect3DResource9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>() -> IDirect3DResource9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&sizeofdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreePrivateData(::core::mem::transmute_copy(&refguid)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prioritynew: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&prioritynew))
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPriority()
        }
        unsafe extern "system" fn PreLoad<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreLoad()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DRESOURCETYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            PreLoad: PreLoad::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DStateBlock9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn Capture(&mut self) -> ::windows::core::Result<()>;
    fn Apply(&mut self) -> ::windows::core::Result<()>;
}
impl IDirect3DStateBlock9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>() -> IDirect3DStateBlock9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capture<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Capture().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Apply().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            Capture: Capture::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSurface9_Impl: Sized + IDirect3DResource9_Impl {
    fn GetContainer(&mut self, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()>;
    fn LockRect(&mut self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockRect(&mut self) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, phdc: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn ReleaseDC(&mut self, hdc: super::Gdi::HDC) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSurface9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>() -> IDirect3DSurface9_Vtbl {
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContainer(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn LockRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockRect(::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockRect().into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&phdc)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSurface9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9_Impl: Sized {
    fn Present(&mut self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetFrontBufferData(&mut self, pdestsurface: &::core::option::Option<IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetBackBuffer(&mut self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&mut self, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&mut self, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(&mut self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSwapChain9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>() -> IDirect3DSwapChain9_Vtbl {
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Present(::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrontBufferData(::core::mem::transmute(&pdestsurface)).into()
        }
        unsafe extern "system" fn GetBackBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackBuffer(::core::mem::transmute_copy(&ibackbuffer), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbackbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRasterStatus(::core::mem::transmute_copy(&prasterstatus)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPresentParameters(::core::mem::transmute_copy(&ppresentationparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, Impl, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, Impl, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, Impl, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPresentParameters: GetPresentParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9Ex_Impl: Sized + IDirect3DSwapChain9_Impl {
    fn GetLastPresentCount(&mut self, plastpresentcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetPresentStats(&mut self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::Result<()>;
    fn GetDisplayModeEx(&mut self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSwapChain9Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>() -> IDirect3DSwapChain9Ex_Vtbl {
        unsafe extern "system" fn GetLastPresentCount<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastPresentCount(::core::mem::transmute_copy(&plastpresentcount)).into()
        }
        unsafe extern "system" fn GetPresentStats<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPresentStats(::core::mem::transmute_copy(&ppresentationstatistics)).into()
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayModeEx(::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into()
        }
        Self {
            base: IDirect3DSwapChain9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLastPresentCount: GetLastPresentCount::<Identity, Impl, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, Impl, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as ::windows::core::Interface>::IID || iid == &<IDirect3DSwapChain9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DTexture9_Impl: Sized + IDirect3DResource9_Impl + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&mut self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()>;
    fn GetSurfaceLevel(&mut self, level: u32) -> ::windows::core::Result<IDirect3DSurface9>;
    fn LockRect(&mut self, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockRect(&mut self, level: u32) -> ::windows::core::Result<()>;
    fn AddDirtyRect(&mut self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirect3DTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>() -> IDirect3DTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLevelDesc(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, ppsurfacelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSurfaceLevel(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsurfacelevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockRect(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockRect(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDirtyRect(::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base: IDirect3DBaseTexture9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DTexture9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVertexBuffer9_Impl: Sized + IDirect3DResource9_Impl {
    fn Lock(&mut self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()>;
    fn Unlock(&mut self) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::Result<()>;
}
impl IDirect3DVertexBuffer9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>() -> IDirect3DVertexBuffer9_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&offsettolock), ::core::mem::transmute_copy(&sizetolock), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock().into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVertexDeclaration9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetDeclaration(&mut self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirect3DVertexDeclaration9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>() -> IDirect3DVertexDeclaration9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeclaration(::core::mem::transmute_copy(&pelement), ::core::mem::transmute_copy(&pnumelements)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVertexShader9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetFunction(&mut self, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirect3DVertexShader9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexShader9_Impl, const OFFSET: isize>() -> IDirect3DVertexShader9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFunction(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVolume9_Impl: Sized {
    fn GetDevice(&mut self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&mut self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
    fn FreePrivateData(&mut self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetContainer(&mut self, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::Result<()>;
    fn LockBox(&mut self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockBox(&mut self) -> ::windows::core::Result<()>;
}
impl IDirect3DVolume9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>() -> IDirect3DVolume9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&sizeofdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreePrivateData(::core::mem::transmute_copy(&refguid)).into()
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContainer(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn LockBox<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockBox(::core::mem::transmute_copy(&plockedvolume), ::core::mem::transmute_copy(&pbox), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockBox<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockBox().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            LockBox: LockBox::<Identity, Impl, OFFSET>,
            UnlockBox: UnlockBox::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVolume9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVolumeTexture9_Impl: Sized + IDirect3DResource9_Impl + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&mut self, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::Result<()>;
    fn GetVolumeLevel(&mut self, level: u32) -> ::windows::core::Result<IDirect3DVolume9>;
    fn LockBox(&mut self, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockBox(&mut self, level: u32) -> ::windows::core::Result<()>;
    fn AddDirtyBox(&mut self, pdirtybox: *const D3DBOX) -> ::windows::core::Result<()>;
}
impl IDirect3DVolumeTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>() -> IDirect3DVolumeTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLevelDesc(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, ppvolumelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVolumeLevel(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvolumelevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBox<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockBox(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedvolume), ::core::mem::transmute_copy(&pbox), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockBox<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockBox(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyBox<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybox: *const D3DBOX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDirtyBox(::core::mem::transmute_copy(&pdirtybox)).into()
        }
        Self {
            base: IDirect3DBaseTexture9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, Impl, OFFSET>,
            LockBox: LockBox::<Identity, Impl, OFFSET>,
            UnlockBox: UnlockBox::<Identity, Impl, OFFSET>,
            AddDirtyBox: AddDirtyBox::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVolumeTexture9 as ::windows::core::Interface>::IID || iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as ::windows::core::Interface>::IID
    }
}

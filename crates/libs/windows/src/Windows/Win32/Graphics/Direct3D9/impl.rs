#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9_Impl: Sized {
    fn RegisterSoftwareDevice(&self, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::Result<()>;
    fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32;
    fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn CheckDeviceType(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::Result<()>;
    fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::Result<()>;
    fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::Result<()>;
    fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::Result<()>;
    fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()>;
    fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR;
    fn CreateDevice(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3D9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3D9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>() -> IDirect3D9_Vtbl {
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterSoftwareDevice(::core::mem::transmute_copy(&pinitializefunction)).into()
        }
        unsafe extern "system" fn GetAdapterCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterCount()
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterIdentifier(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pidentifier)).into()
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterModeCount(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAdapterModes(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterDisplayMode(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn CheckDeviceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckDeviceType(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devtype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&backbufferformat), ::core::mem::transmute_copy(&bwindowed)).into()
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckDeviceFormat(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&rtype), ::core::mem::transmute_copy(&checkformat)).into()
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckDeviceMultiSampleType(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&surfaceformat), ::core::mem::transmute_copy(&windowed), ::core::mem::transmute_copy(&multisampletype), ::core::mem::transmute_copy(&pqualitylevels)).into()
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckDepthStencilMatch(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&rendertargetformat), ::core::mem::transmute_copy(&depthstencilformat)).into()
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckDeviceFormatConversion(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&sourceformat), ::core::mem::transmute_copy(&targetformat)).into()
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceCaps(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterMonitor(::core::mem::transmute_copy(&adapter))
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&hfocuswindow), ::core::mem::transmute_copy(&behaviorflags), ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirect3D9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9Ex_Impl: Sized + IDirect3D9_Impl {
    fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32;
    fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::Result<()>;
    fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()>;
    fn CreateDeviceEx(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9Ex>) -> ::windows::core::Result<()>;
    fn GetAdapterLUID(&self, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3D9Ex {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3D9Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>() -> IDirect3D9Ex_Vtbl {
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterModeCountEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pfilter))
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAdapterModesEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterDisplayModeEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into()
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDeviceEx(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&hfocuswindow), ::core::mem::transmute_copy(&behaviorflags), ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pfullscreendisplaymode), ::core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterLUID(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pluid)).into()
        }
        Self {
            base__: IDirect3D9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Identity, Impl, OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Identity, Impl, OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Identity, Impl, OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D9Ex as ::windows::core::ComInterface>::IID || iid == &<IDirect3D9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DBaseTexture9_Impl: Sized + IDirect3DResource9_Impl {
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()>;
    fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
}
impl ::windows::core::RuntimeName for IDirect3DBaseTexture9 {}
impl IDirect3DBaseTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>() -> IDirect3DBaseTexture9_Vtbl {
        unsafe extern "system" fn SetLOD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lodnew: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLOD(::core::mem::transmute_copy(&lodnew))
        }
        unsafe extern "system" fn GetLOD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLOD()
        }
        unsafe extern "system" fn GetLevelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLevelCount()
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoGenFilterType(::core::mem::transmute_copy(&filtertype)).into()
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAutoGenFilterType()
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateMipSubLevels()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetLOD: SetLOD::<Identity, Impl, OFFSET>,
            GetLOD: GetLOD::<Identity, Impl, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, Impl, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, Impl, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, Impl, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DBaseTexture9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DCubeTexture9_Impl: Sized + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()>;
    fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::Result<IDirect3DSurface9>;
    fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::Result<()>;
    fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirect3DCubeTexture9 {}
#[cfg(feature = "Win32_Foundation")]
impl IDirect3DCubeTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>() -> IDirect3DCubeTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLevelDesc(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCubeMapSurface(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcubemapsurface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockRect(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockRect(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDirtyRect(::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DCubeTexture9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DBaseTexture9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9_Impl: Sized {
    fn TestCooperativeLevel(&self) -> ::windows::core::Result<()>;
    fn GetAvailableTextureMem(&self) -> u32;
    fn EvictManagedResources(&self) -> ::windows::core::Result<()>;
    fn GetDirect3D(&self) -> ::windows::core::Result<IDirect3D9>;
    fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::Result<()>;
    fn SetCursorProperties(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: ::core::option::Option<&IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn SetCursorPosition(&self, x: i32, y: i32, flags: u32);
    fn ShowCursor(&self, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::core::option::Option<IDirect3DSwapChain9>) -> ::windows::core::Result<()>;
    fn GetSwapChain(&self, iswapchain: u32) -> ::windows::core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(&self) -> u32;
    fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()>;
    fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::Result<()>;
    fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()>;
    fn SetDialogBoxMode(&self, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP);
    fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP);
    fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::core::option::Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::core::option::Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::core::option::Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::core::option::Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::core::option::Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateRenderTarget(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn UpdateSurface(&self, psourcesurface: ::core::option::Option<&IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: ::core::option::Option<&IDirect3DSurface9>, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn UpdateTexture(&self, psourcetexture: ::core::option::Option<&IDirect3DBaseTexture9>, pdestinationtexture: ::core::option::Option<&IDirect3DBaseTexture9>) -> ::windows::core::Result<()>;
    fn GetRenderTargetData(&self, prendertarget: ::core::option::Option<&IDirect3DSurface9>, pdestsurface: ::core::option::Option<&IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetFrontBufferData(&self, iswapchain: u32, pdestsurface: ::core::option::Option<&IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn StretchRect(&self, psourcesurface: ::core::option::Option<&IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestsurface: ::core::option::Option<&IDirect3DSurface9>, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()>;
    fn ColorFill(&self, psurface: ::core::option::Option<&IDirect3DSurface9>, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::Result<()>;
    fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetRenderTarget(&self, rendertargetindex: u32, prendertarget: ::core::option::Option<&IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetRenderTarget(&self, rendertargetindex: u32) -> ::windows::core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(&self, pnewzstencil: ::core::option::Option<&IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetDepthStencilSurface(&self) -> ::windows::core::Result<IDirect3DSurface9>;
    fn BeginScene(&self) -> ::windows::core::Result<()>;
    fn EndScene(&self) -> ::windows::core::Result<()>;
    fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::Result<()>;
    fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> ::windows::core::Result<()>;
    fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::Result<()>;
    fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> ::windows::core::Result<()>;
    fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::Result<()>;
    fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::Result<()>;
    fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::Result<()>;
    fn LightEnable(&self, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLightEnable(&self, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClipPlane(&self, index: u32, pplane: *const f32) -> ::windows::core::Result<()>;
    fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> ::windows::core::Result<()>;
    fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::Result<()>;
    fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> ::windows::core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(&self) -> ::windows::core::Result<()>;
    fn EndStateBlock(&self) -> ::windows::core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::Result<()>;
    fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::Result<()>;
    fn GetTexture(&self, stage: u32) -> ::windows::core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(&self, stage: u32, ptexture: ::core::option::Option<&IDirect3DBaseTexture9>) -> ::windows::core::Result<()>;
    fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::Result<()>;
    fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::Result<()>;
    fn ValidateDevice(&self, pnumpasses: *mut u32) -> ::windows::core::Result<()>;
    fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
    fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
    fn SetCurrentTexturePalette(&self, palettenumber: u32) -> ::windows::core::Result<()>;
    fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> ::windows::core::Result<()>;
    fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetSoftwareVertexProcessing(&self, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSoftwareVertexProcessing(&self) -> super::super::Foundation::BOOL;
    fn SetNPatchMode(&self, nsegments: f32) -> ::windows::core::Result<()>;
    fn GetNPatchMode(&self) -> f32;
    fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::Result<()>;
    fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::Result<()>;
    fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()>;
    fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()>;
    fn ProcessVertices(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: ::core::option::Option<&IDirect3DVertexBuffer9>, pvertexdecl: ::core::option::Option<&IDirect3DVertexDeclaration9>, flags: u32) -> ::windows::core::Result<()>;
    fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> ::windows::core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(&self, pdecl: ::core::option::Option<&IDirect3DVertexDeclaration9>) -> ::windows::core::Result<()>;
    fn GetVertexDeclaration(&self) -> ::windows::core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(&self, fvf: u32) -> ::windows::core::Result<()>;
    fn GetFVF(&self, pfvf: *mut u32) -> ::windows::core::Result<()>;
    fn CreateVertexShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(&self, pshader: ::core::option::Option<&IDirect3DVertexShader9>) -> ::windows::core::Result<()>;
    fn GetVertexShader(&self) -> ::windows::core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn SetStreamSource(&self, streamnumber: u32, pstreamdata: ::core::option::Option<&IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> ::windows::core::Result<()>;
    fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut ::core::option::Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::Result<()>;
    fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> ::windows::core::Result<()>;
    fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> ::windows::core::Result<()>;
    fn SetIndices(&self, pindexdata: ::core::option::Option<&IDirect3DIndexBuffer9>) -> ::windows::core::Result<()>;
    fn GetIndices(&self) -> ::windows::core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(&self, pshader: ::core::option::Option<&IDirect3DPixelShader9>) -> ::windows::core::Result<()>;
    fn GetPixelShader(&self) -> ::windows::core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()>;
    fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()>;
    fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()>;
    fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::Result<()>;
    fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::Result<()>;
    fn DeletePatch(&self, handle: u32) -> ::windows::core::Result<()>;
    fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> ::windows::core::Result<IDirect3DQuery9>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3DDevice9 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>() -> IDirect3DDevice9_Vtbl {
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableTextureMem()
        }
        unsafe extern "system" fn EvictManagedResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EvictManagedResources().into()
        }
        unsafe extern "system" fn GetDirect3D<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppd3d9: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirect3D() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppd3d9, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceCaps(::core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayMode(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetCreationParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCreationParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn SetCursorProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCursorProperties(::core::mem::transmute_copy(&xhotspot), ::core::mem::transmute_copy(&yhotspot), ::windows::core::from_raw_borrowed(&pcursorbitmap)).into()
        }
        unsafe extern "system" fn SetCursorPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, flags: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCursorPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ShowCursor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowCursor(::core::mem::transmute_copy(&bshow))
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAdditionalSwapChain(::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSwapChain(::core::mem::transmute_copy(&iswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumberOfSwapChains()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset(::core::mem::transmute_copy(&ppresentationparameters)).into()
        }
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Present(::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion)).into()
        }
        unsafe extern "system" fn GetBackBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackBuffer(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&ibackbuffer), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRasterStatus(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&prasterstatus)).into()
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDialogBoxMode(::core::mem::transmute_copy(&benabledialogs)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGammaRamp(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pramp))
        }
        unsafe extern "system" fn GetGammaRamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGammaRamp(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pramp))
        }
        unsafe extern "system" fn CreateTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTexture(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&pptexture), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVolumeTexture(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppvolumetexture), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateCubeTexture(::core::mem::transmute_copy(&edgelength), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppcubetexture), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVertexBuffer(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&fvf), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppvertexbuffer), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateIndexBuffer(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppindexbuffer), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateRenderTarget(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&lockable), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDepthStencilSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&discard), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn UpdateSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcesurface: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: *mut ::core::ffi::c_void, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateSurface(::windows::core::from_raw_borrowed(&psourcesurface), ::core::mem::transmute_copy(&psourcerect), ::windows::core::from_raw_borrowed(&pdestinationsurface), ::core::mem::transmute_copy(&pdestpoint)).into()
        }
        unsafe extern "system" fn UpdateTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetexture: *mut ::core::ffi::c_void, pdestinationtexture: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateTexture(::windows::core::from_raw_borrowed(&psourcetexture), ::windows::core::from_raw_borrowed(&pdestinationtexture)).into()
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: *mut ::core::ffi::c_void, pdestsurface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRenderTargetData(::windows::core::from_raw_borrowed(&prendertarget), ::windows::core::from_raw_borrowed(&pdestsurface)).into()
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pdestsurface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrontBufferData(::core::mem::transmute_copy(&iswapchain), ::windows::core::from_raw_borrowed(&pdestsurface)).into()
        }
        unsafe extern "system" fn StretchRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcesurface: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestsurface: *mut ::core::ffi::c_void, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StretchRect(::windows::core::from_raw_borrowed(&psourcesurface), ::core::mem::transmute_copy(&psourcerect), ::windows::core::from_raw_borrowed(&pdestsurface), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn ColorFill<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ColorFill(::windows::core::from_raw_borrowed(&psurface), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateOffscreenPlainSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into()
        }
        unsafe extern "system" fn SetRenderTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, prendertarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRenderTarget(::core::mem::transmute_copy(&rendertargetindex), ::windows::core::from_raw_borrowed(&prendertarget)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRenderTarget(::core::mem::transmute_copy(&rendertargetindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprendertarget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewzstencil: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDepthStencilSurface(::windows::core::from_raw_borrowed(&pnewzstencil)).into()
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppzstencilsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDepthStencilSurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppzstencilsurface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginScene<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginScene().into()
        }
        unsafe extern "system" fn EndScene<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndScene().into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prects), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&stencil)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransform(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransform(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn MultiplyTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MultiplyTransform(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetViewport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewport(::core::mem::transmute_copy(&pviewport)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetViewport(::core::mem::transmute_copy(&pviewport)).into()
        }
        unsafe extern "system" fn SetMaterial<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaterial(::core::mem::transmute_copy(&pmaterial)).into()
        }
        unsafe extern "system" fn GetMaterial<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaterial(::core::mem::transmute_copy(&pmaterial)).into()
        }
        unsafe extern "system" fn SetLight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLight(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetLight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLight(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn LightEnable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LightEnable(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn GetLightEnable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLightEnable(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&penable)).into()
        }
        unsafe extern "system" fn SetClipPlane<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipPlane(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pplane)).into()
        }
        unsafe extern "system" fn GetClipPlane<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClipPlane(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pplane)).into()
        }
        unsafe extern "system" fn SetRenderState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRenderState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetRenderState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRenderState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn CreateStateBlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStateBlock(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginStateBlock().into()
        }
        unsafe extern "system" fn EndStateBlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndStateBlock() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipStatus(::core::mem::transmute_copy(&pclipstatus)).into()
        }
        unsafe extern "system" fn GetClipStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClipStatus(::core::mem::transmute_copy(&pclipstatus)).into()
        }
        unsafe extern "system" fn GetTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, pptexture: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTexture(::core::mem::transmute_copy(&stage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, ptexture: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTexture(::core::mem::transmute_copy(&stage), ::windows::core::from_raw_borrowed(&ptexture)).into()
        }
        unsafe extern "system" fn GetTextureStageState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextureStageState(::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetTextureStageState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextureStageState(::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSamplerState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSamplerState(::core::mem::transmute_copy(&sampler), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetSamplerState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSamplerState(::core::mem::transmute_copy(&sampler), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValidateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidateDevice(::core::mem::transmute_copy(&pnumpasses)).into()
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPaletteEntries(::core::mem::transmute_copy(&palettenumber), ::core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPaletteEntries(::core::mem::transmute_copy(&palettenumber), ::core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentTexturePalette(::core::mem::transmute_copy(&palettenumber)).into()
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentTexturePalette(::core::mem::transmute_copy(&palettenumber)).into()
        }
        unsafe extern "system" fn SetScissorRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScissorRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn GetScissorRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScissorRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSoftwareVertexProcessing(::core::mem::transmute_copy(&bsoftware)).into()
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSoftwareVertexProcessing()
        }
        unsafe extern "system" fn SetNPatchMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsegments: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNPatchMode(::core::mem::transmute_copy(&nsegments)).into()
        }
        unsafe extern "system" fn GetNPatchMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNPatchMode()
        }
        unsafe extern "system" fn DrawPrimitive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawPrimitive(::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&startvertex), ::core::mem::transmute_copy(&primitivecount)).into()
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawIndexedPrimitive(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&basevertexindex), ::core::mem::transmute_copy(&minvertexindex), ::core::mem::transmute_copy(&numvertices), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&primcount)).into()
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawPrimitiveUP(::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&primitivecount), ::core::mem::transmute_copy(&pvertexstreamzerodata), ::core::mem::transmute_copy(&vertexstreamzerostride)).into()
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawIndexedPrimitiveUP(::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&minvertexindex), ::core::mem::transmute_copy(&numvertices), ::core::mem::transmute_copy(&primitivecount), ::core::mem::transmute_copy(&pindexdata), ::core::mem::transmute_copy(&indexdataformat), ::core::mem::transmute_copy(&pvertexstreamzerodata), ::core::mem::transmute_copy(&vertexstreamzerostride)).into()
        }
        unsafe extern "system" fn ProcessVertices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: *mut ::core::ffi::c_void, pvertexdecl: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessVertices(::core::mem::transmute_copy(&srcstartindex), ::core::mem::transmute_copy(&destindex), ::core::mem::transmute_copy(&vertexcount), ::windows::core::from_raw_borrowed(&pdestbuffer), ::windows::core::from_raw_borrowed(&pvertexdecl), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVertexDeclaration(::core::mem::transmute_copy(&pvertexelements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdecl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVertexDeclaration(::windows::core::from_raw_borrowed(&pdecl)).into()
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdecl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVertexDeclaration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdecl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFVF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvf: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFVF(::core::mem::transmute_copy(&fvf)).into()
        }
        unsafe extern "system" fn GetFVF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFVF(::core::mem::transmute_copy(&pfvf)).into()
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVertexShader(::core::mem::transmute_copy(&pfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVertexShader(::windows::core::from_raw_borrowed(&pshader)).into()
        }
        unsafe extern "system" fn GetVertexShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVertexShader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVertexShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVertexShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVertexShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVertexShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVertexShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVertexShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn SetStreamSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, pstreamdata: *mut ::core::ffi::c_void, offsetinbytes: u32, stride: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamSource(::core::mem::transmute_copy(&streamnumber), ::windows::core::from_raw_borrowed(&pstreamdata), ::core::mem::transmute_copy(&offsetinbytes), ::core::mem::transmute_copy(&stride)).into()
        }
        unsafe extern "system" fn GetStreamSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut *mut ::core::ffi::c_void, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamSource(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&ppstreamdata), ::core::mem::transmute_copy(&poffsetinbytes), ::core::mem::transmute_copy(&pstride)).into()
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, setting: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamSourceFreq(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamSourceFreq(::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&psetting)).into()
        }
        unsafe extern "system" fn SetIndices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndices(::windows::core::from_raw_borrowed(&pindexdata)).into()
        }
        unsafe extern "system" fn GetIndices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppindexdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppindexdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePixelShader(::core::mem::transmute_copy(&pfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPixelShader(::windows::core::from_raw_borrowed(&pshader)).into()
        }
        unsafe extern "system" fn GetPixelShader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelShader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPixelShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelShaderConstantF(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into()
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPixelShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelShaderConstantI(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into()
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPixelShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelShaderConstantB(::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into()
        }
        unsafe extern "system" fn DrawRectPatch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawRectPatch(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&pnumsegs), ::core::mem::transmute_copy(&prectpatchinfo)).into()
        }
        unsafe extern "system" fn DrawTriPatch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawTriPatch(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&pnumsegs), ::core::mem::transmute_copy(&ptripatchinfo)).into()
        }
        unsafe extern "system" fn DeletePatch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePatch(::core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQuery(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirect3DDevice9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9Ex_Impl: Sized + IDirect3DDevice9_Impl {
    fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::Result<()>;
    fn ComposeRects(&self, psrc: ::core::option::Option<&IDirect3DSurface9>, pdst: ::core::option::Option<&IDirect3DSurface9>, psrcrectdescs: ::core::option::Option<&IDirect3DVertexBuffer9>, numrects: u32, pdstrectdescs: ::core::option::Option<&IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::Result<()>;
    fn PresentEx(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()>;
    fn WaitForVBlank(&self, iswapchain: u32) -> ::windows::core::Result<()>;
    fn CheckResourceResidency(&self, presourcearray: *mut ::core::option::Option<IDirect3DResource9>, numresources: u32) -> ::windows::core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::core::Result<()>;
    fn CheckDeviceState(&self, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CreateRenderTargetEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()>;
    fn CreateDepthStencilSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()>;
    fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::Result<()>;
    fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3DDevice9Ex {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>() -> IDirect3DDevice9Ex_Vtbl {
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConvolutionMonoKernel(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&rows), ::core::mem::transmute_copy(&columns)).into()
        }
        unsafe extern "system" fn ComposeRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *mut ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, psrcrectdescs: *mut ::core::ffi::c_void, numrects: u32, pdstrectdescs: *mut ::core::ffi::c_void, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComposeRects(::windows::core::from_raw_borrowed(&psrc), ::windows::core::from_raw_borrowed(&pdst), ::windows::core::from_raw_borrowed(&psrcrectdescs), ::core::mem::transmute_copy(&numrects), ::windows::core::from_raw_borrowed(&pdstrectdescs), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&xoffset), ::core::mem::transmute_copy(&yoffset)).into()
        }
        unsafe extern "system" fn PresentEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PresentEx(::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGPUThreadPriority(::core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGPUThreadPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForVBlank(::core::mem::transmute_copy(&iswapchain)).into()
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcearray: *mut *mut ::core::ffi::c_void, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckResourceResidency(::core::mem::transmute_copy(&presourcearray), ::core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaximumFrameLatency(::core::mem::transmute_copy(&pmaxlatency)).into()
        }
        unsafe extern "system" fn CheckDeviceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckDeviceState(::core::mem::transmute_copy(&hdestinationwindow)).into()
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateRenderTargetEx(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&lockable), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateOffscreenPlainSurfaceEx(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDepthStencilSurfaceEx(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&discard), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into()
        }
        unsafe extern "system" fn ResetEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetEx(::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pfullscreendisplaymode)).into()
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayModeEx(::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into()
        }
        Self {
            base__: IDirect3DDevice9_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDirect3DDevice9Ex as ::windows::core::ComInterface>::IID || iid == &<IDirect3DDevice9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DIndexBuffer9_Impl: Sized + IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()>;
    fn Unlock(&self) -> ::windows::core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DIndexBuffer9 {}
impl IDirect3DIndexBuffer9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>() -> IDirect3DIndexBuffer9_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&offsettolock), ::core::mem::transmute_copy(&sizetolock), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock().into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DPixelShader9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DPixelShader9 {}
impl IDirect3DPixelShader9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DPixelShader9_Impl, const OFFSET: isize>() -> IDirect3DPixelShader9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunction(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DQuery9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetType(&self) -> D3DQUERYTYPE;
    fn GetDataSize(&self) -> u32;
    fn Issue(&self, dwissueflags: u32) -> ::windows::core::Result<()>;
    fn GetData(&self, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DQuery9 {}
impl IDirect3DQuery9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>() -> IDirect3DQuery9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DQUERYTYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType()
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataSize()
        }
        unsafe extern "system" fn Issue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwissueflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Issue(::core::mem::transmute_copy(&dwissueflags)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwgetdataflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
            Issue: Issue::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DQuery9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DResource9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
    fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> D3DRESOURCETYPE;
}
impl ::windows::core::RuntimeName for IDirect3DResource9 {}
impl IDirect3DResource9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>() -> IDirect3DResource9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&sizeofdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreePrivateData(::core::mem::transmute_copy(&refguid)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prioritynew: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&prioritynew))
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPriority()
        }
        unsafe extern "system" fn PreLoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreLoad()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DRESOURCETYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DStateBlock9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn Capture(&self) -> ::windows::core::Result<()>;
    fn Apply(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DStateBlock9 {}
impl IDirect3DStateBlock9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>() -> IDirect3DStateBlock9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Capture().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            Capture: Capture::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSurface9_Impl: Sized + IDirect3DResource9_Impl {
    fn GetContainer(&self, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()>;
    fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockRect(&self) -> ::windows::core::Result<()>;
    fn GetDC(&self, phdc: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn ReleaseDC(&self, hdc: super::Gdi::HDC) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3DSurface9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSurface9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>() -> IDirect3DSurface9_Vtbl {
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContainer(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn LockRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockRect(::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockRect().into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDC(::core::mem::transmute_copy(&phdc)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSurface9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9_Impl: Sized {
    fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetFrontBufferData(&self, pdestsurface: ::core::option::Option<&IDirect3DSurface9>) -> ::windows::core::Result<()>;
    fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()>;
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3DSwapChain9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSwapChain9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>() -> IDirect3DSwapChain9_Vtbl {
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Present(::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestsurface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrontBufferData(::windows::core::from_raw_borrowed(&pdestsurface)).into()
        }
        unsafe extern "system" fn GetBackBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackBuffer(::core::mem::transmute_copy(&ibackbuffer), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRasterStatus(::core::mem::transmute_copy(&prasterstatus)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayMode(::core::mem::transmute_copy(&pmode)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresentParameters(::core::mem::transmute_copy(&ppresentationparameters)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirect3DSwapChain9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9Ex_Impl: Sized + IDirect3DSwapChain9_Impl {
    fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::Result<()>;
    fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDirect3DSwapChain9Ex {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSwapChain9Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>() -> IDirect3DSwapChain9Ex_Vtbl {
        unsafe extern "system" fn GetLastPresentCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastPresentCount(::core::mem::transmute_copy(&plastpresentcount)).into()
        }
        unsafe extern "system" fn GetPresentStats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresentStats(::core::mem::transmute_copy(&ppresentationstatistics)).into()
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayModeEx(::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into()
        }
        Self {
            base__: IDirect3DSwapChain9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLastPresentCount: GetLastPresentCount::<Identity, Impl, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, Impl, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as ::windows::core::ComInterface>::IID || iid == &<IDirect3DSwapChain9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DTexture9_Impl: Sized + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()>;
    fn GetSurfaceLevel(&self, level: u32) -> ::windows::core::Result<IDirect3DSurface9>;
    fn LockRect(&self, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockRect(&self, level: u32) -> ::windows::core::Result<()>;
    fn AddDirtyRect(&self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirect3DTexture9 {}
#[cfg(feature = "Win32_Foundation")]
impl IDirect3DTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>() -> IDirect3DTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLevelDesc(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, ppsurfacelevel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSurfaceLevel(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsurfacelevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockRect(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockRect(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDirtyRect(::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DTexture9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DBaseTexture9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DVertexBuffer9_Impl: Sized + IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()>;
    fn Unlock(&self) -> ::windows::core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DVertexBuffer9 {}
impl IDirect3DVertexBuffer9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>() -> IDirect3DVertexBuffer9_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&offsettolock), ::core::mem::transmute_copy(&sizetolock), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock().into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, Impl, OFFSET>(),
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DVertexDeclaration9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DVertexDeclaration9 {}
impl IDirect3DVertexDeclaration9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>() -> IDirect3DVertexDeclaration9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeclaration(::core::mem::transmute_copy(&pelement), ::core::mem::transmute_copy(&pnumelements)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DVertexShader9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DVertexShader9 {}
impl IDirect3DVertexShader9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexShader9_Impl, const OFFSET: isize>() -> IDirect3DVertexShader9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunction(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DVolume9_Impl: Sized {
    fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()>;
    fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetContainer(&self, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::Result<()>;
    fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockBox(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DVolume9 {}
impl IDirect3DVolume9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>() -> IDirect3DVolume9_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&sizeofdata), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&psizeofdata)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreePrivateData(::core::mem::transmute_copy(&refguid)).into()
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContainer(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn LockBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockBox(::core::mem::transmute_copy(&plockedvolume), ::core::mem::transmute_copy(&pbox), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockBox().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirect3DVolume9 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
pub trait IDirect3DVolumeTexture9_Impl: Sized + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::Result<()>;
    fn GetVolumeLevel(&self, level: u32) -> ::windows::core::Result<IDirect3DVolume9>;
    fn LockBox(&self, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::Result<()>;
    fn UnlockBox(&self, level: u32) -> ::windows::core::Result<()>;
    fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirect3DVolumeTexture9 {}
impl IDirect3DVolumeTexture9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>() -> IDirect3DVolumeTexture9_Vtbl {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLevelDesc(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, ppvolumelevel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVolumeLevel(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvolumelevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockBox(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedvolume), ::core::mem::transmute_copy(&pbox), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnlockBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockBox(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn AddDirtyBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybox: *const D3DBOX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDirtyBox(::core::mem::transmute_copy(&pdirtybox)).into()
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, Impl, OFFSET>,
            LockBox: LockBox::<Identity, Impl, OFFSET>,
            UnlockBox: UnlockBox::<Identity, Impl, OFFSET>,
            AddDirtyBox: AddDirtyBox::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVolumeTexture9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DResource9 as ::windows::core::ComInterface>::IID || iid == &<IDirect3DBaseTexture9 as ::windows::core::ComInterface>::IID
    }
}

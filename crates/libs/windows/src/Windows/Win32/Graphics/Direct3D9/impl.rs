#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9Impl: Sized {
    fn RegisterSoftwareDevice();
    fn GetAdapterCount();
    fn GetAdapterIdentifier();
    fn GetAdapterModeCount();
    fn EnumAdapterModes();
    fn GetAdapterDisplayMode();
    fn CheckDeviceType();
    fn CheckDeviceFormat();
    fn CheckDeviceMultiSampleType();
    fn CheckDepthStencilMatch();
    fn CheckDeviceFormatConversion();
    fn GetDeviceCaps();
    fn GetAdapterMonitor();
    fn CreateDevice();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3D9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D9Vtbl {
        unsafe extern "system" fn RegisterSoftwareDevice<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterCount<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterIdentifier<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterModeCount<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAdapterModes<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDeviceType<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDeviceFormat<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceCaps<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterMonitor<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDevice<Impl: IDirect3D9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterSoftwareDevice: RegisterSoftwareDevice::<Impl, IMPL_OFFSET>,
            GetAdapterCount: GetAdapterCount::<Impl, IMPL_OFFSET>,
            GetAdapterIdentifier: GetAdapterIdentifier::<Impl, IMPL_OFFSET>,
            GetAdapterModeCount: GetAdapterModeCount::<Impl, IMPL_OFFSET>,
            EnumAdapterModes: EnumAdapterModes::<Impl, IMPL_OFFSET>,
            GetAdapterDisplayMode: GetAdapterDisplayMode::<Impl, IMPL_OFFSET>,
            CheckDeviceType: CheckDeviceType::<Impl, IMPL_OFFSET>,
            CheckDeviceFormat: CheckDeviceFormat::<Impl, IMPL_OFFSET>,
            CheckDeviceMultiSampleType: CheckDeviceMultiSampleType::<Impl, IMPL_OFFSET>,
            CheckDepthStencilMatch: CheckDepthStencilMatch::<Impl, IMPL_OFFSET>,
            CheckDeviceFormatConversion: CheckDeviceFormatConversion::<Impl, IMPL_OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Impl, IMPL_OFFSET>,
            GetAdapterMonitor: GetAdapterMonitor::<Impl, IMPL_OFFSET>,
            CreateDevice: CreateDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9ExImpl: Sized + IDirect3D9Impl {
    fn GetAdapterModeCountEx();
    fn EnumAdapterModesEx();
    fn GetAdapterDisplayModeEx();
    fn CreateDeviceEx();
    fn GetAdapterLUID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3D9ExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D9ExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D9ExVtbl {
        unsafe extern "system" fn GetAdapterModeCountEx<Impl: IDirect3D9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAdapterModesEx<Impl: IDirect3D9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Impl: IDirect3D9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirect3D9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterLUID<Impl: IDirect3D9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3D9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Impl, IMPL_OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Impl, IMPL_OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Impl, IMPL_OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Impl, IMPL_OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D9Ex as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DBaseTexture9Impl: Sized + IDirect3DResource9Impl {
    fn SetLOD();
    fn GetLOD();
    fn GetLevelCount();
    fn SetAutoGenFilterType();
    fn GetAutoGenFilterType();
    fn GenerateMipSubLevels();
}
impl IDirect3DBaseTexture9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DBaseTexture9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DBaseTexture9Vtbl {
        unsafe extern "system" fn SetLOD<Impl: IDirect3DBaseTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lodnew: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLOD<Impl: IDirect3DBaseTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLevelCount<Impl: IDirect3DBaseTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoGenFilterType<Impl: IDirect3DBaseTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutoGenFilterType<Impl: IDirect3DBaseTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateMipSubLevels<Impl: IDirect3DBaseTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DResource9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetLOD: SetLOD::<Impl, IMPL_OFFSET>,
            GetLOD: GetLOD::<Impl, IMPL_OFFSET>,
            GetLevelCount: GetLevelCount::<Impl, IMPL_OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Impl, IMPL_OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Impl, IMPL_OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DBaseTexture9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DCubeTexture9Impl: Sized + IDirect3DResource9Impl + IDirect3DBaseTexture9Impl {
    fn GetLevelDesc();
    fn GetCubeMapSurface();
    fn LockRect();
    fn UnlockRect();
    fn AddDirtyRect();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirect3DCubeTexture9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DCubeTexture9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DCubeTexture9Vtbl {
        unsafe extern "system" fn GetLevelDesc<Impl: IDirect3DCubeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCubeMapSurface<Impl: IDirect3DCubeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockRect<Impl: IDirect3DCubeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockRect<Impl: IDirect3DCubeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDirtyRect<Impl: IDirect3DCubeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DBaseTexture9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Impl, IMPL_OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Impl, IMPL_OFFSET>,
            LockRect: LockRect::<Impl, IMPL_OFFSET>,
            UnlockRect: UnlockRect::<Impl, IMPL_OFFSET>,
            AddDirtyRect: AddDirtyRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DCubeTexture9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9Impl: Sized {
    fn TestCooperativeLevel();
    fn GetAvailableTextureMem();
    fn EvictManagedResources();
    fn GetDirect3D();
    fn GetDeviceCaps();
    fn GetDisplayMode();
    fn GetCreationParameters();
    fn SetCursorProperties();
    fn SetCursorPosition();
    fn ShowCursor();
    fn CreateAdditionalSwapChain();
    fn GetSwapChain();
    fn GetNumberOfSwapChains();
    fn Reset();
    fn Present();
    fn GetBackBuffer();
    fn GetRasterStatus();
    fn SetDialogBoxMode();
    fn SetGammaRamp();
    fn GetGammaRamp();
    fn CreateTexture();
    fn CreateVolumeTexture();
    fn CreateCubeTexture();
    fn CreateVertexBuffer();
    fn CreateIndexBuffer();
    fn CreateRenderTarget();
    fn CreateDepthStencilSurface();
    fn UpdateSurface();
    fn UpdateTexture();
    fn GetRenderTargetData();
    fn GetFrontBufferData();
    fn StretchRect();
    fn ColorFill();
    fn CreateOffscreenPlainSurface();
    fn SetRenderTarget();
    fn GetRenderTarget();
    fn SetDepthStencilSurface();
    fn GetDepthStencilSurface();
    fn BeginScene();
    fn EndScene();
    fn Clear();
    fn SetTransform();
    fn GetTransform();
    fn MultiplyTransform();
    fn SetViewport();
    fn GetViewport();
    fn SetMaterial();
    fn GetMaterial();
    fn SetLight();
    fn GetLight();
    fn LightEnable();
    fn GetLightEnable();
    fn SetClipPlane();
    fn GetClipPlane();
    fn SetRenderState();
    fn GetRenderState();
    fn CreateStateBlock();
    fn BeginStateBlock();
    fn EndStateBlock();
    fn SetClipStatus();
    fn GetClipStatus();
    fn GetTexture();
    fn SetTexture();
    fn GetTextureStageState();
    fn SetTextureStageState();
    fn GetSamplerState();
    fn SetSamplerState();
    fn ValidateDevice();
    fn SetPaletteEntries();
    fn GetPaletteEntries();
    fn SetCurrentTexturePalette();
    fn GetCurrentTexturePalette();
    fn SetScissorRect();
    fn GetScissorRect();
    fn SetSoftwareVertexProcessing();
    fn GetSoftwareVertexProcessing();
    fn SetNPatchMode();
    fn GetNPatchMode();
    fn DrawPrimitive();
    fn DrawIndexedPrimitive();
    fn DrawPrimitiveUP();
    fn DrawIndexedPrimitiveUP();
    fn ProcessVertices();
    fn CreateVertexDeclaration();
    fn SetVertexDeclaration();
    fn GetVertexDeclaration();
    fn SetFVF();
    fn GetFVF();
    fn CreateVertexShader();
    fn SetVertexShader();
    fn GetVertexShader();
    fn SetVertexShaderConstantF();
    fn GetVertexShaderConstantF();
    fn SetVertexShaderConstantI();
    fn GetVertexShaderConstantI();
    fn SetVertexShaderConstantB();
    fn GetVertexShaderConstantB();
    fn SetStreamSource();
    fn GetStreamSource();
    fn SetStreamSourceFreq();
    fn GetStreamSourceFreq();
    fn SetIndices();
    fn GetIndices();
    fn CreatePixelShader();
    fn SetPixelShader();
    fn GetPixelShader();
    fn SetPixelShaderConstantF();
    fn GetPixelShaderConstantF();
    fn SetPixelShaderConstantI();
    fn GetPixelShaderConstantI();
    fn SetPixelShaderConstantB();
    fn GetPixelShaderConstantB();
    fn DrawRectPatch();
    fn DrawTriPatch();
    fn DeletePatch();
    fn CreateQuery();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DDevice9Vtbl {
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableTextureMem<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EvictManagedResources<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDirect3D<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppd3d9: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceCaps<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreationParameters<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCursorProperties<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCursorPosition<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowCursor<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSwapChain<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Present<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackBuffer<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRasterStatus<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDialogBoxMode<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGammaRamp<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGammaRamp<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVolumeTexture<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCubeTexture<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVertexBuffer<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateIndexBuffer<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRenderTarget<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateSurface<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: ::windows::core::RawPtr, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateTexture<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcetexture: ::windows::core::RawPtr, pdestinationtexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderTargetData<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrontBufferData<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StretchRect<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestsurface: ::windows::core::RawPtr, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ColorFill<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderTarget<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, prendertarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderTarget<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDepthStencilSurface<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewzstencil: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDepthStencilSurface<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppzstencilsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginScene<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndScene<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultiplyTransform<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewport<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewport<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaterial<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaterial<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLight<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLight<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LightEnable<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLightEnable<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipPlane<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipPlane<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderState<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderState<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStateBlock<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginStateBlock<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndStateBlock<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipStatus<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipStatus<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTexture<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, pptexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTexture<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, ptexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextureStageState<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextureStageState<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSamplerState<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSamplerState<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateDevice<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPaletteEntries<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPaletteEntries<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palettenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScissorRect<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScissorRect<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNPatchMode<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsegments: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNPatchMode<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawPrimitive<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawPrimitiveUP<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessVertices<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: ::windows::core::RawPtr, pvertexdecl: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVertexDeclaration<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexDeclaration<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexDeclaration<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFVF<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFVF<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVertexShader<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexShader<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexShader<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSource<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, pstreamdata: ::windows::core::RawPtr, offsetinbytes: u32, stride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSource<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut ::windows::core::RawPtr, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamSourceFreq<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, setting: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSourceFreq<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndices<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndices<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppindexdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePixelShader<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelShader<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelShader<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawRectPatch<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawTriPatch<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeletePatch<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQuery<Impl: IDirect3DDevice9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TestCooperativeLevel: TestCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetAvailableTextureMem: GetAvailableTextureMem::<Impl, IMPL_OFFSET>,
            EvictManagedResources: EvictManagedResources::<Impl, IMPL_OFFSET>,
            GetDirect3D: GetDirect3D::<Impl, IMPL_OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Impl, IMPL_OFFSET>,
            GetDisplayMode: GetDisplayMode::<Impl, IMPL_OFFSET>,
            GetCreationParameters: GetCreationParameters::<Impl, IMPL_OFFSET>,
            SetCursorProperties: SetCursorProperties::<Impl, IMPL_OFFSET>,
            SetCursorPosition: SetCursorPosition::<Impl, IMPL_OFFSET>,
            ShowCursor: ShowCursor::<Impl, IMPL_OFFSET>,
            CreateAdditionalSwapChain: CreateAdditionalSwapChain::<Impl, IMPL_OFFSET>,
            GetSwapChain: GetSwapChain::<Impl, IMPL_OFFSET>,
            GetNumberOfSwapChains: GetNumberOfSwapChains::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Present: Present::<Impl, IMPL_OFFSET>,
            GetBackBuffer: GetBackBuffer::<Impl, IMPL_OFFSET>,
            GetRasterStatus: GetRasterStatus::<Impl, IMPL_OFFSET>,
            SetDialogBoxMode: SetDialogBoxMode::<Impl, IMPL_OFFSET>,
            SetGammaRamp: SetGammaRamp::<Impl, IMPL_OFFSET>,
            GetGammaRamp: GetGammaRamp::<Impl, IMPL_OFFSET>,
            CreateTexture: CreateTexture::<Impl, IMPL_OFFSET>,
            CreateVolumeTexture: CreateVolumeTexture::<Impl, IMPL_OFFSET>,
            CreateCubeTexture: CreateCubeTexture::<Impl, IMPL_OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Impl, IMPL_OFFSET>,
            CreateIndexBuffer: CreateIndexBuffer::<Impl, IMPL_OFFSET>,
            CreateRenderTarget: CreateRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDepthStencilSurface: CreateDepthStencilSurface::<Impl, IMPL_OFFSET>,
            UpdateSurface: UpdateSurface::<Impl, IMPL_OFFSET>,
            UpdateTexture: UpdateTexture::<Impl, IMPL_OFFSET>,
            GetRenderTargetData: GetRenderTargetData::<Impl, IMPL_OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Impl, IMPL_OFFSET>,
            StretchRect: StretchRect::<Impl, IMPL_OFFSET>,
            ColorFill: ColorFill::<Impl, IMPL_OFFSET>,
            CreateOffscreenPlainSurface: CreateOffscreenPlainSurface::<Impl, IMPL_OFFSET>,
            SetRenderTarget: SetRenderTarget::<Impl, IMPL_OFFSET>,
            GetRenderTarget: GetRenderTarget::<Impl, IMPL_OFFSET>,
            SetDepthStencilSurface: SetDepthStencilSurface::<Impl, IMPL_OFFSET>,
            GetDepthStencilSurface: GetDepthStencilSurface::<Impl, IMPL_OFFSET>,
            BeginScene: BeginScene::<Impl, IMPL_OFFSET>,
            EndScene: EndScene::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform: GetTransform::<Impl, IMPL_OFFSET>,
            MultiplyTransform: MultiplyTransform::<Impl, IMPL_OFFSET>,
            SetViewport: SetViewport::<Impl, IMPL_OFFSET>,
            GetViewport: GetViewport::<Impl, IMPL_OFFSET>,
            SetMaterial: SetMaterial::<Impl, IMPL_OFFSET>,
            GetMaterial: GetMaterial::<Impl, IMPL_OFFSET>,
            SetLight: SetLight::<Impl, IMPL_OFFSET>,
            GetLight: GetLight::<Impl, IMPL_OFFSET>,
            LightEnable: LightEnable::<Impl, IMPL_OFFSET>,
            GetLightEnable: GetLightEnable::<Impl, IMPL_OFFSET>,
            SetClipPlane: SetClipPlane::<Impl, IMPL_OFFSET>,
            GetClipPlane: GetClipPlane::<Impl, IMPL_OFFSET>,
            SetRenderState: SetRenderState::<Impl, IMPL_OFFSET>,
            GetRenderState: GetRenderState::<Impl, IMPL_OFFSET>,
            CreateStateBlock: CreateStateBlock::<Impl, IMPL_OFFSET>,
            BeginStateBlock: BeginStateBlock::<Impl, IMPL_OFFSET>,
            EndStateBlock: EndStateBlock::<Impl, IMPL_OFFSET>,
            SetClipStatus: SetClipStatus::<Impl, IMPL_OFFSET>,
            GetClipStatus: GetClipStatus::<Impl, IMPL_OFFSET>,
            GetTexture: GetTexture::<Impl, IMPL_OFFSET>,
            SetTexture: SetTexture::<Impl, IMPL_OFFSET>,
            GetTextureStageState: GetTextureStageState::<Impl, IMPL_OFFSET>,
            SetTextureStageState: SetTextureStageState::<Impl, IMPL_OFFSET>,
            GetSamplerState: GetSamplerState::<Impl, IMPL_OFFSET>,
            SetSamplerState: SetSamplerState::<Impl, IMPL_OFFSET>,
            ValidateDevice: ValidateDevice::<Impl, IMPL_OFFSET>,
            SetPaletteEntries: SetPaletteEntries::<Impl, IMPL_OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Impl, IMPL_OFFSET>,
            SetCurrentTexturePalette: SetCurrentTexturePalette::<Impl, IMPL_OFFSET>,
            GetCurrentTexturePalette: GetCurrentTexturePalette::<Impl, IMPL_OFFSET>,
            SetScissorRect: SetScissorRect::<Impl, IMPL_OFFSET>,
            GetScissorRect: GetScissorRect::<Impl, IMPL_OFFSET>,
            SetSoftwareVertexProcessing: SetSoftwareVertexProcessing::<Impl, IMPL_OFFSET>,
            GetSoftwareVertexProcessing: GetSoftwareVertexProcessing::<Impl, IMPL_OFFSET>,
            SetNPatchMode: SetNPatchMode::<Impl, IMPL_OFFSET>,
            GetNPatchMode: GetNPatchMode::<Impl, IMPL_OFFSET>,
            DrawPrimitive: DrawPrimitive::<Impl, IMPL_OFFSET>,
            DrawIndexedPrimitive: DrawIndexedPrimitive::<Impl, IMPL_OFFSET>,
            DrawPrimitiveUP: DrawPrimitiveUP::<Impl, IMPL_OFFSET>,
            DrawIndexedPrimitiveUP: DrawIndexedPrimitiveUP::<Impl, IMPL_OFFSET>,
            ProcessVertices: ProcessVertices::<Impl, IMPL_OFFSET>,
            CreateVertexDeclaration: CreateVertexDeclaration::<Impl, IMPL_OFFSET>,
            SetVertexDeclaration: SetVertexDeclaration::<Impl, IMPL_OFFSET>,
            GetVertexDeclaration: GetVertexDeclaration::<Impl, IMPL_OFFSET>,
            SetFVF: SetFVF::<Impl, IMPL_OFFSET>,
            GetFVF: GetFVF::<Impl, IMPL_OFFSET>,
            CreateVertexShader: CreateVertexShader::<Impl, IMPL_OFFSET>,
            SetVertexShader: SetVertexShader::<Impl, IMPL_OFFSET>,
            GetVertexShader: GetVertexShader::<Impl, IMPL_OFFSET>,
            SetVertexShaderConstantF: SetVertexShaderConstantF::<Impl, IMPL_OFFSET>,
            GetVertexShaderConstantF: GetVertexShaderConstantF::<Impl, IMPL_OFFSET>,
            SetVertexShaderConstantI: SetVertexShaderConstantI::<Impl, IMPL_OFFSET>,
            GetVertexShaderConstantI: GetVertexShaderConstantI::<Impl, IMPL_OFFSET>,
            SetVertexShaderConstantB: SetVertexShaderConstantB::<Impl, IMPL_OFFSET>,
            GetVertexShaderConstantB: GetVertexShaderConstantB::<Impl, IMPL_OFFSET>,
            SetStreamSource: SetStreamSource::<Impl, IMPL_OFFSET>,
            GetStreamSource: GetStreamSource::<Impl, IMPL_OFFSET>,
            SetStreamSourceFreq: SetStreamSourceFreq::<Impl, IMPL_OFFSET>,
            GetStreamSourceFreq: GetStreamSourceFreq::<Impl, IMPL_OFFSET>,
            SetIndices: SetIndices::<Impl, IMPL_OFFSET>,
            GetIndices: GetIndices::<Impl, IMPL_OFFSET>,
            CreatePixelShader: CreatePixelShader::<Impl, IMPL_OFFSET>,
            SetPixelShader: SetPixelShader::<Impl, IMPL_OFFSET>,
            GetPixelShader: GetPixelShader::<Impl, IMPL_OFFSET>,
            SetPixelShaderConstantF: SetPixelShaderConstantF::<Impl, IMPL_OFFSET>,
            GetPixelShaderConstantF: GetPixelShaderConstantF::<Impl, IMPL_OFFSET>,
            SetPixelShaderConstantI: SetPixelShaderConstantI::<Impl, IMPL_OFFSET>,
            GetPixelShaderConstantI: GetPixelShaderConstantI::<Impl, IMPL_OFFSET>,
            SetPixelShaderConstantB: SetPixelShaderConstantB::<Impl, IMPL_OFFSET>,
            GetPixelShaderConstantB: GetPixelShaderConstantB::<Impl, IMPL_OFFSET>,
            DrawRectPatch: DrawRectPatch::<Impl, IMPL_OFFSET>,
            DrawTriPatch: DrawTriPatch::<Impl, IMPL_OFFSET>,
            DeletePatch: DeletePatch::<Impl, IMPL_OFFSET>,
            CreateQuery: CreateQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9ExImpl: Sized + IDirect3DDevice9Impl {
    fn SetConvolutionMonoKernel();
    fn ComposeRects();
    fn PresentEx();
    fn GetGPUThreadPriority();
    fn SetGPUThreadPriority();
    fn WaitForVBlank();
    fn CheckResourceResidency();
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
    fn CheckDeviceState();
    fn CreateRenderTargetEx();
    fn CreateOffscreenPlainSurfaceEx();
    fn CreateDepthStencilSurfaceEx();
    fn ResetEx();
    fn GetDisplayModeEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9ExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9ExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DDevice9ExVtbl {
        unsafe extern "system" fn SetConvolutionMonoKernel<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComposeRects<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr, psrcrectdescs: ::windows::core::RawPtr, numrects: u32, pdstrectdescs: ::windows::core::RawPtr, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PresentEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForVBlank<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckResourceResidency<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcearray: *mut ::windows::core::RawPtr, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDeviceState<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRenderTargetEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayModeEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DDevice9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetConvolutionMonoKernel: SetConvolutionMonoKernel::<Impl, IMPL_OFFSET>,
            ComposeRects: ComposeRects::<Impl, IMPL_OFFSET>,
            PresentEx: PresentEx::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            WaitForVBlank: WaitForVBlank::<Impl, IMPL_OFFSET>,
            CheckResourceResidency: CheckResourceResidency::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            CheckDeviceState: CheckDeviceState::<Impl, IMPL_OFFSET>,
            CreateRenderTargetEx: CreateRenderTargetEx::<Impl, IMPL_OFFSET>,
            CreateOffscreenPlainSurfaceEx: CreateOffscreenPlainSurfaceEx::<Impl, IMPL_OFFSET>,
            CreateDepthStencilSurfaceEx: CreateDepthStencilSurfaceEx::<Impl, IMPL_OFFSET>,
            ResetEx: ResetEx::<Impl, IMPL_OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice9Ex as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DIndexBuffer9Impl: Sized + IDirect3DResource9Impl {
    fn Lock();
    fn Unlock();
    fn GetDesc();
}
impl IDirect3DIndexBuffer9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DIndexBuffer9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DIndexBuffer9Vtbl {
        unsafe extern "system" fn Lock<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DResource9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Lock: Lock::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DPixelShader9Impl: Sized {
    fn GetDevice();
    fn GetFunction();
}
impl IDirect3DPixelShader9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DPixelShader9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DPixelShader9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DPixelShader9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFunction<Impl: IDirect3DPixelShader9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetFunction: GetFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DQuery9Impl: Sized {
    fn GetDevice();
    fn GetType();
    fn GetDataSize();
    fn Issue();
    fn GetData();
}
impl IDirect3DQuery9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DQuery9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DQuery9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DQuery9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IDirect3DQuery9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DQUERYTYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataSize<Impl: IDirect3DQuery9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Issue<Impl: IDirect3DQuery9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwissueflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetData<Impl: IDirect3DQuery9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetDataSize: GetDataSize::<Impl, IMPL_OFFSET>,
            Issue: Issue::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DQuery9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DResource9Impl: Sized {
    fn GetDevice();
    fn SetPrivateData();
    fn GetPrivateData();
    fn FreePrivateData();
    fn SetPriority();
    fn GetPriority();
    fn PreLoad();
    fn GetType();
}
impl IDirect3DResource9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DResource9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DResource9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prioritynew: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPriority<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreLoad<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IDirect3DResource9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3DRESOURCETYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            FreePrivateData: FreePrivateData::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority: GetPriority::<Impl, IMPL_OFFSET>,
            PreLoad: PreLoad::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DResource9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DStateBlock9Impl: Sized {
    fn GetDevice();
    fn Capture();
    fn Apply();
}
impl IDirect3DStateBlock9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DStateBlock9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DStateBlock9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DStateBlock9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Capture<Impl: IDirect3DStateBlock9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: IDirect3DStateBlock9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            Capture: Capture::<Impl, IMPL_OFFSET>,
            Apply: Apply::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSurface9Impl: Sized + IDirect3DResource9Impl {
    fn GetContainer();
    fn GetDesc();
    fn LockRect();
    fn UnlockRect();
    fn GetDC();
    fn ReleaseDC();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSurface9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DSurface9Vtbl {
        unsafe extern "system" fn GetContainer<Impl: IDirect3DSurface9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DSurface9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockRect<Impl: IDirect3DSurface9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockRect<Impl: IDirect3DSurface9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IDirect3DSurface9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirect3DSurface9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DResource9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetContainer: GetContainer::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            LockRect: LockRect::<Impl, IMPL_OFFSET>,
            UnlockRect: UnlockRect::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSurface9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9Impl: Sized {
    fn Present();
    fn GetFrontBufferData();
    fn GetBackBuffer();
    fn GetRasterStatus();
    fn GetDisplayMode();
    fn GetDevice();
    fn GetPresentParameters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSwapChain9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DSwapChain9Vtbl {
        unsafe extern "system" fn Present<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrontBufferData<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackBuffer<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRasterStatus<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevice<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentParameters<Impl: IDirect3DSwapChain9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Present: Present::<Impl, IMPL_OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Impl, IMPL_OFFSET>,
            GetBackBuffer: GetBackBuffer::<Impl, IMPL_OFFSET>,
            GetRasterStatus: GetRasterStatus::<Impl, IMPL_OFFSET>,
            GetDisplayMode: GetDisplayMode::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetPresentParameters: GetPresentParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9ExImpl: Sized + IDirect3DSwapChain9Impl {
    fn GetLastPresentCount();
    fn GetPresentStats();
    fn GetDisplayModeEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DSwapChain9ExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSwapChain9ExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DSwapChain9ExVtbl {
        unsafe extern "system" fn GetLastPresentCount<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentStats<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayModeEx<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DSwapChain9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLastPresentCount: GetLastPresentCount::<Impl, IMPL_OFFSET>,
            GetPresentStats: GetPresentStats::<Impl, IMPL_OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DTexture9Impl: Sized + IDirect3DResource9Impl + IDirect3DBaseTexture9Impl {
    fn GetLevelDesc();
    fn GetSurfaceLevel();
    fn LockRect();
    fn UnlockRect();
    fn AddDirtyRect();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirect3DTexture9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DTexture9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DTexture9Vtbl {
        unsafe extern "system" fn GetLevelDesc<Impl: IDirect3DTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceLevel<Impl: IDirect3DTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, ppsurfacelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockRect<Impl: IDirect3DTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockRect<Impl: IDirect3DTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDirtyRect<Impl: IDirect3DTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DBaseTexture9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Impl, IMPL_OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Impl, IMPL_OFFSET>,
            LockRect: LockRect::<Impl, IMPL_OFFSET>,
            UnlockRect: UnlockRect::<Impl, IMPL_OFFSET>,
            AddDirtyRect: AddDirtyRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DTexture9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVertexBuffer9Impl: Sized + IDirect3DResource9Impl {
    fn Lock();
    fn Unlock();
    fn GetDesc();
}
impl IDirect3DVertexBuffer9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexBuffer9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DVertexBuffer9Vtbl {
        unsafe extern "system" fn Lock<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DResource9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Lock: Lock::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVertexDeclaration9Impl: Sized {
    fn GetDevice();
    fn GetDeclaration();
}
impl IDirect3DVertexDeclaration9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexDeclaration9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DVertexDeclaration9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DVertexDeclaration9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeclaration<Impl: IDirect3DVertexDeclaration9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetDeclaration: GetDeclaration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVertexShader9Impl: Sized {
    fn GetDevice();
    fn GetFunction();
}
impl IDirect3DVertexShader9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVertexShader9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DVertexShader9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DVertexShader9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFunction<Impl: IDirect3DVertexShader9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetFunction: GetFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVolume9Impl: Sized {
    fn GetDevice();
    fn SetPrivateData();
    fn GetPrivateData();
    fn FreePrivateData();
    fn GetContainer();
    fn GetDesc();
    fn LockBox();
    fn UnlockBox();
}
impl IDirect3DVolume9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolume9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DVolume9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainer<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockBox<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockBox<Impl: IDirect3DVolume9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            FreePrivateData: FreePrivateData::<Impl, IMPL_OFFSET>,
            GetContainer: GetContainer::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            LockBox: LockBox::<Impl, IMPL_OFFSET>,
            UnlockBox: UnlockBox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVolume9 as ::windows::core::Interface>::IID
    }
}
pub trait IDirect3DVolumeTexture9Impl: Sized + IDirect3DResource9Impl + IDirect3DBaseTexture9Impl {
    fn GetLevelDesc();
    fn GetVolumeLevel();
    fn LockBox();
    fn UnlockBox();
    fn AddDirtyBox();
}
impl IDirect3DVolumeTexture9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DVolumeTexture9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DVolumeTexture9Vtbl {
        unsafe extern "system" fn GetLevelDesc<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolumeLevel<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, ppvolumelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockBox<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockBox<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDirtyBox<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybox: *const D3DBOX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDirect3DBaseTexture9Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Impl, IMPL_OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Impl, IMPL_OFFSET>,
            LockBox: LockBox::<Impl, IMPL_OFFSET>,
            UnlockBox: UnlockBox::<Impl, IMPL_OFFSET>,
            AddDirtyBox: AddDirtyBox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DVolumeTexture9 as ::windows::core::Interface>::IID
    }
}

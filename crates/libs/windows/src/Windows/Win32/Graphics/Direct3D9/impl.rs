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
pub trait IDirect3D9ExImpl: Sized + IDirect3D9Impl {
    fn GetAdapterModeCountEx();
    fn EnumAdapterModesEx();
    fn GetAdapterDisplayModeEx();
    fn CreateDeviceEx();
    fn GetAdapterLUID();
}
pub trait IDirect3DBaseTexture9Impl: Sized + IDirect3DResource9Impl {
    fn SetLOD();
    fn GetLOD();
    fn GetLevelCount();
    fn SetAutoGenFilterType();
    fn GetAutoGenFilterType();
    fn GenerateMipSubLevels();
}
pub trait IDirect3DCubeTexture9Impl: Sized + IDirect3DBaseTexture9Impl + IDirect3DResource9Impl {
    fn GetLevelDesc();
    fn GetCubeMapSurface();
    fn LockRect();
    fn UnlockRect();
    fn AddDirtyRect();
}
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
pub trait IDirect3DIndexBuffer9Impl: Sized + IDirect3DResource9Impl {
    fn Lock();
    fn Unlock();
    fn GetDesc();
}
pub trait IDirect3DPixelShader9Impl: Sized {
    fn GetDevice();
    fn GetFunction();
}
pub trait IDirect3DQuery9Impl: Sized {
    fn GetDevice();
    fn GetType();
    fn GetDataSize();
    fn Issue();
    fn GetData();
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
pub trait IDirect3DStateBlock9Impl: Sized {
    fn GetDevice();
    fn Capture();
    fn Apply();
}
pub trait IDirect3DSurface9Impl: Sized + IDirect3DResource9Impl {
    fn GetContainer();
    fn GetDesc();
    fn LockRect();
    fn UnlockRect();
    fn GetDC();
    fn ReleaseDC();
}
pub trait IDirect3DSwapChain9Impl: Sized {
    fn Present();
    fn GetFrontBufferData();
    fn GetBackBuffer();
    fn GetRasterStatus();
    fn GetDisplayMode();
    fn GetDevice();
    fn GetPresentParameters();
}
pub trait IDirect3DSwapChain9ExImpl: Sized + IDirect3DSwapChain9Impl {
    fn GetLastPresentCount();
    fn GetPresentStats();
    fn GetDisplayModeEx();
}
pub trait IDirect3DTexture9Impl: Sized + IDirect3DBaseTexture9Impl + IDirect3DResource9Impl {
    fn GetLevelDesc();
    fn GetSurfaceLevel();
    fn LockRect();
    fn UnlockRect();
    fn AddDirtyRect();
}
pub trait IDirect3DVertexBuffer9Impl: Sized + IDirect3DResource9Impl {
    fn Lock();
    fn Unlock();
    fn GetDesc();
}
pub trait IDirect3DVertexDeclaration9Impl: Sized {
    fn GetDevice();
    fn GetDeclaration();
}
pub trait IDirect3DVertexShader9Impl: Sized {
    fn GetDevice();
    fn GetFunction();
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
pub trait IDirect3DVolumeTexture9Impl: Sized + IDirect3DBaseTexture9Impl + IDirect3DResource9Impl {
    fn GetLevelDesc();
    fn GetVolumeLevel();
    fn LockBox();
    fn UnlockBox();
    fn AddDirtyBox();
}

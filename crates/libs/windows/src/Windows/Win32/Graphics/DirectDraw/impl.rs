#[cfg(feature = "Win32_Foundation")]
pub trait IDDVideoPortContainerImpl: Sized {
    fn CreateVideoPort();
    fn EnumVideoPorts();
    fn GetVideoPortConnectInfo();
    fn QueryVideoPortStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IDDVideoPortContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDDVideoPortContainerVtbl {
        unsafe extern "system" fn CreateVideoPort<Impl: IDDVideoPortContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumVideoPorts<Impl: IDDVideoPortContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Impl: IDDVideoPortContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryVideoPortStatus<Impl: IDDVideoPortContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateVideoPort: CreateVideoPort::<Impl, IMPL_OFFSET>,
            EnumVideoPorts: EnumVideoPorts::<Impl, IMPL_OFFSET>,
            GetVideoPortConnectInfo: GetVideoPortConnectInfo::<Impl, IMPL_OFFSET>,
            QueryVideoPortStatus: QueryVideoPortStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDDVideoPortContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawImpl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawVtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDrawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Compact: Compact::<Impl, IMPL_OFFSET>,
            CreateClipper: CreateClipper::<Impl, IMPL_OFFSET>,
            CreatePalette: CreatePalette::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            DuplicateSurface: DuplicateSurface::<Impl, IMPL_OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Impl, IMPL_OFFSET>,
            EnumSurfaces: EnumSurfaces::<Impl, IMPL_OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetDisplayMode: GetDisplayMode::<Impl, IMPL_OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Impl, IMPL_OFFSET>,
            GetGDISurface: GetGDISurface::<Impl, IMPL_OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Impl, IMPL_OFFSET>,
            GetScanLine: GetScanLine::<Impl, IMPL_OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SetDisplayMode: SetDisplayMode::<Impl, IMPL_OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDraw as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw2Impl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
    fn GetAvailableVidMem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw2Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Compact: Compact::<Impl, IMPL_OFFSET>,
            CreateClipper: CreateClipper::<Impl, IMPL_OFFSET>,
            CreatePalette: CreatePalette::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            DuplicateSurface: DuplicateSurface::<Impl, IMPL_OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Impl, IMPL_OFFSET>,
            EnumSurfaces: EnumSurfaces::<Impl, IMPL_OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetDisplayMode: GetDisplayMode::<Impl, IMPL_OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Impl, IMPL_OFFSET>,
            GetGDISurface: GetGDISurface::<Impl, IMPL_OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Impl, IMPL_OFFSET>,
            GetScanLine: GetScanLine::<Impl, IMPL_OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SetDisplayMode: SetDisplayMode::<Impl, IMPL_OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Impl, IMPL_OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDraw2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw4Impl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
    fn GetAvailableVidMem();
    fn GetSurfaceFromDC();
    fn RestoreAllSurfaces();
    fn TestCooperativeLevel();
    fn GetDeviceIdentifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw4Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreAllSurfaces<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Impl: IDirectDraw4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Compact: Compact::<Impl, IMPL_OFFSET>,
            CreateClipper: CreateClipper::<Impl, IMPL_OFFSET>,
            CreatePalette: CreatePalette::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            DuplicateSurface: DuplicateSurface::<Impl, IMPL_OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Impl, IMPL_OFFSET>,
            EnumSurfaces: EnumSurfaces::<Impl, IMPL_OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetDisplayMode: GetDisplayMode::<Impl, IMPL_OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Impl, IMPL_OFFSET>,
            GetGDISurface: GetGDISurface::<Impl, IMPL_OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Impl, IMPL_OFFSET>,
            GetScanLine: GetScanLine::<Impl, IMPL_OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SetDisplayMode: SetDisplayMode::<Impl, IMPL_OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Impl, IMPL_OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Impl, IMPL_OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Impl, IMPL_OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Impl, IMPL_OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDraw4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw7Impl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
    fn GetAvailableVidMem();
    fn GetSurfaceFromDC();
    fn RestoreAllSurfaces();
    fn TestCooperativeLevel();
    fn GetDeviceIdentifier();
    fn StartModeTest();
    fn EvaluateMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw7Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreAllSurfaces<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartModeTest<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EvaluateMode<Impl: IDirectDraw7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Compact: Compact::<Impl, IMPL_OFFSET>,
            CreateClipper: CreateClipper::<Impl, IMPL_OFFSET>,
            CreatePalette: CreatePalette::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            DuplicateSurface: DuplicateSurface::<Impl, IMPL_OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Impl, IMPL_OFFSET>,
            EnumSurfaces: EnumSurfaces::<Impl, IMPL_OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetDisplayMode: GetDisplayMode::<Impl, IMPL_OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Impl, IMPL_OFFSET>,
            GetGDISurface: GetGDISurface::<Impl, IMPL_OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Impl, IMPL_OFFSET>,
            GetScanLine: GetScanLine::<Impl, IMPL_OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SetDisplayMode: SetDisplayMode::<Impl, IMPL_OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Impl, IMPL_OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Impl, IMPL_OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Impl, IMPL_OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Impl, IMPL_OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Impl, IMPL_OFFSET>,
            StartModeTest: StartModeTest::<Impl, IMPL_OFFSET>,
            EvaluateMode: EvaluateMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDraw7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawClipperImpl: Sized {
    fn GetClipList();
    fn GetHWnd();
    fn Initialize();
    fn IsClipListChanged();
    fn SetClipList();
    fn SetHWnd();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawClipperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawClipperVtbl {
        unsafe extern "system" fn GetClipList<Impl: IDirectDrawClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHWnd<Impl: IDirectDrawClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsClipListChanged<Impl: IDirectDrawClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipList<Impl: IDirectDrawClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHWnd<Impl: IDirectDrawClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClipList: GetClipList::<Impl, IMPL_OFFSET>,
            GetHWnd: GetHWnd::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsClipListChanged: IsClipListChanged::<Impl, IMPL_OFFSET>,
            SetClipList: SetClipList::<Impl, IMPL_OFFSET>,
            SetHWnd: SetHWnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawClipper as ::windows::core::Interface>::IID
    }
}
pub trait IDirectDrawColorControlImpl: Sized {
    fn GetColorControls();
    fn SetColorControls();
}
impl IDirectDrawColorControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawColorControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawColorControlVtbl {
        unsafe extern "system" fn GetColorControls<Impl: IDirectDrawColorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorControls<Impl: IDirectDrawColorControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetColorControls: GetColorControls::<Impl, IMPL_OFFSET>,
            SetColorControls: SetColorControls::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawColorControl as ::windows::core::Interface>::IID
    }
}
pub trait IDirectDrawGammaControlImpl: Sized {
    fn GetGammaRamp();
    fn SetGammaRamp();
}
impl IDirectDrawGammaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawGammaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawGammaControlVtbl {
        unsafe extern "system" fn GetGammaRamp<Impl: IDirectDrawGammaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGammaRamp<Impl: IDirectDrawGammaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGammaRamp: GetGammaRamp::<Impl, IMPL_OFFSET>,
            SetGammaRamp: SetGammaRamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawGammaControl as ::windows::core::Interface>::IID
    }
}
pub trait IDirectDrawKernelImpl: Sized {
    fn GetCaps();
    fn GetKernelHandle();
    fn ReleaseKernelHandle();
}
impl IDirectDrawKernelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawKernelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawKernelVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawKernelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKernelHandle<Impl: IDirectDrawKernelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Impl: IDirectDrawKernelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetKernelHandle: GetKernelHandle::<Impl, IMPL_OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawKernel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawPaletteImpl: Sized {
    fn GetCaps();
    fn GetEntries();
    fn Initialize();
    fn SetEntries();
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawPaletteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPaletteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawPaletteVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntries<Impl: IDirectDrawPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEntries<Impl: IDirectDrawPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetEntries: GetEntries::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SetEntries: SetEntries::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawPalette as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurfaceImpl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurfaceVtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Impl, IMPL_OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Impl, IMPL_OFFSET>,
            Blt: Blt::<Impl, IMPL_OFFSET>,
            BltBatch: BltBatch::<Impl, IMPL_OFFSET>,
            BltFast: BltFast::<Impl, IMPL_OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Impl, IMPL_OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Impl, IMPL_OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Impl, IMPL_OFFSET>,
            Flip: Flip::<Impl, IMPL_OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Impl, IMPL_OFFSET>,
            GetBltStatus: GetBltStatus::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetClipper: GetClipper::<Impl, IMPL_OFFSET>,
            GetColorKey: GetColorKey::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            GetFlipStatus: GetFlipStatus::<Impl, IMPL_OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Impl, IMPL_OFFSET>,
            GetPalette: GetPalette::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsLost: IsLost::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            SetClipper: SetClipper::<Impl, IMPL_OFFSET>,
            SetColorKey: SetColorKey::<Impl, IMPL_OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            UpdateOverlay: UpdateOverlay::<Impl, IMPL_OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Impl, IMPL_OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface2Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface2Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Impl, IMPL_OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Impl, IMPL_OFFSET>,
            Blt: Blt::<Impl, IMPL_OFFSET>,
            BltBatch: BltBatch::<Impl, IMPL_OFFSET>,
            BltFast: BltFast::<Impl, IMPL_OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Impl, IMPL_OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Impl, IMPL_OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Impl, IMPL_OFFSET>,
            Flip: Flip::<Impl, IMPL_OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Impl, IMPL_OFFSET>,
            GetBltStatus: GetBltStatus::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetClipper: GetClipper::<Impl, IMPL_OFFSET>,
            GetColorKey: GetColorKey::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            GetFlipStatus: GetFlipStatus::<Impl, IMPL_OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Impl, IMPL_OFFSET>,
            GetPalette: GetPalette::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsLost: IsLost::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            SetClipper: SetClipper::<Impl, IMPL_OFFSET>,
            SetColorKey: SetColorKey::<Impl, IMPL_OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            UpdateOverlay: UpdateOverlay::<Impl, IMPL_OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Impl, IMPL_OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Impl, IMPL_OFFSET>,
            GetDDInterface: GetDDInterface::<Impl, IMPL_OFFSET>,
            PageLock: PageLock::<Impl, IMPL_OFFSET>,
            PageUnlock: PageUnlock::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawSurface2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface3Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
    fn SetSurfaceDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface3Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Impl, IMPL_OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Impl, IMPL_OFFSET>,
            Blt: Blt::<Impl, IMPL_OFFSET>,
            BltBatch: BltBatch::<Impl, IMPL_OFFSET>,
            BltFast: BltFast::<Impl, IMPL_OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Impl, IMPL_OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Impl, IMPL_OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Impl, IMPL_OFFSET>,
            Flip: Flip::<Impl, IMPL_OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Impl, IMPL_OFFSET>,
            GetBltStatus: GetBltStatus::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetClipper: GetClipper::<Impl, IMPL_OFFSET>,
            GetColorKey: GetColorKey::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            GetFlipStatus: GetFlipStatus::<Impl, IMPL_OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Impl, IMPL_OFFSET>,
            GetPalette: GetPalette::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsLost: IsLost::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            SetClipper: SetClipper::<Impl, IMPL_OFFSET>,
            SetColorKey: SetColorKey::<Impl, IMPL_OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            UpdateOverlay: UpdateOverlay::<Impl, IMPL_OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Impl, IMPL_OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Impl, IMPL_OFFSET>,
            GetDDInterface: GetDDInterface::<Impl, IMPL_OFFSET>,
            PageLock: PageLock::<Impl, IMPL_OFFSET>,
            PageUnlock: PageUnlock::<Impl, IMPL_OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawSurface3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface4Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
    fn SetSurfaceDesc();
    fn SetPrivateData();
    fn GetPrivateData();
    fn FreePrivateData();
    fn GetUniquenessValue();
    fn ChangeUniquenessValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface4Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUniquenessValue<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Impl: IDirectDrawSurface4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Impl, IMPL_OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Impl, IMPL_OFFSET>,
            Blt: Blt::<Impl, IMPL_OFFSET>,
            BltBatch: BltBatch::<Impl, IMPL_OFFSET>,
            BltFast: BltFast::<Impl, IMPL_OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Impl, IMPL_OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Impl, IMPL_OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Impl, IMPL_OFFSET>,
            Flip: Flip::<Impl, IMPL_OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Impl, IMPL_OFFSET>,
            GetBltStatus: GetBltStatus::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetClipper: GetClipper::<Impl, IMPL_OFFSET>,
            GetColorKey: GetColorKey::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            GetFlipStatus: GetFlipStatus::<Impl, IMPL_OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Impl, IMPL_OFFSET>,
            GetPalette: GetPalette::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsLost: IsLost::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            SetClipper: SetClipper::<Impl, IMPL_OFFSET>,
            SetColorKey: SetColorKey::<Impl, IMPL_OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            UpdateOverlay: UpdateOverlay::<Impl, IMPL_OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Impl, IMPL_OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Impl, IMPL_OFFSET>,
            GetDDInterface: GetDDInterface::<Impl, IMPL_OFFSET>,
            PageLock: PageLock::<Impl, IMPL_OFFSET>,
            PageUnlock: PageUnlock::<Impl, IMPL_OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            FreePrivateData: FreePrivateData::<Impl, IMPL_OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Impl, IMPL_OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawSurface4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface7Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
    fn SetSurfaceDesc();
    fn SetPrivateData();
    fn GetPrivateData();
    fn FreePrivateData();
    fn GetUniquenessValue();
    fn ChangeUniquenessValue();
    fn SetPriority();
    fn GetPriority();
    fn SetLOD();
    fn GetLOD();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface7Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUniquenessValue<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPriority<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLOD<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLOD<Impl: IDirectDrawSurface7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Impl, IMPL_OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Impl, IMPL_OFFSET>,
            Blt: Blt::<Impl, IMPL_OFFSET>,
            BltBatch: BltBatch::<Impl, IMPL_OFFSET>,
            BltFast: BltFast::<Impl, IMPL_OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Impl, IMPL_OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Impl, IMPL_OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Impl, IMPL_OFFSET>,
            Flip: Flip::<Impl, IMPL_OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Impl, IMPL_OFFSET>,
            GetBltStatus: GetBltStatus::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetClipper: GetClipper::<Impl, IMPL_OFFSET>,
            GetColorKey: GetColorKey::<Impl, IMPL_OFFSET>,
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            GetFlipStatus: GetFlipStatus::<Impl, IMPL_OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Impl, IMPL_OFFSET>,
            GetPalette: GetPalette::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsLost: IsLost::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            SetClipper: SetClipper::<Impl, IMPL_OFFSET>,
            SetColorKey: SetColorKey::<Impl, IMPL_OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            UpdateOverlay: UpdateOverlay::<Impl, IMPL_OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Impl, IMPL_OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Impl, IMPL_OFFSET>,
            GetDDInterface: GetDDInterface::<Impl, IMPL_OFFSET>,
            PageLock: PageLock::<Impl, IMPL_OFFSET>,
            PageUnlock: PageUnlock::<Impl, IMPL_OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            FreePrivateData: FreePrivateData::<Impl, IMPL_OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Impl, IMPL_OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority: GetPriority::<Impl, IMPL_OFFSET>,
            SetLOD: SetLOD::<Impl, IMPL_OFFSET>,
            GetLOD: GetLOD::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawSurface7 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectDrawSurfaceKernelImpl: Sized {
    fn GetKernelHandle();
    fn ReleaseKernelHandle();
}
impl IDirectDrawSurfaceKernelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurfaceKernelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurfaceKernelVtbl {
        unsafe extern "system" fn GetKernelHandle<Impl: IDirectDrawSurfaceKernelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Impl: IDirectDrawSurfaceKernelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetKernelHandle: GetKernelHandle::<Impl, IMPL_OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawSurfaceKernel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectDrawVideoPortImpl: Sized {
    fn Flip();
    fn GetBandwidthInfo();
    fn GetColorControls();
    fn GetInputFormats();
    fn GetOutputFormats();
    fn GetFieldPolarity();
    fn GetVideoLine();
    fn GetVideoSignalStatus();
    fn SetColorControls();
    fn SetTargetSurface();
    fn StartVideo();
    fn StopVideo();
    fn UpdateVideo();
    fn WaitForSync();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectDrawVideoPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawVideoPortVtbl {
        unsafe extern "system" fn Flip<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBandwidthInfo<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorControls<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputFormats<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFormats<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFieldPolarity<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoLine<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoSignalStatus<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorControls<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetSurface<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartVideo<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopVideo<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateVideo<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForSync<Impl: IDirectDrawVideoPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Flip: Flip::<Impl, IMPL_OFFSET>,
            GetBandwidthInfo: GetBandwidthInfo::<Impl, IMPL_OFFSET>,
            GetColorControls: GetColorControls::<Impl, IMPL_OFFSET>,
            GetInputFormats: GetInputFormats::<Impl, IMPL_OFFSET>,
            GetOutputFormats: GetOutputFormats::<Impl, IMPL_OFFSET>,
            GetFieldPolarity: GetFieldPolarity::<Impl, IMPL_OFFSET>,
            GetVideoLine: GetVideoLine::<Impl, IMPL_OFFSET>,
            GetVideoSignalStatus: GetVideoSignalStatus::<Impl, IMPL_OFFSET>,
            SetColorControls: SetColorControls::<Impl, IMPL_OFFSET>,
            SetTargetSurface: SetTargetSurface::<Impl, IMPL_OFFSET>,
            StartVideo: StartVideo::<Impl, IMPL_OFFSET>,
            StopVideo: StopVideo::<Impl, IMPL_OFFSET>,
            UpdateVideo: UpdateVideo::<Impl, IMPL_OFFSET>,
            WaitForSync: WaitForSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawVideoPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectDrawVideoPortNotifyImpl: Sized {
    fn AcquireNotification();
    fn ReleaseNotification();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectDrawVideoPortNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPortNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawVideoPortNotifyVtbl {
        unsafe extern "system" fn AcquireNotification<Impl: IDirectDrawVideoPortNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseNotification<Impl: IDirectDrawVideoPortNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AcquireNotification: AcquireNotification::<Impl, IMPL_OFFSET>,
            ReleaseNotification: ReleaseNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawVideoPortNotify as ::windows::core::Interface>::IID
    }
}

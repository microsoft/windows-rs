#[cfg(feature = "Win32_Foundation")]
pub trait IDDVideoPortContainer_Impl: Sized {
    fn CreateVideoPort(&mut self, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::core::option::Option<IDirectDrawVideoPort>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumVideoPorts(&mut self, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMVIDEOCALLBACK) -> ::windows::core::Result<()>;
    fn GetVideoPortConnectInfo(&mut self, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::Result<()>;
    fn QueryVideoPortStatus(&mut self, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDDVideoPortContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDDVideoPortContainer_Vtbl {
        unsafe extern "system" fn CreateVideoPort<Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateVideoPort(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumVideoPorts<Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumVideoPorts(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVideoPortConnectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&pcinfo), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn QueryVideoPortStatus<Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryVideoPortStatus(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDraw_Impl: Sized {
    fn Compact(&mut self) -> ::windows::core::Result<()>;
    fn CreateClipper(&mut self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreatePalette(&mut self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreateSurface(&mut self, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DuplicateSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&mut self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMMODESCALLBACK) -> ::windows::core::Result<()>;
    fn EnumSurfaces(&mut self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn FlipToGDISurface(&mut self) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&mut self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn GetFourCCCodes(&mut self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()>;
    fn GetGDISurface(&mut self) -> ::windows::core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetScanLine(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetVerticalBlankStatus(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreDisplayMode(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SetDisplayMode(&mut self, param0: u32, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn WaitForVerticalBlank(&mut self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw_Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDraw2_Impl: Sized {
    fn Compact(&mut self) -> ::windows::core::Result<()>;
    fn CreateClipper(&mut self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreatePalette(&mut self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreateSurface(&mut self, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DuplicateSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&mut self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMMODESCALLBACK) -> ::windows::core::Result<()>;
    fn EnumSurfaces(&mut self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn FlipToGDISurface(&mut self) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&mut self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn GetFourCCCodes(&mut self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()>;
    fn GetGDISurface(&mut self) -> ::windows::core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetScanLine(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetVerticalBlankStatus(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreDisplayMode(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SetDisplayMode(&mut self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::Result<()>;
    fn WaitForVerticalBlank(&mut self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetAvailableVidMem(&mut self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw2_Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
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
pub trait IDirectDraw4_Impl: Sized {
    fn Compact(&mut self) -> ::windows::core::Result<()>;
    fn CreateClipper(&mut self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreatePalette(&mut self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreateSurface(&mut self, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface4>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DuplicateSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface4>) -> ::windows::core::Result<IDirectDrawSurface4>;
    fn EnumDisplayModes(&mut self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMMODESCALLBACK2) -> ::windows::core::Result<()>;
    fn EnumSurfaces(&mut self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMSURFACESCALLBACK2) -> ::windows::core::Result<()>;
    fn FlipToGDISurface(&mut self) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&mut self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>;
    fn GetFourCCCodes(&mut self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()>;
    fn GetGDISurface(&mut self) -> ::windows::core::Result<IDirectDrawSurface4>;
    fn GetMonitorFrequency(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetScanLine(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetVerticalBlankStatus(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreDisplayMode(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SetDisplayMode(&mut self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::Result<()>;
    fn WaitForVerticalBlank(&mut self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetAvailableVidMem(&mut self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::Result<()>;
    fn GetSurfaceFromDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<IDirectDrawSurface4>;
    fn RestoreAllSurfaces(&mut self) -> ::windows::core::Result<()>;
    fn TestCooperativeLevel(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceIdentifier(&mut self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw4_Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSurfaceFromDC(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreAllSurfaces().into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceIdentifier(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDraw7_Impl: Sized {
    fn Compact(&mut self) -> ::windows::core::Result<()>;
    fn CreateClipper(&mut self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreatePalette(&mut self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreateSurface(&mut self, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface7>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DuplicateSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface7>) -> ::windows::core::Result<IDirectDrawSurface7>;
    fn EnumDisplayModes(&mut self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMMODESCALLBACK2) -> ::windows::core::Result<()>;
    fn EnumSurfaces(&mut self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMSURFACESCALLBACK7) -> ::windows::core::Result<()>;
    fn FlipToGDISurface(&mut self) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()>;
    fn GetDisplayMode(&mut self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>;
    fn GetFourCCCodes(&mut self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()>;
    fn GetGDISurface(&mut self) -> ::windows::core::Result<IDirectDrawSurface7>;
    fn GetMonitorFrequency(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetScanLine(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetVerticalBlankStatus(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreDisplayMode(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SetDisplayMode(&mut self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::Result<()>;
    fn WaitForVerticalBlank(&mut self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetAvailableVidMem(&mut self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::Result<()>;
    fn GetSurfaceFromDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<IDirectDrawSurface7>;
    fn RestoreAllSurfaces(&mut self) -> ::windows::core::Result<()>;
    fn TestCooperativeLevel(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceIdentifier(&mut self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::Result<()>;
    fn StartModeTest(&mut self, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn EvaluateMode(&mut self, param0: u32, param1: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDraw7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDraw7_Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSurfaceFromDC(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreAllSurfaces().into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceIdentifier(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartModeTest<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartModeTest(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EvaluateMode<Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EvaluateMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDrawClipper_Impl: Sized {
    fn GetClipList(&mut self, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::Result<()>;
    fn GetHWnd(&mut self, param0: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: u32) -> ::windows::core::Result<()>;
    fn IsClipListChanged(&mut self, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClipList(&mut self, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::Result<()>;
    fn SetHWnd(&mut self, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawClipper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawClipper_Vtbl {
        unsafe extern "system" fn GetClipList<Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClipList(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetHWnd<Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHWnd(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsClipListChanged<Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsClipListChanged(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetClipList<Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipList(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetHWnd<Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHWnd(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDrawColorControl_Impl: Sized {
    fn GetColorControls(&mut self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()>;
    fn SetColorControls(&mut self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()>;
}
impl IDirectDrawColorControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawColorControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawColorControl_Vtbl {
        unsafe extern "system" fn GetColorControls<Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorControls(::core::mem::transmute_copy(&param0)).into()
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
pub trait IDirectDrawGammaControl_Impl: Sized {
    fn GetGammaRamp(&mut self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::Result<()>;
    fn SetGammaRamp(&mut self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::Result<()>;
}
impl IDirectDrawGammaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawGammaControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawGammaControl_Vtbl {
        unsafe extern "system" fn GetGammaRamp<Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGammaRamp(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGammaRamp(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDrawKernel_Impl: Sized {
    fn GetCaps(&mut self, param0: *mut DDKERNELCAPS) -> ::windows::core::Result<()>;
    fn GetKernelHandle(&mut self, param0: *mut usize) -> ::windows::core::Result<()>;
    fn ReleaseKernelHandle(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectDrawKernel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawKernel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawKernel_Vtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetKernelHandle<Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKernelHandle(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseKernelHandle().into()
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
pub trait IDirectDrawPalette_Impl: Sized {
    fn GetCaps(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetEntries(&mut self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
    fn SetEntries(&mut self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawPalette_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPalette_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawPalette_Vtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetEntries<Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEntries(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetEntries<Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntries(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
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
pub trait IDirectDrawSurface_Impl: Sized {
    fn AddAttachedSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<()>;
    fn AddOverlayDirtyRect(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Blt(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>;
    fn BltBatch(&mut self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn BltFast(&mut self, param0: u32, param1: u32, param2: &::core::option::Option<IDirectDrawSurface>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>;
    fn DeleteAttachedSurface(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<()>;
    fn EnumAttachedSurfaces(&mut self, param0: *mut ::core::ffi::c_void, param1: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn EnumOverlayZOrders(&mut self, param0: u32, param1: *mut ::core::ffi::c_void, param2: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn Flip(&mut self, param0: &::core::option::Option<IDirectDrawSurface>, param1: u32) -> ::windows::core::Result<()>;
    fn GetAttachedSurface(&mut self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<()>;
    fn GetBltStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDSCAPS) -> ::windows::core::Result<()>;
    fn GetClipper(&mut self) -> ::windows::core::Result<IDirectDrawClipper>;
    fn GetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn GetFlipStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetOverlayPosition(&mut self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()>;
    fn GetPalette(&mut self) -> ::windows::core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&mut self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()>;
    fn GetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn IsLost(&mut self) -> ::windows::core::Result<()>;
    fn Lock(&mut self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn ReleaseDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
    fn SetClipper(&mut self, param0: &::core::option::Option<IDirectDrawClipper>) -> ::windows::core::Result<()>;
    fn SetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn SetOverlayPosition(&mut self, param0: i32, param1: i32) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, param0: &::core::option::Option<IDirectDrawPalette>) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, param0: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UpdateOverlay(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>;
    fn UpdateOverlayDisplay(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn UpdateOverlayZOrder(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
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
pub trait IDirectDrawSurface2_Impl: Sized {
    fn AddAttachedSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface2>) -> ::windows::core::Result<()>;
    fn AddOverlayDirtyRect(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Blt(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>;
    fn BltBatch(&mut self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn BltFast(&mut self, param0: u32, param1: u32, param2: &::core::option::Option<IDirectDrawSurface2>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>;
    fn DeleteAttachedSurface(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface2>) -> ::windows::core::Result<()>;
    fn EnumAttachedSurfaces(&mut self, param0: *mut ::core::ffi::c_void, param1: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn EnumOverlayZOrders(&mut self, param0: u32, param1: *mut ::core::ffi::c_void, param2: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn Flip(&mut self, param0: &::core::option::Option<IDirectDrawSurface2>, param1: u32) -> ::windows::core::Result<()>;
    fn GetAttachedSurface(&mut self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface2>) -> ::windows::core::Result<()>;
    fn GetBltStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDSCAPS) -> ::windows::core::Result<()>;
    fn GetClipper(&mut self) -> ::windows::core::Result<IDirectDrawClipper>;
    fn GetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn GetFlipStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetOverlayPosition(&mut self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()>;
    fn GetPalette(&mut self) -> ::windows::core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&mut self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()>;
    fn GetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn IsLost(&mut self) -> ::windows::core::Result<()>;
    fn Lock(&mut self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn ReleaseDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
    fn SetClipper(&mut self, param0: &::core::option::Option<IDirectDrawClipper>) -> ::windows::core::Result<()>;
    fn SetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn SetOverlayPosition(&mut self, param0: i32, param1: i32) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, param0: &::core::option::Option<IDirectDrawPalette>) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, param0: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UpdateOverlay(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>;
    fn UpdateOverlayDisplay(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn UpdateOverlayZOrder(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface2>) -> ::windows::core::Result<()>;
    fn GetDDInterface(&mut self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn PageLock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn PageUnlock(&mut self, param0: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface2_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
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
pub trait IDirectDrawSurface3_Impl: Sized {
    fn AddAttachedSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface3>) -> ::windows::core::Result<()>;
    fn AddOverlayDirtyRect(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Blt(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>;
    fn BltBatch(&mut self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn BltFast(&mut self, param0: u32, param1: u32, param2: &::core::option::Option<IDirectDrawSurface3>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>;
    fn DeleteAttachedSurface(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface3>) -> ::windows::core::Result<()>;
    fn EnumAttachedSurfaces(&mut self, param0: *mut ::core::ffi::c_void, param1: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn EnumOverlayZOrders(&mut self, param0: u32, param1: *mut ::core::ffi::c_void, param2: &LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()>;
    fn Flip(&mut self, param0: &::core::option::Option<IDirectDrawSurface3>, param1: u32) -> ::windows::core::Result<()>;
    fn GetAttachedSurface(&mut self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface3>) -> ::windows::core::Result<()>;
    fn GetBltStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDSCAPS) -> ::windows::core::Result<()>;
    fn GetClipper(&mut self) -> ::windows::core::Result<IDirectDrawClipper>;
    fn GetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn GetFlipStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetOverlayPosition(&mut self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()>;
    fn GetPalette(&mut self) -> ::windows::core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&mut self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()>;
    fn GetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows::core::Result<()>;
    fn IsLost(&mut self) -> ::windows::core::Result<()>;
    fn Lock(&mut self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn ReleaseDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
    fn SetClipper(&mut self, param0: &::core::option::Option<IDirectDrawClipper>) -> ::windows::core::Result<()>;
    fn SetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn SetOverlayPosition(&mut self, param0: i32, param1: i32) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, param0: &::core::option::Option<IDirectDrawPalette>) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, param0: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UpdateOverlay(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>;
    fn UpdateOverlayDisplay(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn UpdateOverlayZOrder(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface3>) -> ::windows::core::Result<()>;
    fn GetDDInterface(&mut self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn PageLock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn PageUnlock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn SetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface3_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
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
pub trait IDirectDrawSurface4_Impl: Sized {
    fn AddAttachedSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface4>) -> ::windows::core::Result<()>;
    fn AddOverlayDirtyRect(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Blt(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>;
    fn BltBatch(&mut self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn BltFast(&mut self, param0: u32, param1: u32, param2: &::core::option::Option<IDirectDrawSurface4>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>;
    fn DeleteAttachedSurface(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface4>) -> ::windows::core::Result<()>;
    fn EnumAttachedSurfaces(&mut self, param0: *mut ::core::ffi::c_void, param1: &LPDDENUMSURFACESCALLBACK2) -> ::windows::core::Result<()>;
    fn EnumOverlayZOrders(&mut self, param0: u32, param1: *mut ::core::ffi::c_void, param2: &LPDDENUMSURFACESCALLBACK2) -> ::windows::core::Result<()>;
    fn Flip(&mut self, param0: &::core::option::Option<IDirectDrawSurface4>, param1: u32) -> ::windows::core::Result<()>;
    fn GetAttachedSurface(&mut self, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface4>) -> ::windows::core::Result<()>;
    fn GetBltStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDSCAPS2) -> ::windows::core::Result<()>;
    fn GetClipper(&mut self) -> ::windows::core::Result<IDirectDrawClipper>;
    fn GetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn GetFlipStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetOverlayPosition(&mut self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()>;
    fn GetPalette(&mut self) -> ::windows::core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&mut self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()>;
    fn GetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>;
    fn IsLost(&mut self) -> ::windows::core::Result<()>;
    fn Lock(&mut self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn ReleaseDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
    fn SetClipper(&mut self, param0: &::core::option::Option<IDirectDrawClipper>) -> ::windows::core::Result<()>;
    fn SetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn SetOverlayPosition(&mut self, param0: i32, param1: i32) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, param0: &::core::option::Option<IDirectDrawPalette>) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn UpdateOverlay(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>;
    fn UpdateOverlayDisplay(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn UpdateOverlayZOrder(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface4>) -> ::windows::core::Result<()>;
    fn GetDDInterface(&mut self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn PageLock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn PageUnlock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn SetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::Result<()>;
    fn FreePrivateData(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetUniquenessValue(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn ChangeUniquenessValue(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface4_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreePrivateData(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUniquenessValue(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeUniquenessValue().into()
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
pub trait IDirectDrawSurface7_Impl: Sized {
    fn AddAttachedSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface7>) -> ::windows::core::Result<()>;
    fn AddOverlayDirtyRect(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Blt(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>;
    fn BltBatch(&mut self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn BltFast(&mut self, param0: u32, param1: u32, param2: &::core::option::Option<IDirectDrawSurface7>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>;
    fn DeleteAttachedSurface(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface7>) -> ::windows::core::Result<()>;
    fn EnumAttachedSurfaces(&mut self, param0: *mut ::core::ffi::c_void, param1: &LPDDENUMSURFACESCALLBACK7) -> ::windows::core::Result<()>;
    fn EnumOverlayZOrders(&mut self, param0: u32, param1: *mut ::core::ffi::c_void, param2: &LPDDENUMSURFACESCALLBACK7) -> ::windows::core::Result<()>;
    fn Flip(&mut self, param0: &::core::option::Option<IDirectDrawSurface7>, param1: u32) -> ::windows::core::Result<()>;
    fn GetAttachedSurface(&mut self, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface7>) -> ::windows::core::Result<()>;
    fn GetBltStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, param0: *mut DDSCAPS2) -> ::windows::core::Result<()>;
    fn GetClipper(&mut self) -> ::windows::core::Result<IDirectDrawClipper>;
    fn GetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn GetDC(&mut self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn GetFlipStatus(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetOverlayPosition(&mut self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()>;
    fn GetPalette(&mut self) -> ::windows::core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&mut self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()>;
    fn GetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: &::core::option::Option<IDirectDraw>, param1: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>;
    fn IsLost(&mut self) -> ::windows::core::Result<()>;
    fn Lock(&mut self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn ReleaseDC(&mut self, param0: super::Gdi::HDC) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
    fn SetClipper(&mut self, param0: &::core::option::Option<IDirectDrawClipper>) -> ::windows::core::Result<()>;
    fn SetColorKey(&mut self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()>;
    fn SetOverlayPosition(&mut self, param0: i32, param1: i32) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, param0: &::core::option::Option<IDirectDrawPalette>) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn UpdateOverlay(&mut self, param0: *mut super::super::Foundation::RECT, param1: &::core::option::Option<IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>;
    fn UpdateOverlayDisplay(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn UpdateOverlayZOrder(&mut self, param0: u32, param1: &::core::option::Option<IDirectDrawSurface7>) -> ::windows::core::Result<()>;
    fn GetDDInterface(&mut self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn PageLock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn PageUnlock(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn SetSurfaceDesc(&mut self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::Result<()>;
    fn FreePrivateData(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetUniquenessValue(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn ChangeUniquenessValue(&mut self) -> ::windows::core::Result<()>;
    fn SetPriority(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetPriority(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SetLOD(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetLOD(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDirectDrawSurface7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurface7_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreePrivateData(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUniquenessValue(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeUniquenessValue().into()
        }
        unsafe extern "system" fn SetPriority<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetPriority<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPriority(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetLOD<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLOD(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetLOD<Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLOD(::core::mem::transmute_copy(&param0)).into()
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
pub trait IDirectDrawSurfaceKernel_Impl: Sized {
    fn GetKernelHandle(&mut self, param0: *mut usize) -> ::windows::core::Result<()>;
    fn ReleaseKernelHandle(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectDrawSurfaceKernel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurfaceKernel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawSurfaceKernel_Vtbl {
        unsafe extern "system" fn GetKernelHandle<Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKernelHandle(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseKernelHandle().into()
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
pub trait IDirectDrawVideoPort_Impl: Sized {
    fn Flip(&mut self, param0: &::core::option::Option<IDirectDrawSurface>, param1: u32) -> ::windows::core::Result<()>;
    fn GetBandwidthInfo(&mut self, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::Result<()>;
    fn GetColorControls(&mut self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()>;
    fn GetInputFormats(&mut self, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::core::Result<()>;
    fn GetOutputFormats(&mut self, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::core::Result<()>;
    fn GetFieldPolarity(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn GetVideoLine(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn GetVideoSignalStatus(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SetColorControls(&mut self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()>;
    fn SetTargetSurface(&mut self, param0: &::core::option::Option<IDirectDrawSurface>, param1: u32) -> ::windows::core::Result<()>;
    fn StartVideo(&mut self, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::Result<()>;
    fn StopVideo(&mut self) -> ::windows::core::Result<()>;
    fn UpdateVideo(&mut self, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::Result<()>;
    fn WaitForSync(&mut self, param0: u32, param1: u32, param2: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectDrawVideoPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawVideoPort_Vtbl {
        unsafe extern "system" fn Flip<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBandwidthInfo<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBandwidthInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetColorControls<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetInputFormats<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputFormats(::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetOutputFormats<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputFormats(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetFieldPolarity<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFieldPolarity(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoLine<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVideoLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoSignalStatus<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVideoSignalStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetTargetSurface<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetSurface(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartVideo<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartVideo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn StopVideo<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopVideo().into()
        }
        unsafe extern "system" fn UpdateVideo<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateVideo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WaitForSync<Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForSync(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
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
pub trait IDirectDrawVideoPortNotify_Impl: Sized {
    fn AcquireNotification(&mut self, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::Result<()>;
    fn ReleaseNotification(&mut self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectDrawVideoPortNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPortNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectDrawVideoPortNotify_Vtbl {
        unsafe extern "system" fn AcquireNotification<Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireNotification(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn ReleaseNotification<Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseNotification(::core::mem::transmute_copy(&param0)).into()
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

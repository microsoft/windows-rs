#[cfg(feature = "Win32_Foundation")]
pub trait IDDVideoPortContainer_Impl: Sized {
    fn CreateVideoPort(&mut self, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::core::option::Option<IDirectDrawVideoPort>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumVideoPorts(&mut self, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: &LPDDENUMVIDEOCALLBACK) -> ::windows::core::Result<()>;
    fn GetVideoPortConnectInfo(&mut self, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::Result<()>;
    fn QueryVideoPortStatus(&mut self, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDDVideoPortContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>() -> IDDVideoPortContainer_Vtbl {
        unsafe extern "system" fn CreateVideoPort<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateVideoPort(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumVideoPorts<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumVideoPorts(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVideoPortConnectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&pcinfo), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn QueryVideoPortStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryVideoPortStatus(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateVideoPort: CreateVideoPort::<Identity, Impl, OFFSET>,
            EnumVideoPorts: EnumVideoPorts::<Identity, Impl, OFFSET>,
            GetVideoPortConnectInfo: GetVideoPortConnectInfo::<Identity, Impl, OFFSET>,
            QueryVideoPortStatus: QueryVideoPortStatus::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>() -> IDirectDraw_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>() -> IDirectDraw2_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>() -> IDirectDraw4_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSurfaceFromDC(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreAllSurfaces().into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceIdentifier(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, Impl, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, Impl, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>() -> IDirectDraw7_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DuplicateSurface(::core::mem::transmute(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSurfaceFromDC(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreAllSurfaces().into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceIdentifier(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartModeTest<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartModeTest(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EvaluateMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EvaluateMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, Impl, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, Impl, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, Impl, OFFSET>,
            StartModeTest: StartModeTest::<Identity, Impl, OFFSET>,
            EvaluateMode: EvaluateMode::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>() -> IDirectDrawClipper_Vtbl {
        unsafe extern "system" fn GetClipList<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClipList(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetHWnd<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHWnd(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsClipListChanged<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsClipListChanged(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetClipList<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipList(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetHWnd<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHWnd(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClipList: GetClipList::<Identity, Impl, OFFSET>,
            GetHWnd: GetHWnd::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsClipListChanged: IsClipListChanged::<Identity, Impl, OFFSET>,
            SetClipList: SetClipList::<Identity, Impl, OFFSET>,
            SetHWnd: SetHWnd::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>() -> IDirectDrawColorControl_Vtbl {
        unsafe extern "system" fn GetColorControls<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetColorControls: GetColorControls::<Identity, Impl, OFFSET>,
            SetColorControls: SetColorControls::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>() -> IDirectDrawGammaControl_Vtbl {
        unsafe extern "system" fn GetGammaRamp<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGammaRamp(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGammaRamp(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetGammaRamp: GetGammaRamp::<Identity, Impl, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>() -> IDirectDrawKernel_Vtbl {
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetKernelHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetKernelHandle(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseKernelHandle().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetKernelHandle: GetKernelHandle::<Identity, Impl, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>() -> IDirectDrawPalette_Vtbl {
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEntries(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEntries(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetEntries: GetEntries::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetEntries: SetEntries::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>() -> IDirectDrawSurface_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>() -> IDirectDrawSurface2_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>() -> IDirectDrawSurface3_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>() -> IDirectDrawSurface4_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreePrivateData(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUniquenessValue(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeUniquenessValue().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, Impl, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>() -> IDirectDrawSurface7_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAttachedSurface(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Blt(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipper(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlay(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreePrivateData(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUniquenessValue(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeUniquenessValue().into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPriority(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetLOD<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLOD(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetLOD<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLOD(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, Impl, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetLOD: SetLOD::<Identity, Impl, OFFSET>,
            GetLOD: GetLOD::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>() -> IDirectDrawSurfaceKernel_Vtbl {
        unsafe extern "system" fn GetKernelHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetKernelHandle(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseKernelHandle().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetKernelHandle: GetKernelHandle::<Identity, Impl, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>() -> IDirectDrawVideoPort_Vtbl {
        unsafe extern "system" fn Flip<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flip(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBandwidthInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBandwidthInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetColorControls<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetInputFormats<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputFormats(::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetOutputFormats<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputFormats(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetFieldPolarity<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFieldPolarity(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoLine<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVideoLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoSignalStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVideoSignalStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetTargetSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTargetSurface(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartVideo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartVideo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn StopVideo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopVideo().into()
        }
        unsafe extern "system" fn UpdateVideo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateVideo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WaitForSync<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForSync(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetBandwidthInfo: GetBandwidthInfo::<Identity, Impl, OFFSET>,
            GetColorControls: GetColorControls::<Identity, Impl, OFFSET>,
            GetInputFormats: GetInputFormats::<Identity, Impl, OFFSET>,
            GetOutputFormats: GetOutputFormats::<Identity, Impl, OFFSET>,
            GetFieldPolarity: GetFieldPolarity::<Identity, Impl, OFFSET>,
            GetVideoLine: GetVideoLine::<Identity, Impl, OFFSET>,
            GetVideoSignalStatus: GetVideoSignalStatus::<Identity, Impl, OFFSET>,
            SetColorControls: SetColorControls::<Identity, Impl, OFFSET>,
            SetTargetSurface: SetTargetSurface::<Identity, Impl, OFFSET>,
            StartVideo: StartVideo::<Identity, Impl, OFFSET>,
            StopVideo: StopVideo::<Identity, Impl, OFFSET>,
            UpdateVideo: UpdateVideo::<Identity, Impl, OFFSET>,
            WaitForSync: WaitForSync::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>() -> IDirectDrawVideoPortNotify_Vtbl {
        unsafe extern "system" fn AcquireNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireNotification(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn ReleaseNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseNotification(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AcquireNotification: AcquireNotification::<Identity, Impl, OFFSET>,
            ReleaseNotification: ReleaseNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectDrawVideoPortNotify as ::windows::core::Interface>::IID
    }
}

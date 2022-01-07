pub trait IDDVideoPortContainerImpl: Sized {
    fn CreateVideoPort();
    fn EnumVideoPorts();
    fn GetVideoPortConnectInfo();
    fn QueryVideoPortStatus();
}
impl ::windows::core::RuntimeName for IDDVideoPortContainer {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDDVideoPortContainer";
}
impl IDDVideoPortContainerVtbl {
    pub const fn new<Impl: IDDVideoPortContainerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDDVideoPortContainerVtbl {
        unsafe extern "system" fn CreateVideoPort<Impl: IDDVideoPortContainerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVideoPort(param0, &*(&param1 as *const <DDVIDEOPORTDESC as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param2), &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumVideoPorts<Impl: IDDVideoPortContainerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumVideoPorts(param0, &*(&param1 as *const <DDVIDEOPORTCAPS as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTCAPS as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMVIDEOCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMVIDEOCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Impl: IDDVideoPortContainerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoPortConnectInfo(param0, pcinfo, ::core::mem::transmute_copy(&param2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVideoPortStatus<Impl: IDDVideoPortContainerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryVideoPortStatus(param0, &*(&param1 as *const <DDVIDEOPORTSTATUS as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTSTATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDDVideoPortContainer>, base.5, CreateVideoPort::<Impl, OFFSET>, EnumVideoPorts::<Impl, OFFSET>, GetVideoPortConnectInfo::<Impl, OFFSET>, QueryVideoPortStatus::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectDraw {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDraw";
}
impl IDirectDrawVtbl {
    pub const fn new<Impl: IDirectDrawImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawVtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClipper(param0, ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePalette(param0, &*(&param1 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param2), &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSurface(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(&*(&param0 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDisplayModes(param0, &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMMODESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMMODESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumSurfaces(param0, &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlipToGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayMode(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFourCCCodes(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGDISurface(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMonitorFrequency(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScanLine(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVerticalBlankStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplayMode(param0, param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDrawImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForVerticalBlank(param0, &*(&param1 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDraw>,
            base.5,
            Compact::<Impl, OFFSET>,
            CreateClipper::<Impl, OFFSET>,
            CreatePalette::<Impl, OFFSET>,
            CreateSurface::<Impl, OFFSET>,
            DuplicateSurface::<Impl, OFFSET>,
            EnumDisplayModes::<Impl, OFFSET>,
            EnumSurfaces::<Impl, OFFSET>,
            FlipToGDISurface::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetDisplayMode::<Impl, OFFSET>,
            GetFourCCCodes::<Impl, OFFSET>,
            GetGDISurface::<Impl, OFFSET>,
            GetMonitorFrequency::<Impl, OFFSET>,
            GetScanLine::<Impl, OFFSET>,
            GetVerticalBlankStatus::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            RestoreDisplayMode::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            SetDisplayMode::<Impl, OFFSET>,
            WaitForVerticalBlank::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDraw2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDraw2";
}
impl IDirectDraw2Vtbl {
    pub const fn new<Impl: IDirectDraw2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDraw2Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClipper(param0, ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePalette(param0, &*(&param1 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param2), &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSurface(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(&*(&param0 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDisplayModes(param0, &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMMODESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMMODESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumSurfaces(param0, &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlipToGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayMode(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFourCCCodes(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGDISurface(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMonitorFrequency(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScanLine(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVerticalBlankStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplayMode(param0, param1, param2, param3, param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForVerticalBlank(param0, &*(&param1 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableVidMem(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDraw2>,
            base.5,
            Compact::<Impl, OFFSET>,
            CreateClipper::<Impl, OFFSET>,
            CreatePalette::<Impl, OFFSET>,
            CreateSurface::<Impl, OFFSET>,
            DuplicateSurface::<Impl, OFFSET>,
            EnumDisplayModes::<Impl, OFFSET>,
            EnumSurfaces::<Impl, OFFSET>,
            FlipToGDISurface::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetDisplayMode::<Impl, OFFSET>,
            GetFourCCCodes::<Impl, OFFSET>,
            GetGDISurface::<Impl, OFFSET>,
            GetMonitorFrequency::<Impl, OFFSET>,
            GetScanLine::<Impl, OFFSET>,
            GetVerticalBlankStatus::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            RestoreDisplayMode::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            SetDisplayMode::<Impl, OFFSET>,
            WaitForVerticalBlank::<Impl, OFFSET>,
            GetAvailableVidMem::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDraw4 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDraw4";
}
impl IDirectDraw4Vtbl {
    pub const fn new<Impl: IDirectDraw4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDraw4Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClipper(param0, ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePalette(param0, &*(&param1 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param2), &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSurface(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(&*(&param0 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDisplayModes(param0, &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMMODESCALLBACK2 as ::windows::core::Abi>::Abi as *const <LPDDENUMMODESCALLBACK2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumSurfaces(param0, &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMSURFACESCALLBACK2 as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlipToGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayMode(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFourCCCodes(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGDISurface(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMonitorFrequency(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScanLine(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVerticalBlankStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplayMode(param0, param1, param2, param3, param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForVerticalBlank(param0, &*(&param1 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableVidMem(&*(&param0 as *const <DDSCAPS2 as ::windows::core::Abi>::Abi as *const <DDSCAPS2 as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceFromDC<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceFromDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreAllSurfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TestCooperativeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIdentifier<Impl: IDirectDraw4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceIdentifier(&*(&param0 as *const <DDDEVICEIDENTIFIER as ::windows::core::Abi>::Abi as *const <DDDEVICEIDENTIFIER as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDraw4>,
            base.5,
            Compact::<Impl, OFFSET>,
            CreateClipper::<Impl, OFFSET>,
            CreatePalette::<Impl, OFFSET>,
            CreateSurface::<Impl, OFFSET>,
            DuplicateSurface::<Impl, OFFSET>,
            EnumDisplayModes::<Impl, OFFSET>,
            EnumSurfaces::<Impl, OFFSET>,
            FlipToGDISurface::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetDisplayMode::<Impl, OFFSET>,
            GetFourCCCodes::<Impl, OFFSET>,
            GetGDISurface::<Impl, OFFSET>,
            GetMonitorFrequency::<Impl, OFFSET>,
            GetScanLine::<Impl, OFFSET>,
            GetVerticalBlankStatus::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            RestoreDisplayMode::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            SetDisplayMode::<Impl, OFFSET>,
            WaitForVerticalBlank::<Impl, OFFSET>,
            GetAvailableVidMem::<Impl, OFFSET>,
            GetSurfaceFromDC::<Impl, OFFSET>,
            RestoreAllSurfaces::<Impl, OFFSET>,
            TestCooperativeLevel::<Impl, OFFSET>,
            GetDeviceIdentifier::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDraw7 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDraw7";
}
impl IDirectDraw7Vtbl {
    pub const fn new<Impl: IDirectDraw7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDraw7Vtbl {
        unsafe extern "system" fn Compact<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClipper<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClipper(param0, ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePalette(param0, &*(&param1 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param2), &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSurface(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateSurface<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DuplicateSurface(&*(&param0 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDisplayModes(param0, &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMMODESCALLBACK2 as ::windows::core::Abi>::Abi as *const <LPDDENUMMODESCALLBACK2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSurfaces<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumSurfaces(param0, &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <LPDDENUMSURFACESCALLBACK7 as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlipToGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDCAPS_DX7 as ::windows::core::Abi>::Abi as *const <DDCAPS_DX7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayMode(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFourCCCodes(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGDISurface<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGDISurface(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMonitorFrequency(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanLine<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScanLine(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVerticalBlankStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMode<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplayMode(param0, param1, param2, param3, param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForVerticalBlank(param0, &*(&param1 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableVidMem<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableVidMem(&*(&param0 as *const <DDSCAPS2 as ::windows::core::Abi>::Abi as *const <DDSCAPS2 as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceFromDC<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceFromDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreAllSurfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TestCooperativeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIdentifier<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceIdentifier(&*(&param0 as *const <DDDEVICEIDENTIFIER2 as ::windows::core::Abi>::Abi as *const <DDDEVICEIDENTIFIER2 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartModeTest<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartModeTest(&*(&param0 as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateMode<Impl: IDirectDraw7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EvaluateMode(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDraw7>,
            base.5,
            Compact::<Impl, OFFSET>,
            CreateClipper::<Impl, OFFSET>,
            CreatePalette::<Impl, OFFSET>,
            CreateSurface::<Impl, OFFSET>,
            DuplicateSurface::<Impl, OFFSET>,
            EnumDisplayModes::<Impl, OFFSET>,
            EnumSurfaces::<Impl, OFFSET>,
            FlipToGDISurface::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetDisplayMode::<Impl, OFFSET>,
            GetFourCCCodes::<Impl, OFFSET>,
            GetGDISurface::<Impl, OFFSET>,
            GetMonitorFrequency::<Impl, OFFSET>,
            GetScanLine::<Impl, OFFSET>,
            GetVerticalBlankStatus::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            RestoreDisplayMode::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            SetDisplayMode::<Impl, OFFSET>,
            WaitForVerticalBlank::<Impl, OFFSET>,
            GetAvailableVidMem::<Impl, OFFSET>,
            GetSurfaceFromDC::<Impl, OFFSET>,
            RestoreAllSurfaces::<Impl, OFFSET>,
            TestCooperativeLevel::<Impl, OFFSET>,
            GetDeviceIdentifier::<Impl, OFFSET>,
            StartModeTest::<Impl, OFFSET>,
            EvaluateMode::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectDrawClipperImpl: Sized {
    fn GetClipList();
    fn GetHWnd();
    fn Initialize();
    fn IsClipListChanged();
    fn SetClipList();
    fn SetHWnd();
}
impl ::windows::core::RuntimeName for IDirectDrawClipper {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawClipper";
}
impl IDirectDrawClipperVtbl {
    pub const fn new<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawClipperVtbl {
        unsafe extern "system" fn GetClipList<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipList(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <super::Gdi::RGNDATA as ::windows::core::Abi>::Abi as *const <super::Gdi::RGNDATA as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHWnd<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHWnd(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClipListChanged<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClipListChanged(&*(&param0 as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipList<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipList(&*(&param0 as *const <super::Gdi::RGNDATA as ::windows::core::Abi>::Abi as *const <super::Gdi::RGNDATA as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWnd<Impl: IDirectDrawClipperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHWnd(param0, &*(&param1 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawClipper>, base.5, GetClipList::<Impl, OFFSET>, GetHWnd::<Impl, OFFSET>, Initialize::<Impl, OFFSET>, IsClipListChanged::<Impl, OFFSET>, SetClipList::<Impl, OFFSET>, SetHWnd::<Impl, OFFSET>)
    }
}
pub trait IDirectDrawColorControlImpl: Sized {
    fn GetColorControls();
    fn SetColorControls();
}
impl ::windows::core::RuntimeName for IDirectDrawColorControl {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawColorControl";
}
impl IDirectDrawColorControlVtbl {
    pub const fn new<Impl: IDirectDrawColorControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawColorControlVtbl {
        unsafe extern "system" fn GetColorControls<Impl: IDirectDrawColorControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorControls(&*(&param0 as *const <DDCOLORCONTROL as ::windows::core::Abi>::Abi as *const <DDCOLORCONTROL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorControls<Impl: IDirectDrawColorControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorControls(&*(&param0 as *const <DDCOLORCONTROL as ::windows::core::Abi>::Abi as *const <DDCOLORCONTROL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawColorControl>, base.5, GetColorControls::<Impl, OFFSET>, SetColorControls::<Impl, OFFSET>)
    }
}
pub trait IDirectDrawGammaControlImpl: Sized {
    fn GetGammaRamp();
    fn SetGammaRamp();
}
impl ::windows::core::RuntimeName for IDirectDrawGammaControl {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawGammaControl";
}
impl IDirectDrawGammaControlVtbl {
    pub const fn new<Impl: IDirectDrawGammaControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawGammaControlVtbl {
        unsafe extern "system" fn GetGammaRamp<Impl: IDirectDrawGammaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGammaRamp(param0, &*(&param1 as *const <DDGAMMARAMP as ::windows::core::Abi>::Abi as *const <DDGAMMARAMP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGammaRamp<Impl: IDirectDrawGammaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGammaRamp(param0, &*(&param1 as *const <DDGAMMARAMP as ::windows::core::Abi>::Abi as *const <DDGAMMARAMP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawGammaControl>, base.5, GetGammaRamp::<Impl, OFFSET>, SetGammaRamp::<Impl, OFFSET>)
    }
}
pub trait IDirectDrawKernelImpl: Sized {
    fn GetCaps();
    fn GetKernelHandle();
    fn ReleaseKernelHandle();
}
impl ::windows::core::RuntimeName for IDirectDrawKernel {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawKernel";
}
impl IDirectDrawKernelVtbl {
    pub const fn new<Impl: IDirectDrawKernelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawKernelVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawKernelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDKERNELCAPS as ::windows::core::Abi>::Abi as *const <DDKERNELCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKernelHandle<Impl: IDirectDrawKernelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKernelHandle(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseKernelHandle<Impl: IDirectDrawKernelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseKernelHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawKernel>, base.5, GetCaps::<Impl, OFFSET>, GetKernelHandle::<Impl, OFFSET>, ReleaseKernelHandle::<Impl, OFFSET>)
    }
}
pub trait IDirectDrawPaletteImpl: Sized {
    fn GetCaps();
    fn GetEntries();
    fn Initialize();
    fn SetEntries();
}
impl ::windows::core::RuntimeName for IDirectDrawPalette {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawPalette";
}
impl IDirectDrawPaletteVtbl {
    pub const fn new<Impl: IDirectDrawPaletteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawPaletteVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawPaletteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntries<Impl: IDirectDrawPaletteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEntries(param0, param1, param2, &*(&param3 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawPaletteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntries<Impl: IDirectDrawPaletteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEntries(param0, param1, param2, &*(&param3 as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawPalette>, base.5, GetCaps::<Impl, OFFSET>, GetEntries::<Impl, OFFSET>, Initialize::<Impl, OFFSET>, SetEntries::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectDrawSurface {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawSurface";
}
impl IDirectDrawSurfaceVtbl {
    pub const fn new<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawSurfaceVtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAttachedSurface(&*(&param0 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddOverlayDirtyRect(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Blt(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDBLTFX as ::windows::core::Abi>::Abi as *const <DDBLTFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltBatch(&*(&param0 as *const <DDBLTBATCH as ::windows::core::Abi>::Abi as *const <DDBLTBATCH as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltFast(param0, param1, &*(&param2 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAttachedSurface(param0, &*(&param1 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAttachedSurfaces(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOverlayZOrders(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip(&*(&param0 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttachedSurface(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBltStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipper(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlipStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPalette(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipper(&*(&param0 as *const <IDirectDrawClipper as ::windows::core::Abi>::Abi as *const <IDirectDrawClipper as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&param0 as *const <IDirectDrawPalette as ::windows::core::Abi>::Abi as *const <IDirectDrawPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlay(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDOVERLAYFX as ::windows::core::Abi>::Abi as *const <DDOVERLAYFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayDisplay(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayZOrder(param0, &*(&param1 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDrawSurface>,
            base.5,
            AddAttachedSurface::<Impl, OFFSET>,
            AddOverlayDirtyRect::<Impl, OFFSET>,
            Blt::<Impl, OFFSET>,
            BltBatch::<Impl, OFFSET>,
            BltFast::<Impl, OFFSET>,
            DeleteAttachedSurface::<Impl, OFFSET>,
            EnumAttachedSurfaces::<Impl, OFFSET>,
            EnumOverlayZOrders::<Impl, OFFSET>,
            Flip::<Impl, OFFSET>,
            GetAttachedSurface::<Impl, OFFSET>,
            GetBltStatus::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetClipper::<Impl, OFFSET>,
            GetColorKey::<Impl, OFFSET>,
            GetDC::<Impl, OFFSET>,
            GetFlipStatus::<Impl, OFFSET>,
            GetOverlayPosition::<Impl, OFFSET>,
            GetPalette::<Impl, OFFSET>,
            GetPixelFormat::<Impl, OFFSET>,
            GetSurfaceDesc::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            IsLost::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
            ReleaseDC::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            SetClipper::<Impl, OFFSET>,
            SetColorKey::<Impl, OFFSET>,
            SetOverlayPosition::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            Unlock::<Impl, OFFSET>,
            UpdateOverlay::<Impl, OFFSET>,
            UpdateOverlayDisplay::<Impl, OFFSET>,
            UpdateOverlayZOrder::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDrawSurface2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawSurface2";
}
impl IDirectDrawSurface2Vtbl {
    pub const fn new<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawSurface2Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAttachedSurface(&*(&param0 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddOverlayDirtyRect(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Blt(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDBLTFX as ::windows::core::Abi>::Abi as *const <DDBLTFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltBatch(&*(&param0 as *const <DDBLTBATCH as ::windows::core::Abi>::Abi as *const <DDBLTBATCH as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltFast(param0, param1, &*(&param2 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAttachedSurface(param0, &*(&param1 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAttachedSurfaces(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOverlayZOrders(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip(&*(&param0 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttachedSurface(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBltStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipper(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlipStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPalette(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipper(&*(&param0 as *const <IDirectDrawClipper as ::windows::core::Abi>::Abi as *const <IDirectDrawClipper as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&param0 as *const <IDirectDrawPalette as ::windows::core::Abi>::Abi as *const <IDirectDrawPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlay(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDOVERLAYFX as ::windows::core::Abi>::Abi as *const <DDOVERLAYFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayDisplay(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayZOrder(param0, &*(&param1 as *const <IDirectDrawSurface2 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDDInterface(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageLock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageUnlock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDrawSurface2>,
            base.5,
            AddAttachedSurface::<Impl, OFFSET>,
            AddOverlayDirtyRect::<Impl, OFFSET>,
            Blt::<Impl, OFFSET>,
            BltBatch::<Impl, OFFSET>,
            BltFast::<Impl, OFFSET>,
            DeleteAttachedSurface::<Impl, OFFSET>,
            EnumAttachedSurfaces::<Impl, OFFSET>,
            EnumOverlayZOrders::<Impl, OFFSET>,
            Flip::<Impl, OFFSET>,
            GetAttachedSurface::<Impl, OFFSET>,
            GetBltStatus::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetClipper::<Impl, OFFSET>,
            GetColorKey::<Impl, OFFSET>,
            GetDC::<Impl, OFFSET>,
            GetFlipStatus::<Impl, OFFSET>,
            GetOverlayPosition::<Impl, OFFSET>,
            GetPalette::<Impl, OFFSET>,
            GetPixelFormat::<Impl, OFFSET>,
            GetSurfaceDesc::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            IsLost::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
            ReleaseDC::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            SetClipper::<Impl, OFFSET>,
            SetColorKey::<Impl, OFFSET>,
            SetOverlayPosition::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            Unlock::<Impl, OFFSET>,
            UpdateOverlay::<Impl, OFFSET>,
            UpdateOverlayDisplay::<Impl, OFFSET>,
            UpdateOverlayZOrder::<Impl, OFFSET>,
            GetDDInterface::<Impl, OFFSET>,
            PageLock::<Impl, OFFSET>,
            PageUnlock::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDrawSurface3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawSurface3";
}
impl IDirectDrawSurface3Vtbl {
    pub const fn new<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawSurface3Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAttachedSurface(&*(&param0 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddOverlayDirtyRect(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Blt(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDBLTFX as ::windows::core::Abi>::Abi as *const <DDBLTFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltBatch(&*(&param0 as *const <DDBLTBATCH as ::windows::core::Abi>::Abi as *const <DDBLTBATCH as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltFast(param0, param1, &*(&param2 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAttachedSurface(param0, &*(&param1 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAttachedSurfaces(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOverlayZOrders(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip(&*(&param0 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttachedSurface(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBltStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDSCAPS as ::windows::core::Abi>::Abi as *const <DDSCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipper(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlipStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPalette(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipper(&*(&param0 as *const <IDirectDrawClipper as ::windows::core::Abi>::Abi as *const <IDirectDrawClipper as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&param0 as *const <IDirectDrawPalette as ::windows::core::Abi>::Abi as *const <IDirectDrawPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlay(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDOVERLAYFX as ::windows::core::Abi>::Abi as *const <DDOVERLAYFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayDisplay(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayZOrder(param0, &*(&param1 as *const <IDirectDrawSurface3 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDDInterface(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageLock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageUnlock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDrawSurface3>,
            base.5,
            AddAttachedSurface::<Impl, OFFSET>,
            AddOverlayDirtyRect::<Impl, OFFSET>,
            Blt::<Impl, OFFSET>,
            BltBatch::<Impl, OFFSET>,
            BltFast::<Impl, OFFSET>,
            DeleteAttachedSurface::<Impl, OFFSET>,
            EnumAttachedSurfaces::<Impl, OFFSET>,
            EnumOverlayZOrders::<Impl, OFFSET>,
            Flip::<Impl, OFFSET>,
            GetAttachedSurface::<Impl, OFFSET>,
            GetBltStatus::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetClipper::<Impl, OFFSET>,
            GetColorKey::<Impl, OFFSET>,
            GetDC::<Impl, OFFSET>,
            GetFlipStatus::<Impl, OFFSET>,
            GetOverlayPosition::<Impl, OFFSET>,
            GetPalette::<Impl, OFFSET>,
            GetPixelFormat::<Impl, OFFSET>,
            GetSurfaceDesc::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            IsLost::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
            ReleaseDC::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            SetClipper::<Impl, OFFSET>,
            SetColorKey::<Impl, OFFSET>,
            SetOverlayPosition::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            Unlock::<Impl, OFFSET>,
            UpdateOverlay::<Impl, OFFSET>,
            UpdateOverlayDisplay::<Impl, OFFSET>,
            UpdateOverlayZOrder::<Impl, OFFSET>,
            GetDDInterface::<Impl, OFFSET>,
            PageLock::<Impl, OFFSET>,
            PageUnlock::<Impl, OFFSET>,
            SetSurfaceDesc::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDrawSurface4 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawSurface4";
}
impl IDirectDrawSurface4Vtbl {
    pub const fn new<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawSurface4Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAttachedSurface(&*(&param0 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddOverlayDirtyRect(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Blt(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDBLTFX as ::windows::core::Abi>::Abi as *const <DDBLTFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltBatch(&*(&param0 as *const <DDBLTBATCH as ::windows::core::Abi>::Abi as *const <DDBLTBATCH as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltFast(param0, param1, &*(&param2 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAttachedSurface(param0, &*(&param1 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAttachedSurfaces(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <LPDDENUMSURFACESCALLBACK2 as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOverlayZOrders(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <LPDDENUMSURFACESCALLBACK2 as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip(&*(&param0 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttachedSurface(&*(&param0 as *const <DDSCAPS2 as ::windows::core::Abi>::Abi as *const <DDSCAPS2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBltStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDSCAPS2 as ::windows::core::Abi>::Abi as *const <DDSCAPS2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipper(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlipStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPalette(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipper(&*(&param0 as *const <IDirectDrawClipper as ::windows::core::Abi>::Abi as *const <IDirectDrawClipper as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&param0 as *const <IDirectDrawPalette as ::windows::core::Abi>::Abi as *const <IDirectDrawPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlay(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDOVERLAYFX as ::windows::core::Abi>::Abi as *const <DDOVERLAYFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayDisplay(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayZOrder(param0, &*(&param1 as *const <IDirectDrawSurface4 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface4 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDDInterface(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageLock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageUnlock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreePrivateData(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniquenessValue<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUniquenessValue(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeUniquenessValue<Impl: IDirectDrawSurface4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeUniquenessValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDrawSurface4>,
            base.5,
            AddAttachedSurface::<Impl, OFFSET>,
            AddOverlayDirtyRect::<Impl, OFFSET>,
            Blt::<Impl, OFFSET>,
            BltBatch::<Impl, OFFSET>,
            BltFast::<Impl, OFFSET>,
            DeleteAttachedSurface::<Impl, OFFSET>,
            EnumAttachedSurfaces::<Impl, OFFSET>,
            EnumOverlayZOrders::<Impl, OFFSET>,
            Flip::<Impl, OFFSET>,
            GetAttachedSurface::<Impl, OFFSET>,
            GetBltStatus::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetClipper::<Impl, OFFSET>,
            GetColorKey::<Impl, OFFSET>,
            GetDC::<Impl, OFFSET>,
            GetFlipStatus::<Impl, OFFSET>,
            GetOverlayPosition::<Impl, OFFSET>,
            GetPalette::<Impl, OFFSET>,
            GetPixelFormat::<Impl, OFFSET>,
            GetSurfaceDesc::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            IsLost::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
            ReleaseDC::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            SetClipper::<Impl, OFFSET>,
            SetColorKey::<Impl, OFFSET>,
            SetOverlayPosition::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            Unlock::<Impl, OFFSET>,
            UpdateOverlay::<Impl, OFFSET>,
            UpdateOverlayDisplay::<Impl, OFFSET>,
            UpdateOverlayZOrder::<Impl, OFFSET>,
            GetDDInterface::<Impl, OFFSET>,
            PageLock::<Impl, OFFSET>,
            PageUnlock::<Impl, OFFSET>,
            SetSurfaceDesc::<Impl, OFFSET>,
            SetPrivateData::<Impl, OFFSET>,
            GetPrivateData::<Impl, OFFSET>,
            FreePrivateData::<Impl, OFFSET>,
            GetUniquenessValue::<Impl, OFFSET>,
            ChangeUniquenessValue::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectDrawSurface7 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawSurface7";
}
impl IDirectDrawSurface7Vtbl {
    pub const fn new<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawSurface7Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAttachedSurface(&*(&param0 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddOverlayDirtyRect(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blt<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Blt(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDBLTFX as ::windows::core::Abi>::Abi as *const <DDBLTFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltBatch<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltBatch(&*(&param0 as *const <DDBLTBATCH as ::windows::core::Abi>::Abi as *const <DDBLTBATCH as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BltFast<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: ::windows::core::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BltFast(param0, param1, &*(&param2 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType), &*(&param3 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAttachedSurface(param0, &*(&param1 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAttachedSurfaces(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <LPDDENUMSURFACESCALLBACK7 as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOverlayZOrders(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <LPDDENUMSURFACESCALLBACK7 as ::windows::core::Abi>::Abi as *const <LPDDENUMSURFACESCALLBACK7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flip<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip(&*(&param0 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttachedSurface(&*(&param0 as *const <DDSCAPS2 as ::windows::core::Abi>::Abi as *const <DDSCAPS2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBltStatus<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBltStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&param0 as *const <DDSCAPS2 as ::windows::core::Abi>::Abi as *const <DDSCAPS2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipper<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipper(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlipStatus<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlipStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPalette<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPalette(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <IDirectDraw as ::windows::core::Abi>::Abi as *const <IDirectDraw as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLost<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&param0 as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipper<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipper(&*(&param0 as *const <IDirectDrawClipper as ::windows::core::Abi>::Abi as *const <IDirectDrawClipper as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorKey<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorKey(param0, &*(&param1 as *const <DDCOLORKEY as ::windows::core::Abi>::Abi as *const <DDCOLORKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOverlayPosition(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&param0 as *const <IDirectDrawPalette as ::windows::core::Abi>::Abi as *const <IDirectDrawPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlay<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: ::windows::core::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlay(
                &*(&param0 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                param3,
                &*(&param4 as *const <DDOVERLAYFX as ::windows::core::Abi>::Abi as *const <DDOVERLAYFX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayDisplay(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOverlayZOrder(param0, &*(&param1 as *const <IDirectDrawSurface7 as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface7 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDDInterface<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDDInterface(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageLock<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageLock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageUnlock<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageUnlock(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSurfaceDesc<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSurfaceDesc(&*(&param0 as *const <DDSURFACEDESC2 as ::windows::core::Abi>::Abi as *const <DDSURFACEDESC2 as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreePrivateData(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniquenessValue<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUniquenessValue(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeUniquenessValue<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeUniquenessValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPriority(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPriority(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLOD<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLOD(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLOD<Impl: IDirectDrawSurface7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLOD(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDrawSurface7>,
            base.5,
            AddAttachedSurface::<Impl, OFFSET>,
            AddOverlayDirtyRect::<Impl, OFFSET>,
            Blt::<Impl, OFFSET>,
            BltBatch::<Impl, OFFSET>,
            BltFast::<Impl, OFFSET>,
            DeleteAttachedSurface::<Impl, OFFSET>,
            EnumAttachedSurfaces::<Impl, OFFSET>,
            EnumOverlayZOrders::<Impl, OFFSET>,
            Flip::<Impl, OFFSET>,
            GetAttachedSurface::<Impl, OFFSET>,
            GetBltStatus::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            GetClipper::<Impl, OFFSET>,
            GetColorKey::<Impl, OFFSET>,
            GetDC::<Impl, OFFSET>,
            GetFlipStatus::<Impl, OFFSET>,
            GetOverlayPosition::<Impl, OFFSET>,
            GetPalette::<Impl, OFFSET>,
            GetPixelFormat::<Impl, OFFSET>,
            GetSurfaceDesc::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            IsLost::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
            ReleaseDC::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            SetClipper::<Impl, OFFSET>,
            SetColorKey::<Impl, OFFSET>,
            SetOverlayPosition::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            Unlock::<Impl, OFFSET>,
            UpdateOverlay::<Impl, OFFSET>,
            UpdateOverlayDisplay::<Impl, OFFSET>,
            UpdateOverlayZOrder::<Impl, OFFSET>,
            GetDDInterface::<Impl, OFFSET>,
            PageLock::<Impl, OFFSET>,
            PageUnlock::<Impl, OFFSET>,
            SetSurfaceDesc::<Impl, OFFSET>,
            SetPrivateData::<Impl, OFFSET>,
            GetPrivateData::<Impl, OFFSET>,
            FreePrivateData::<Impl, OFFSET>,
            GetUniquenessValue::<Impl, OFFSET>,
            ChangeUniquenessValue::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            GetPriority::<Impl, OFFSET>,
            SetLOD::<Impl, OFFSET>,
            GetLOD::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectDrawSurfaceKernelImpl: Sized {
    fn GetKernelHandle();
    fn ReleaseKernelHandle();
}
impl ::windows::core::RuntimeName for IDirectDrawSurfaceKernel {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawSurfaceKernel";
}
impl IDirectDrawSurfaceKernelVtbl {
    pub const fn new<Impl: IDirectDrawSurfaceKernelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawSurfaceKernelVtbl {
        unsafe extern "system" fn GetKernelHandle<Impl: IDirectDrawSurfaceKernelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKernelHandle(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseKernelHandle<Impl: IDirectDrawSurfaceKernelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseKernelHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawSurfaceKernel>, base.5, GetKernelHandle::<Impl, OFFSET>, ReleaseKernelHandle::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectDrawVideoPort {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawVideoPort";
}
impl IDirectDrawVideoPortVtbl {
    pub const fn new<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawVideoPortVtbl {
        unsafe extern "system" fn Flip<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip(&*(&param0 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthInfo<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBandwidthInfo(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType), param1, param2, param3, &*(&param4 as *const <DDVIDEOPORTBANDWIDTH as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTBANDWIDTH as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorControls<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorControls(&*(&param0 as *const <DDCOLORCONTROL as ::windows::core::Abi>::Abi as *const <DDCOLORCONTROL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormats<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputFormats(lpnumformats, ::core::mem::transmute_copy(&param1), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormats<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputFormats(&*(&param0 as *const <DDPIXELFORMAT as ::windows::core::Abi>::Abi as *const <DDPIXELFORMAT as ::windows::core::DefaultType>::DefaultType), lpnumformats, ::core::mem::transmute_copy(&param2), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldPolarity<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFieldPolarity(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoLine<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoLine(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoSignalStatus<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoSignalStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorControls<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorControls(&*(&param0 as *const <DDCOLORCONTROL as ::windows::core::Abi>::Abi as *const <DDCOLORCONTROL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetSurface<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTargetSurface(&*(&param0 as *const <IDirectDrawSurface as ::windows::core::Abi>::Abi as *const <IDirectDrawSurface as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartVideo<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartVideo(&*(&param0 as *const <DDVIDEOPORTINFO as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopVideo<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateVideo<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateVideo(&*(&param0 as *const <DDVIDEOPORTINFO as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForSync<Impl: IDirectDrawVideoPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForSync(param0, param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectDrawVideoPort>,
            base.5,
            Flip::<Impl, OFFSET>,
            GetBandwidthInfo::<Impl, OFFSET>,
            GetColorControls::<Impl, OFFSET>,
            GetInputFormats::<Impl, OFFSET>,
            GetOutputFormats::<Impl, OFFSET>,
            GetFieldPolarity::<Impl, OFFSET>,
            GetVideoLine::<Impl, OFFSET>,
            GetVideoSignalStatus::<Impl, OFFSET>,
            SetColorControls::<Impl, OFFSET>,
            SetTargetSurface::<Impl, OFFSET>,
            StartVideo::<Impl, OFFSET>,
            StopVideo::<Impl, OFFSET>,
            UpdateVideo::<Impl, OFFSET>,
            WaitForSync::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectDrawVideoPortNotifyImpl: Sized {
    fn AcquireNotification();
    fn ReleaseNotification();
}
impl ::windows::core::RuntimeName for IDirectDrawVideoPortNotify {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectDraw.IDirectDrawVideoPortNotify";
}
impl IDirectDrawVideoPortNotifyVtbl {
    pub const fn new<Impl: IDirectDrawVideoPortNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectDrawVideoPortNotifyVtbl {
        unsafe extern "system" fn AcquireNotification<Impl: IDirectDrawVideoPortNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireNotification(&*(&param0 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DDVIDEOPORTNOTIFY as ::windows::core::Abi>::Abi as *const <DDVIDEOPORTNOTIFY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseNotification<Impl: IDirectDrawVideoPortNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseNotification(&*(&param0 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectDrawVideoPortNotify>, base.5, AcquireNotification::<Impl, OFFSET>, ReleaseNotification::<Impl, OFFSET>)
    }
}

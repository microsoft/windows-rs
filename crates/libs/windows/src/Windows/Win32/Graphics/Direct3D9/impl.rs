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
impl ::windows::core::RuntimeName for IDirect3D9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3D9";
}
impl IDirect3D9Vtbl {
    pub const fn new<Impl: IDirect3D9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3D9Vtbl {
        unsafe extern "system" fn RegisterSoftwareDevice<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterSoftwareDevice(&*(&pinitializefunction as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterCount<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterIdentifier<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterIdentifier(adapter, flags, &*(&pidentifier as *const <D3DADAPTER_IDENTIFIER9 as ::windows::core::Abi>::Abi as *const <D3DADAPTER_IDENTIFIER9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterModeCount<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterModeCount(adapter, format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAdapterModes<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAdapterModes(adapter, format, mode, &*(&pmode as *const <D3DDISPLAYMODE as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterDisplayMode(adapter, &*(&pmode as *const <D3DDISPLAYMODE as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceType<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDeviceType(adapter, devtype, adapterformat, backbufferformat, &*(&bwindowed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceFormat<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDeviceFormat(adapter, devicetype, adapterformat, usage, rtype, checkformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDeviceMultiSampleType(adapter, devicetype, surfaceformat, &*(&windowed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), multisampletype, pqualitylevels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDepthStencilMatch(adapter, devicetype, adapterformat, rendertargetformat, depthstencilformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDeviceFormatConversion(adapter, devicetype, sourceformat, targetformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceCaps(adapter, devicetype, &*(&pcaps as *const <D3DCAPS9 as ::windows::core::Abi>::Abi as *const <D3DCAPS9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterMonitor<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterMonitor(adapter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Impl: IDirect3D9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(adapter, devicetype, &*(&hfocuswindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), behaviorflags, &*(&ppresentationparameters as *const <D3DPRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DPRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppreturneddeviceinterface)) {
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
            ::windows::core::GetRuntimeClassName::<IDirect3D9>,
            base.5,
            RegisterSoftwareDevice::<Impl, OFFSET>,
            GetAdapterCount::<Impl, OFFSET>,
            GetAdapterIdentifier::<Impl, OFFSET>,
            GetAdapterModeCount::<Impl, OFFSET>,
            EnumAdapterModes::<Impl, OFFSET>,
            GetAdapterDisplayMode::<Impl, OFFSET>,
            CheckDeviceType::<Impl, OFFSET>,
            CheckDeviceFormat::<Impl, OFFSET>,
            CheckDeviceMultiSampleType::<Impl, OFFSET>,
            CheckDepthStencilMatch::<Impl, OFFSET>,
            CheckDeviceFormatConversion::<Impl, OFFSET>,
            GetDeviceCaps::<Impl, OFFSET>,
            GetAdapterMonitor::<Impl, OFFSET>,
            CreateDevice::<Impl, OFFSET>,
        )
    }
}
pub trait IDirect3D9ExImpl: Sized + IDirect3D9Impl {
    fn GetAdapterModeCountEx();
    fn EnumAdapterModesEx();
    fn GetAdapterDisplayModeEx();
    fn CreateDeviceEx();
    fn GetAdapterLUID();
}
impl ::windows::core::RuntimeName for IDirect3D9Ex {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3D9Ex";
}
impl IDirect3D9ExVtbl {
    pub const fn new<Impl: IDirect3D9ExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3D9ExVtbl {
        unsafe extern "system" fn GetAdapterModeCountEx<Impl: IDirect3D9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterModeCountEx(adapter, &*(&pfilter as *const <D3DDISPLAYMODEFILTER as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEFILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAdapterModesEx<Impl: IDirect3D9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumAdapterModesEx(adapter, &*(&pfilter as *const <D3DDISPLAYMODEFILTER as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEFILTER as ::windows::core::DefaultType>::DefaultType), mode, &*(&pmode as *const <D3DDISPLAYMODEEX as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Impl: IDirect3D9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterDisplayModeEx(adapter, &*(&pmode as *const <D3DDISPLAYMODEEX as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEEX as ::windows::core::DefaultType>::DefaultType), protation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirect3D9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceEx(
                adapter,
                devicetype,
                &*(&hfocuswindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                behaviorflags,
                &*(&ppresentationparameters as *const <D3DPRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DPRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType),
                &*(&pfullscreendisplaymode as *const <D3DDISPLAYMODEEX as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEEX as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppreturneddeviceinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterLUID<Impl: IDirect3D9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterLUID(adapter, &*(&pluid as *const <super::super::Foundation::LUID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3D9Ex>, base.5, GetAdapterModeCountEx::<Impl, OFFSET>, EnumAdapterModesEx::<Impl, OFFSET>, GetAdapterDisplayModeEx::<Impl, OFFSET>, CreateDeviceEx::<Impl, OFFSET>, GetAdapterLUID::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirect3DBaseTexture9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DBaseTexture9";
}
impl IDirect3DBaseTexture9Vtbl {
    pub const fn new<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DBaseTexture9Vtbl {
        unsafe extern "system" fn SetLOD<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lodnew: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLOD(lodnew) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLOD<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLOD() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelCount<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoGenFilterType(filtertype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutoGenFilterType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Impl: IDirect3DBaseTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GenerateMipSubLevels().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DBaseTexture9>, base.5, SetLOD::<Impl, OFFSET>, GetLOD::<Impl, OFFSET>, GetLevelCount::<Impl, OFFSET>, SetAutoGenFilterType::<Impl, OFFSET>, GetAutoGenFilterType::<Impl, OFFSET>, GenerateMipSubLevels::<Impl, OFFSET>)
    }
}
pub trait IDirect3DCubeTexture9Impl: Sized + IDirect3DBaseTexture9Impl + IDirect3DResource9Impl {
    fn GetLevelDesc();
    fn GetCubeMapSurface();
    fn LockRect();
    fn UnlockRect();
    fn AddDirtyRect();
}
impl ::windows::core::RuntimeName for IDirect3DCubeTexture9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DCubeTexture9";
}
impl IDirect3DCubeTexture9Vtbl {
    pub const fn new<Impl: IDirect3DCubeTexture9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DCubeTexture9Vtbl {
        unsafe extern "system" fn GetLevelDesc<Impl: IDirect3DCubeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevelDesc(level, &*(&pdesc as *const <D3DSURFACE_DESC as ::windows::core::Abi>::Abi as *const <D3DSURFACE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCubeMapSurface<Impl: IDirect3DCubeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCubeMapSurface(facetype, level, ::core::mem::transmute_copy(&ppcubemapsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Impl: IDirect3DCubeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockRect(facetype, level, &*(&plockedrect as *const <D3DLOCKED_RECT as ::windows::core::Abi>::Abi as *const <D3DLOCKED_RECT as ::windows::core::DefaultType>::DefaultType), &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockRect<Impl: IDirect3DCubeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockRect(facetype, level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirtyRect<Impl: IDirect3DCubeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDirtyRect(facetype, &*(&pdirtyrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DCubeTexture9>, base.5, GetLevelDesc::<Impl, OFFSET>, GetCubeMapSurface::<Impl, OFFSET>, LockRect::<Impl, OFFSET>, UnlockRect::<Impl, OFFSET>, AddDirtyRect::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IDirect3DDevice9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DDevice9";
}
impl IDirect3DDevice9Vtbl {
    pub const fn new<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DDevice9Vtbl {
        unsafe extern "system" fn TestCooperativeLevel<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAvailableTextureMem<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableTextureMem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvictManagedResources<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EvictManagedResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirect3D<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppd3d9: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDirect3D(::core::mem::transmute_copy(&ppd3d9)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceCaps(&*(&pcaps as *const <D3DCAPS9 as ::windows::core::Abi>::Abi as *const <D3DCAPS9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayMode(iswapchain, &*(&pmode as *const <D3DDISPLAYMODE as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreationParameters<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCreationParameters(&*(&pparameters as *const <D3DDEVICE_CREATION_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DDEVICE_CREATION_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursorProperties<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCursorProperties(xhotspot, yhotspot, &*(&pcursorbitmap as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursorPosition<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCursorPosition(x, y, flags).into()
        }
        unsafe extern "system" fn ShowCursor<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowCursor(&*(&bshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAdditionalSwapChain(&*(&ppresentationparameters as *const <D3DPRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DPRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSwapChain<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSwapChain(iswapchain, ::core::mem::transmute_copy(&pswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumberOfSwapChains() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset(&*(&ppresentationparameters as *const <D3DPRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DPRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Present<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Present(
                &*(&psourcerect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&hdestwindowoverride as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pdirtyregion as *const <super::Gdi::RGNDATA as ::windows::core::Abi>::Abi as *const <super::Gdi::RGNDATA as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackBuffer<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBackBuffer(iswapchain, ibackbuffer, r#type, ::core::mem::transmute_copy(&ppbackbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRasterStatus(iswapchain, &*(&prasterstatus as *const <D3DRASTER_STATUS as ::windows::core::Abi>::Abi as *const <D3DRASTER_STATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDialogBoxMode<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDialogBoxMode(&*(&benabledialogs as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGammaRamp<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGammaRamp(iswapchain, flags, &*(&pramp as *const <D3DGAMMARAMP as ::windows::core::Abi>::Abi as *const <D3DGAMMARAMP as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetGammaRamp<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetGammaRamp(iswapchain, &*(&pramp as *const <D3DGAMMARAMP as ::windows::core::Abi>::Abi as *const <D3DGAMMARAMP as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateTexture<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTexture(width, height, levels, usage, format, pool, ::core::mem::transmute_copy(&pptexture), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVolumeTexture<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVolumeTexture(width, height, depth, levels, usage, format, pool, ::core::mem::transmute_copy(&ppvolumetexture), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubeTexture<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCubeTexture(edgelength, levels, usage, format, pool, ::core::mem::transmute_copy(&ppcubetexture), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVertexBuffer(length, usage, fvf, pool, ::core::mem::transmute_copy(&ppvertexbuffer), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIndexBuffer<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateIndexBuffer(length, usage, format, pool, ::core::mem::transmute_copy(&ppindexbuffer), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTarget<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRenderTarget(width, height, format, multisample, multisamplequality, &*(&lockable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsurface), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilSurface(width, height, format, multisample, multisamplequality, &*(&discard as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsurface), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSurface<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: ::windows::core::RawPtr, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateSurface(
                &*(&psourcesurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcerect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestinationsurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestpoint as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTexture<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcetexture: ::windows::core::RawPtr, pdestinationtexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateTexture(&*(&psourcetexture as *const <IDirect3DBaseTexture9 as ::windows::core::Abi>::Abi as *const <IDirect3DBaseTexture9 as ::windows::core::DefaultType>::DefaultType), &*(&pdestinationtexture as *const <IDirect3DBaseTexture9 as ::windows::core::Abi>::Abi as *const <IDirect3DBaseTexture9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderTargetData<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRenderTargetData(&*(&prendertarget as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType), &*(&pdestsurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrontBufferData(iswapchain, &*(&pdestsurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StretchRect<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestsurface: ::windows::core::RawPtr, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StretchRect(
                &*(&psourcesurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcerect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestsurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                filter,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorFill<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorFill(&*(&psurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType), &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), color) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOffscreenPlainSurface(width, height, format, pool, ::core::mem::transmute_copy(&ppsurface), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTarget<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, prendertarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRenderTarget(rendertargetindex, &*(&prendertarget as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderTarget<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRenderTarget(rendertargetindex, ::core::mem::transmute_copy(&pprendertarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewzstencil: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDepthStencilSurface(&*(&pnewzstencil as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepthStencilSurface<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppzstencilsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDepthStencilSurface(::core::mem::transmute_copy(&ppzstencilsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginScene<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginScene() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndScene<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndScene() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear(count, &*(&prects as *const <D3DRECT as ::windows::core::Abi>::Abi as *const <D3DRECT as ::windows::core::DefaultType>::DefaultType), flags, color, z, stencil) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransform(state, &*(&pmatrix as *const <super::Direct3D::D3DMATRIX as ::windows::core::Abi>::Abi as *const <super::Direct3D::D3DMATRIX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransform(state, &*(&pmatrix as *const <super::Direct3D::D3DMATRIX as ::windows::core::Abi>::Abi as *const <super::Direct3D::D3DMATRIX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultiplyTransform<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MultiplyTransform(param0, &*(&param1 as *const <super::Direct3D::D3DMATRIX as ::windows::core::Abi>::Abi as *const <super::Direct3D::D3DMATRIX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewport(&*(&pviewport as *const <D3DVIEWPORT9 as ::windows::core::Abi>::Abi as *const <D3DVIEWPORT9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewport<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewport(&*(&pviewport as *const <D3DVIEWPORT9 as ::windows::core::Abi>::Abi as *const <D3DVIEWPORT9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaterial<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaterial(&*(&pmaterial as *const <D3DMATERIAL9 as ::windows::core::Abi>::Abi as *const <D3DMATERIAL9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaterial<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaterial(&*(&pmaterial as *const <D3DMATERIAL9 as ::windows::core::Abi>::Abi as *const <D3DMATERIAL9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLight<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLight(index, &*(&param1 as *const <D3DLIGHT9 as ::windows::core::Abi>::Abi as *const <D3DLIGHT9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLight<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLight(index, &*(&param1 as *const <D3DLIGHT9 as ::windows::core::Abi>::Abi as *const <D3DLIGHT9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightEnable<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LightEnable(index, &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLightEnable<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLightEnable(index, &*(&penable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipPlane<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipPlane(index, pplane) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipPlane<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipPlane(index, pplane) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderState<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRenderState(state, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderState<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRenderState(state, pvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStateBlock<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStateBlock(r#type, ::core::mem::transmute_copy(&ppsb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStateBlock<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginStateBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStateBlock<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndStateBlock(::core::mem::transmute_copy(&ppsb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipStatus<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipStatus(&*(&pclipstatus as *const <D3DCLIPSTATUS9 as ::windows::core::Abi>::Abi as *const <D3DCLIPSTATUS9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipStatus<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipStatus(&*(&pclipstatus as *const <D3DCLIPSTATUS9 as ::windows::core::Abi>::Abi as *const <D3DCLIPSTATUS9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTexture<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, pptexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTexture(stage, ::core::mem::transmute_copy(&pptexture)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTexture<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, ptexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTexture(stage, &*(&ptexture as *const <IDirect3DBaseTexture9 as ::windows::core::Abi>::Abi as *const <IDirect3DBaseTexture9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextureStageState<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTextureStageState(stage, r#type, pvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureStageState<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTextureStageState(stage, r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSamplerState<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSamplerState(sampler, r#type, pvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplerState<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSamplerState(sampler, r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateDevice<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateDevice(pnumpasses) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaletteEntries<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPaletteEntries(palettenumber, &*(&pentries as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPaletteEntries(palettenumber, &*(&pentries as *const <super::Gdi::PALETTEENTRY as ::windows::core::Abi>::Abi as *const <super::Gdi::PALETTEENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCurrentTexturePalette(palettenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentTexturePalette(palettenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScissorRect<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScissorRect(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScissorRect<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScissorRect(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSoftwareVertexProcessing(&*(&bsoftware as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSoftwareVertexProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNPatchMode<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nsegments: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNPatchMode(nsegments) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNPatchMode<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNPatchMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawPrimitive<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawPrimitive(primitivetype, startvertex, primitivecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawIndexedPrimitive(param0, basevertexindex, minvertexindex, numvertices, startindex, primcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawPrimitiveUP<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawPrimitiveUP(primitivetype, primitivecount, &*(&pvertexstreamzerodata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), vertexstreamzerostride) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawIndexedPrimitiveUP(primitivetype, minvertexindex, numvertices, primitivecount, &*(&pindexdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), indexdataformat, &*(&pvertexstreamzerodata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), vertexstreamzerostride) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessVertices<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: ::windows::core::RawPtr, pvertexdecl: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessVertices(srcstartindex, destindex, vertexcount, &*(&pdestbuffer as *const <IDirect3DVertexBuffer9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexBuffer9 as ::windows::core::DefaultType>::DefaultType), &*(&pvertexdecl as *const <IDirect3DVertexDeclaration9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexDeclaration9 as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexDeclaration<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVertexDeclaration(&*(&pvertexelements as *const <D3DVERTEXELEMENT9 as ::windows::core::Abi>::Abi as *const <D3DVERTEXELEMENT9 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdecl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexDeclaration(&*(&pdecl as *const <IDirect3DVertexDeclaration9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexDeclaration9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexDeclaration<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVertexDeclaration(::core::mem::transmute_copy(&ppdecl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFVF<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFVF(fvf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFVF<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfvf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFVF(pfvf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVertexShader(pfunction, ::core::mem::transmute_copy(&ppshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShader<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexShader(&*(&pshader as *const <IDirect3DVertexShader9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexShader9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShader<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVertexShader(::core::mem::transmute_copy(&ppshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexShaderConstantF(startregister, pconstantdata, vector4fcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVertexShaderConstantF(startregister, pconstantdata, vector4fcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexShaderConstantI(startregister, pconstantdata, vector4icount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVertexShaderConstantI(startregister, pconstantdata, vector4icount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexShaderConstantB(startregister, &*(&pconstantdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), boolcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVertexShaderConstantB(startregister, &*(&pconstantdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), boolcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamSource<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, pstreamdata: ::windows::core::RawPtr, offsetinbytes: u32, stride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamSource(streamnumber, &*(&pstreamdata as *const <IDirect3DVertexBuffer9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexBuffer9 as ::windows::core::DefaultType>::DefaultType), offsetinbytes, stride) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamSource<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut ::windows::core::RawPtr, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamSource(streamnumber, ::core::mem::transmute_copy(&ppstreamdata), poffsetinbytes, pstride) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamSourceFreq<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, setting: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamSourceFreq(streamnumber, setting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamSourceFreq<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamSourceFreq(streamnumber, psetting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndices<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIndices(&*(&pindexdata as *const <IDirect3DIndexBuffer9 as ::windows::core::Abi>::Abi as *const <IDirect3DIndexBuffer9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndices<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppindexdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndices(::core::mem::transmute_copy(&ppindexdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePixelShader(pfunction, ::core::mem::transmute_copy(&ppshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShader<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPixelShader(&*(&pshader as *const <IDirect3DPixelShader9 as ::windows::core::Abi>::Abi as *const <IDirect3DPixelShader9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShader<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelShader(::core::mem::transmute_copy(&ppshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPixelShaderConstantF(startregister, pconstantdata, vector4fcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelShaderConstantF(startregister, pconstantdata, vector4fcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPixelShaderConstantI(startregister, pconstantdata, vector4icount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelShaderConstantI(startregister, pconstantdata, vector4icount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPixelShaderConstantB(startregister, &*(&pconstantdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), boolcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelShaderConstantB(startregister, &*(&pconstantdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), boolcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawRectPatch<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawRectPatch(handle, pnumsegs, &*(&prectpatchinfo as *const <D3DRECTPATCH_INFO as ::windows::core::Abi>::Abi as *const <D3DRECTPATCH_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawTriPatch<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawTriPatch(handle, pnumsegs, &*(&ptripatchinfo as *const <D3DTRIPATCH_INFO as ::windows::core::Abi>::Abi as *const <D3DTRIPATCH_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePatch<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeletePatch(handle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Impl: IDirect3DDevice9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateQuery(r#type, ::core::mem::transmute_copy(&ppquery)) {
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
            ::windows::core::GetRuntimeClassName::<IDirect3DDevice9>,
            base.5,
            TestCooperativeLevel::<Impl, OFFSET>,
            GetAvailableTextureMem::<Impl, OFFSET>,
            EvictManagedResources::<Impl, OFFSET>,
            GetDirect3D::<Impl, OFFSET>,
            GetDeviceCaps::<Impl, OFFSET>,
            GetDisplayMode::<Impl, OFFSET>,
            GetCreationParameters::<Impl, OFFSET>,
            SetCursorProperties::<Impl, OFFSET>,
            SetCursorPosition::<Impl, OFFSET>,
            ShowCursor::<Impl, OFFSET>,
            CreateAdditionalSwapChain::<Impl, OFFSET>,
            GetSwapChain::<Impl, OFFSET>,
            GetNumberOfSwapChains::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            Present::<Impl, OFFSET>,
            GetBackBuffer::<Impl, OFFSET>,
            GetRasterStatus::<Impl, OFFSET>,
            SetDialogBoxMode::<Impl, OFFSET>,
            SetGammaRamp::<Impl, OFFSET>,
            GetGammaRamp::<Impl, OFFSET>,
            CreateTexture::<Impl, OFFSET>,
            CreateVolumeTexture::<Impl, OFFSET>,
            CreateCubeTexture::<Impl, OFFSET>,
            CreateVertexBuffer::<Impl, OFFSET>,
            CreateIndexBuffer::<Impl, OFFSET>,
            CreateRenderTarget::<Impl, OFFSET>,
            CreateDepthStencilSurface::<Impl, OFFSET>,
            UpdateSurface::<Impl, OFFSET>,
            UpdateTexture::<Impl, OFFSET>,
            GetRenderTargetData::<Impl, OFFSET>,
            GetFrontBufferData::<Impl, OFFSET>,
            StretchRect::<Impl, OFFSET>,
            ColorFill::<Impl, OFFSET>,
            CreateOffscreenPlainSurface::<Impl, OFFSET>,
            SetRenderTarget::<Impl, OFFSET>,
            GetRenderTarget::<Impl, OFFSET>,
            SetDepthStencilSurface::<Impl, OFFSET>,
            GetDepthStencilSurface::<Impl, OFFSET>,
            BeginScene::<Impl, OFFSET>,
            EndScene::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            GetTransform::<Impl, OFFSET>,
            MultiplyTransform::<Impl, OFFSET>,
            SetViewport::<Impl, OFFSET>,
            GetViewport::<Impl, OFFSET>,
            SetMaterial::<Impl, OFFSET>,
            GetMaterial::<Impl, OFFSET>,
            SetLight::<Impl, OFFSET>,
            GetLight::<Impl, OFFSET>,
            LightEnable::<Impl, OFFSET>,
            GetLightEnable::<Impl, OFFSET>,
            SetClipPlane::<Impl, OFFSET>,
            GetClipPlane::<Impl, OFFSET>,
            SetRenderState::<Impl, OFFSET>,
            GetRenderState::<Impl, OFFSET>,
            CreateStateBlock::<Impl, OFFSET>,
            BeginStateBlock::<Impl, OFFSET>,
            EndStateBlock::<Impl, OFFSET>,
            SetClipStatus::<Impl, OFFSET>,
            GetClipStatus::<Impl, OFFSET>,
            GetTexture::<Impl, OFFSET>,
            SetTexture::<Impl, OFFSET>,
            GetTextureStageState::<Impl, OFFSET>,
            SetTextureStageState::<Impl, OFFSET>,
            GetSamplerState::<Impl, OFFSET>,
            SetSamplerState::<Impl, OFFSET>,
            ValidateDevice::<Impl, OFFSET>,
            SetPaletteEntries::<Impl, OFFSET>,
            GetPaletteEntries::<Impl, OFFSET>,
            SetCurrentTexturePalette::<Impl, OFFSET>,
            GetCurrentTexturePalette::<Impl, OFFSET>,
            SetScissorRect::<Impl, OFFSET>,
            GetScissorRect::<Impl, OFFSET>,
            SetSoftwareVertexProcessing::<Impl, OFFSET>,
            GetSoftwareVertexProcessing::<Impl, OFFSET>,
            SetNPatchMode::<Impl, OFFSET>,
            GetNPatchMode::<Impl, OFFSET>,
            DrawPrimitive::<Impl, OFFSET>,
            DrawIndexedPrimitive::<Impl, OFFSET>,
            DrawPrimitiveUP::<Impl, OFFSET>,
            DrawIndexedPrimitiveUP::<Impl, OFFSET>,
            ProcessVertices::<Impl, OFFSET>,
            CreateVertexDeclaration::<Impl, OFFSET>,
            SetVertexDeclaration::<Impl, OFFSET>,
            GetVertexDeclaration::<Impl, OFFSET>,
            SetFVF::<Impl, OFFSET>,
            GetFVF::<Impl, OFFSET>,
            CreateVertexShader::<Impl, OFFSET>,
            SetVertexShader::<Impl, OFFSET>,
            GetVertexShader::<Impl, OFFSET>,
            SetVertexShaderConstantF::<Impl, OFFSET>,
            GetVertexShaderConstantF::<Impl, OFFSET>,
            SetVertexShaderConstantI::<Impl, OFFSET>,
            GetVertexShaderConstantI::<Impl, OFFSET>,
            SetVertexShaderConstantB::<Impl, OFFSET>,
            GetVertexShaderConstantB::<Impl, OFFSET>,
            SetStreamSource::<Impl, OFFSET>,
            GetStreamSource::<Impl, OFFSET>,
            SetStreamSourceFreq::<Impl, OFFSET>,
            GetStreamSourceFreq::<Impl, OFFSET>,
            SetIndices::<Impl, OFFSET>,
            GetIndices::<Impl, OFFSET>,
            CreatePixelShader::<Impl, OFFSET>,
            SetPixelShader::<Impl, OFFSET>,
            GetPixelShader::<Impl, OFFSET>,
            SetPixelShaderConstantF::<Impl, OFFSET>,
            GetPixelShaderConstantF::<Impl, OFFSET>,
            SetPixelShaderConstantI::<Impl, OFFSET>,
            GetPixelShaderConstantI::<Impl, OFFSET>,
            SetPixelShaderConstantB::<Impl, OFFSET>,
            GetPixelShaderConstantB::<Impl, OFFSET>,
            DrawRectPatch::<Impl, OFFSET>,
            DrawTriPatch::<Impl, OFFSET>,
            DeletePatch::<Impl, OFFSET>,
            CreateQuery::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for IDirect3DDevice9Ex {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DDevice9Ex";
}
impl IDirect3DDevice9ExVtbl {
    pub const fn new<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DDevice9ExVtbl {
        unsafe extern "system" fn SetConvolutionMonoKernel<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConvolutionMonoKernel(width, height, rows, columns) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComposeRects<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr, psrcrectdescs: ::windows::core::RawPtr, numrects: u32, pdstrectdescs: ::windows::core::RawPtr, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComposeRects(
                &*(&psrc as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType),
                &*(&pdst as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType),
                &*(&psrcrectdescs as *const <IDirect3DVertexBuffer9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexBuffer9 as ::windows::core::DefaultType>::DefaultType),
                numrects,
                &*(&pdstrectdescs as *const <IDirect3DVertexBuffer9 as ::windows::core::Abi>::Abi as *const <IDirect3DVertexBuffer9 as ::windows::core::DefaultType>::DefaultType),
                operation,
                xoffset,
                yoffset,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PresentEx(
                &*(&psourcerect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&hdestwindowoverride as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pdirtyregion as *const <super::Gdi::RGNDATA as ::windows::core::Abi>::Abi as *const <super::Gdi::RGNDATA as ::windows::core::DefaultType>::DefaultType),
                dwflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGPUThreadPriority(ppriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGPUThreadPriority(priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVBlank<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForVBlank(iswapchain) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckResourceResidency<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcearray: *mut ::windows::core::RawPtr, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckResourceResidency(::core::mem::transmute_copy(&presourcearray), numresources) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaximumFrameLatency(maxlatency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumFrameLatency(pmaxlatency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceState<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDeviceState(&*(&hdestinationwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetEx(width, height, format, multisample, multisamplequality, &*(&lockable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsurface), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), usage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOffscreenPlainSurfaceEx(width, height, format, pool, ::core::mem::transmute_copy(&ppsurface), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), usage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilSurfaceEx(width, height, format, multisample, multisamplequality, &*(&discard as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsurface), &*(&psharedhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), usage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetEx(&*(&ppresentationparameters as *const <D3DPRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DPRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType), &*(&pfullscreendisplaymode as *const <D3DDISPLAYMODEEX as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Impl: IDirect3DDevice9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayModeEx(iswapchain, &*(&pmode as *const <D3DDISPLAYMODEEX as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEEX as ::windows::core::DefaultType>::DefaultType), protation) {
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
            ::windows::core::GetRuntimeClassName::<IDirect3DDevice9Ex>,
            base.5,
            SetConvolutionMonoKernel::<Impl, OFFSET>,
            ComposeRects::<Impl, OFFSET>,
            PresentEx::<Impl, OFFSET>,
            GetGPUThreadPriority::<Impl, OFFSET>,
            SetGPUThreadPriority::<Impl, OFFSET>,
            WaitForVBlank::<Impl, OFFSET>,
            CheckResourceResidency::<Impl, OFFSET>,
            SetMaximumFrameLatency::<Impl, OFFSET>,
            GetMaximumFrameLatency::<Impl, OFFSET>,
            CheckDeviceState::<Impl, OFFSET>,
            CreateRenderTargetEx::<Impl, OFFSET>,
            CreateOffscreenPlainSurfaceEx::<Impl, OFFSET>,
            CreateDepthStencilSurfaceEx::<Impl, OFFSET>,
            ResetEx::<Impl, OFFSET>,
            GetDisplayModeEx::<Impl, OFFSET>,
        )
    }
}
pub trait IDirect3DIndexBuffer9Impl: Sized + IDirect3DResource9Impl {
    fn Lock();
    fn Unlock();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for IDirect3DIndexBuffer9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DIndexBuffer9";
}
impl IDirect3DIndexBuffer9Vtbl {
    pub const fn new<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DIndexBuffer9Vtbl {
        unsafe extern "system" fn Lock<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(offsettolock, sizetolock, &*(&ppbdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DIndexBuffer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3DINDEXBUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3DINDEXBUFFER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DIndexBuffer9>, base.5, Lock::<Impl, OFFSET>, Unlock::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
}
pub trait IDirect3DPixelShader9Impl: Sized {
    fn GetDevice();
    fn GetFunction();
}
impl ::windows::core::RuntimeName for IDirect3DPixelShader9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DPixelShader9";
}
impl IDirect3DPixelShader9Vtbl {
    pub const fn new<Impl: IDirect3DPixelShader9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DPixelShader9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DPixelShader9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Impl: IDirect3DPixelShader9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFunction(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), psizeofdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DPixelShader9>, base.5, GetDevice::<Impl, OFFSET>, GetFunction::<Impl, OFFSET>)
    }
}
pub trait IDirect3DQuery9Impl: Sized {
    fn GetDevice();
    fn GetType();
    fn GetDataSize();
    fn Issue();
    fn GetData();
}
impl ::windows::core::RuntimeName for IDirect3DQuery9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DQuery9";
}
impl IDirect3DQuery9Vtbl {
    pub const fn new<Impl: IDirect3DQuery9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DQuery9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DQuery9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IDirect3DQuery9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3DQUERYTYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataSize<Impl: IDirect3DQuery9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Issue<Impl: IDirect3DQuery9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwissueflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Issue(dwissueflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: IDirect3DQuery9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwsize, dwgetdataflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DQuery9>, base.5, GetDevice::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetDataSize::<Impl, OFFSET>, Issue::<Impl, OFFSET>, GetData::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirect3DResource9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DResource9";
}
impl IDirect3DResource9Vtbl {
    pub const fn new<Impl: IDirect3DResource9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DResource9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&refguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), sizeofdata, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&refguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), psizeofdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreePrivateData(&*(&refguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prioritynew: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPriority(prioritynew) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreLoad<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PreLoad().into()
        }
        unsafe extern "system" fn GetType<Impl: IDirect3DResource9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3DRESOURCETYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DResource9>, base.5, GetDevice::<Impl, OFFSET>, SetPrivateData::<Impl, OFFSET>, GetPrivateData::<Impl, OFFSET>, FreePrivateData::<Impl, OFFSET>, SetPriority::<Impl, OFFSET>, GetPriority::<Impl, OFFSET>, PreLoad::<Impl, OFFSET>, GetType::<Impl, OFFSET>)
    }
}
pub trait IDirect3DStateBlock9Impl: Sized {
    fn GetDevice();
    fn Capture();
    fn Apply();
}
impl ::windows::core::RuntimeName for IDirect3DStateBlock9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DStateBlock9";
}
impl IDirect3DStateBlock9Vtbl {
    pub const fn new<Impl: IDirect3DStateBlock9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DStateBlock9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DStateBlock9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capture<Impl: IDirect3DStateBlock9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Apply<Impl: IDirect3DStateBlock9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Apply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DStateBlock9>, base.5, GetDevice::<Impl, OFFSET>, Capture::<Impl, OFFSET>, Apply::<Impl, OFFSET>)
    }
}
pub trait IDirect3DSurface9Impl: Sized + IDirect3DResource9Impl {
    fn GetContainer();
    fn GetDesc();
    fn LockRect();
    fn UnlockRect();
    fn GetDC();
    fn ReleaseDC();
}
impl ::windows::core::RuntimeName for IDirect3DSurface9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DSurface9";
}
impl IDirect3DSurface9Vtbl {
    pub const fn new<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DSurface9Vtbl {
        unsafe extern "system" fn GetContainer<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContainer(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppcontainer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3DSURFACE_DESC as ::windows::core::Abi>::Abi as *const <D3DSURFACE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockRect(&*(&plockedrect as *const <D3DLOCKED_RECT as ::windows::core::Abi>::Abi as *const <D3DLOCKED_RECT as ::windows::core::DefaultType>::DefaultType), &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockRect<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDC<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&phdc as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDirect3DSurface9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&hdc as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DSurface9>, base.5, GetContainer::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>, LockRect::<Impl, OFFSET>, UnlockRect::<Impl, OFFSET>, GetDC::<Impl, OFFSET>, ReleaseDC::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IDirect3DSwapChain9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DSwapChain9";
}
impl IDirect3DSwapChain9Vtbl {
    pub const fn new<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DSwapChain9Vtbl {
        unsafe extern "system" fn Present<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Present(
                &*(&psourcerect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&hdestwindowoverride as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pdirtyregion as *const <super::Gdi::RGNDATA as ::windows::core::Abi>::Abi as *const <super::Gdi::RGNDATA as ::windows::core::DefaultType>::DefaultType),
                dwflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrontBufferData(&*(&pdestsurface as *const <IDirect3DSurface9 as ::windows::core::Abi>::Abi as *const <IDirect3DSurface9 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackBuffer<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBackBuffer(ibackbuffer, r#type, ::core::mem::transmute_copy(&ppbackbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRasterStatus<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRasterStatus(&*(&prasterstatus as *const <D3DRASTER_STATUS as ::windows::core::Abi>::Abi as *const <D3DRASTER_STATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayMode<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayMode(&*(&pmode as *const <D3DDISPLAYMODE as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentParameters<Impl: IDirect3DSwapChain9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentParameters(&*(&ppresentationparameters as *const <D3DPRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <D3DPRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DSwapChain9>, base.5, Present::<Impl, OFFSET>, GetFrontBufferData::<Impl, OFFSET>, GetBackBuffer::<Impl, OFFSET>, GetRasterStatus::<Impl, OFFSET>, GetDisplayMode::<Impl, OFFSET>, GetDevice::<Impl, OFFSET>, GetPresentParameters::<Impl, OFFSET>)
    }
}
pub trait IDirect3DSwapChain9ExImpl: Sized + IDirect3DSwapChain9Impl {
    fn GetLastPresentCount();
    fn GetPresentStats();
    fn GetDisplayModeEx();
}
impl ::windows::core::RuntimeName for IDirect3DSwapChain9Ex {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DSwapChain9Ex";
}
impl IDirect3DSwapChain9ExVtbl {
    pub const fn new<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DSwapChain9ExVtbl {
        unsafe extern "system" fn GetLastPresentCount<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastPresentCount(plastpresentcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentStats<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentStats(&*(&ppresentationstatistics as *const <D3DPRESENTSTATS as ::windows::core::Abi>::Abi as *const <D3DPRESENTSTATS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Impl: IDirect3DSwapChain9ExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayModeEx(&*(&pmode as *const <D3DDISPLAYMODEEX as ::windows::core::Abi>::Abi as *const <D3DDISPLAYMODEEX as ::windows::core::DefaultType>::DefaultType), protation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DSwapChain9Ex>, base.5, GetLastPresentCount::<Impl, OFFSET>, GetPresentStats::<Impl, OFFSET>, GetDisplayModeEx::<Impl, OFFSET>)
    }
}
pub trait IDirect3DTexture9Impl: Sized + IDirect3DBaseTexture9Impl + IDirect3DResource9Impl {
    fn GetLevelDesc();
    fn GetSurfaceLevel();
    fn LockRect();
    fn UnlockRect();
    fn AddDirtyRect();
}
impl ::windows::core::RuntimeName for IDirect3DTexture9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DTexture9";
}
impl IDirect3DTexture9Vtbl {
    pub const fn new<Impl: IDirect3DTexture9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DTexture9Vtbl {
        unsafe extern "system" fn GetLevelDesc<Impl: IDirect3DTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevelDesc(level, &*(&pdesc as *const <D3DSURFACE_DESC as ::windows::core::Abi>::Abi as *const <D3DSURFACE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurfaceLevel<Impl: IDirect3DTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, ppsurfacelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurfaceLevel(level, ::core::mem::transmute_copy(&ppsurfacelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRect<Impl: IDirect3DTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockRect(level, &*(&plockedrect as *const <D3DLOCKED_RECT as ::windows::core::Abi>::Abi as *const <D3DLOCKED_RECT as ::windows::core::DefaultType>::DefaultType), &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockRect<Impl: IDirect3DTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockRect(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirtyRect<Impl: IDirect3DTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDirtyRect(&*(&pdirtyrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DTexture9>, base.5, GetLevelDesc::<Impl, OFFSET>, GetSurfaceLevel::<Impl, OFFSET>, LockRect::<Impl, OFFSET>, UnlockRect::<Impl, OFFSET>, AddDirtyRect::<Impl, OFFSET>)
    }
}
pub trait IDirect3DVertexBuffer9Impl: Sized + IDirect3DResource9Impl {
    fn Lock();
    fn Unlock();
    fn GetDesc();
}
impl ::windows::core::RuntimeName for IDirect3DVertexBuffer9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DVertexBuffer9";
}
impl IDirect3DVertexBuffer9Vtbl {
    pub const fn new<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DVertexBuffer9Vtbl {
        unsafe extern "system" fn Lock<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(offsettolock, sizetolock, &*(&ppbdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DVertexBuffer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3DVERTEXBUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3DVERTEXBUFFER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DVertexBuffer9>, base.5, Lock::<Impl, OFFSET>, Unlock::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>)
    }
}
pub trait IDirect3DVertexDeclaration9Impl: Sized {
    fn GetDevice();
    fn GetDeclaration();
}
impl ::windows::core::RuntimeName for IDirect3DVertexDeclaration9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DVertexDeclaration9";
}
impl IDirect3DVertexDeclaration9Vtbl {
    pub const fn new<Impl: IDirect3DVertexDeclaration9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DVertexDeclaration9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DVertexDeclaration9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeclaration<Impl: IDirect3DVertexDeclaration9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeclaration(&*(&pelement as *const <D3DVERTEXELEMENT9 as ::windows::core::Abi>::Abi as *const <D3DVERTEXELEMENT9 as ::windows::core::DefaultType>::DefaultType), pnumelements) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DVertexDeclaration9>, base.5, GetDevice::<Impl, OFFSET>, GetDeclaration::<Impl, OFFSET>)
    }
}
pub trait IDirect3DVertexShader9Impl: Sized {
    fn GetDevice();
    fn GetFunction();
}
impl ::windows::core::RuntimeName for IDirect3DVertexShader9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DVertexShader9";
}
impl IDirect3DVertexShader9Vtbl {
    pub const fn new<Impl: IDirect3DVertexShader9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DVertexShader9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DVertexShader9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Impl: IDirect3DVertexShader9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFunction(&*(&param0 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), psizeofdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DVertexShader9>, base.5, GetDevice::<Impl, OFFSET>, GetFunction::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirect3DVolume9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DVolume9";
}
impl IDirect3DVolume9Vtbl {
    pub const fn new<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DVolume9Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&refguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), sizeofdata, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&refguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), psizeofdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreePrivateData<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreePrivateData(&*(&refguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContainer(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppcontainer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3DVOLUME_DESC as ::windows::core::Abi>::Abi as *const <D3DVOLUME_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBox<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockBox(&*(&plockedvolume as *const <D3DLOCKED_BOX as ::windows::core::Abi>::Abi as *const <D3DLOCKED_BOX as ::windows::core::DefaultType>::DefaultType), &*(&pbox as *const <D3DBOX as ::windows::core::Abi>::Abi as *const <D3DBOX as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockBox<Impl: IDirect3DVolume9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DVolume9>, base.5, GetDevice::<Impl, OFFSET>, SetPrivateData::<Impl, OFFSET>, GetPrivateData::<Impl, OFFSET>, FreePrivateData::<Impl, OFFSET>, GetContainer::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>, LockBox::<Impl, OFFSET>, UnlockBox::<Impl, OFFSET>)
    }
}
pub trait IDirect3DVolumeTexture9Impl: Sized + IDirect3DBaseTexture9Impl + IDirect3DResource9Impl {
    fn GetLevelDesc();
    fn GetVolumeLevel();
    fn LockBox();
    fn UnlockBox();
    fn AddDirtyBox();
}
impl ::windows::core::RuntimeName for IDirect3DVolumeTexture9 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9.IDirect3DVolumeTexture9";
}
impl IDirect3DVolumeTexture9Vtbl {
    pub const fn new<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DVolumeTexture9Vtbl {
        unsafe extern "system" fn GetLevelDesc<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevelDesc(level, &*(&pdesc as *const <D3DVOLUME_DESC as ::windows::core::Abi>::Abi as *const <D3DVOLUME_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeLevel<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, ppvolumelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVolumeLevel(level, ::core::mem::transmute_copy(&ppvolumelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBox<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockBox(level, &*(&plockedvolume as *const <D3DLOCKED_BOX as ::windows::core::Abi>::Abi as *const <D3DLOCKED_BOX as ::windows::core::DefaultType>::DefaultType), &*(&pbox as *const <D3DBOX as ::windows::core::Abi>::Abi as *const <D3DBOX as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockBox<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockBox(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirtyBox<Impl: IDirect3DVolumeTexture9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtybox: *const D3DBOX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDirtyBox(&*(&pdirtybox as *const <D3DBOX as ::windows::core::Abi>::Abi as *const <D3DBOX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DVolumeTexture9>, base.5, GetLevelDesc::<Impl, OFFSET>, GetVolumeLevel::<Impl, OFFSET>, LockBox::<Impl, OFFSET>, UnlockBox::<Impl, OFFSET>, AddDirtyBox::<Impl, OFFSET>)
    }
}

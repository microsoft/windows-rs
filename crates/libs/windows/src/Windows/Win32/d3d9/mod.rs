#[cfg(feature = "Win32_dsound")]
#[inline]
pub unsafe fn D3DPERF_BeginEvent<P1>(col: super::dsound::D3DCOLOR, wszname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_BeginEvent(col : super::dsound::D3DCOLOR, wszname : windows_core::PCWSTR) -> i32);
    unsafe { D3DPERF_BeginEvent(col, wszname.param().abi()) }
}
#[inline]
pub unsafe fn D3DPERF_EndEvent() -> i32 {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_EndEvent() -> i32);
    unsafe { D3DPERF_EndEvent() }
}
#[inline]
pub unsafe fn D3DPERF_GetStatus() -> u32 {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_GetStatus() -> u32);
    unsafe { D3DPERF_GetStatus() }
}
#[inline]
pub unsafe fn D3DPERF_QueryRepeatFrame() -> windows_core::BOOL {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_QueryRepeatFrame() -> windows_core::BOOL);
    unsafe { D3DPERF_QueryRepeatFrame() }
}
#[cfg(feature = "Win32_dsound")]
#[inline]
pub unsafe fn D3DPERF_SetMarker<P1>(col: super::dsound::D3DCOLOR, wszname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetMarker(col : super::dsound::D3DCOLOR, wszname : windows_core::PCWSTR));
    unsafe { D3DPERF_SetMarker(col, wszname.param().abi()) }
}
#[inline]
pub unsafe fn D3DPERF_SetOptions(dwoptions: u32) {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetOptions(dwoptions : u32));
    unsafe { D3DPERF_SetOptions(dwoptions) }
}
#[cfg(feature = "Win32_dsound")]
#[inline]
pub unsafe fn D3DPERF_SetRegion<P1>(col: super::dsound::D3DCOLOR, wszname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetRegion(col : super::dsound::D3DCOLOR, wszname : windows_core::PCWSTR));
    unsafe { D3DPERF_SetRegion(col, wszname.param().abi()) }
}
#[inline]
pub unsafe fn Direct3DCreate9(sdkversion: u32) -> Option<IDirect3D9> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9(sdkversion : u32) -> Option < IDirect3D9 >);
    unsafe { Direct3DCreate9(sdkversion) }
}
#[inline]
pub unsafe fn Direct3DCreate9Ex(sdkversion: u32) -> windows_core::Result<IDirect3D9Ex> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9Ex(sdkversion : u32, param1 : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        Direct3DCreate9Ex(sdkversion, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const D3D9b_SDK_VERSION: u32 = 31;
pub const D3DADAPTER_DEFAULT: u32 = 0;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: u32 = 512;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: u32 = 256;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: u32 = 1024;
pub const D3DCREATE_DISABLE_PRINTSCREEN: u32 = 32768;
pub const D3DCREATE_DISABLE_PSGP_THREADING: u32 = 8192;
pub const D3DCREATE_ENABLE_PRESENTSTATS: u32 = 16384;
pub const D3DCREATE_FPU_PRESERVE: u32 = 2;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: u32 = 64;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: u32 = 128;
pub const D3DCREATE_MULTITHREADED: u32 = 4;
pub const D3DCREATE_NOWINDOWCHANGES: u32 = 2048;
pub const D3DCREATE_PUREDEVICE: u32 = 16;
pub const D3DCREATE_SCREENSAVER: u32 = 268435456;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: u32 = 32;
pub const D3DCURSOR_IMMEDIATE_UPDATE: u32 = 1;
pub const D3DENUM_NO_DRIVERVERSION: u32 = 4;
pub const D3DENUM_WHQL_LEVEL: u32 = 2;
pub const D3DERR_CANNOTPROTECTCONTENT: i32 = -2005530499;
pub const D3DERR_CONFLICTINGRENDERSTATE: i32 = -2005530591;
pub const D3DERR_CONFLICTINGTEXTUREFILTER: i32 = -2005530594;
pub const D3DERR_CONFLICTINGTEXTUREPALETTE: i32 = -2005530586;
pub const D3DERR_DEVICEHUNG: i32 = -2005530508;
pub const D3DERR_DEVICELOST: i32 = -2005530520;
pub const D3DERR_DEVICENOTRESET: i32 = -2005530519;
pub const D3DERR_DEVICEREMOVED: i32 = -2005530512;
pub const D3DERR_DRIVERINTERNALERROR: i32 = -2005530585;
pub const D3DERR_DRIVERINVALIDCALL: i32 = -2005530515;
pub const D3DERR_INVALIDCALL: i32 = -2005530516;
pub const D3DERR_INVALIDDEVICE: i32 = -2005530517;
pub const D3DERR_MOREDATA: i32 = -2005530521;
pub const D3DERR_NOTAVAILABLE: i32 = -2005530518;
pub const D3DERR_NOTFOUND: i32 = -2005530522;
pub const D3DERR_OUTOFVIDEOMEMORY: i32 = -2005532292;
pub const D3DERR_PRESENT_STATISTICS_DISJOINT: i32 = -2005530492;
pub const D3DERR_TOOMANYOPERATIONS: i32 = -2005530595;
pub const D3DERR_UNSUPPORTEDALPHAARG: i32 = -2005530596;
pub const D3DERR_UNSUPPORTEDALPHAOPERATION: i32 = -2005530597;
pub const D3DERR_UNSUPPORTEDCOLORARG: i32 = -2005530598;
pub const D3DERR_UNSUPPORTEDCOLOROPERATION: i32 = -2005530599;
pub const D3DERR_UNSUPPORTEDCRYPTO: i32 = -2005530498;
pub const D3DERR_UNSUPPORTEDFACTORVALUE: i32 = -2005530593;
pub const D3DERR_UNSUPPORTEDOVERLAY: i32 = -2005530501;
pub const D3DERR_UNSUPPORTEDOVERLAYFORMAT: i32 = -2005530500;
pub const D3DERR_UNSUPPORTEDTEXTUREFILTER: i32 = -2005530590;
pub const D3DERR_WASSTILLDRAWING: i32 = -2005532132;
pub const D3DERR_WRONGTEXTUREFORMAT: i32 = -2005530600;
pub const D3DOK_NOAUTOGEN: u32 = 141953135;
pub const D3DPRESENT_BACK_BUFFERS_MAX: u32 = 3;
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: u32 = 30;
pub const D3DPRESENT_DONOTFLIP: u32 = 4;
pub const D3DPRESENT_DONOTWAIT: u32 = 1;
pub const D3DPRESENT_FLIPRESTART: u32 = 8;
pub const D3DPRESENT_FORCEIMMEDIATE: u32 = 256;
pub const D3DPRESENT_HIDEOVERLAY: u32 = 64;
pub const D3DPRESENT_LINEAR_CONTENT: u32 = 2;
pub const D3DPRESENT_UPDATECOLORKEY: u32 = 128;
pub const D3DPRESENT_UPDATEOVERLAYONLY: u32 = 32;
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: u32 = 16;
pub const D3DSGR_CALIBRATE: u32 = 1;
pub const D3DSGR_NO_CALIBRATION: u32 = 0;
pub const D3DSPD_IUNKNOWN: u32 = 1;
pub const D3D_OK: u32 = 0;
pub const D3D_SDK_VERSION: u32 = 32;
windows_core::imp::define_interface!(IDirect3D9, IDirect3D9_Vtbl, 0x81bdcbca_64d4_426d_ae8d_ad0147f4275c);
windows_core::imp::interface_hierarchy!(IDirect3D9, windows_core::IUnknown);
impl IDirect3D9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterSoftwareDevice)(windows_core::Interface::as_raw(self), pinitializefunction as _) }
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterIdentifier)(windows_core::Interface::as_raw(self), adapter, flags, pidentifier as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterModeCount(&self, adapter: u32, format: super::d3d9types::D3DFORMAT) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCount)(windows_core::Interface::as_raw(self), adapter, format) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn EnumAdapterModes(&self, adapter: u32, format: super::d3d9types::D3DFORMAT, mode: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapterModes)(windows_core::Interface::as_raw(self), adapter, format, mode, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapterDisplayMode)(windows_core::Interface::as_raw(self), adapter, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceType(&self, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, backbufferformat: super::d3d9types::D3DFORMAT, bwindowed: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceType)(windows_core::Interface::as_raw(self), adapter, devtype, adapterformat, backbufferformat, bwindowed.into()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceFormat(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, usage: u32, rtype: super::d3d9types::D3DRESOURCETYPE, checkformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormat)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, usage, rtype, checkformat) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, surfaceformat: super::d3d9types::D3DFORMAT, windowed: bool, multisampletype: super::d3d9types::D3DMULTISAMPLE_TYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckDeviceMultiSampleType)(windows_core::Interface::as_raw(self), adapter, devicetype, surfaceformat, windowed.into(), multisampletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, rendertargetformat: super::d3d9types::D3DFORMAT, depthstencilformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDepthStencilMatch)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, rendertargetformat, depthstencilformat) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, sourceformat: super::d3d9types::D3DFORMAT, targetformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormatConversion)(windows_core::Interface::as_raw(self), adapter, devicetype, sourceformat, targetformat) }
    }
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub unsafe fn GetDeviceCaps(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), adapter, devicetype, pcaps as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetAdapterMonitor(&self, adapter: u32) -> super::windef::HMONITOR {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterMonitor)(windows_core::Interface::as_raw(self), adapter) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn CreateDevice(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow, behaviorflags, ppresentationparameters as _, core::mem::transmute(ppreturneddeviceinterface)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub RegisterSoftwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterIdentifier: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterModeCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DFORMAT) -> u32,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterModeCount: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub EnumAdapterModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DFORMAT, u32, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    EnumAdapterModes: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterDisplayMode: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, u32, super::d3d9types::D3DRESOURCETYPE, super::d3d9types::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceFormat: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceMultiSampleType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, windows_core::BOOL, super::d3d9types::D3DMULTISAMPLE_TYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceMultiSampleType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDepthStencilMatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDepthStencilMatch: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceFormatConversion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceFormatConversion: usize,
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types")))]
    GetDeviceCaps: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetAdapterMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::windef::HMONITOR,
    #[cfg(not(feature = "Win32_windef"))]
    GetAdapterMonitor: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::windef::HWND, u32, *mut super::d3d9types::D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_windef"))]
pub trait IDirect3D9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::Result<()>;
    fn GetAdapterModeCount(&self, adapter: u32, format: super::d3d9types::D3DFORMAT) -> u32;
    fn EnumAdapterModes(&self, adapter: u32, format: super::d3d9types::D3DFORMAT, mode: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn GetAdapterDisplayMode(&self, adapter: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn CheckDeviceType(&self, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, backbufferformat: super::d3d9types::D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::Result<()>;
    fn CheckDeviceFormat(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, usage: u32, rtype: super::d3d9types::D3DRESOURCETYPE, checkformat: super::d3d9types::D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, surfaceformat: super::d3d9types::D3DFORMAT, windowed: windows_core::BOOL, multisampletype: super::d3d9types::D3DMULTISAMPLE_TYPE) -> windows_core::Result<u32>;
    fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, rendertargetformat: super::d3d9types::D3DFORMAT, depthstencilformat: super::d3d9types::D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, sourceformat: super::d3d9types::D3DFORMAT, targetformat: super::d3d9types::D3DFORMAT) -> windows_core::Result<()>;
    fn GetDeviceCaps(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::Result<()>;
    fn GetAdapterMonitor(&self, adapter: u32) -> super::windef::HMONITOR;
    fn CreateDevice(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: windows_core::OutRef<IDirect3DDevice9>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl IDirect3D9_Vtbl {
    pub const fn new<Identity: IDirect3D9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::RegisterSoftwareDevice(this, core::mem::transmute_copy(&pinitializefunction)).into()
            }
        }
        unsafe extern "system" fn GetAdapterCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterCount(this)
            }
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterIdentifier(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pidentifier)).into()
            }
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: super::d3d9types::D3DFORMAT) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterModeCount(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format))
            }
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: super::d3d9types::D3DFORMAT, mode: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9_Impl::EnumAdapterModes(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format), core::mem::transmute_copy(&mode)) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9_Impl::GetAdapterDisplayMode(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckDeviceType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, backbufferformat: super::d3d9types::D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&backbufferformat), core::mem::transmute_copy(&bwindowed)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, usage: u32, rtype: super::d3d9types::D3DRESOURCETYPE, checkformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceFormat(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&rtype), core::mem::transmute_copy(&checkformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, surfaceformat: super::d3d9types::D3DFORMAT, windowed: windows_core::BOOL, multisampletype: super::d3d9types::D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9_Impl::CheckDeviceMultiSampleType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&surfaceformat), core::mem::transmute_copy(&windowed), core::mem::transmute_copy(&multisampletype)) {
                    Ok(ok__) => {
                        pqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, rendertargetformat: super::d3d9types::D3DFORMAT, depthstencilformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDepthStencilMatch(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&depthstencilformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, sourceformat: super::d3d9types::D3DFORMAT, targetformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceFormatConversion(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&sourceformat), core::mem::transmute_copy(&targetformat)).into()
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32) -> super::windef::HMONITOR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterMonitor(this, core::mem::transmute_copy(&adapter))
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CreateDevice(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDirect3D9 {}
windows_core::imp::define_interface!(IDirect3D9Ex, IDirect3D9Ex_Vtbl, 0x02177241_69fc_400c_8ff1_93a44df6861d);
impl core::ops::Deref for IDirect3D9Ex {
    type Target = IDirect3D9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3D9Ex, windows_core::IUnknown, IDirect3D9);
impl IDirect3D9Ex {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterSoftwareDevice)(windows_core::Interface::as_raw(self), pinitializefunction as _) }
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterIdentifier)(windows_core::Interface::as_raw(self), adapter, flags, pidentifier as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterModeCount(&self, adapter: u32, format: super::d3d9types::D3DFORMAT) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCount)(windows_core::Interface::as_raw(self), adapter, format) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn EnumAdapterModes(&self, adapter: u32, format: super::d3d9types::D3DFORMAT, mode: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapterModes)(windows_core::Interface::as_raw(self), adapter, format, mode, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapterDisplayMode)(windows_core::Interface::as_raw(self), adapter, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceType(&self, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, backbufferformat: super::d3d9types::D3DFORMAT, bwindowed: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceType)(windows_core::Interface::as_raw(self), adapter, devtype, adapterformat, backbufferformat, bwindowed.into()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceFormat(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, usage: u32, rtype: super::d3d9types::D3DRESOURCETYPE, checkformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormat)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, usage, rtype, checkformat) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, surfaceformat: super::d3d9types::D3DFORMAT, windowed: bool, multisampletype: super::d3d9types::D3DMULTISAMPLE_TYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckDeviceMultiSampleType)(windows_core::Interface::as_raw(self), adapter, devicetype, surfaceformat, windowed.into(), multisampletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, rendertargetformat: super::d3d9types::D3DFORMAT, depthstencilformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDepthStencilMatch)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, rendertargetformat, depthstencilformat) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, sourceformat: super::d3d9types::D3DFORMAT, targetformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormatConversion)(windows_core::Interface::as_raw(self), adapter, devicetype, sourceformat, targetformat) }
    }
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub unsafe fn GetDeviceCaps(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), adapter, devicetype, pcaps as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetAdapterMonitor(&self, adapter: u32) -> super::windef::HMONITOR {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterMonitor)(windows_core::Interface::as_raw(self), adapter) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn CreateDevice(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow, behaviorflags, ppresentationparameters as _, core::mem::transmute(ppreturneddeviceinterface)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const super::d3d9types::D3DDISPLAYMODEFILTER) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCountEx)(windows_core::Interface::as_raw(self), adapter, pfilter) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const super::d3d9types::D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterModesEx)(windows_core::Interface::as_raw(self), adapter, pfilter, mode, pmode as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDisplayModeEx)(windows_core::Interface::as_raw(self), adapter, pmode as _, protation as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn CreateDeviceEx(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9Ex>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeviceEx)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow, behaviorflags, ppresentationparameters as _, pfullscreendisplaymode as _, core::mem::transmute(ppreturneddeviceinterface)) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetAdapterLUID(&self, adapter: u32) -> windows_core::Result<super::winnt::LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapterLUID)(windows_core::Interface::as_raw(self), adapter, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9Ex_Vtbl {
    pub base__: IDirect3D9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub RegisterSoftwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterIdentifier: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterModeCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DFORMAT) -> u32,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterModeCount: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub EnumAdapterModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DFORMAT, u32, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    EnumAdapterModes: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterDisplayMode: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, u32, super::d3d9types::D3DRESOURCETYPE, super::d3d9types::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceFormat: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceMultiSampleType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, windows_core::BOOL, super::d3d9types::D3DMULTISAMPLE_TYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceMultiSampleType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDepthStencilMatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDepthStencilMatch: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CheckDeviceFormatConversion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::d3d9types::D3DFORMAT, super::d3d9types::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CheckDeviceFormatConversion: usize,
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types")))]
    GetDeviceCaps: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetAdapterMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::windef::HMONITOR,
    #[cfg(not(feature = "Win32_windef"))]
    GetAdapterMonitor: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::windef::HWND, u32, *mut super::d3d9types::D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    CreateDevice: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterModeCountEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d9types::D3DDISPLAYMODEFILTER) -> u32,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterModeCountEx: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub EnumAdapterModesEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d9types::D3DDISPLAYMODEFILTER, u32, *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    EnumAdapterModesEx: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAdapterDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DDISPLAYMODEEX, *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAdapterDisplayModeEx: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub CreateDeviceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, super::windef::HWND, u32, *mut super::d3d9types::D3DPRESENT_PARAMETERS, *mut super::d3d9types::D3DDISPLAYMODEEX, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    CreateDeviceEx: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetAdapterLUID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::winnt::LUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetAdapterLUID: usize,
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDirect3D9Ex_Impl: IDirect3D9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::Result<()>;
    fn GetAdapterModeCount(&self, adapter: u32, format: super::d3d9types::D3DFORMAT) -> u32;
    fn EnumAdapterModes(&self, adapter: u32, format: super::d3d9types::D3DFORMAT, mode: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn GetAdapterDisplayMode(&self, adapter: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn CheckDeviceType(&self, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, backbufferformat: super::d3d9types::D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::Result<()>;
    fn CheckDeviceFormat(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, usage: u32, rtype: super::d3d9types::D3DRESOURCETYPE, checkformat: super::d3d9types::D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, surfaceformat: super::d3d9types::D3DFORMAT, windowed: windows_core::BOOL, multisampletype: super::d3d9types::D3DMULTISAMPLE_TYPE) -> windows_core::Result<u32>;
    fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, rendertargetformat: super::d3d9types::D3DFORMAT, depthstencilformat: super::d3d9types::D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, sourceformat: super::d3d9types::D3DFORMAT, targetformat: super::d3d9types::D3DFORMAT) -> windows_core::Result<()>;
    fn GetDeviceCaps(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::Result<()>;
    fn GetAdapterMonitor(&self, adapter: u32) -> super::windef::HMONITOR;
    fn CreateDevice(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: windows_core::OutRef<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const super::d3d9types::D3DDISPLAYMODEFILTER) -> u32;
    fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const super::d3d9types::D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::Result<()>;
    fn CreateDeviceEx(&self, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX, ppreturneddeviceinterface: windows_core::OutRef<IDirect3DDevice9Ex>) -> windows_core::Result<()>;
    fn GetAdapterLUID(&self, adapter: u32) -> windows_core::Result<super::winnt::LUID>;
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDirect3D9Ex_Vtbl {
    pub const fn new<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::Release(this)
            }
        }
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::RegisterSoftwareDevice(this, core::mem::transmute_copy(&pinitializefunction)).into()
            }
        }
        unsafe extern "system" fn GetAdapterCount<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterCount(this)
            }
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut super::d3d9types::D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterIdentifier(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pidentifier)).into()
            }
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: super::d3d9types::D3DFORMAT) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterModeCount(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format))
            }
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: super::d3d9types::D3DFORMAT, mode: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9Ex_Impl::EnumAdapterModes(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format), core::mem::transmute_copy(&mode)) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9Ex_Impl::GetAdapterDisplayMode(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckDeviceType<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, backbufferformat: super::d3d9types::D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CheckDeviceType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&backbufferformat), core::mem::transmute_copy(&bwindowed)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, usage: u32, rtype: super::d3d9types::D3DRESOURCETYPE, checkformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CheckDeviceFormat(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&rtype), core::mem::transmute_copy(&checkformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, surfaceformat: super::d3d9types::D3DFORMAT, windowed: windows_core::BOOL, multisampletype: super::d3d9types::D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9Ex_Impl::CheckDeviceMultiSampleType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&surfaceformat), core::mem::transmute_copy(&windowed), core::mem::transmute_copy(&multisampletype)) {
                    Ok(ok__) => {
                        pqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, adapterformat: super::d3d9types::D3DFORMAT, rendertargetformat: super::d3d9types::D3DFORMAT, depthstencilformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CheckDepthStencilMatch(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&depthstencilformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, sourceformat: super::d3d9types::D3DFORMAT, targetformat: super::d3d9types::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CheckDeviceFormatConversion(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&sourceformat), core::mem::transmute_copy(&targetformat)).into()
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32) -> super::windef::HMONITOR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterMonitor(this, core::mem::transmute_copy(&adapter))
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CreateDevice(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pfilter: *const super::d3d9types::D3DDISPLAYMODEFILTER) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterModeCountEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter))
            }
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pfilter: *const super::d3d9types::D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::EnumAdapterModesEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterDisplayModeEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: super::d3d9types::D3DDEVTYPE, hfocuswindow: super::windef::HWND, behaviorflags: u32, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pluid: *mut super::winnt::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3D9Ex_Impl::GetAdapterLUID(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        pluid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDirect3D9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirect3D9Ex {}
windows_core::imp::define_interface!(IDirect3D9ExOverlayExtension, IDirect3D9ExOverlayExtension_Vtbl, 0x187aeb13_aaf5_4c59_876d_e059088c0df8);
windows_core::imp::interface_hierarchy!(IDirect3D9ExOverlayExtension, windows_core::IUnknown);
impl IDirect3D9ExOverlayExtension {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub unsafe fn CheckDeviceOverlayType(&self, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, overlaywidth: u32, overlayheight: u32, overlayformat: super::d3d9types::D3DFORMAT, pdisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX, displayrotation: super::d3d9types::D3DDISPLAYROTATION, poverlaycaps: *mut super::d3d9caps::D3DOVERLAYCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceOverlayType)(windows_core::Interface::as_raw(self), adapter, devtype, overlaywidth, overlayheight, overlayformat, pdisplaymode as _, displayrotation, poverlaycaps as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9ExOverlayExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub CheckDeviceOverlayType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DDEVTYPE, u32, u32, super::d3d9types::D3DFORMAT, *mut super::d3d9types::D3DDISPLAYMODEEX, super::d3d9types::D3DDISPLAYROTATION, *mut super::d3d9caps::D3DOVERLAYCAPS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types")))]
    CheckDeviceOverlayType: usize,
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
pub trait IDirect3D9ExOverlayExtension_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn CheckDeviceOverlayType(&self, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, overlaywidth: u32, overlayheight: u32, overlayformat: super::d3d9types::D3DFORMAT, pdisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX, displayrotation: super::d3d9types::D3DDISPLAYROTATION, poverlaycaps: *mut super::d3d9caps::D3DOVERLAYCAPS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
impl IDirect3D9ExOverlayExtension_Vtbl {
    pub const fn new<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9ExOverlayExtension_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9ExOverlayExtension_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9ExOverlayExtension_Impl::Release(this)
            }
        }
        unsafe extern "system" fn CheckDeviceOverlayType<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devtype: super::d3d9types::D3DDEVTYPE, overlaywidth: u32, overlayheight: u32, overlayformat: super::d3d9types::D3DFORMAT, pdisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX, displayrotation: super::d3d9types::D3DDISPLAYROTATION, poverlaycaps: *mut super::d3d9caps::D3DOVERLAYCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9ExOverlayExtension_Impl::CheckDeviceOverlayType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&overlaywidth), core::mem::transmute_copy(&overlayheight), core::mem::transmute_copy(&overlayformat), core::mem::transmute_copy(&pdisplaymode), core::mem::transmute_copy(&displayrotation), core::mem::transmute_copy(&poverlaycaps)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            CheckDeviceOverlayType: CheckDeviceOverlayType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9ExOverlayExtension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
impl windows_core::RuntimeName for IDirect3D9ExOverlayExtension {}
windows_core::imp::define_interface!(IDirect3DAuthenticatedChannel9, IDirect3DAuthenticatedChannel9_Vtbl, 0xff24beee_da21_4beb_98b5_d2f899f98af9);
windows_core::imp::interface_hierarchy!(IDirect3DAuthenticatedChannel9, windows_core::IUnknown);
impl IDirect3DAuthenticatedChannel9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCertificateSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCertificate(&self, certifactesize: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), certifactesize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NegotiateKeyExchange(&self, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NegotiateKeyExchange)(windows_core::Interface::as_raw(self), datasize, pdata as _) }
    }
    pub unsafe fn Query(&self, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), inputsize, pinput, outputsize, poutput as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn Configure(&self, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut super::d3d9types::D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), inputsize, pinput, poutput as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DAuthenticatedChannel9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub NegotiateKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut super::d3d9types::D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    Configure: usize,
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
pub trait IDirect3DAuthenticatedChannel9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetCertificateSize(&self) -> windows_core::Result<u32>;
    fn GetCertificate(&self, certifactesize: u32) -> windows_core::Result<u8>;
    fn NegotiateKeyExchange(&self, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Query(&self, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Configure(&self, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut super::d3d9types::D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
impl IDirect3DAuthenticatedChannel9_Vtbl {
    pub const fn new<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetCertificateSize<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DAuthenticatedChannel9_Impl::GetCertificateSize(this) {
                    Ok(ok__) => {
                        pcertificatesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certifactesize: u32, ppcertificate: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DAuthenticatedChannel9_Impl::GetCertificate(this, core::mem::transmute_copy(&certifactesize)) {
                    Ok(ok__) => {
                        ppcertificate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NegotiateKeyExchange<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::NegotiateKeyExchange(this, core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::Query(this, core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&outputsize), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn Configure<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut super::d3d9types::D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::Configure(this, core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetCertificateSize: GetCertificateSize::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            NegotiateKeyExchange: NegotiateKeyExchange::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DAuthenticatedChannel9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirect3DAuthenticatedChannel9 {}
windows_core::imp::define_interface!(IDirect3DBaseTexture9, IDirect3DBaseTexture9_Vtbl, 0x580ca87e_1d3c_4d54_991d_b7d3e3c298ce);
impl core::ops::Deref for IDirect3DBaseTexture9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DBaseTexture9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DBaseTexture9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), lodnew) }
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoGenFilterType)(windows_core::Interface::as_raw(self), filtertype) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetAutoGenFilterType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMipSubLevels)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DBaseTexture9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetAutoGenFilterType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAutoGenFilterType: usize,
    pub GenerateMipSubLevels: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DBaseTexture9_Impl: IDirect3DResource9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DBaseTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DBaseTexture9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn SetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetLOD(this)
            }
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetLevelCount(this)
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetAutoGenFilterType(this)
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GenerateMipSubLevels(this);
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
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
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DBaseTexture9 {}
windows_core::imp::define_interface!(IDirect3DCryptoSession9, IDirect3DCryptoSession9_Vtbl, 0xfa0ab799_7a9c_48ca_8c5b_237e71a54434);
windows_core::imp::interface_hierarchy!(IDirect3DCryptoSession9, windows_core::IUnknown);
impl IDirect3DCryptoSession9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCertificateSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCertificate(&self, certifactesize: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), certifactesize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NegotiateKeyExchange(&self, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NegotiateKeyExchange)(windows_core::Interface::as_raw(self), datasize, pdata as _) }
    }
    pub unsafe fn EncryptionBlt<P0, P1>(&self, psrcsurface: P0, pdstsurface: P1, dstsurfacesize: u32, piv: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncryptionBlt)(windows_core::Interface::as_raw(self), psrcsurface.param().abi(), pdstsurface.param().abi(), dstsurfacesize, piv as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DecryptionBlt<P0, P1>(&self, psrcsurface: P0, pdstsurface: P1, srcsurfacesize: u32, pencryptedblockinfo: *mut super::d3d9types::D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut core::ffi::c_void, piv: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecryptionBlt)(windows_core::Interface::as_raw(self), psrcsurface.param().abi(), pdstsurface.param().abi(), srcsurfacesize, pencryptedblockinfo as _, pcontentkey as _, piv as _) }
    }
    pub unsafe fn GetSurfacePitch<P0>(&self, psrcsurface: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurfacePitch)(windows_core::Interface::as_raw(self), psrcsurface.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartSessionKeyRefresh(&self, prandomnumber: *mut core::ffi::c_void, randomnumbersize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartSessionKeyRefresh)(windows_core::Interface::as_raw(self), prandomnumber as _, randomnumbersize) }
    }
    pub unsafe fn FinishSessionKeyRefresh(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FinishSessionKeyRefresh)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetEncryptionBltKey(&self, preadbackkey: *mut core::ffi::c_void, keysize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEncryptionBltKey)(windows_core::Interface::as_raw(self), preadbackkey as _, keysize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DCryptoSession9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub NegotiateKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub DecryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::d3d9types::D3DENCRYPTED_BLOCK_INFO, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DecryptionBlt: usize,
    pub GetSurfacePitch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub StartSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FinishSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEncryptionBltKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DCryptoSession9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetCertificateSize(&self) -> windows_core::Result<u32>;
    fn GetCertificate(&self, certifactesize: u32) -> windows_core::Result<u8>;
    fn NegotiateKeyExchange(&self, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EncryptionBlt(&self, psrcsurface: windows_core::Ref<IDirect3DSurface9>, pdstsurface: windows_core::Ref<IDirect3DSurface9>, dstsurfacesize: u32, piv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DecryptionBlt(&self, psrcsurface: windows_core::Ref<IDirect3DSurface9>, pdstsurface: windows_core::Ref<IDirect3DSurface9>, srcsurfacesize: u32, pencryptedblockinfo: *mut super::d3d9types::D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut core::ffi::c_void, piv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSurfacePitch(&self, psrcsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<u32>;
    fn StartSessionKeyRefresh(&self, prandomnumber: *mut core::ffi::c_void, randomnumbersize: u32) -> windows_core::Result<()>;
    fn FinishSessionKeyRefresh(&self) -> windows_core::Result<()>;
    fn GetEncryptionBltKey(&self, preadbackkey: *mut core::ffi::c_void, keysize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DCryptoSession9_Vtbl {
    pub const fn new<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetCertificateSize<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DCryptoSession9_Impl::GetCertificateSize(this) {
                    Ok(ok__) => {
                        pcertificatesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certifactesize: u32, ppcertificate: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DCryptoSession9_Impl::GetCertificate(this, core::mem::transmute_copy(&certifactesize)) {
                    Ok(ok__) => {
                        ppcertificate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NegotiateKeyExchange<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::NegotiateKeyExchange(this, core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn EncryptionBlt<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, dstsurfacesize: u32, piv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::EncryptionBlt(this, core::mem::transmute_copy(&psrcsurface), core::mem::transmute_copy(&pdstsurface), core::mem::transmute_copy(&dstsurfacesize), core::mem::transmute_copy(&piv)).into()
            }
        }
        unsafe extern "system" fn DecryptionBlt<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, srcsurfacesize: u32, pencryptedblockinfo: *mut super::d3d9types::D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut core::ffi::c_void, piv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::DecryptionBlt(this, core::mem::transmute_copy(&psrcsurface), core::mem::transmute_copy(&pdstsurface), core::mem::transmute_copy(&srcsurfacesize), core::mem::transmute_copy(&pencryptedblockinfo), core::mem::transmute_copy(&pcontentkey), core::mem::transmute_copy(&piv)).into()
            }
        }
        unsafe extern "system" fn GetSurfacePitch<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, psurfacepitch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DCryptoSession9_Impl::GetSurfacePitch(this, core::mem::transmute_copy(&psrcsurface)) {
                    Ok(ok__) => {
                        psurfacepitch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomnumber: *mut core::ffi::c_void, randomnumbersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::StartSessionKeyRefresh(this, core::mem::transmute_copy(&prandomnumber), core::mem::transmute_copy(&randomnumbersize)).into()
            }
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::FinishSessionKeyRefresh(this).into()
            }
        }
        unsafe extern "system" fn GetEncryptionBltKey<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadbackkey: *mut core::ffi::c_void, keysize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::GetEncryptionBltKey(this, core::mem::transmute_copy(&preadbackkey), core::mem::transmute_copy(&keysize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetCertificateSize: GetCertificateSize::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            NegotiateKeyExchange: NegotiateKeyExchange::<Identity, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, OFFSET>,
            GetSurfacePitch: GetSurfacePitch::<Identity, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DCryptoSession9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DCryptoSession9 {}
windows_core::imp::define_interface!(IDirect3DCubeTexture9, IDirect3DCubeTexture9_Vtbl, 0xfff32f81_d953_473a_9223_93d652aba93f);
impl core::ops::Deref for IDirect3DCubeTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DCubeTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DCubeTexture9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), lodnew) }
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoGenFilterType)(windows_core::Interface::as_raw(self), filtertype) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetAutoGenFilterType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMipSubLevels)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetCubeMapSurface(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCubeMapSurface)(windows_core::Interface::as_raw(self), facetype, level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn LockRect(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), facetype, level, plockedrect as _, prect, flags) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn UnlockRect(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self), facetype, level) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn AddDirtyRect(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, pdirtyrect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyRect)(windows_core::Interface::as_raw(self), facetype, pdirtyrect) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DCubeTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetAutoGenFilterType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAutoGenFilterType: usize,
    pub GenerateMipSubLevels: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetLevelDesc: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetCubeMapSurface: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DCUBEMAP_FACES, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetCubeMapSurface: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DCUBEMAP_FACES, u32, *mut super::d3d9types::D3DLOCKED_RECT, *const super::windef::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    LockRect: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DCUBEMAP_FACES, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    UnlockRect: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub AddDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DCUBEMAP_FACES, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    AddDirtyRect: usize,
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
pub trait IDirect3DCubeTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
    fn GetLevelDesc(&self, level: u32, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetCubeMapSurface(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn LockRect(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, facetype: super::d3d9types::D3DCUBEMAP_FACES, pdirtyrect: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl IDirect3DCubeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DCubeTexture9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn SetLOD<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetLOD(this)
            }
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetLevelCount(this)
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetAutoGenFilterType(this)
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GenerateMipSubLevels(this);
            }
        }
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DCubeTexture9_Impl::GetCubeMapSurface(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        ppcubemapsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::LockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: super::d3d9types::D3DCUBEMAP_FACES, level: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: super::d3d9types::D3DCUBEMAP_FACES, pdirtyrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDirect3DCubeTexture9 {}
windows_core::imp::define_interface!(IDirect3DDevice9, IDirect3DDevice9_Vtbl, 0xd0223b96_bf7a_43fd_92bd_a43b0d82b9eb);
windows_core::imp::interface_hierarchy!(IDirect3DDevice9, windows_core::IUnknown);
impl IDirect3DDevice9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TestCooperativeLevel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestCooperativeLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAvailableTextureMem(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableTextureMem)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EvictManagedResources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EvictManagedResources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirect3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDisplayMode(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn GetCreationParameters(&self) -> windows_core::Result<super::d3d9types::D3DDEVICE_CREATION_PARAMETERS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCursorProperties<P2>(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCursorProperties)(windows_core::Interface::as_raw(self), xhotspot, yhotspot, pcursorbitmap.param().abi()) }
    }
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetCursorPosition)(windows_core::Interface::as_raw(self), x, y, flags);
        }
    }
    pub unsafe fn ShowCursor(&self, bshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).ShowCursor)(windows_core::Interface::as_raw(self), bshow.into()) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAdditionalSwapChain)(windows_core::Interface::as_raw(self), ppresentationparameters as _, core::mem::transmute(pswapchain)) }
    }
    pub unsafe fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSwapChain)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNumberOfSwapChains(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumberOfSwapChains)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn Reset(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), ppresentationparameters as _) }
    }
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub unsafe fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), iswapchain, ibackbuffer, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetRasterStatus(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDialogBoxMode(&self, benabledialogs: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDialogBoxMode)(windows_core::Interface::as_raw(self), benabledialogs.into()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const super::d3d9types::D3DGAMMARAMP) {
        unsafe {
            (windows_core::Interface::vtable(self).SetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, flags, pramp);
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut super::d3d9types::D3DGAMMARAMP) {
        unsafe {
            (windows_core::Interface::vtable(self).GetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, pramp as _);
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, pptexture: *mut Option<IDirect3DTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture)(windows_core::Interface::as_raw(self), width, height, levels, usage, format, pool, core::mem::transmute(pptexture), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVolumeTexture)(windows_core::Interface::as_raw(self), width, height, depth, levels, usage, format, pool, core::mem::transmute(ppvolumetexture), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppcubetexture: *mut Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCubeTexture)(windows_core::Interface::as_raw(self), edgelength, levels, usage, format, pool, core::mem::transmute(ppcubetexture), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: super::d3d9types::D3DPOOL, ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexBuffer)(windows_core::Interface::as_raw(self), length, usage, fvf, pool, core::mem::transmute(ppvertexbuffer), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateIndexBuffer(&self, length: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateIndexBuffer)(windows_core::Interface::as_raw(self), length, usage, format, pool, core::mem::transmute(ppindexbuffer), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateRenderTarget(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTarget)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable.into(), core::mem::transmute(ppsurface), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurface)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard.into(), core::mem::transmute(ppsurface), psharedhandle as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn UpdateSurface<P0, P2>(&self, psourcesurface: P0, psourcerect: *const super::windef::RECT, pdestinationsurface: P2, pdestpoint: *const super::windef::POINT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateSurface)(windows_core::Interface::as_raw(self), psourcesurface.param().abi(), psourcerect, pdestinationsurface.param().abi(), pdestpoint) }
    }
    pub unsafe fn UpdateTexture<P0, P1>(&self, psourcetexture: P0, pdestinationtexture: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DBaseTexture9>,
        P1: windows_core::Param<IDirect3DBaseTexture9>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateTexture)(windows_core::Interface::as_raw(self), psourcetexture.param().abi(), pdestinationtexture.param().abi()) }
    }
    pub unsafe fn GetRenderTargetData<P0, P1>(&self, prendertarget: P0, pdestsurface: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTargetData)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), pdestsurface.param().abi()) }
    }
    pub unsafe fn GetFrontBufferData<P1>(&self, iswapchain: u32, pdestsurface: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), iswapchain, pdestsurface.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn StretchRect<P0, P2>(&self, psourcesurface: P0, psourcerect: *const super::windef::RECT, pdestsurface: P2, pdestrect: *const super::windef::RECT, filter: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).StretchRect)(windows_core::Interface::as_raw(self), psourcesurface.param().abi(), psourcerect, pdestsurface.param().abi(), pdestrect, filter) }
    }
    #[cfg(all(feature = "Win32_dsound", feature = "Win32_windef"))]
    pub unsafe fn ColorFill<P0>(&self, psurface: P0, prect: *const super::windef::RECT, color: super::dsound::D3DCOLOR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ColorFill)(windows_core::Interface::as_raw(self), psurface.param().abi(), prect, color) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurface)(windows_core::Interface::as_raw(self), width, height, format, pool, core::mem::transmute(ppsurface), psharedhandle as _) }
    }
    pub unsafe fn SetRenderTarget<P1>(&self, rendertargetindex: u32, prendertarget: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, prendertarget.param().abi()) }
    }
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDepthStencilSurface<P0>(&self, pnewzstencil: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencilSurface)(windows_core::Interface::as_raw(self), pnewzstencil.param().abi()) }
    }
    pub unsafe fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencilSurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginScene(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginScene)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndScene(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndScene)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound"))]
    pub unsafe fn Clear(&self, count: u32, prects: *const super::d3d9types::D3DRECT, flags: u32, color: super::dsound::D3DCOLOR, z: f32, stencil: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), count, prects, flags, color, z, stencil) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), state, pmatrix) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), state, pmatrix as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn MultiplyTransform(&self, param0: super::d3d9types::D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MultiplyTransform)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetViewport(&self, pviewport: *const super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewport)(windows_core::Interface::as_raw(self), pviewport) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetViewport(&self, pviewport: *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), pviewport as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub unsafe fn SetMaterial(&self, pmaterial: *const super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaterial)(windows_core::Interface::as_raw(self), pmaterial) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub unsafe fn GetMaterial(&self, pmaterial: *mut super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaterial)(windows_core::Interface::as_raw(self), pmaterial as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub unsafe fn SetLight(&self, index: u32, param1: *const super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLight)(windows_core::Interface::as_raw(self), index, param1) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub unsafe fn GetLight(&self, index: u32, param1: *mut super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLight)(windows_core::Interface::as_raw(self), index, param1 as _) }
    }
    pub unsafe fn LightEnable(&self, index: u32, enable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LightEnable)(windows_core::Interface::as_raw(self), index, enable.into()) }
    }
    pub unsafe fn GetLightEnable(&self, index: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLightEnable)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipPlane)(windows_core::Interface::as_raw(self), index, pplane) }
    }
    pub unsafe fn GetClipPlane(&self, index: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipPlane)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderState)(windows_core::Interface::as_raw(self), state, value) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderState)(windows_core::Interface::as_raw(self), state, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CreateStateBlock(&self, r#type: super::d3d9types::D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStateBlock)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginStateBlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginStateBlock)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndStateBlock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipStatus)(windows_core::Interface::as_raw(self), pclipstatus) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetClipStatus(&self) -> windows_core::Result<super::d3d9types::D3DCLIPSTATUS9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTexture)(windows_core::Interface::as_raw(self), stage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTexture<P1>(&self, stage: u32, ptexture: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DBaseTexture9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTexture)(windows_core::Interface::as_raw(self), stage, ptexture.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, value) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, value) }
    }
    pub unsafe fn ValidateDevice(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::wingdi::PALETTEENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, pentries) }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: u32) -> windows_core::Result<super::wingdi::PALETTEENTRY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTexturePalette)(windows_core::Interface::as_raw(self), palettenumber) }
    }
    pub unsafe fn GetCurrentTexturePalette(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentTexturePalette)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetScissorRect(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScissorRect)(windows_core::Interface::as_raw(self), prect) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetScissorRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScissorRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSoftwareVertexProcessing(&self, bsoftware: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self), bsoftware.into()) }
    }
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNPatchMode)(windows_core::Interface::as_raw(self), nsegments) }
    }
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetNPatchMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawPrimitive(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitive)(windows_core::Interface::as_raw(self), primitivetype, startvertex, primitivecount) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawIndexedPrimitive(&self, param0: super::d3d9types::D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitive)(windows_core::Interface::as_raw(self), param0, basevertexindex, minvertexindex, numvertices, startindex, primcount) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, primitivecount, pvertexstreamzerodata, vertexstreamzerostride) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: super::d3d9types::D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, minvertexindex, numvertices, primitivecount, pindexdata, indexdataformat, pvertexstreamzerodata, vertexstreamzerostride) }
    }
    pub unsafe fn ProcessVertices<P3, P4>(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: P3, pvertexdecl: P4, flags: u32) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IDirect3DVertexBuffer9>,
        P4: windows_core::Param<IDirect3DVertexDeclaration9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessVertices)(windows_core::Interface::as_raw(self), srcstartindex, destindex, vertexcount, pdestbuffer.param().abi(), pvertexdecl.param().abi(), flags) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const super::d3d9types::D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVertexDeclaration)(windows_core::Interface::as_raw(self), pvertexelements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexDeclaration<P0>(&self, pdecl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DVertexDeclaration9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVertexDeclaration)(windows_core::Interface::as_raw(self), pdecl.param().abi()) }
    }
    pub unsafe fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexDeclaration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFVF(&self, fvf: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFVF)(windows_core::Interface::as_raw(self), fvf) }
    }
    pub unsafe fn GetFVF(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFVF)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), pfunction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexShader<P0>(&self, pshader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DVertexShader9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShader)(windows_core::Interface::as_raw(self), pshader.param().abi()) }
    }
    pub unsafe fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4fcount) }
    }
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4icount) }
    }
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata, boolcount) }
    }
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    pub unsafe fn SetStreamSource<P1>(&self, streamnumber: u32, pstreamdata: P1, offsetinbytes: u32, stride: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DVertexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, pstreamdata.param().abi(), offsetinbytes, stride) }
    }
    pub unsafe fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, core::mem::transmute(ppstreamdata), poffsetinbytes as _, pstride as _) }
    }
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, setting) }
    }
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndices<P0>(&self, pindexdata: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DIndexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIndices)(windows_core::Interface::as_raw(self), pindexdata.param().abi()) }
    }
    pub unsafe fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), pfunction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPixelShader<P0>(&self, pshader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DPixelShader9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShader)(windows_core::Interface::as_raw(self), pshader.param().abi()) }
    }
    pub unsafe fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4fcount) }
    }
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4icount) }
    }
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata, boolcount) }
    }
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawRectPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs, prectpatchinfo) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawTriPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs, ptripatchinfo) }
    }
    pub unsafe fn DeletePatch(&self, handle: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePatch)(windows_core::Interface::as_raw(self), handle) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CreateQuery(&self, r#type: super::d3d9types::D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub TestCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAvailableTextureMem: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub EvictManagedResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDirect3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types")))]
    GetDeviceCaps: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDisplayMode: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    GetCreationParameters: usize,
    pub SetCursorProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCursorPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32),
    pub ShowCursor: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub CreateAdditionalSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    CreateAdditionalSwapChain: usize,
    pub GetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumberOfSwapChains: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    Reset: usize,
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *const super::windef::RECT, super::windef::HWND, *const super::wingdi::RGNDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_windef", feature = "Win32_wingdi")))]
    Present: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetBackBuffer: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetRasterStatus: usize,
    pub SetDialogBoxMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::d3d9types::D3DGAMMARAMP),
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetGammaRamp: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DGAMMARAMP),
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetGammaRamp: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateTexture: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateVolumeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateVolumeTexture: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateCubeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateCubeTexture: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateVertexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateVertexBuffer: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateIndexBuffer: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateRenderTarget: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateDepthStencilSurface: usize,
    #[cfg(feature = "Win32_windef")]
    pub UpdateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, *mut core::ffi::c_void, *const super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    UpdateSurface: usize,
    pub UpdateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTargetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub StretchRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, *mut core::ffi::c_void, *const super::windef::RECT, super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    StretchRect: usize,
    #[cfg(all(feature = "Win32_dsound", feature = "Win32_windef"))]
    pub ColorFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, super::dsound::D3DCOLOR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dsound", feature = "Win32_windef")))]
    ColorFill: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateOffscreenPlainSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateOffscreenPlainSurface: usize,
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound"))]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d9types::D3DRECT, u32, super::dsound::D3DCOLOR, f32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dsound")))]
    Clear: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetTransform: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTRANSFORMSTATETYPE, *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetTransform: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub MultiplyTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    MultiplyTransform: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetViewport: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetViewport: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub SetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype")))]
    SetMaterial: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub GetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype")))]
    GetMaterial: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub SetLight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype")))]
    SetLight: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub GetLight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype")))]
    GetLight: usize,
    pub LightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DRENDERSTATETYPE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetRenderState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DRENDERSTATETYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetRenderState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CreateStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DSTATEBLOCKTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CreateStateBlock: usize,
    pub BeginStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetClipStatus: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetClipStatus: usize,
    pub GetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DTEXTURESTAGESTATETYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetTextureStageState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DTEXTURESTAGESTATETYPE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetTextureStageState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DSAMPLERSTATETYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetSamplerState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DSAMPLERSTATETYPE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetSamplerState: usize,
    pub ValidateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wingdi")]
    pub SetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wingdi::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    SetPaletteEntries: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub GetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::wingdi::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    GetPaletteEntries: usize,
    pub SetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetScissorRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetScissorRect: usize,
    pub SetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawPrimitive: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawIndexedPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, i32, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawIndexedPrimitive: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawPrimitiveUP: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawIndexedPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, u32, u32, u32, *const core::ffi::c_void, super::d3d9types::D3DFORMAT, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawIndexedPrimitiveUP: usize,
    pub ProcessVertices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub CreateVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DVERTEXELEMENT9, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CreateVertexDeclaration: usize,
    pub SetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, u32) -> windows_core::HRESULT,
    pub SetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub SetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, u32) -> windows_core::HRESULT,
    pub SetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawRectPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawRectPatch: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawTriPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawTriPatch: usize,
    pub DeletePatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DQUERYTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CreateQuery: usize,
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub trait IDirect3DDevice9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetAvailableTextureMem(&self) -> u32;
    fn EvictManagedResources(&self) -> windows_core::Result<()>;
    fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9>;
    fn GetDeviceCaps(&self, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn GetCreationParameters(&self) -> windows_core::Result<super::d3d9types::D3DDEVICE_CREATION_PARAMETERS>;
    fn SetCursorProperties(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn SetCursorPosition(&self, x: i32, y: i32, flags: u32);
    fn ShowCursor(&self, bshow: windows_core::BOOL) -> windows_core::BOOL;
    fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pswapchain: windows_core::OutRef<IDirect3DSwapChain9>) -> windows_core::Result<()>;
    fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(&self) -> u32;
    fn Reset(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS>;
    fn SetDialogBoxMode(&self, benabledialogs: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const super::d3d9types::D3DGAMMARAMP);
    fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut super::d3d9types::D3DGAMMARAMP);
    fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, pptexture: windows_core::OutRef<IDirect3DTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppvolumetexture: windows_core::OutRef<IDirect3DVolumeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppcubetexture: windows_core::OutRef<IDirect3DCubeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: super::d3d9types::D3DPOOL, ppvertexbuffer: windows_core::OutRef<IDirect3DVertexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateIndexBuffer(&self, length: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppindexbuffer: windows_core::OutRef<IDirect3DIndexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateRenderTarget(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn UpdateSurface(&self, psourcesurface: windows_core::Ref<IDirect3DSurface9>, psourcerect: *const super::windef::RECT, pdestinationsurface: windows_core::Ref<IDirect3DSurface9>, pdestpoint: *const super::windef::POINT) -> windows_core::Result<()>;
    fn UpdateTexture(&self, psourcetexture: windows_core::Ref<IDirect3DBaseTexture9>, pdestinationtexture: windows_core::Ref<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetRenderTargetData(&self, prendertarget: windows_core::Ref<IDirect3DSurface9>, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, iswapchain: u32, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn StretchRect(&self, psourcesurface: windows_core::Ref<IDirect3DSurface9>, psourcerect: *const super::windef::RECT, pdestsurface: windows_core::Ref<IDirect3DSurface9>, pdestrect: *const super::windef::RECT, filter: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn ColorFill(&self, psurface: windows_core::Ref<IDirect3DSurface9>, prect: *const super::windef::RECT, color: super::dsound::D3DCOLOR) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn SetRenderTarget(&self, rendertargetindex: u32, prendertarget: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(&self, pnewzstencil: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9>;
    fn BeginScene(&self) -> windows_core::Result<()>;
    fn EndScene(&self) -> windows_core::Result<()>;
    fn Clear(&self, count: u32, prects: *const super::d3d9types::D3DRECT, flags: u32, color: super::dsound::D3DCOLOR, z: f32, stencil: u32) -> windows_core::Result<()>;
    fn SetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn GetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn MultiplyTransform(&self, param0: super::d3d9types::D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetViewport(&self, pviewport: *const super::d3d9types::D3DVIEWPORT9) -> windows_core::Result<()>;
    fn GetViewport(&self, pviewport: *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::Result<()>;
    fn SetMaterial(&self, pmaterial: *const super::d3d9types::D3DMATERIAL9) -> windows_core::Result<()>;
    fn GetMaterial(&self, pmaterial: *mut super::d3d9types::D3DMATERIAL9) -> windows_core::Result<()>;
    fn SetLight(&self, index: u32, param1: *const super::d3d9types::D3DLIGHT9) -> windows_core::Result<()>;
    fn GetLight(&self, index: u32, param1: *mut super::d3d9types::D3DLIGHT9) -> windows_core::Result<()>;
    fn LightEnable(&self, index: u32, enable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLightEnable(&self, index: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::Result<()>;
    fn GetClipPlane(&self, index: u32) -> windows_core::Result<f32>;
    fn SetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE) -> windows_core::Result<u32>;
    fn CreateStateBlock(&self, r#type: super::d3d9types::D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(&self) -> windows_core::Result<()>;
    fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(&self, pclipstatus: *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetClipStatus(&self) -> windows_core::Result<super::d3d9types::D3DCLIPSTATUS9>;
    fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(&self, stage: u32, ptexture: windows_core::Ref<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE) -> windows_core::Result<u32>;
    fn SetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE) -> windows_core::Result<u32>;
    fn SetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn ValidateDevice(&self) -> windows_core::Result<u32>;
    fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::wingdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn GetPaletteEntries(&self, palettenumber: u32) -> windows_core::Result<super::wingdi::PALETTEENTRY>;
    fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::Result<()>;
    fn GetCurrentTexturePalette(&self) -> windows_core::Result<u32>;
    fn SetScissorRect(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn GetScissorRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn SetSoftwareVertexProcessing(&self, bsoftware: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL;
    fn SetNPatchMode(&self, nsegments: f32) -> windows_core::Result<()>;
    fn GetNPatchMode(&self) -> f32;
    fn DrawPrimitive(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitive(&self, param0: super::d3d9types::D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::Result<()>;
    fn DrawPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: super::d3d9types::D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn ProcessVertices(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: windows_core::Ref<IDirect3DVertexBuffer9>, pvertexdecl: windows_core::Ref<IDirect3DVertexDeclaration9>, flags: u32) -> windows_core::Result<()>;
    fn CreateVertexDeclaration(&self, pvertexelements: *const super::d3d9types::D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(&self, pdecl: windows_core::Ref<IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(&self, fvf: u32) -> windows_core::Result<()>;
    fn GetFVF(&self) -> windows_core::Result<u32>;
    fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(&self, pshader: windows_core::Ref<IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn SetStreamSource(&self, streamnumber: u32, pstreamdata: windows_core::Ref<IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> windows_core::Result<()>;
    fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: windows_core::OutRef<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::Result<()>;
    fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::Result<()>;
    fn GetStreamSourceFreq(&self, streamnumber: u32) -> windows_core::Result<u32>;
    fn SetIndices(&self, pindexdata: windows_core::Ref<IDirect3DIndexBuffer9>) -> windows_core::Result<()>;
    fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(&self, pshader: windows_core::Ref<IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::Result<()>;
    fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::Result<()>;
    fn DeletePatch(&self, handle: u32) -> windows_core::Result<()>;
    fn CreateQuery(&self, r#type: super::d3d9types::D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9>;
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl IDirect3DDevice9_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::TestCooperativeLevel(this).into()
            }
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetAvailableTextureMem(this)
            }
        }
        unsafe extern "system" fn EvictManagedResources<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EvictManagedResources(this).into()
            }
        }
        unsafe extern "system" fn GetDirect3D<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppd3d9: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetDirect3D(this) {
                    Ok(ok__) => {
                        ppd3d9.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut super::d3d9types::D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetCreationParameters(this) {
                    Ok(ok__) => {
                        pparameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCursorProperties<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCursorProperties(this, core::mem::transmute_copy(&xhotspot), core::mem::transmute_copy(&yhotspot), core::mem::transmute_copy(&pcursorbitmap)).into()
            }
        }
        unsafe extern "system" fn SetCursorPosition<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, flags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCursorPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&flags));
            }
        }
        unsafe extern "system" fn ShowCursor<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ShowCursor(this, core::mem::transmute_copy(&bshow))
            }
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateAdditionalSwapChain(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pswapchain)).into()
            }
        }
        unsafe extern "system" fn GetSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetSwapChain(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        pswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetNumberOfSwapChains(this)
            }
        }
        unsafe extern "system" fn Reset<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Reset(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        unsafe extern "system" fn Present<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppbackbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, prasterstatus: *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        prasterstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabledialogs: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetDialogBoxMode(this, core::mem::transmute_copy(&benabledialogs)).into()
            }
        }
        unsafe extern "system" fn SetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const super::d3d9types::D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pramp));
            }
        }
        unsafe extern "system" fn GetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pramp: *mut super::d3d9types::D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pramp));
            }
        }
        unsafe extern "system" fn CreateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, pptexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&pptexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppvolumetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVolumeTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvolumetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppcubetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateCubeTexture(this, core::mem::transmute_copy(&edgelength), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppcubetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: super::d3d9types::D3DPOOL, ppvertexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVertexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&fvf), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvertexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppindexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateIndexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppindexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateRenderTarget(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateDepthStencilSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn UpdateSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestinationsurface: *mut core::ffi::c_void, pdestpoint: *const super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::UpdateSurface(this, core::mem::transmute_copy(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestinationsurface), core::mem::transmute_copy(&pdestpoint)).into()
            }
        }
        unsafe extern "system" fn UpdateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetexture: *mut core::ffi::c_void, pdestinationtexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::UpdateTexture(this, core::mem::transmute_copy(&psourcetexture), core::mem::transmute_copy(&pdestinationtexture)).into()
            }
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRenderTargetData(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn StretchRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestsurface: *mut core::ffi::c_void, pdestrect: *const super::windef::RECT, filter: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::StretchRect(this, core::mem::transmute_copy(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestsurface), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn ColorFill<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, prect: *const super::windef::RECT, color: super::dsound::D3DCOLOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ColorFill(this, core::mem::transmute_copy(&psurface), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateOffscreenPlainSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn SetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, prendertarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex), core::mem::transmute_copy(&prendertarget)).into()
            }
        }
        unsafe extern "system" fn GetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex)) {
                    Ok(ok__) => {
                        pprendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewzstencil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetDepthStencilSurface(this, core::mem::transmute_copy(&pnewzstencil)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppzstencilsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetDepthStencilSurface(this) {
                    Ok(ok__) => {
                        ppzstencilsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::BeginScene(this).into()
            }
        }
        unsafe extern "system" fn EndScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EndScene(this).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prects: *const super::d3d9types::D3DRECT, flags: u32, color: super::dsound::D3DCOLOR, z: f32, stencil: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Clear(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&color), core::mem::transmute_copy(&z), core::mem::transmute_copy(&stencil)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn MultiplyTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::d3d9types::D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::MultiplyTransform(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *const super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn GetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn SetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *const super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn GetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *mut super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn SetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *const super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *mut super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn LightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::LightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn GetLightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, penable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetLightEnable(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        penable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
            }
        }
        unsafe extern "system" fn GetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetClipPlane(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pplane.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DRENDERSTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetRenderState(this, core::mem::transmute_copy(&state)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::d3d9types::D3DSTATEBLOCKTYPE, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateStateBlock(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppsb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::BeginStateBlock(this).into()
            }
        }
        unsafe extern "system" fn EndStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::EndStateBlock(this) {
                    Ok(ok__) => {
                        ppsb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
            }
        }
        unsafe extern "system" fn GetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *mut super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetClipStatus(this) {
                    Ok(ok__) => {
                        pclipstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetTexture(this, core::mem::transmute_copy(&stage)) {
                    Ok(ok__) => {
                        pptexture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, ptexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTexture(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&ptexture)).into()
            }
        }
        unsafe extern "system" fn GetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ValidateDevice<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpasses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::ValidateDevice(this) {
                    Ok(ok__) => {
                        pnumpasses.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *const super::wingdi::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *mut super::wingdi::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&palettenumber)) {
                    Ok(ok__) => {
                        pentries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
            }
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetCurrentTexturePalette(this) {
                    Ok(ok__) => {
                        palettenumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetScissorRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn GetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetScissorRect(this) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsoftware: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetSoftwareVertexProcessing(this, core::mem::transmute_copy(&bsoftware)).into()
            }
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetSoftwareVertexProcessing(this)
            }
        }
        unsafe extern "system" fn SetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsegments: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetNPatchMode(this, core::mem::transmute_copy(&nsegments)).into()
            }
        }
        unsafe extern "system" fn GetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetNPatchMode(this)
            }
        }
        unsafe extern "system" fn DrawPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawPrimitive(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&startvertex), core::mem::transmute_copy(&primitivecount)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::d3d9types::D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawIndexedPrimitive(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&basevertexindex), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&primcount)).into()
            }
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: super::d3d9types::D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawIndexedPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pindexdata), core::mem::transmute_copy(&indexdataformat), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn ProcessVertices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: *mut core::ffi::c_void, pvertexdecl: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ProcessVertices(this, core::mem::transmute_copy(&srcstartindex), core::mem::transmute_copy(&destindex), core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&pdestbuffer), core::mem::transmute_copy(&pvertexdecl), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexelements: *const super::d3d9types::D3DVERTEXELEMENT9, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateVertexDeclaration(this, core::mem::transmute_copy(&pvertexelements)) {
                    Ok(ok__) => {
                        ppdecl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexDeclaration(this, core::mem::transmute_copy(&pdecl)).into()
            }
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetVertexDeclaration(this) {
                    Ok(ok__) => {
                        ppdecl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvf: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetFVF(this, core::mem::transmute_copy(&fvf)).into()
            }
        }
        unsafe extern "system" fn GetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvf: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetFVF(this) {
                    Ok(ok__) => {
                        pfvf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pfunction)) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShader(this, core::mem::transmute_copy(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetVertexShader(this) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn SetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, pstreamdata: *mut core::ffi::c_void, offsetinbytes: u32, stride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&pstreamdata), core::mem::transmute_copy(&offsetinbytes), core::mem::transmute_copy(&stride)).into()
            }
        }
        unsafe extern "system" fn GetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut *mut core::ffi::c_void, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&ppstreamdata), core::mem::transmute_copy(&poffsetinbytes), core::mem::transmute_copy(&pstride)).into()
            }
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, setting: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&setting)).into()
            }
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber)) {
                    Ok(ok__) => {
                        psetting.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetIndices(this, core::mem::transmute_copy(&pindexdata)).into()
            }
        }
        unsafe extern "system" fn GetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetIndices(this) {
                    Ok(ok__) => {
                        ppindexdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pfunction)) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShader(this, core::mem::transmute_copy(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetPixelShader(this) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn DrawRectPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawRectPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&prectpatchinfo)).into()
            }
        }
        unsafe extern "system" fn DrawTriPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawTriPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&ptripatchinfo)).into()
            }
        }
        unsafe extern "system" fn DeletePatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DeletePatch(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::d3d9types::D3DQUERYTYPE, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateQuery(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirect3DDevice9 {}
windows_core::imp::define_interface!(IDirect3DDevice9Ex, IDirect3DDevice9Ex_Vtbl, 0xb18b10ce_2649_405a_870f_95f777d4313a);
impl core::ops::Deref for IDirect3DDevice9Ex {
    type Target = IDirect3DDevice9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DDevice9Ex, windows_core::IUnknown, IDirect3DDevice9);
impl IDirect3DDevice9Ex {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TestCooperativeLevel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestCooperativeLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAvailableTextureMem(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableTextureMem)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EvictManagedResources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EvictManagedResources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirect3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDisplayMode(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn GetCreationParameters(&self) -> windows_core::Result<super::d3d9types::D3DDEVICE_CREATION_PARAMETERS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCursorProperties<P2>(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCursorProperties)(windows_core::Interface::as_raw(self), xhotspot, yhotspot, pcursorbitmap.param().abi()) }
    }
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetCursorPosition)(windows_core::Interface::as_raw(self), x, y, flags);
        }
    }
    pub unsafe fn ShowCursor(&self, bshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).ShowCursor)(windows_core::Interface::as_raw(self), bshow.into()) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAdditionalSwapChain)(windows_core::Interface::as_raw(self), ppresentationparameters as _, core::mem::transmute(pswapchain)) }
    }
    pub unsafe fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSwapChain)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNumberOfSwapChains(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumberOfSwapChains)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn Reset(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), ppresentationparameters as _) }
    }
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub unsafe fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), iswapchain, ibackbuffer, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetRasterStatus(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDialogBoxMode(&self, benabledialogs: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDialogBoxMode)(windows_core::Interface::as_raw(self), benabledialogs.into()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const super::d3d9types::D3DGAMMARAMP) {
        unsafe {
            (windows_core::Interface::vtable(self).SetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, flags, pramp);
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut super::d3d9types::D3DGAMMARAMP) {
        unsafe {
            (windows_core::Interface::vtable(self).GetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, pramp as _);
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, pptexture: *mut Option<IDirect3DTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture)(windows_core::Interface::as_raw(self), width, height, levels, usage, format, pool, core::mem::transmute(pptexture), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVolumeTexture)(windows_core::Interface::as_raw(self), width, height, depth, levels, usage, format, pool, core::mem::transmute(ppvolumetexture), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppcubetexture: *mut Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCubeTexture)(windows_core::Interface::as_raw(self), edgelength, levels, usage, format, pool, core::mem::transmute(ppcubetexture), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: super::d3d9types::D3DPOOL, ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexBuffer)(windows_core::Interface::as_raw(self), length, usage, fvf, pool, core::mem::transmute(ppvertexbuffer), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateIndexBuffer(&self, length: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateIndexBuffer)(windows_core::Interface::as_raw(self), length, usage, format, pool, core::mem::transmute(ppindexbuffer), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateRenderTarget(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTarget)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable.into(), core::mem::transmute(ppsurface), psharedhandle as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurface)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard.into(), core::mem::transmute(ppsurface), psharedhandle as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn UpdateSurface<P0, P2>(&self, psourcesurface: P0, psourcerect: *const super::windef::RECT, pdestinationsurface: P2, pdestpoint: *const super::windef::POINT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateSurface)(windows_core::Interface::as_raw(self), psourcesurface.param().abi(), psourcerect, pdestinationsurface.param().abi(), pdestpoint) }
    }
    pub unsafe fn UpdateTexture<P0, P1>(&self, psourcetexture: P0, pdestinationtexture: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DBaseTexture9>,
        P1: windows_core::Param<IDirect3DBaseTexture9>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateTexture)(windows_core::Interface::as_raw(self), psourcetexture.param().abi(), pdestinationtexture.param().abi()) }
    }
    pub unsafe fn GetRenderTargetData<P0, P1>(&self, prendertarget: P0, pdestsurface: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTargetData)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), pdestsurface.param().abi()) }
    }
    pub unsafe fn GetFrontBufferData<P1>(&self, iswapchain: u32, pdestsurface: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), iswapchain, pdestsurface.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn StretchRect<P0, P2>(&self, psourcesurface: P0, psourcerect: *const super::windef::RECT, pdestsurface: P2, pdestrect: *const super::windef::RECT, filter: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).StretchRect)(windows_core::Interface::as_raw(self), psourcesurface.param().abi(), psourcerect, pdestsurface.param().abi(), pdestrect, filter) }
    }
    #[cfg(all(feature = "Win32_dsound", feature = "Win32_windef"))]
    pub unsafe fn ColorFill<P0>(&self, psurface: P0, prect: *const super::windef::RECT, color: super::dsound::D3DCOLOR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ColorFill)(windows_core::Interface::as_raw(self), psurface.param().abi(), prect, color) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurface)(windows_core::Interface::as_raw(self), width, height, format, pool, core::mem::transmute(ppsurface), psharedhandle as _) }
    }
    pub unsafe fn SetRenderTarget<P1>(&self, rendertargetindex: u32, prendertarget: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, prendertarget.param().abi()) }
    }
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDepthStencilSurface<P0>(&self, pnewzstencil: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencilSurface)(windows_core::Interface::as_raw(self), pnewzstencil.param().abi()) }
    }
    pub unsafe fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencilSurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginScene(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginScene)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndScene(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndScene)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound"))]
    pub unsafe fn Clear(&self, count: u32, prects: *const super::d3d9types::D3DRECT, flags: u32, color: super::dsound::D3DCOLOR, z: f32, stencil: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), count, prects, flags, color, z, stencil) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), state, pmatrix) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), state, pmatrix as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn MultiplyTransform(&self, param0: super::d3d9types::D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MultiplyTransform)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetViewport(&self, pviewport: *const super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewport)(windows_core::Interface::as_raw(self), pviewport) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetViewport(&self, pviewport: *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), pviewport as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub unsafe fn SetMaterial(&self, pmaterial: *const super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaterial)(windows_core::Interface::as_raw(self), pmaterial) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub unsafe fn GetMaterial(&self, pmaterial: *mut super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaterial)(windows_core::Interface::as_raw(self), pmaterial as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub unsafe fn SetLight(&self, index: u32, param1: *const super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLight)(windows_core::Interface::as_raw(self), index, param1) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub unsafe fn GetLight(&self, index: u32, param1: *mut super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLight)(windows_core::Interface::as_raw(self), index, param1 as _) }
    }
    pub unsafe fn LightEnable(&self, index: u32, enable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LightEnable)(windows_core::Interface::as_raw(self), index, enable.into()) }
    }
    pub unsafe fn GetLightEnable(&self, index: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLightEnable)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipPlane)(windows_core::Interface::as_raw(self), index, pplane) }
    }
    pub unsafe fn GetClipPlane(&self, index: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipPlane)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderState)(windows_core::Interface::as_raw(self), state, value) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderState)(windows_core::Interface::as_raw(self), state, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CreateStateBlock(&self, r#type: super::d3d9types::D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStateBlock)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginStateBlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginStateBlock)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndStateBlock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipStatus)(windows_core::Interface::as_raw(self), pclipstatus) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetClipStatus(&self) -> windows_core::Result<super::d3d9types::D3DCLIPSTATUS9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTexture)(windows_core::Interface::as_raw(self), stage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTexture<P1>(&self, stage: u32, ptexture: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DBaseTexture9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTexture)(windows_core::Interface::as_raw(self), stage, ptexture.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, value) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, value) }
    }
    pub unsafe fn ValidateDevice(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::wingdi::PALETTEENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, pentries) }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: u32) -> windows_core::Result<super::wingdi::PALETTEENTRY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTexturePalette)(windows_core::Interface::as_raw(self), palettenumber) }
    }
    pub unsafe fn GetCurrentTexturePalette(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentTexturePalette)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetScissorRect(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScissorRect)(windows_core::Interface::as_raw(self), prect) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetScissorRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScissorRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSoftwareVertexProcessing(&self, bsoftware: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self), bsoftware.into()) }
    }
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNPatchMode)(windows_core::Interface::as_raw(self), nsegments) }
    }
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetNPatchMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawPrimitive(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitive)(windows_core::Interface::as_raw(self), primitivetype, startvertex, primitivecount) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawIndexedPrimitive(&self, param0: super::d3d9types::D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitive)(windows_core::Interface::as_raw(self), param0, basevertexindex, minvertexindex, numvertices, startindex, primcount) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, primitivecount, pvertexstreamzerodata, vertexstreamzerostride) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: super::d3d9types::D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, minvertexindex, numvertices, primitivecount, pindexdata, indexdataformat, pvertexstreamzerodata, vertexstreamzerostride) }
    }
    pub unsafe fn ProcessVertices<P3, P4>(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: P3, pvertexdecl: P4, flags: u32) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IDirect3DVertexBuffer9>,
        P4: windows_core::Param<IDirect3DVertexDeclaration9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessVertices)(windows_core::Interface::as_raw(self), srcstartindex, destindex, vertexcount, pdestbuffer.param().abi(), pvertexdecl.param().abi(), flags) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const super::d3d9types::D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVertexDeclaration)(windows_core::Interface::as_raw(self), pvertexelements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexDeclaration<P0>(&self, pdecl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DVertexDeclaration9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVertexDeclaration)(windows_core::Interface::as_raw(self), pdecl.param().abi()) }
    }
    pub unsafe fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexDeclaration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFVF(&self, fvf: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFVF)(windows_core::Interface::as_raw(self), fvf) }
    }
    pub unsafe fn GetFVF(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFVF)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), pfunction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexShader<P0>(&self, pshader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DVertexShader9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShader)(windows_core::Interface::as_raw(self), pshader.param().abi()) }
    }
    pub unsafe fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4fcount) }
    }
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4icount) }
    }
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata, boolcount) }
    }
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    pub unsafe fn SetStreamSource<P1>(&self, streamnumber: u32, pstreamdata: P1, offsetinbytes: u32, stride: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDirect3DVertexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, pstreamdata.param().abi(), offsetinbytes, stride) }
    }
    pub unsafe fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, core::mem::transmute(ppstreamdata), poffsetinbytes as _, pstride as _) }
    }
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, setting) }
    }
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndices<P0>(&self, pindexdata: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DIndexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIndices)(windows_core::Interface::as_raw(self), pindexdata.param().abi()) }
    }
    pub unsafe fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), pfunction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPixelShader<P0>(&self, pshader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DPixelShader9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShader)(windows_core::Interface::as_raw(self), pshader.param().abi()) }
    }
    pub unsafe fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4fcount) }
    }
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4icount) }
    }
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata, boolcount) }
    }
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawRectPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs, prectpatchinfo) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawTriPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs, ptripatchinfo) }
    }
    pub unsafe fn DeletePatch(&self, handle: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePatch)(windows_core::Interface::as_raw(self), handle) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn CreateQuery(&self, r#type: super::d3d9types::D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConvolutionMonoKernel)(windows_core::Interface::as_raw(self), width, height, rows as _, columns as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn ComposeRects<P0, P1, P2, P4>(&self, psrc: P0, pdst: P1, psrcrectdescs: P2, numrects: u32, pdstrectdescs: P4, operation: super::d3d9types::D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DVertexBuffer9>,
        P4: windows_core::Param<IDirect3DVertexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ComposeRects)(windows_core::Interface::as_raw(self), psrc.param().abi(), pdst.param().abi(), psrcrectdescs.param().abi(), numrects, pdstrectdescs.param().abi(), operation, xoffset, yoffset) }
    }
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub unsafe fn PresentEx(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PresentEx)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion, dwflags) }
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority) }
    }
    pub unsafe fn WaitForVBlank(&self, iswapchain: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(self), iswapchain) }
    }
    pub unsafe fn CheckResourceResidency(&self, presourcearray: *mut Option<IDirect3DResource9>, numresources: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckResourceResidency)(windows_core::Interface::as_raw(self), core::mem::transmute(presourcearray), numresources) }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency) }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CheckDeviceState(&self, hdestinationwindow: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceState)(windows_core::Interface::as_raw(self), hdestinationwindow) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateRenderTargetEx(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetEx)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable.into(), core::mem::transmute(ppsurface), psharedhandle as _, usage) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurfaceEx)(windows_core::Interface::as_raw(self), width, height, format, pool, core::mem::transmute(ppsurface), psharedhandle as _, usage) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateDepthStencilSurfaceEx(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurfaceEx)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard.into(), core::mem::transmute(ppsurface), psharedhandle as _, usage) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn ResetEx(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetEx)(windows_core::Interface::as_raw(self), ppresentationparameters as _, pfullscreendisplaymode as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeEx)(windows_core::Interface::as_raw(self), iswapchain, pmode as _, protation as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9Ex_Vtbl {
    pub base__: IDirect3DDevice9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub TestCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAvailableTextureMem: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub EvictManagedResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDirect3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types"))]
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types")))]
    GetDeviceCaps: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDisplayMode: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    GetCreationParameters: usize,
    pub SetCursorProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCursorPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32),
    pub ShowCursor: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub CreateAdditionalSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    CreateAdditionalSwapChain: usize,
    pub GetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumberOfSwapChains: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    Reset: usize,
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *const super::windef::RECT, super::windef::HWND, *const super::wingdi::RGNDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_windef", feature = "Win32_wingdi")))]
    Present: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetBackBuffer: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetRasterStatus: usize,
    pub SetDialogBoxMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::d3d9types::D3DGAMMARAMP),
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetGammaRamp: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DGAMMARAMP),
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetGammaRamp: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateTexture: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateVolumeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateVolumeTexture: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateCubeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateCubeTexture: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateVertexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateVertexBuffer: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateIndexBuffer: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateRenderTarget: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateDepthStencilSurface: usize,
    #[cfg(feature = "Win32_windef")]
    pub UpdateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, *mut core::ffi::c_void, *const super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    UpdateSurface: usize,
    pub UpdateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTargetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub StretchRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, *mut core::ffi::c_void, *const super::windef::RECT, super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    StretchRect: usize,
    #[cfg(all(feature = "Win32_dsound", feature = "Win32_windef"))]
    pub ColorFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, super::dsound::D3DCOLOR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dsound", feature = "Win32_windef")))]
    ColorFill: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateOffscreenPlainSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateOffscreenPlainSurface: usize,
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound"))]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d9types::D3DRECT, u32, super::dsound::D3DCOLOR, f32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dsound")))]
    Clear: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetTransform: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTRANSFORMSTATETYPE, *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetTransform: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub MultiplyTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    MultiplyTransform: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetViewport: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetViewport: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub SetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype")))]
    SetMaterial: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype"))]
    pub GetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dxgitype")))]
    GetMaterial: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub SetLight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype")))]
    SetLight: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype"))]
    pub GetLight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype")))]
    GetLight: usize,
    pub LightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DRENDERSTATETYPE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetRenderState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DRENDERSTATETYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetRenderState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub CreateStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DSTATEBLOCKTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CreateStateBlock: usize,
    pub BeginStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetClipStatus: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetClipStatus: usize,
    pub GetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DTEXTURESTAGESTATETYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetTextureStageState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DTEXTURESTAGESTATETYPE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetTextureStageState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DSAMPLERSTATETYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetSamplerState: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DSAMPLERSTATETYPE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetSamplerState: usize,
    pub ValidateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wingdi")]
    pub SetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wingdi::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    SetPaletteEntries: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub GetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::wingdi::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    GetPaletteEntries: usize,
    pub SetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetScissorRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetScissorRect: usize,
    pub SetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawPrimitive: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawIndexedPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, i32, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawIndexedPrimitive: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawPrimitiveUP: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawIndexedPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DPRIMITIVETYPE, u32, u32, u32, *const core::ffi::c_void, super::d3d9types::D3DFORMAT, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawIndexedPrimitiveUP: usize,
    pub ProcessVertices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub CreateVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DVERTEXELEMENT9, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CreateVertexDeclaration: usize,
    pub SetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, u32) -> windows_core::HRESULT,
    pub SetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub SetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, u32) -> windows_core::HRESULT,
    pub SetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawRectPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawRectPatch: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub DrawTriPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    DrawTriPatch: usize,
    pub DeletePatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DQUERYTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    CreateQuery: usize,
    pub SetConvolutionMonoKernel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub ComposeRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, super::d3d9types::D3DCOMPOSERECTSOP, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    ComposeRects: usize,
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub PresentEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *const super::windef::RECT, super::windef::HWND, *const super::wingdi::RGNDATA, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_windef", feature = "Win32_wingdi")))]
    PresentEx: usize,
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CheckResourceResidency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub CheckDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CheckDeviceState: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateRenderTargetEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateRenderTargetEx: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateOffscreenPlainSurfaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DPOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateOffscreenPlainSurfaceEx: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateDepthStencilSurfaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::d3d9types::D3DFORMAT, super::d3d9types::D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateDepthStencilSurfaceEx: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub ResetEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS, *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    ResetEx: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DDISPLAYMODEEX, *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDisplayModeEx: usize,
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub trait IDirect3DDevice9Ex_Impl: IDirect3DDevice9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetAvailableTextureMem(&self) -> u32;
    fn EvictManagedResources(&self) -> windows_core::Result<()>;
    fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9>;
    fn GetDeviceCaps(&self, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn GetCreationParameters(&self) -> windows_core::Result<super::d3d9types::D3DDEVICE_CREATION_PARAMETERS>;
    fn SetCursorProperties(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn SetCursorPosition(&self, x: i32, y: i32, flags: u32);
    fn ShowCursor(&self, bshow: windows_core::BOOL) -> windows_core::BOOL;
    fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pswapchain: windows_core::OutRef<IDirect3DSwapChain9>) -> windows_core::Result<()>;
    fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(&self) -> u32;
    fn Reset(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, iswapchain: u32) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS>;
    fn SetDialogBoxMode(&self, benabledialogs: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const super::d3d9types::D3DGAMMARAMP);
    fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut super::d3d9types::D3DGAMMARAMP);
    fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, pptexture: windows_core::OutRef<IDirect3DTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppvolumetexture: windows_core::OutRef<IDirect3DVolumeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppcubetexture: windows_core::OutRef<IDirect3DCubeTexture9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: super::d3d9types::D3DPOOL, ppvertexbuffer: windows_core::OutRef<IDirect3DVertexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateIndexBuffer(&self, length: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppindexbuffer: windows_core::OutRef<IDirect3DIndexBuffer9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateRenderTarget(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn UpdateSurface(&self, psourcesurface: windows_core::Ref<IDirect3DSurface9>, psourcerect: *const super::windef::RECT, pdestinationsurface: windows_core::Ref<IDirect3DSurface9>, pdestpoint: *const super::windef::POINT) -> windows_core::Result<()>;
    fn UpdateTexture(&self, psourcetexture: windows_core::Ref<IDirect3DBaseTexture9>, pdestinationtexture: windows_core::Ref<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetRenderTargetData(&self, prendertarget: windows_core::Ref<IDirect3DSurface9>, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, iswapchain: u32, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn StretchRect(&self, psourcesurface: windows_core::Ref<IDirect3DSurface9>, psourcerect: *const super::windef::RECT, pdestsurface: windows_core::Ref<IDirect3DSurface9>, pdestrect: *const super::windef::RECT, filter: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn ColorFill(&self, psurface: windows_core::Ref<IDirect3DSurface9>, prect: *const super::windef::RECT, color: super::dsound::D3DCOLOR) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn SetRenderTarget(&self, rendertargetindex: u32, prendertarget: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(&self, pnewzstencil: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9>;
    fn BeginScene(&self) -> windows_core::Result<()>;
    fn EndScene(&self) -> windows_core::Result<()>;
    fn Clear(&self, count: u32, prects: *const super::d3d9types::D3DRECT, flags: u32, color: super::dsound::D3DCOLOR, z: f32, stencil: u32) -> windows_core::Result<()>;
    fn SetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn GetTransform(&self, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn MultiplyTransform(&self, param0: super::d3d9types::D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetViewport(&self, pviewport: *const super::d3d9types::D3DVIEWPORT9) -> windows_core::Result<()>;
    fn GetViewport(&self, pviewport: *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::Result<()>;
    fn SetMaterial(&self, pmaterial: *const super::d3d9types::D3DMATERIAL9) -> windows_core::Result<()>;
    fn GetMaterial(&self, pmaterial: *mut super::d3d9types::D3DMATERIAL9) -> windows_core::Result<()>;
    fn SetLight(&self, index: u32, param1: *const super::d3d9types::D3DLIGHT9) -> windows_core::Result<()>;
    fn GetLight(&self, index: u32, param1: *mut super::d3d9types::D3DLIGHT9) -> windows_core::Result<()>;
    fn LightEnable(&self, index: u32, enable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLightEnable(&self, index: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::Result<()>;
    fn GetClipPlane(&self, index: u32) -> windows_core::Result<f32>;
    fn SetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetRenderState(&self, state: super::d3d9types::D3DRENDERSTATETYPE) -> windows_core::Result<u32>;
    fn CreateStateBlock(&self, r#type: super::d3d9types::D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(&self) -> windows_core::Result<()>;
    fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(&self, pclipstatus: *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetClipStatus(&self) -> windows_core::Result<super::d3d9types::D3DCLIPSTATUS9>;
    fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(&self, stage: u32, ptexture: windows_core::Ref<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE) -> windows_core::Result<u32>;
    fn SetTextureStageState(&self, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE) -> windows_core::Result<u32>;
    fn SetSamplerState(&self, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn ValidateDevice(&self) -> windows_core::Result<u32>;
    fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::wingdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn GetPaletteEntries(&self, palettenumber: u32) -> windows_core::Result<super::wingdi::PALETTEENTRY>;
    fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::Result<()>;
    fn GetCurrentTexturePalette(&self) -> windows_core::Result<u32>;
    fn SetScissorRect(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn GetScissorRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn SetSoftwareVertexProcessing(&self, bsoftware: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL;
    fn SetNPatchMode(&self, nsegments: f32) -> windows_core::Result<()>;
    fn GetNPatchMode(&self) -> f32;
    fn DrawPrimitive(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitive(&self, param0: super::d3d9types::D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::Result<()>;
    fn DrawPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitiveUP(&self, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: super::d3d9types::D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn ProcessVertices(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: windows_core::Ref<IDirect3DVertexBuffer9>, pvertexdecl: windows_core::Ref<IDirect3DVertexDeclaration9>, flags: u32) -> windows_core::Result<()>;
    fn CreateVertexDeclaration(&self, pvertexelements: *const super::d3d9types::D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(&self, pdecl: windows_core::Ref<IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(&self, fvf: u32) -> windows_core::Result<()>;
    fn GetFVF(&self) -> windows_core::Result<u32>;
    fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(&self, pshader: windows_core::Ref<IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn SetStreamSource(&self, streamnumber: u32, pstreamdata: windows_core::Ref<IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> windows_core::Result<()>;
    fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: windows_core::OutRef<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::Result<()>;
    fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::Result<()>;
    fn GetStreamSourceFreq(&self, streamnumber: u32) -> windows_core::Result<u32>;
    fn SetIndices(&self, pindexdata: windows_core::Ref<IDirect3DIndexBuffer9>) -> windows_core::Result<()>;
    fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(&self, pshader: windows_core::Ref<IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::Result<()>;
    fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::Result<()>;
    fn DeletePatch(&self, handle: u32) -> windows_core::Result<()>;
    fn CreateQuery(&self, r#type: super::d3d9types::D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9>;
    fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::Result<()>;
    fn ComposeRects(&self, psrc: windows_core::Ref<IDirect3DSurface9>, pdst: windows_core::Ref<IDirect3DSurface9>, psrcrectdescs: windows_core::Ref<IDirect3DVertexBuffer9>, numrects: u32, pdstrectdescs: windows_core::Ref<IDirect3DVertexBuffer9>, operation: super::d3d9types::D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::Result<()>;
    fn PresentEx(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn WaitForVBlank(&self, iswapchain: u32) -> windows_core::Result<()>;
    fn CheckResourceResidency(&self, presourcearray: windows_core::OutRef<IDirect3DResource9>, numresources: u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
    fn CheckDeviceState(&self, hdestinationwindow: super::windef::HWND) -> windows_core::Result<()>;
    fn CreateRenderTargetEx(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn CreateDepthStencilSurfaceEx(&self, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn ResetEx(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl IDirect3DDevice9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::Release(this)
            }
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::TestCooperativeLevel(this).into()
            }
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetAvailableTextureMem(this)
            }
        }
        unsafe extern "system" fn EvictManagedResources<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::EvictManagedResources(this).into()
            }
        }
        unsafe extern "system" fn GetDirect3D<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppd3d9: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetDirect3D(this) {
                    Ok(ok__) => {
                        ppd3d9.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut super::d3d9caps::D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetDisplayMode(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut super::d3d9types::D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetCreationParameters(this) {
                    Ok(ok__) => {
                        pparameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCursorProperties<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetCursorProperties(this, core::mem::transmute_copy(&xhotspot), core::mem::transmute_copy(&yhotspot), core::mem::transmute_copy(&pcursorbitmap)).into()
            }
        }
        unsafe extern "system" fn SetCursorPosition<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, flags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetCursorPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&flags));
            }
        }
        unsafe extern "system" fn ShowCursor<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ShowCursor(this, core::mem::transmute_copy(&bshow))
            }
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateAdditionalSwapChain(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pswapchain)).into()
            }
        }
        unsafe extern "system" fn GetSwapChain<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetSwapChain(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        pswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetNumberOfSwapChains(this)
            }
        }
        unsafe extern "system" fn Reset<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::Reset(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        unsafe extern "system" fn Present<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetBackBuffer(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppbackbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, prasterstatus: *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetRasterStatus(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        prasterstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabledialogs: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetDialogBoxMode(this, core::mem::transmute_copy(&benabledialogs)).into()
            }
        }
        unsafe extern "system" fn SetGammaRamp<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const super::d3d9types::D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pramp));
            }
        }
        unsafe extern "system" fn GetGammaRamp<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pramp: *mut super::d3d9types::D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pramp));
            }
        }
        unsafe extern "system" fn CreateTexture<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, pptexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&pptexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppvolumetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateVolumeTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvolumetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppcubetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateCubeTexture(this, core::mem::transmute_copy(&edgelength), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppcubetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: super::d3d9types::D3DPOOL, ppvertexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateVertexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&fvf), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvertexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppindexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateIndexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppindexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateRenderTarget(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateDepthStencilSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn UpdateSurface<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestinationsurface: *mut core::ffi::c_void, pdestpoint: *const super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::UpdateSurface(this, core::mem::transmute_copy(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestinationsurface), core::mem::transmute_copy(&pdestpoint)).into()
            }
        }
        unsafe extern "system" fn UpdateTexture<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetexture: *mut core::ffi::c_void, pdestinationtexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::UpdateTexture(this, core::mem::transmute_copy(&psourcetexture), core::mem::transmute_copy(&pdestinationtexture)).into()
            }
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetRenderTargetData(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn StretchRect<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestsurface: *mut core::ffi::c_void, pdestrect: *const super::windef::RECT, filter: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::StretchRect(this, core::mem::transmute_copy(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestsurface), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn ColorFill<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, prect: *const super::windef::RECT, color: super::dsound::D3DCOLOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ColorFill(this, core::mem::transmute_copy(&psurface), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateOffscreenPlainSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn SetRenderTarget<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, prendertarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex), core::mem::transmute_copy(&prendertarget)).into()
            }
        }
        unsafe extern "system" fn GetRenderTarget<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex)) {
                    Ok(ok__) => {
                        pprendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewzstencil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetDepthStencilSurface(this, core::mem::transmute_copy(&pnewzstencil)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppzstencilsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetDepthStencilSurface(this) {
                    Ok(ok__) => {
                        ppzstencilsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginScene<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::BeginScene(this).into()
            }
        }
        unsafe extern "system" fn EndScene<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::EndScene(this).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prects: *const super::d3d9types::D3DRECT, flags: u32, color: super::dsound::D3DCOLOR, z: f32, stencil: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::Clear(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&color), core::mem::transmute_copy(&z), core::mem::transmute_copy(&stencil)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn MultiplyTransform<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::d3d9types::D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::MultiplyTransform(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetViewport<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *const super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn GetViewport<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *mut super::d3d9types::D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn SetMaterial<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *const super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn GetMaterial<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *mut super::d3d9types::D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn SetLight<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *const super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetLight<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *mut super::d3d9types::D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn LightEnable<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::LightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn GetLightEnable<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, penable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetLightEnable(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        penable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipPlane<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
            }
        }
        unsafe extern "system" fn GetClipPlane<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetClipPlane(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pplane.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DRENDERSTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetRenderState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::d3d9types::D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetRenderState(this, core::mem::transmute_copy(&state)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStateBlock<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::d3d9types::D3DSTATEBLOCKTYPE, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::CreateStateBlock(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppsb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::BeginStateBlock(this).into()
            }
        }
        unsafe extern "system" fn EndStateBlock<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::EndStateBlock(this) {
                    Ok(ok__) => {
                        ppsb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *const super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
            }
        }
        unsafe extern "system" fn GetClipStatus<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *mut super::d3d9types::D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetClipStatus(this) {
                    Ok(ok__) => {
                        pclipstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTexture<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetTexture(this, core::mem::transmute_copy(&stage)) {
                    Ok(ok__) => {
                        pptexture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTexture<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, ptexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetTexture(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&ptexture)).into()
            }
        }
        unsafe extern "system" fn GetTextureStageState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTextureStageState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: super::d3d9types::D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSamplerState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSamplerState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: super::d3d9types::D3DSAMPLERSTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ValidateDevice<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpasses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::ValidateDevice(this) {
                    Ok(ok__) => {
                        pnumpasses.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *const super::wingdi::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *mut super::wingdi::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&palettenumber)) {
                    Ok(ok__) => {
                        pentries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
            }
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetCurrentTexturePalette(this) {
                    Ok(ok__) => {
                        palettenumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScissorRect<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetScissorRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn GetScissorRect<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetScissorRect(this) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsoftware: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetSoftwareVertexProcessing(this, core::mem::transmute_copy(&bsoftware)).into()
            }
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetSoftwareVertexProcessing(this)
            }
        }
        unsafe extern "system" fn SetNPatchMode<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsegments: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetNPatchMode(this, core::mem::transmute_copy(&nsegments)).into()
            }
        }
        unsafe extern "system" fn GetNPatchMode<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetNPatchMode(this)
            }
        }
        unsafe extern "system" fn DrawPrimitive<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DrawPrimitive(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&startvertex), core::mem::transmute_copy(&primitivecount)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::d3d9types::D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DrawIndexedPrimitive(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&basevertexindex), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&primcount)).into()
            }
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DrawPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: super::d3d9types::D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: super::d3d9types::D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DrawIndexedPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pindexdata), core::mem::transmute_copy(&indexdataformat), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn ProcessVertices<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: *mut core::ffi::c_void, pvertexdecl: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ProcessVertices(this, core::mem::transmute_copy(&srcstartindex), core::mem::transmute_copy(&destindex), core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&pdestbuffer), core::mem::transmute_copy(&pvertexdecl), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexelements: *const super::d3d9types::D3DVERTEXELEMENT9, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::CreateVertexDeclaration(this, core::mem::transmute_copy(&pvertexelements)) {
                    Ok(ok__) => {
                        ppdecl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetVertexDeclaration(this, core::mem::transmute_copy(&pdecl)).into()
            }
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetVertexDeclaration(this) {
                    Ok(ok__) => {
                        ppdecl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFVF<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvf: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetFVF(this, core::mem::transmute_copy(&fvf)).into()
            }
        }
        unsafe extern "system" fn GetFVF<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvf: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetFVF(this) {
                    Ok(ok__) => {
                        pfvf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pfunction)) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetVertexShader(this, core::mem::transmute_copy(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetVertexShader(this) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn SetStreamSource<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, pstreamdata: *mut core::ffi::c_void, offsetinbytes: u32, stride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&pstreamdata), core::mem::transmute_copy(&offsetinbytes), core::mem::transmute_copy(&stride)).into()
            }
        }
        unsafe extern "system" fn GetStreamSource<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut *mut core::ffi::c_void, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&ppstreamdata), core::mem::transmute_copy(&poffsetinbytes), core::mem::transmute_copy(&pstride)).into()
            }
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, setting: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&setting)).into()
            }
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber)) {
                    Ok(ok__) => {
                        psetting.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndices<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetIndices(this, core::mem::transmute_copy(&pindexdata)).into()
            }
        }
        unsafe extern "system" fn GetIndices<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetIndices(this) {
                    Ok(ok__) => {
                        ppindexdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pfunction)) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetPixelShader(this, core::mem::transmute_copy(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetPixelShader(this) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn DrawRectPatch<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const super::d3d9types::D3DRECTPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DrawRectPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&prectpatchinfo)).into()
            }
        }
        unsafe extern "system" fn DrawTriPatch<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const super::d3d9types::D3DTRIPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DrawTriPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&ptripatchinfo)).into()
            }
        }
        unsafe extern "system" fn DeletePatch<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::DeletePatch(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::d3d9types::D3DQUERYTYPE, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::CreateQuery(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetConvolutionMonoKernel(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&rows), core::mem::transmute_copy(&columns)).into()
            }
        }
        unsafe extern "system" fn ComposeRects<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void, psrcrectdescs: *mut core::ffi::c_void, numrects: u32, pdstrectdescs: *mut core::ffi::c_void, operation: super::d3d9types::D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ComposeRects(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&psrcrectdescs), core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&pdstrectdescs), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&xoffset), core::mem::transmute_copy(&yoffset)).into()
            }
        }
        unsafe extern "system" fn PresentEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::PresentEx(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetGPUThreadPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::WaitForVBlank(this, core::mem::transmute_copy(&iswapchain)).into()
            }
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcearray: *mut *mut core::ffi::c_void, numresources: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CheckResourceResidency(this, core::mem::transmute_copy(&presourcearray), core::mem::transmute_copy(&numresources)).into()
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9Ex_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckDeviceState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdestinationwindow: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CheckDeviceState(this, core::mem::transmute_copy(&hdestinationwindow)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateRenderTargetEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, pool: super::d3d9types::D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateOffscreenPlainSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: super::d3d9types::D3DFORMAT, multisample: super::d3d9types::D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE, usage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateDepthStencilSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn ResetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut super::d3d9types::D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ResetEx(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        Self {
            base__: IDirect3DDevice9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_dsound", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirect3DDevice9Ex {}
windows_core::imp::define_interface!(IDirect3DDevice9Video, IDirect3DDevice9Video_Vtbl, 0x26dc4561_a1ee_4ae7_96da_118a36c0ec95);
windows_core::imp::interface_hierarchy!(IDirect3DDevice9Video, windows_core::IUnknown);
impl IDirect3DDevice9Video {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9caps")]
    pub unsafe fn GetContentProtectionCaps(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, pcaps: *mut super::d3d9caps::D3DCONTENTPROTECTIONCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContentProtectionCaps)(windows_core::Interface::as_raw(self), pcryptotype, pdecodeprofile, pcaps as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub unsafe fn CreateAuthenticatedChannel(&self, channeltype: super::d3d9types::D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: *mut Option<IDirect3DAuthenticatedChannel9>, pchannelhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAuthenticatedChannel)(windows_core::Interface::as_raw(self), channeltype, core::mem::transmute(ppauthenticatedchannel), pchannelhandle as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, ppcryptosession: *mut Option<IDirect3DCryptoSession9>, pcryptohandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCryptoSession)(windows_core::Interface::as_raw(self), pcryptotype, pdecodeprofile, core::mem::transmute(ppcryptosession), pcryptohandle as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9Video_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9caps")]
    pub GetContentProtectionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut super::d3d9caps::D3DCONTENTPROTECTIONCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9caps"))]
    GetContentProtectionCaps: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_winnt"))]
    pub CreateAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DAUTHENTICATEDCHANNELTYPE, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_winnt")))]
    CreateAuthenticatedChannel: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateCryptoSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateCryptoSession: usize,
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_winnt"))]
pub trait IDirect3DDevice9Video_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetContentProtectionCaps(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, pcaps: *mut super::d3d9caps::D3DCONTENTPROTECTIONCAPS) -> windows_core::Result<()>;
    fn CreateAuthenticatedChannel(&self, channeltype: super::d3d9types::D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: windows_core::OutRef<IDirect3DAuthenticatedChannel9>, pchannelhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, ppcryptosession: windows_core::OutRef<IDirect3DCryptoSession9>, pcryptohandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_winnt"))]
impl IDirect3DDevice9Video_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetContentProtectionCaps<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, pcaps: *mut super::d3d9caps::D3DCONTENTPROTECTIONCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::GetContentProtectionCaps(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecodeprofile), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channeltype: super::d3d9types::D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: *mut *mut core::ffi::c_void, pchannelhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::CreateAuthenticatedChannel(this, core::mem::transmute_copy(&channeltype), core::mem::transmute_copy(&ppauthenticatedchannel), core::mem::transmute_copy(&pchannelhandle)).into()
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, ppcryptosession: *mut *mut core::ffi::c_void, pcryptohandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::CreateCryptoSession(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecodeprofile), core::mem::transmute_copy(&ppcryptosession), core::mem::transmute_copy(&pcryptohandle)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9Video as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9caps", feature = "Win32_d3d9types", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirect3DDevice9Video {}
windows_core::imp::define_interface!(IDirect3DIndexBuffer9, IDirect3DIndexBuffer9_Vtbl, 0x7c9dd65e_d3f7_4529_acee_785830acde35);
impl core::ops::Deref for IDirect3DIndexBuffer9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DIndexBuffer9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DIndexBuffer9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), offsettolock, sizetolock, ppbdata as _, flags) }
    }
    pub unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DINDEXBUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DIndexBuffer9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DINDEXBUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DIndexBuffer9_Impl: IDirect3DResource9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DINDEXBUFFER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DIndexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DIndexBuffer9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut super::d3d9types::D3DINDEXBUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DIndexBuffer9 {}
windows_core::imp::define_interface!(IDirect3DPixelShader9, IDirect3DPixelShader9_Vtbl, 0x6d3bdbdc_5b02_4415_b852_ce5e8bccb289);
windows_core::imp::interface_hierarchy!(IDirect3DPixelShader9, windows_core::IUnknown);
impl IDirect3DPixelShader9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), param0 as _, psizeofdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DPixelShader9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirect3DPixelShader9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
}
impl IDirect3DPixelShader9_Vtbl {
    pub const fn new<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DPixelShader9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DPixelShader9 {}
windows_core::imp::define_interface!(IDirect3DQuery9, IDirect3DQuery9_Vtbl, 0xd9771460_a695_4f26_bbd3_27b840b541cc);
windows_core::imp::interface_hierarchy!(IDirect3DQuery9, windows_core::IUnknown);
impl IDirect3DQuery9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DQUERYTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Issue(&self, dwissueflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Issue)(windows_core::Interface::as_raw(self), dwissueflags) }
    }
    pub unsafe fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pdata as _, dwsize, dwgetdataflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DQuery9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DQUERYTYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Issue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DQuery9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetType(&self) -> super::d3d9types::D3DQUERYTYPE;
    fn GetDataSize(&self) -> u32;
    fn Issue(&self, dwissueflags: u32) -> windows_core::Result<()>;
    fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DQuery9_Vtbl {
    pub const fn new<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DQuery9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DQUERYTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDataSize<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetDataSize(this)
            }
        }
        unsafe extern "system" fn Issue<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwissueflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::Issue(this, core::mem::transmute_copy(&dwissueflags)).into()
            }
        }
        unsafe extern "system" fn GetData<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwgetdataflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DQuery9 {}
windows_core::imp::define_interface!(IDirect3DResource9, IDirect3DResource9_Vtbl, 0x05eec05d_8f7d_4362_b999_d1baf357c704);
windows_core::imp::interface_hierarchy!(IDirect3DResource9, windows_core::IUnknown);
impl IDirect3DResource9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DResource9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DResource9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DResource9_Vtbl {
    pub const fn new<Identity: IDirect3DResource9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DResource9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetType(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DResource9 {}
windows_core::imp::define_interface!(IDirect3DStateBlock9, IDirect3DStateBlock9_Vtbl, 0xb07c4fe5_310d_4ba8_a23c_4f0f206f218b);
windows_core::imp::interface_hierarchy!(IDirect3DStateBlock9, windows_core::IUnknown);
impl IDirect3DStateBlock9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Capture(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Apply(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DStateBlock9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirect3DStateBlock9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
}
impl IDirect3DStateBlock9_Vtbl {
    pub const fn new<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DStateBlock9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Capture<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Capture(this).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Apply(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DStateBlock9 {}
windows_core::imp::define_interface!(IDirect3DSurface9, IDirect3DSurface9_Vtbl, 0x0cfbaf3a_9ff6_429a_99b3_a2796af8b89b);
impl core::ops::Deref for IDirect3DSurface9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DSurface9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DSurface9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), riid, ppcontainer as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn LockRect(&self, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), plockedrect as _, prect, flags) }
    }
    pub unsafe fn UnlockRect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDC(&self) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ReleaseDC(&self, hdc: super::windef::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), hdc) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDesc: usize,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DLOCKED_RECT, *const super::windef::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    LockRect: usize,
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDC: usize,
    #[cfg(feature = "Win32_windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    ReleaseDC: usize,
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
pub trait IDirect3DSurface9_Impl: IDirect3DResource9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn LockRect(&self, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self) -> windows_core::Result<()>;
    fn GetDC(&self) -> windows_core::Result<super::windef::HDC>;
    fn ReleaseDC(&self, hdc: super::windef::HDC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl IDirect3DSurface9_Vtbl {
    pub const fn new<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSurface9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetContainer<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::LockRect(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::UnlockRect(this).into()
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSurface9_Impl::GetDC(this) {
                    Ok(ok__) => {
                        phdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDirect3DSurface9 {}
windows_core::imp::define_interface!(IDirect3DSwapChain9, IDirect3DSwapChain9_Vtbl, 0x794950f2_adfc_458a_905e_10a10b0b503b);
windows_core::imp::interface_hierarchy!(IDirect3DSwapChain9, windows_core::IUnknown);
impl IDirect3DSwapChain9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub unsafe fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion, dwflags) }
    }
    pub unsafe fn GetFrontBufferData<P0>(&self, pdestsurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), pdestsurface.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), ibackbuffer, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetRasterStatus(&self) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDisplayMode(&self) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPresentParameters)(windows_core::Interface::as_raw(self), ppresentationparameters as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *const super::windef::RECT, super::windef::HWND, *const super::wingdi::RGNDATA, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_windef", feature = "Win32_wingdi")))]
    Present: usize,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetBackBuffer: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetRasterStatus: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDisplayMode: usize,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub GetPresentParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    GetPresentParameters: usize,
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub trait IDirect3DSwapChain9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS>;
    fn GetDisplayMode(&self) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl IDirect3DSwapChain9_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn Present<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppbackbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterstatus: *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9_Impl::GetRasterStatus(this) {
                    Ok(ok__) => {
                        prasterstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9_Impl::GetDisplayMode(this) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetPresentParameters(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl windows_core::RuntimeName for IDirect3DSwapChain9 {}
windows_core::imp::define_interface!(IDirect3DSwapChain9Ex, IDirect3DSwapChain9Ex_Vtbl, 0x91886caf_1c3d_4d2e_a0ab_3e4c7d8d3303);
impl core::ops::Deref for IDirect3DSwapChain9Ex {
    type Target = IDirect3DSwapChain9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DSwapChain9Ex, windows_core::IUnknown, IDirect3DSwapChain9);
impl IDirect3DSwapChain9Ex {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub unsafe fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion, dwflags) }
    }
    pub unsafe fn GetFrontBufferData<P0>(&self, pdestsurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), pdestsurface.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), ibackbuffer, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetRasterStatus(&self) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDisplayMode(&self) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPresentParameters)(windows_core::Interface::as_raw(self), ppresentationparameters as _) }
    }
    pub unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetPresentStats(&self, ppresentationstatistics: *mut super::d3d9types::D3DPRESENTSTATS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPresentStats)(windows_core::Interface::as_raw(self), ppresentationstatistics as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDisplayModeEx(&self, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeEx)(windows_core::Interface::as_raw(self), pmode as _, protation as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9Ex_Vtbl {
    pub base__: IDirect3DSwapChain9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *const super::windef::RECT, super::windef::HWND, *const super::wingdi::RGNDATA, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_windef", feature = "Win32_wingdi")))]
    Present: usize,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::d3d9types::D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetBackBuffer: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetRasterStatus: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDisplayMode: usize,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub GetPresentParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    GetPresentParameters: usize,
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetPresentStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DPRESENTSTATS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetPresentStats: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DDISPLAYMODEEX, *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDisplayModeEx: usize,
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub trait IDirect3DSwapChain9Ex_Impl: IDirect3DSwapChain9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn Present(&self, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self) -> windows_core::Result<super::d3d9types::D3DRASTER_STATUS>;
    fn GetDisplayMode(&self) -> windows_core::Result<super::d3d9types::D3DDISPLAYMODE>;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(&self, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn GetLastPresentCount(&self) -> windows_core::Result<u32>;
    fn GetPresentStats(&self, ppresentationstatistics: *mut super::d3d9types::D3DPRESENTSTATS) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl IDirect3DSwapChain9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::Release(this)
            }
        }
        unsafe extern "system" fn Present<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::windef::RECT, pdestrect: *const super::windef::RECT, hdestwindowoverride: super::windef::HWND, pdirtyregion: *const super::wingdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ibackbuffer: u32, r#type: super::d3d9types::D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9Ex_Impl::GetBackBuffer(this, core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppbackbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterstatus: *mut super::d3d9types::D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9Ex_Impl::GetRasterStatus(this) {
                    Ok(ok__) => {
                        prasterstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut super::d3d9types::D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9Ex_Impl::GetDisplayMode(this) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9Ex_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut super::d3d9types::D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetPresentParameters(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9Ex_Impl::GetLastPresentCount(this) {
                    Ok(ok__) => {
                        plastpresentcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentStats<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationstatistics: *mut super::d3d9types::D3DPRESENTSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetPresentStats(this, core::mem::transmute_copy(&ppresentationstatistics)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut super::d3d9types::D3DDISPLAYMODEEX, protation: *mut super::d3d9types::D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        Self {
            base__: IDirect3DSwapChain9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            Present: Present::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPresentParameters: GetPresentParameters::<Identity, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as windows_core::Interface>::IID || iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl windows_core::RuntimeName for IDirect3DSwapChain9Ex {}
windows_core::imp::define_interface!(IDirect3DTexture9, IDirect3DTexture9_Vtbl, 0x85c31227_3de5_4f00_9b3a_f11ac38c18b5);
impl core::ops::Deref for IDirect3DTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DTexture9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), lodnew) }
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoGenFilterType)(windows_core::Interface::as_raw(self), filtertype) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetAutoGenFilterType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMipSubLevels)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _) }
    }
    pub unsafe fn GetSurfaceLevel(&self, level: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurfaceLevel)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub unsafe fn LockRect(&self, level: u32, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), level, plockedrect as _, prect, flags) }
    }
    pub unsafe fn UnlockRect(&self, level: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self), level) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn AddDirtyRect(&self, pdirtyrect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyRect)(windows_core::Interface::as_raw(self), pdirtyrect) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetAutoGenFilterType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAutoGenFilterType: usize,
    pub GenerateMipSubLevels: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetLevelDesc: usize,
    pub GetSurfaceLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DLOCKED_RECT, *const super::windef::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9types", feature = "Win32_windef")))]
    LockRect: usize,
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub AddDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    AddDirtyRect: usize,
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
pub trait IDirect3DTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
    fn GetLevelDesc(&self, level: u32, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetSurfaceLevel(&self, level: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn LockRect(&self, level: u32, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self, level: u32) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, pdirtyrect: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl IDirect3DTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DTexture9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn SetLOD<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetLOD(this)
            }
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetLevelCount(this)
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetAutoGenFilterType(this)
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GenerateMipSubLevels(this);
            }
        }
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut super::d3d9types::D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, ppsurfacelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DTexture9_Impl::GetSurfaceLevel(this, core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        ppsurfacelevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, plockedrect: *mut super::d3d9types::D3DLOCKED_RECT, prect: *const super::windef::RECT, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::LockRect(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_d3d9types", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDirect3DTexture9 {}
windows_core::imp::define_interface!(IDirect3DVertexBuffer9, IDirect3DVertexBuffer9_Vtbl, 0xb64bb1b5_fd70_4df6_bf91_19d0a12455e3);
impl core::ops::Deref for IDirect3DVertexBuffer9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DVertexBuffer9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DVertexBuffer9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), offsettolock, sizetolock, ppbdata as _, flags) }
    }
    pub unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexBuffer9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DVertexBuffer9_Impl: IDirect3DResource9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DVERTEXBUFFER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DVertexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVertexBuffer9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut super::d3d9types::D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DVertexBuffer9 {}
windows_core::imp::define_interface!(IDirect3DVertexDeclaration9, IDirect3DVertexDeclaration9_Vtbl, 0xdd13c59c_36fa_4098_a8fb_c7ed39dc8546);
windows_core::imp::interface_hierarchy!(IDirect3DVertexDeclaration9, windows_core::IUnknown);
impl IDirect3DVertexDeclaration9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDeclaration(&self, pelement: *mut super::d3d9types::D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeclaration)(windows_core::Interface::as_raw(self), pelement as _, pnumelements as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexDeclaration9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DVERTEXELEMENT9, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDeclaration: usize,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DVertexDeclaration9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetDeclaration(&self, pelement: *mut super::d3d9types::D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DVertexDeclaration9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVertexDeclaration9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeclaration<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut super::d3d9types::D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::GetDeclaration(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&pnumelements)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DVertexDeclaration9 {}
windows_core::imp::define_interface!(IDirect3DVertexShader9, IDirect3DVertexShader9_Vtbl, 0xefc5557e_6265_4613_8a94_43857889eb36);
windows_core::imp::interface_hierarchy!(IDirect3DVertexShader9, windows_core::IUnknown);
impl IDirect3DVertexShader9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), param0 as _, psizeofdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexShader9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirect3DVertexShader9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
}
impl IDirect3DVertexShader9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVertexShader9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DVertexShader9 {}
windows_core::imp::define_interface!(IDirect3DVolume9, IDirect3DVolume9_Vtbl, 0x24f416e6_1f67_4aa7_b88e_d33f6f3128a1);
windows_core::imp::interface_hierarchy!(IDirect3DVolume9, windows_core::IUnknown);
impl IDirect3DVolume9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), riid, ppcontainer as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn LockBox(&self, plockedvolume: *mut super::d3d9types::D3DLOCKED_BOX, pbox: *const super::d3d9types::D3DBOX, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockBox)(windows_core::Interface::as_raw(self), plockedvolume as _, pbox, flags) }
    }
    pub unsafe fn UnlockBox(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockBox)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolume9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetDesc: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub LockBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d9types::D3DLOCKED_BOX, *const super::d3d9types::D3DBOX, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    LockBox: usize,
    pub UnlockBox: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DVolume9_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn LockBox(&self, plockedvolume: *mut super::d3d9types::D3DLOCKED_BOX, pbox: *const super::d3d9types::D3DBOX, flags: u32) -> windows_core::Result<()>;
    fn UnlockBox(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DVolume9_Vtbl {
    pub const fn new<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVolume9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn GetContainer<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedvolume: *mut super::d3d9types::D3DLOCKED_BOX, pbox: *const super::d3d9types::D3DBOX, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::LockBox(this, core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::UnlockBox(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DVolume9 {}
windows_core::imp::define_interface!(IDirect3DVolumeTexture9, IDirect3DVolumeTexture9_Vtbl, 0x2518526c_e789_4111_a7b9_47ef328d13e6);
impl core::ops::Deref for IDirect3DVolumeTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DVolumeTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DVolumeTexture9 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags) }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid) }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), lodnew) }
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoGenFilterType)(windows_core::Interface::as_raw(self), filtertype) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetAutoGenFilterType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMipSubLevels)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _) }
    }
    pub unsafe fn GetVolumeLevel(&self, level: u32) -> windows_core::Result<IDirect3DVolume9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolumeLevel)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn LockBox(&self, level: u32, plockedvolume: *mut super::d3d9types::D3DLOCKED_BOX, pbox: *const super::d3d9types::D3DBOX, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockBox)(windows_core::Interface::as_raw(self), level, plockedvolume as _, pbox, flags) }
    }
    pub unsafe fn UnlockBox(&self, level: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockBox)(windows_core::Interface::as_raw(self), level) }
    }
    #[cfg(feature = "Win32_d3d9types")]
    pub unsafe fn AddDirtyBox(&self, pdirtybox: *const super::d3d9types::D3DBOX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyBox)(windows_core::Interface::as_raw(self), pdirtybox) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolumeTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetType: usize,
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3d9types")]
    pub SetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    SetAutoGenFilterType: usize,
    #[cfg(feature = "Win32_d3d9types")]
    pub GetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetAutoGenFilterType: usize,
    pub GenerateMipSubLevels: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3d9types")]
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    GetLevelDesc: usize,
    pub GetVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub LockBox: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d9types::D3DLOCKED_BOX, *const super::d3d9types::D3DBOX, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    LockBox: usize,
    pub UnlockBox: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9types")]
    pub AddDirtyBox: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d9types::D3DBOX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9types"))]
    AddDirtyBox: usize,
}
#[cfg(feature = "Win32_d3d9types")]
pub trait IDirect3DVolumeTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> super::d3d9types::D3DRESOURCETYPE;
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> super::d3d9types::D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
    fn GetLevelDesc(&self, level: u32, pdesc: *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn GetVolumeLevel(&self, level: u32) -> windows_core::Result<IDirect3DVolume9>;
    fn LockBox(&self, level: u32, plockedvolume: *mut super::d3d9types::D3DLOCKED_BOX, pbox: *const super::d3d9types::D3DBOX, flags: u32) -> windows_core::Result<()>;
    fn UnlockBox(&self, level: u32) -> windows_core::Result<()>;
    fn AddDirtyBox(&self, pdirtybox: *const super::d3d9types::D3DBOX) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d9types")]
impl IDirect3DVolumeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVolumeTexture9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn SetLOD<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetLOD(this)
            }
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetLevelCount(this)
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: super::d3d9types::D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3d9types::D3DTEXTUREFILTERTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetAutoGenFilterType(this)
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GenerateMipSubLevels(this);
            }
        }
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut super::d3d9types::D3DVOLUME_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, ppvolumelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVolumeTexture9_Impl::GetVolumeLevel(this, core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        ppvolumelevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, plockedvolume: *mut super::d3d9types::D3DLOCKED_BOX, pbox: *const super::d3d9types::D3DBOX, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::LockBox(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::UnlockBox(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybox: *const super::d3d9types::D3DBOX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::AddDirtyBox(this, core::mem::transmute_copy(&pdirtybox)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, OFFSET>,
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
#[cfg(feature = "Win32_d3d9types")]
impl windows_core::RuntimeName for IDirect3DVolumeTexture9 {}
pub const IID_HelperName: windows_core::GUID = windows_core::GUID::from_u128(0xe4a36723_fdfe_4b22_b146_3c04c07f4cc8);
pub const S_NOT_RESIDENT: u32 = 141953141;
pub const S_PRESENT_MODE_CHANGED: u32 = 141953143;
pub const S_PRESENT_OCCLUDED: u32 = 141953144;
pub const S_RESIDENT_IN_SHARED_MEMORY: u32 = 141953142;

#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn StiCreateInstanceW<P3>(hinst: super::minwindef::HINSTANCE, dwver: u32, ppsti: *mut Option<IStillImageW>, punkouter: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("sti.dll" "system" fn StiCreateInstanceW(hinst : super::minwindef::HINSTANCE, dwver : u32, ppsti : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StiCreateInstanceW(hinst, dwver, core::mem::transmute(ppsti), punkouter.param().abi()) }
}
pub const CLSID_Sti: windows_core::GUID = windows_core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
pub type DIAG = STI_DIAG;
pub const GUID_DeviceArrivedLaunch: windows_core::GUID = windows_core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
pub const GUID_STIUserDefined1: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined2: windows_core::GUID = windows_core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined3: windows_core::GUID = windows_core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanFaxImage: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanImage: windows_core::GUID = windows_core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanPrintImage: windows_core::GUID = windows_core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
windows_core::imp::define_interface!(IStiDevice, IStiDevice_Vtbl, 0x6cfa5a80_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStiDevice, windows_core::IUnknown);
impl IStiDevice {
    #[cfg(feature = "minwindef")]
    pub unsafe fn Initialize<P1>(&self, hinst: super::minwindef::HINSTANCE, pwszdevicename: P1, dwversion: u32, dwmode: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hinst, pwszdevicename.param().abi(), dwversion, dwmode) }
    }
    pub unsafe fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), pdevcaps as _) }
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdevstatus as _) }
    }
    pub unsafe fn DeviceReset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceReset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer as _) }
    }
    pub unsafe fn Escape(&self, escapefunction: STI_RAW_CONTROL_CODE, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata as _, dwoutdatasize, pdwactualdata as _) }
    }
    pub unsafe fn GetLastError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LockDevice(&self, dwtimeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockDevice)(windows_core::Interface::as_raw(self), dwtimeout) }
    }
    pub unsafe fn UnLockDevice(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnLockDevice)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawReadData)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawWriteData)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawReadCommand)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawWriteCommand)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "windef", feature = "winnt"))]
    pub unsafe fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Subscribe)(windows_core::Interface::as_raw(self), lpsubsribe as _) }
    }
    pub unsafe fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastNotificationData)(windows_core::Interface::as_raw(self), lpnotify as _) }
    }
    pub unsafe fn UnSubscribe(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnSubscribe)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLastErrorInfo(&self, plasterrorinfo: *mut STI_ERROR_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastErrorInfo)(windows_core::Interface::as_raw(self), plasterrorinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HINSTANCE, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DEV_CAPS) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DEVICE_STATUS) -> windows_core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DIAG) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, STI_RAW_CONTROL_CODE, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub RawReadData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    RawReadData: usize,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub RawWriteData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    RawWriteData: usize,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub RawReadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub RawWriteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    RawWriteCommand: usize,
    #[cfg(all(feature = "windef", feature = "winnt"))]
    pub Subscribe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STISUBSCRIBE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "windef", feature = "winnt")))]
    Subscribe: usize,
    pub GetLastNotificationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STINOTIFY) -> windows_core::HRESULT,
    pub UnSubscribe: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_ERROR_INFO) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
pub trait IStiDevice_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hinst: super::minwindef::HINSTANCE, pwszdevicename: &windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> windows_core::Result<()>;
    fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::Result<()>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()>;
    fn DeviceReset(&self) -> windows_core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()>;
    fn Escape(&self, escapefunction: STI_RAW_CONTROL_CODE, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<u32>;
    fn LockDevice(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn UnLockDevice(&self) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::Result<()>;
    fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::Result<()>;
    fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()>;
    fn UnSubscribe(&self) -> windows_core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut STI_ERROR_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
impl IStiDevice_Vtbl {
    pub const fn new<Identity: IStiDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::minwindef::HINSTANCE, pwszdevicename: windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetCapabilities(this, core::mem::transmute_copy(&pdevcaps)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetStatus(this, core::mem::transmute_copy(&pdevstatus)).into()
            }
        }
        unsafe extern "system" fn DeviceReset<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::DeviceReset(this).into()
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut STI_DIAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Diagnostic(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: STI_RAW_CONTROL_CODE, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Escape(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
            }
        }
        unsafe extern "system" fn GetLastError<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStiDevice_Impl::GetLastError(this) {
                    Ok(ok__) => {
                        pdwlastdeviceerror.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockDevice<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::LockDevice(this, core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn UnLockDevice<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::UnLockDevice(this).into()
            }
        }
        unsafe extern "system" fn RawReadData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawReadCommand<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteCommand<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn Subscribe<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Subscribe(this, core::mem::transmute_copy(&lpsubsribe)).into()
            }
        }
        unsafe extern "system" fn GetLastNotificationData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotify: *mut STINOTIFY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetLastNotificationData(this, core::mem::transmute_copy(&lpnotify)).into()
            }
        }
        unsafe extern "system" fn UnSubscribe<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::UnSubscribe(this).into()
            }
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasterrorinfo: *mut STI_ERROR_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetLastErrorInfo(this, core::mem::transmute_copy(&plasterrorinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            DeviceReset: DeviceReset::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            GetLastError: GetLastError::<Identity, OFFSET>,
            LockDevice: LockDevice::<Identity, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, OFFSET>,
            RawReadData: RawReadData::<Identity, OFFSET>,
            RawWriteData: RawWriteData::<Identity, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, OFFSET>,
            Subscribe: Subscribe::<Identity, OFFSET>,
            GetLastNotificationData: GetLastNotificationData::<Identity, OFFSET>,
            UnSubscribe: UnSubscribe::<Identity, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IStiDevice {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IStiDeviceW(pub u8);
windows_core::imp::define_interface!(IStillImageW, IStillImageW_Vtbl, 0x641bd880_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStillImageW, windows_core::IUnknown);
impl IStillImageW {
    #[cfg(feature = "minwindef")]
    pub unsafe fn Initialize(&self, hinst: super::minwindef::HINSTANCE, dwversion: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hinst, dwversion) }
    }
    pub unsafe fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceList)(windows_core::Interface::as_raw(self), dwtype, dwflags, pdwitemsreturned as _, ppbuffer as _) }
    }
    pub unsafe fn GetDeviceInfo<P0>(&self, pwszdevicename: P0, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), ppbuffer as _) }
    }
    pub unsafe fn CreateDevice<P0, P3>(&self, pwszdevicename: P0, dwmode: u32, pdevice: *mut Option<IStiDevice>, punkouter: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), dwmode, core::mem::transmute(pdevice), punkouter.param().abi()) }
    }
    pub unsafe fn GetSTILaunchInformation(&self, pwszdevicename: windows_core::PWSTR, pdweventcode: Option<*mut u32>, pwszeventname: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSTILaunchInformation)(windows_core::Interface::as_raw(self), pwszdevicename, pdweventcode.unwrap_or(core::mem::zeroed()) as _, pwszeventname) }
    }
    pub unsafe fn RegisterLaunchApplication<P0, P1>(&self, pwszappname: P0, pwszcommandline: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterLaunchApplication)(windows_core::Interface::as_raw(self), pwszappname.param().abi(), pwszcommandline.param().abi()) }
    }
    pub unsafe fn UnregisterLaunchApplication<P0>(&self, pwszappname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterLaunchApplication)(windows_core::Interface::as_raw(self), pwszappname.param().abi()) }
    }
    pub unsafe fn EnableHwNotifications<P0>(&self, pwszdevicename: P0, bnewstate: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableHwNotifications)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), bnewstate.into()) }
    }
    pub unsafe fn GetHwNotificationState<P0>(&self, pwszdevicename: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwNotificationState)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RefreshDeviceBus<P0>(&self, pwszdevicename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RefreshDeviceBus)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi()) }
    }
    pub unsafe fn WriteToErrorLog<P1>(&self, dwmessagetype: u32, pszmessage: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToErrorLog)(windows_core::Interface::as_raw(self), dwmessagetype, pszmessage.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStillImageW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HINSTANCE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Initialize: usize,
    pub GetDeviceList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSTILaunchInformation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub RegisterLaunchApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterLaunchApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnableHwNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetHwNotificationState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub RefreshDeviceBus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait IStillImageW_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hinst: super::minwindef::HINSTANCE, dwversion: u32) -> windows_core::Result<()>;
    fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, pwszdevicename: &windows_core::PCWSTR, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateDevice(&self, pwszdevicename: &windows_core::PCWSTR, dwmode: u32, pdevice: windows_core::OutRef<IStiDevice>, punkouter: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetSTILaunchInformation(&self, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RegisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR, pwszcommandline: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnableHwNotifications(&self, pwszdevicename: &windows_core::PCWSTR, bnewstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetHwNotificationState(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn RefreshDeviceBus(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "minwindef")]
impl IStillImageW_Vtbl {
    pub const fn new<Identity: IStillImageW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::minwindef::HINSTANCE, dwversion: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute_copy(&dwversion)).into()
            }
        }
        unsafe extern "system" fn GetDeviceList<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceList(this, core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwitemsreturned), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceInfo(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, dwmode: u32, pdevice: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::CreateDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetSTILaunchInformation<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetSTILaunchInformation(this, core::mem::transmute_copy(&pwszdevicename), core::mem::transmute_copy(&pdweventcode), core::mem::transmute_copy(&pwszeventname)).into()
            }
        }
        unsafe extern "system" fn RegisterLaunchApplication<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszappname: windows_core::PCWSTR, pwszcommandline: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::RegisterLaunchApplication(this, core::mem::transmute(&pwszappname), core::mem::transmute(&pwszcommandline)).into()
            }
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszappname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::UnregisterLaunchApplication(this, core::mem::transmute(&pwszappname)).into()
            }
        }
        unsafe extern "system" fn EnableHwNotifications<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, bnewstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::EnableHwNotifications(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&bnewstate)).into()
            }
        }
        unsafe extern "system" fn GetHwNotificationState<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pbcurrentstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStillImageW_Impl::GetHwNotificationState(this, core::mem::transmute(&pwszdevicename)) {
                    Ok(ok__) => {
                        pbcurrentstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RefreshDeviceBus<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::RefreshDeviceBus(this, core::mem::transmute(&pwszdevicename)).into()
            }
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::WriteToErrorLog(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pszmessage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetDeviceList: GetDeviceList::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Identity, OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Identity, OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Identity, OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Identity, OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Identity, OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Identity, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStillImageW as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IStillImageW {}
pub type LPDIAG = LPSTI_DIAG;
pub type LPSTINOTIFY = *mut STINOTIFY;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type LPSTISUBSCRIBE = *mut STISUBSCRIBE;
pub type LPSTI_DIAG = *mut STI_DIAG;
pub const MAX_NOTIFICATION_DATA: u32 = 64;
pub type PSTIDEVICEW = *mut IStiDeviceW;
pub type PSTI_DEVICE_INFORMATION = PSTI_DEVICE_INFORMATIONW;
pub type PSTI_DEVICE_INFORMATIONW = *mut STI_DEVICE_INFORMATIONW;
pub type PSTI_DEVICE_STATUS = *mut STI_DEVICE_STATUS;
pub type PSTI_DEV_CAPS = *mut STI_DEV_CAPS;
pub type PSTI_ERROR_INFO = *mut STI_ERROR_INFO;
pub type PSTI_ERROR_INFOW = *mut STI_ERROR_INFOW;
pub type PSTI_WIA_DEVICE_INFORMATION = PSTI_WIA_DEVICE_INFORMATIONW;
pub type PSTI_WIA_DEVICE_INFORMATIONW = *mut STI_WIA_DEVICE_INFORMATIONW;
pub const STIEDFL_ALLDEVICES: u32 = 0;
pub const STIEDFL_ATTACHEDONLY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: windows_core::GUID,
    pub abNotificationData: [u8; 64],
}
impl Default for STINOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::windef::HWND,
    pub hEvent: super::winnt::HANDLE,
    pub uiNotificationMessage: u32,
}
pub const STI_ADD_DEVICE_BROADCAST_ACTION: windows_core::PCSTR = windows_core::s!("Arrival");
pub const STI_DEVICE_CREATE_BOTH: u32 = 3;
pub const STI_DEVICE_CREATE_DATA: u32 = 2;
pub const STI_DEVICE_CREATE_MASK: u32 = 65535;
pub const STI_DEVICE_CREATE_STATUS: u32 = 1;
pub type STI_DEVICE_INFORMATION = STI_DEVICE_INFORMATIONW;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: STI_DEVICE_TYPE,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_core::PWSTR,
    pub pszDeviceDescription: windows_core::PWSTR,
    pub pszPortName: windows_core::PWSTR,
    pub pszPropProvider: windows_core::PWSTR,
    pub pszLocalName: windows_core::PWSTR,
}
impl Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STI_DEVICE_MJ_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct STI_DEVICE_TYPE(pub u32);
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2;
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: STI_ERROR_INFO,
}
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1;
pub type STI_ERROR_INFO = STI_ERROR_INFOW;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STI_ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl Default for STI_ERROR_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STI_EVENTHANDLING_ENABLED: u32 = 1;
pub const STI_EVENTHANDLING_PENDING: u32 = 4;
pub const STI_EVENTHANDLING_POLLING: u32 = 2;
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8;
pub const STI_GENCAP_COMMON_MASK: u32 = 255;
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4;
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1;
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2;
pub const STI_GENCAP_SUBSET: u32 = 32;
pub const STI_GENCAP_WIA: u32 = 16;
pub const STI_HW_CONFIG_PARALLEL: u32 = 16;
pub const STI_HW_CONFIG_SCSI: u32 = 2;
pub const STI_HW_CONFIG_SERIAL: u32 = 8;
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1;
pub const STI_HW_CONFIG_USB: u32 = 4;
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128;
pub const STI_ONLINESTATE_BUSY: u32 = 256;
pub const STI_ONLINESTATE_ERROR: u32 = 4;
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024;
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128;
pub const STI_ONLINESTATE_OFFLINE: u32 = 64;
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1;
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16;
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32;
pub const STI_ONLINESTATE_PAUSED: u32 = 8;
pub const STI_ONLINESTATE_PENDING: u32 = 2;
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192;
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512;
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096;
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct STI_RAW_CONTROL_CODE(pub u32);
pub const STI_RAW_RESERVED: u32 = 4096;
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: windows_core::PCSTR = windows_core::s!("Removal");
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2;
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1;
pub const STI_TRACE_ERROR: u32 = 4;
pub const STI_TRACE_INFORMATION: u32 = 1;
pub const STI_TRACE_WARNING: u32 = 2;
pub const STI_UNICODE: u32 = 1;
pub const STI_VERSION: u32 = 2;
pub const STI_VERSION_3: u32 = 16777219;
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080;
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216;
pub const STI_VERSION_MIN_ALLOWED: u32 = 2;
pub const STI_VERSION_REAL: u32 = 2;
pub type STI_WIA_DEVICE_INFORMATION = STI_WIA_DEVICE_INFORMATIONW;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: STI_DEVICE_TYPE,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_core::PWSTR,
    pub pszDeviceDescription: windows_core::PWSTR,
    pub pszPortName: windows_core::PWSTR,
    pub pszPropProvider: windows_core::PWSTR,
    pub pszLocalName: windows_core::PWSTR,
    pub pszUiDll: windows_core::PWSTR,
    pub pszServer: windows_core::PWSTR,
}
impl Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = 0;
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = 2;
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = 1;
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = 3;
pub const WIA_INCOMPAT_XP: u32 = 1;

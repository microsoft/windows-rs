#[cfg(all(feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
#[inline]
pub unsafe fn StiCreateInstance<P3>(hinst: super::HINSTANCE, dwver: super::DWORD, ppsti: *mut Option<IStillImageW>, punkouter: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("sti.dll" "system" "StiCreateInstanceW" fn StiCreateInstance(hinst : super::HINSTANCE, dwver : super::DWORD, ppsti : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StiCreateInstance(hinst as _, dwver, ppsti as _, punkouter.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
#[inline]
pub unsafe fn StiCreateInstanceW<P3>(hinst: super::HINSTANCE, dwver: super::DWORD, ppsti: *mut Option<IStillImageW>, punkouter: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("sti.dll" "system" fn StiCreateInstanceW(hinst : super::HINSTANCE, dwver : super::DWORD, ppsti : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StiCreateInstanceW(hinst as _, dwver, ppsti as _, punkouter.param().abi()) }
}
pub const CLSID_Sti: windows_core::GUID = windows_core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type DIAG = STI_DIAG;
pub const GUID_DeviceArrivedLaunch: windows_core::GUID = windows_core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
pub const GUID_STIUserDefined1: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined2: windows_core::GUID = windows_core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined3: windows_core::GUID = windows_core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanFaxImage: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanImage: windows_core::GUID = windows_core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanPrintImage: windows_core::GUID = windows_core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
pub const IID_IStiDevice: windows_core::GUID = windows_core::GUID::from_u128(0x6cfa5a80_2dc8_11d0_90ea_00aa0060f86c);
pub const IID_IStillImageW: windows_core::GUID = windows_core::GUID::from_u128(0x641bd880_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::define_interface!(IStiDevice, IStiDevice_Vtbl, 0x6cfa5a80_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStiDevice, windows_core::IUnknown);
impl IStiDevice {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Initialize<P1>(&self, hinst: super::HINSTANCE, pwszdevicename: P1, dwversion: super::DWORD, dwmode: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hinst, pwszdevicename.param().abi(), dwversion, dwmode) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCapabilities(&self, pdevcaps: PSTI_DEV_CAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), pdevcaps as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatus(&self, pdevstatus: PSTI_DEVICE_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdevstatus as _) }
    }
    pub unsafe fn DeviceReset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceReset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Diagnostic(&self, pbuffer: LPSTI_DIAG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Escape(&self, escapefunction: STI_RAW_CONTROL_CODE, lpindata: super::LPVOID, cbindatasize: super::DWORD, poutdata: super::LPVOID, dwoutdatasize: super::DWORD, pdwactualdata: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata as _, dwoutdatasize, pdwactualdata as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetLastError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LockDevice(&self, dwtimeout: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockDevice)(windows_core::Interface::as_raw(self), dwtimeout) }
    }
    pub unsafe fn UnLockDevice(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnLockDevice)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn RawReadData(&self, lpbuffer: super::LPVOID, lpdwnumberofbytes: super::LPDWORD, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawReadData)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: super::LPVOID, nnumberofbytes: super::DWORD, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawWriteData)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: super::LPVOID, lpdwnumberofbytes: super::LPDWORD, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawReadCommand)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: super::LPVOID, nnumberofbytes: super::DWORD, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RawWriteCommand)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
    pub unsafe fn Subscribe(&self, lpsubsribe: LPSTISUBSCRIBE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Subscribe)(windows_core::Interface::as_raw(self), lpsubsribe as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLastNotificationData(&self, lpnotify: LPSTINOTIFY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastNotificationData)(windows_core::Interface::as_raw(self), lpnotify as _) }
    }
    pub unsafe fn UnSubscribe(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnSubscribe)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetLastErrorInfo(&self, plasterrorinfo: *mut STI_ERROR_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastErrorInfo)(windows_core::Interface::as_raw(self), plasterrorinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::HINSTANCE, windows_core::PCWSTR, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Initialize: usize,
    #[cfg(feature = "shared")]
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, PSTI_DEV_CAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCapabilities: usize,
    #[cfg(feature = "shared")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, PSTI_DEVICE_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatus: usize,
    pub DeviceReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, LPSTI_DIAG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Diagnostic: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, STI_RAW_CONTROL_CODE, super::LPVOID, super::DWORD, super::LPVOID, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Escape: usize,
    #[cfg(feature = "minwindef")]
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetLastError: usize,
    #[cfg(feature = "shared")]
    pub LockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LockDevice: usize,
    pub UnLockDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub RawReadData: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::LPDWORD, super::LPOVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt")))]
    RawReadData: usize,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub RawWriteData: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::DWORD, super::LPOVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt")))]
    RawWriteData: usize,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub RawReadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::LPDWORD, super::LPOVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub RawWriteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::DWORD, super::LPOVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt")))]
    RawWriteCommand: usize,
    #[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
    pub Subscribe: unsafe extern "system" fn(*mut core::ffi::c_void, LPSTISUBSCRIBE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "winnt")))]
    Subscribe: usize,
    #[cfg(feature = "shared")]
    pub GetLastNotificationData: unsafe extern "system" fn(*mut core::ffi::c_void, LPSTINOTIFY) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLastNotificationData: usize,
    pub UnSubscribe: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetLastErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_ERROR_INFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetLastErrorInfo: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait IStiDevice_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hinst: super::HINSTANCE, pwszdevicename: &windows_core::PCWSTR, dwversion: super::DWORD, dwmode: super::DWORD) -> windows_core::Result<()>;
    fn GetCapabilities(&self, pdevcaps: PSTI_DEV_CAPS) -> windows_core::Result<()>;
    fn GetStatus(&self, pdevstatus: PSTI_DEVICE_STATUS) -> windows_core::Result<()>;
    fn DeviceReset(&self) -> windows_core::Result<()>;
    fn Diagnostic(&self, pbuffer: LPSTI_DIAG) -> windows_core::Result<()>;
    fn Escape(&self, escapefunction: STI_RAW_CONTROL_CODE, lpindata: super::LPVOID, cbindatasize: super::DWORD, poutdata: super::LPVOID, dwoutdatasize: super::DWORD, pdwactualdata: super::LPDWORD) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<u32>;
    fn LockDevice(&self, dwtimeout: super::DWORD) -> windows_core::Result<()>;
    fn UnLockDevice(&self) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: super::LPVOID, lpdwnumberofbytes: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: super::LPVOID, nnumberofbytes: super::DWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: super::LPVOID, lpdwnumberofbytes: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: super::LPVOID, nnumberofbytes: super::DWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::Result<()>;
    fn Subscribe(&self, lpsubsribe: LPSTISUBSCRIBE) -> windows_core::Result<()>;
    fn GetLastNotificationData(&self, lpnotify: LPSTINOTIFY) -> windows_core::Result<()>;
    fn UnSubscribe(&self) -> windows_core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut STI_ERROR_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl IStiDevice_Vtbl {
    pub const fn new<Identity: IStiDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::HINSTANCE, pwszdevicename: windows_core::PCWSTR, dwversion: super::DWORD, dwmode: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevcaps: PSTI_DEV_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetCapabilities(this, core::mem::transmute_copy(&pdevcaps)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevstatus: PSTI_DEVICE_STATUS) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Diagnostic<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: LPSTI_DIAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Diagnostic(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: STI_RAW_CONTROL_CODE, lpindata: super::LPVOID, cbindatasize: super::DWORD, poutdata: super::LPVOID, dwoutdatasize: super::DWORD, pdwactualdata: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Escape(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
            }
        }
        unsafe extern "system" fn GetLastError<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastdeviceerror: super::LPDWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn LockDevice<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RawReadData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: super::LPVOID, lpdwnumberofbytes: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: super::LPVOID, nnumberofbytes: super::DWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawReadCommand<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: super::LPVOID, lpdwnumberofbytes: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteCommand<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: super::LPVOID, nnumberofbytes: super::DWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn Subscribe<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsubsribe: LPSTISUBSCRIBE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Subscribe(this, core::mem::transmute_copy(&lpsubsribe)).into()
            }
        }
        unsafe extern "system" fn GetLastNotificationData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotify: LPSTINOTIFY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IStiDevice {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IStiDeviceW(pub u8);
windows_core::imp::define_interface!(IStillImageW, IStillImageW_Vtbl, 0x641bd880_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStillImageW, windows_core::IUnknown);
impl IStillImageW {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Initialize(&self, hinst: super::HINSTANCE, dwversion: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hinst, dwversion) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetDeviceList(&self, dwtype: super::DWORD, dwflags: super::DWORD, pdwitemsreturned: *mut super::DWORD, ppbuffer: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceList)(windows_core::Interface::as_raw(self), dwtype, dwflags, pdwitemsreturned as _, ppbuffer as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetDeviceInfo<P0>(&self, pwszdevicename: P0, ppbuffer: *mut super::LPVOID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), ppbuffer as _) }
    }
    #[cfg(all(feature = "shared", feature = "unknwnbase"))]
    pub unsafe fn CreateDevice<P0, P3>(&self, pwszdevicename: P0, dwmode: super::DWORD, pdevice: *mut Option<PSTIDEVICE>, punkouter: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::LPUNKNOWN>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), dwmode, core::mem::transmute(pdevice), punkouter.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetDeviceValue<P0, P1>(&self, pwszdevicename: P0, pvaluename: P1, ptype: super::LPDWORD, pdata: super::LPBYTE, cbdata: super::LPDWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceValue)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), pvaluename.param().abi(), ptype as _, pdata as _, cbdata as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDeviceValue<P0, P1>(&self, pwszdevicename: P0, pvaluename: P1, r#type: super::DWORD, pdata: &[u8]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDeviceValue)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), pvaluename.param().abi(), r#type, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSTILaunchInformation(&self, pwszdevicename: windows_core::PWSTR, pdweventcode: Option<*mut super::DWORD>, pwszeventname: windows_core::PWSTR) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub unsafe fn LaunchApplicationForDevice<P0, P1>(&self, pwszdevicename: P0, pwszappname: P1, pstinotify: LPSTINOTIFY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LaunchApplicationForDevice)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), pwszappname.param().abi(), pstinotify) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetupDeviceParameters(&self, param0: PSTI_DEVICE_INFORMATIONW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetupDeviceParameters)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn WriteToErrorLog<P1>(&self, dwmessagetype: super::DWORD, pszmessage: P1) -> windows_core::HRESULT
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
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::HINSTANCE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Initialize: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetDeviceList: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut super::DWORD, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetDeviceList: usize,
    #[cfg(feature = "minwindef")]
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetDeviceInfo: usize,
    #[cfg(all(feature = "shared", feature = "unknwnbase"))]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::DWORD, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "unknwnbase")))]
    CreateDevice: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetDeviceValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::LPDWORD, super::LPBYTE, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetDeviceValue: usize,
    #[cfg(feature = "shared")]
    pub SetDeviceValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::DWORD, super::LPBYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDeviceValue: usize,
    #[cfg(feature = "shared")]
    pub GetSTILaunchInformation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut super::DWORD, windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSTILaunchInformation: usize,
    pub RegisterLaunchApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterLaunchApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnableHwNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetHwNotificationState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub RefreshDeviceBus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub LaunchApplicationForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, LPSTINOTIFY) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LaunchApplicationForDevice: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetupDeviceParameters: unsafe extern "system" fn(*mut core::ffi::c_void, PSTI_DEVICE_INFORMATIONW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetupDeviceParameters: usize,
    #[cfg(feature = "shared")]
    pub WriteToErrorLog: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WriteToErrorLog: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "unknwnbase", feature = "winnt"))]
pub trait IStillImageW_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hinst: super::HINSTANCE, dwversion: super::DWORD) -> windows_core::Result<()>;
    fn GetDeviceList(&self, dwtype: super::DWORD, dwflags: super::DWORD, pdwitemsreturned: *mut super::DWORD, ppbuffer: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, pwszdevicename: &windows_core::PCWSTR, ppbuffer: *mut super::LPVOID) -> windows_core::Result<()>;
    fn CreateDevice(&self, pwszdevicename: &windows_core::PCWSTR, dwmode: super::DWORD, pdevice: windows_core::OutRef<PSTIDEVICE>, punkouter: windows_core::Ref<super::LPUNKNOWN>) -> windows_core::Result<()>;
    fn GetDeviceValue(&self, pwszdevicename: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, ptype: super::LPDWORD, pdata: super::LPBYTE, cbdata: super::LPDWORD) -> windows_core::Result<()>;
    fn SetDeviceValue(&self, pwszdevicename: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, r#type: super::DWORD, pdata: super::LPBYTE, cbdata: super::DWORD) -> windows_core::Result<()>;
    fn GetSTILaunchInformation(&self, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut super::DWORD, pwszeventname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RegisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR, pwszcommandline: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnableHwNotifications(&self, pwszdevicename: &windows_core::PCWSTR, bnewstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetHwNotificationState(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn RefreshDeviceBus(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LaunchApplicationForDevice(&self, pwszdevicename: &windows_core::PCWSTR, pwszappname: &windows_core::PCWSTR, pstinotify: LPSTINOTIFY) -> windows_core::Result<()>;
    fn SetupDeviceParameters(&self, param0: PSTI_DEVICE_INFORMATIONW) -> windows_core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: super::DWORD, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "unknwnbase", feature = "winnt"))]
impl IStillImageW_Vtbl {
    pub const fn new<Identity: IStillImageW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::HINSTANCE, dwversion: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute_copy(&dwversion)).into()
            }
        }
        unsafe extern "system" fn GetDeviceList<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: super::DWORD, dwflags: super::DWORD, pdwitemsreturned: *mut super::DWORD, ppbuffer: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceList(this, core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwitemsreturned), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, ppbuffer: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceInfo(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, dwmode: super::DWORD, pdevice: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::CreateDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetDeviceValue<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, ptype: super::LPDWORD, pdata: super::LPBYTE, cbdata: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceValue(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn SetDeviceValue<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, r#type: super::DWORD, pdata: super::LPBYTE, cbdata: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::SetDeviceValue(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn GetSTILaunchInformation<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut super::DWORD, pwszeventname: windows_core::PWSTR) -> windows_core::HRESULT {
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
        unsafe extern "system" fn LaunchApplicationForDevice<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pwszappname: windows_core::PCWSTR, pstinotify: LPSTINOTIFY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::LaunchApplicationForDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pwszappname), core::mem::transmute_copy(&pstinotify)).into()
            }
        }
        unsafe extern "system" fn SetupDeviceParameters<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: PSTI_DEVICE_INFORMATIONW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::SetupDeviceParameters(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: super::DWORD, pszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
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
            GetDeviceValue: GetDeviceValue::<Identity, OFFSET>,
            SetDeviceValue: SetDeviceValue::<Identity, OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Identity, OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Identity, OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Identity, OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Identity, OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Identity, OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Identity, OFFSET>,
            LaunchApplicationForDevice: LaunchApplicationForDevice::<Identity, OFFSET>,
            SetupDeviceParameters: SetupDeviceParameters::<Identity, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStillImageW as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "unknwnbase", feature = "winnt"))]
impl windows_core::RuntimeName for IStillImageW {}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDIAG = *mut STI_DIAG;
pub type LPSTILLIMAGE = IStillImageW;
pub type LPSTILLIMAGEDEVICE = IStiDevice;
pub type LPSTILLIMAGEW = IStillImageW;
#[cfg(feature = "shared")]
pub type LPSTINOTIFY = *mut STINOTIFY;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPSTISUBSCRIBE = *mut STISUBSCRIBE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTI_DIAG = *mut STI_DIAG;
pub const MAX_NOTIFICATION_DATA: i32 = 64;
pub type PSTI = IStillImageW;
pub type PSTIDEVICE = IStiDevice;
pub type PSTIDEVICEW = *mut IStiDeviceW;
pub type PSTIW = IStillImageW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTI_DEVICE_INFORMATION = *mut STI_DEVICE_INFORMATIONW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTI_DEVICE_INFORMATIONW = *mut STI_DEVICE_INFORMATIONW;
#[cfg(feature = "shared")]
pub type PSTI_DEVICE_STATUS = *mut STI_DEVICE_STATUS;
#[cfg(feature = "shared")]
pub type PSTI_DEV_CAPS = *mut STI_DEV_CAPS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTI_ERROR_INFO = *mut STI_ERROR_INFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTI_ERROR_INFOW = *mut STI_ERROR_INFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTI_WIA_DEVICE_INFORMATION = *mut STI_WIA_DEVICE_INFORMATIONW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTI_WIA_DEVICE_INFORMATIONW = *mut STI_WIA_DEVICE_INFORMATIONW;
pub const STIEDFL_ALLDEVICES: i32 = 0;
pub const STIEDFL_ATTACHEDONLY: i32 = 1;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STINOTIFY {
    pub dwSize: super::DWORD,
    pub guidNotificationCode: windows_core::GUID,
    pub abNotificationData: [super::BYTE; 64],
}
#[cfg(feature = "shared")]
impl Default for STINOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct STISUBSCRIBE {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwFilter: super::DWORD,
    pub hWndNotify: super::HWND,
    pub hEvent: super::HANDLE,
    pub uiNotificationMessage: super::UINT,
}
pub const STI_ADD_DEVICE_BROADCAST_ACTION: windows_core::PCSTR = windows_core::s!("Arrival");
pub const STI_ADD_DEVICE_BROADCAST_STRING: windows_core::PCSTR = windows_core::s!("STI\\Arrival\\%s");
pub const STI_DEVICE_CREATE_BOTH: i32 = 3;
pub const STI_DEVICE_CREATE_DATA: i32 = 2;
pub const STI_DEVICE_CREATE_MASK: i32 = 65535;
pub const STI_DEVICE_CREATE_STATUS: i32 = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type STI_DEVICE_INFORMATION = STI_DEVICE_INFORMATIONW;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: super::DWORD,
    pub DeviceType: STI_DEVICE_TYPE,
    pub szDeviceInternalName: [super::WCHAR; 128],
    pub DeviceCapabilities: STI_DEV_CAPS,
    pub dwHardwareConfiguration: super::DWORD,
    pub pszVendorDescription: windows_core::PWSTR,
    pub pszDeviceDescription: windows_core::PWSTR,
    pub pszPortName: windows_core::PWSTR,
    pub pszPropProvider: windows_core::PWSTR,
    pub pszLocalName: windows_core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STI_DEVICE_MJ_TYPE = i32;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct STI_DEVICE_STATUS {
    pub dwSize: super::DWORD,
    pub StatusMask: super::DWORD,
    pub dwOnlineState: super::DWORD,
    pub dwHardwareStatusCode: super::DWORD,
    pub dwEventHandlingState: super::DWORD,
    pub dwPollingInterval: super::DWORD,
}
pub type STI_DEVICE_TYPE = u32;
pub const STI_DEVSTATUS_EVENTS_STATE: i32 = 2;
pub const STI_DEVSTATUS_ONLINE_STATE: i32 = 1;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct STI_DEV_CAPS {
    pub dwGeneric: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct STI_DIAG {
    pub dwSize: super::DWORD,
    pub dwBasicDiagCode: super::DWORD,
    pub dwVendorDiagCode: super::DWORD,
    pub dwStatusMask: super::DWORD,
    pub sErrorInfo: STI_ERROR_INFO,
}
pub const STI_DIAGCODE_HWPRESENCE: i32 = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type STI_ERROR_INFO = STI_ERROR_INFOW;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STI_ERROR_INFOW {
    pub dwSize: super::DWORD,
    pub dwGenericError: super::DWORD,
    pub dwVendorError: super::DWORD,
    pub szExtendedErrorText: [super::WCHAR; 255],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STI_ERROR_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STI_EVENTHANDLING_ENABLED: i32 = 1;
pub const STI_EVENTHANDLING_PENDING: i32 = 4;
pub const STI_EVENTHANDLING_POLLING: i32 = 2;
pub const STI_GENCAP_AUTO_PORTSELECT: i32 = 8;
#[cfg(feature = "shared")]
pub const STI_GENCAP_COMMON_MASK: super::DWORD = 255;
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: i32 = 4;
pub const STI_GENCAP_NOTIFICATIONS: i32 = 1;
pub const STI_GENCAP_POLLING_NEEDED: i32 = 2;
pub const STI_GENCAP_SUBSET: i32 = 32;
pub const STI_GENCAP_WIA: i32 = 16;
pub const STI_HW_CONFIG_PARALLEL: i32 = 16;
pub const STI_HW_CONFIG_SCSI: i32 = 2;
pub const STI_HW_CONFIG_SERIAL: i32 = 8;
pub const STI_HW_CONFIG_UNKNOWN: i32 = 1;
pub const STI_HW_CONFIG_USB: i32 = 4;
pub const STI_MAX_INTERNAL_NAME_LENGTH: i32 = 128;
pub const STI_ONLINESTATE_BUSY: i32 = 256;
pub const STI_ONLINESTATE_ERROR: i32 = 4;
pub const STI_ONLINESTATE_INITIALIZING: i32 = 1024;
pub const STI_ONLINESTATE_IO_ACTIVE: i32 = 128;
pub const STI_ONLINESTATE_OFFLINE: i32 = 64;
pub const STI_ONLINESTATE_OPERATIONAL: i32 = 1;
pub const STI_ONLINESTATE_PAPER_JAM: i32 = 16;
pub const STI_ONLINESTATE_PAPER_PROBLEM: i32 = 32;
pub const STI_ONLINESTATE_PAUSED: i32 = 8;
pub const STI_ONLINESTATE_PENDING: i32 = 2;
pub const STI_ONLINESTATE_POWER_SAVE: i32 = 8192;
pub const STI_ONLINESTATE_TRANSFERRING: i32 = 512;
pub const STI_ONLINESTATE_USER_INTERVENTION: i32 = 4096;
pub const STI_ONLINESTATE_WARMING_UP: i32 = 2048;
pub type STI_RAW_CONTROL_CODE = u32;
pub const STI_RAW_RESERVED: i32 = 4096;
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: windows_core::PCSTR = windows_core::s!("Removal");
pub const STI_REMOVE_DEVICE_BROADCAST_STRING: windows_core::PCSTR = windows_core::s!("STI\\Removal\\%s");
pub const STI_SUBSCRIBE_FLAG_EVENT: i32 = 2;
pub const STI_SUBSCRIBE_FLAG_WINDOW: i32 = 1;
pub const STI_TRACE_ERROR: i32 = 4;
pub const STI_TRACE_INFORMATION: i32 = 1;
pub const STI_TRACE_WARNING: i32 = 2;
pub const STI_UNICODE: i32 = 1;
pub const STI_VERSION: i32 = 2;
pub const STI_VERSION_3: i32 = 16777219;
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080;
pub const STI_VERSION_FLAG_UNICODE: i32 = 16777216;
pub const STI_VERSION_MIN_ALLOWED: i32 = 2;
pub const STI_VERSION_REAL: i32 = 2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type STI_WIA_DEVICE_INFORMATION = STI_WIA_DEVICE_INFORMATIONW;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: super::DWORD,
    pub DeviceType: STI_DEVICE_TYPE,
    pub szDeviceInternalName: [super::WCHAR; 128],
    pub DeviceCapabilities: STI_DEV_CAPS,
    pub dwHardwareConfiguration: super::DWORD,
    pub pszVendorDescription: windows_core::PWSTR,
    pub pszDeviceDescription: windows_core::PWSTR,
    pub pszPortName: windows_core::PWSTR,
    pub pszPropProvider: windows_core::PWSTR,
    pub pszLocalName: windows_core::PWSTR,
    pub pszUiDll: windows_core::PWSTR,
    pub pszServer: windows_core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = 0;
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = 2;
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = 1;
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = 3;
pub const WIA_INCOMPAT_XP: i32 = 1;

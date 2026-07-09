#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn StartXpsPrintJob<P0, P1, P2>(printername: P0, jobname: P1, outputfilename: P2, progressevent: super::winnt::HANDLE, completionevent: super::winnt::HANDLE, printablepageson: *const u8, printablepagesoncount: u32, xpsprintjob: *mut Option<IXpsPrintJob>, documentstream: *mut Option<IXpsPrintJobStream>, printticketstream: *mut Option<IXpsPrintJobStream>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : windows_core::PCWSTR, jobname : windows_core::PCWSTR, outputfilename : windows_core::PCWSTR, progressevent : super::winnt::HANDLE, completionevent : super::winnt::HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut *mut core::ffi::c_void, documentstream : *mut *mut core::ffi::c_void, printticketstream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StartXpsPrintJob(printername.param().abi(), jobname.param().abi(), outputfilename.param().abi(), progressevent, completionevent, printablepageson, printablepagesoncount, core::mem::transmute(xpsprintjob), core::mem::transmute(documentstream), core::mem::transmute(printticketstream)) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_xpsobjectmodel"))]
#[inline]
pub unsafe fn StartXpsPrintJob1<P0, P1, P2>(printername: P0, jobname: P1, outputfilename: P2, progressevent: super::winnt::HANDLE, completionevent: super::winnt::HANDLE, xpsprintjob: *mut Option<IXpsPrintJob>, printcontentreceiver: *mut Option<super::xpsobjectmodel::IXpsOMPackageTarget>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : windows_core::PCWSTR, jobname : windows_core::PCWSTR, outputfilename : windows_core::PCWSTR, progressevent : super::winnt::HANDLE, completionevent : super::winnt::HANDLE, xpsprintjob : *mut *mut core::ffi::c_void, printcontentreceiver : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StartXpsPrintJob1(printername.param().abi(), jobname.param().abi(), outputfilename.param().abi(), progressevent, completionevent, core::mem::transmute(xpsprintjob), core::mem::transmute(printcontentreceiver)) }
}
windows_core::imp::define_interface!(IXpsPrintJob, IXpsPrintJob_Vtbl, 0x5ab89b06_8194_425f_ab3b_d7a96e350161);
windows_core::imp::interface_hierarchy!(IXpsPrintJob, windows_core::IUnknown);
impl IXpsPrintJob {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetJobStatus(&self) -> windows_core::Result<XPS_JOB_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJobStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJobStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_JOB_STATUS) -> windows_core::HRESULT,
}
pub trait IXpsPrintJob_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn GetJobStatus(&self) -> windows_core::Result<XPS_JOB_STATUS>;
}
impl IXpsPrintJob_Vtbl {
    pub const fn new<Identity: IXpsPrintJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPrintJob_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn GetJobStatus<Identity: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsPrintJob_Impl::GetJobStatus(this) {
                    Ok(ok__) => {
                        jobstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            GetJobStatus: GetJobStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJob as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsPrintJob {}
#[cfg(feature = "Win32_objidlbase")]
windows_core::imp::define_interface!(IXpsPrintJobStream, IXpsPrintJobStream_Vtbl, 0x7a77dc5f_45d6_4dff_9307_d8cb846347ca);
#[cfg(feature = "Win32_objidlbase")]
impl core::ops::Deref for IXpsPrintJobStream {
    type Target = super::objidlbase::ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_objidlbase")]
windows_core::imp::interface_hierarchy!(IXpsPrintJobStream, windows_core::IUnknown, super::objidlbase::ISequentialStream);
#[cfg(feature = "Win32_objidlbase")]
impl IXpsPrintJobStream {
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_Vtbl {
    pub base__: super::objidlbase::ISequentialStream_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IXpsPrintJobStream_Impl: super::objidlbase::ISequentialStream_Impl {
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IXpsPrintJobStream_Vtbl {
    pub const fn new<Identity: IXpsPrintJobStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IXpsPrintJobStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPrintJobStream_Impl::Close(this).into()
            }
        }
        Self { base__: super::objidlbase::ISequentialStream_Vtbl::new::<Identity, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IXpsPrintJobStream {}
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = 2;
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = 1;
pub type XPS_JOB_COMPLETION = i32;
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = 3;
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: windows_core::HRESULT,
}

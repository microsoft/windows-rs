#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintDocumentPackageStatusEvent_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintDocumentPackageStatusEvent {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintDocumentPackageStatusEvent_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>() -> IPrintDocumentPackageStatusEvent_Vtbl {
        unsafe extern "system" fn PackageStatusUpdated<Identity: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintDocumentPackageStatusEvent_Impl::PackageStatusUpdated(this, core::mem::transmute_copy(&packagestatus)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PackageStatusUpdated: PackageStatusUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageStatusEvent as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IPrintDocumentPackageTarget_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPackageTarget(&self, guidtargettype: *const windows_core::GUID, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintDocumentPackageTarget {}
impl IPrintDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetPackageTargetTypes<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintDocumentPackageTarget_Impl::GetPackageTargetTypes(this, core::mem::transmute_copy(&targetcount), core::mem::transmute_copy(&targettypes)).into()
        }
        unsafe extern "system" fn GetPackageTarget<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtargettype: *const windows_core::GUID, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintDocumentPackageTarget_Impl::GetPackageTarget(this, core::mem::transmute_copy(&guidtargettype), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvtarget)).into()
        }
        unsafe extern "system" fn Cancel<Identity: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintDocumentPackageTarget_Impl::Cancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageTargetTypes: GetPackageTargetTypes::<Identity, OFFSET>,
            GetPackageTarget: GetPackageTarget::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as windows_core::Interface>::IID
    }
}
pub trait IPrintDocumentPackageTarget2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetIsTargetIppPrinter(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetTargetIppPrintDevice(&self, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintDocumentPackageTarget2 {}
impl IPrintDocumentPackageTarget2_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTarget2_Vtbl {
        unsafe extern "system" fn GetIsTargetIppPrinter<Identity: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isippprinter: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintDocumentPackageTarget2_Impl::GetIsTargetIppPrinter(this) {
                Ok(ok__) => {
                    isippprinter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetIppPrintDevice<Identity: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvtarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintDocumentPackageTarget2_Impl::GetTargetIppPrintDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvtarget)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsTargetIppPrinter: GetIsTargetIppPrinter::<Identity, OFFSET>,
            GetTargetIppPrintDevice: GetTargetIppPrintDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageTargetFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateDocumentPackageTargetForPrintJob(&self, printername: &windows_core::PCWSTR, jobname: &windows_core::PCWSTR, joboutputstream: Option<&super::super::super::System::Com::IStream>, jobprintticketstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintDocumentPackageTargetFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageTargetFactory_Vtbl {
    pub const fn new<Identity: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTargetFactory_Vtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Identity: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printername: windows_core::PCWSTR, jobname: windows_core::PCWSTR, joboutputstream: *mut core::ffi::c_void, jobprintticketstream: *mut core::ffi::c_void, docpackagetarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintDocumentPackageTargetFactory_Impl::CreateDocumentPackageTargetForPrintJob(this, core::mem::transmute(&printername), core::mem::transmute(&jobname), windows_core::from_raw_borrowed(&joboutputstream), windows_core::from_raw_borrowed(&jobprintticketstream)) {
                Ok(ok__) => {
                    docpackagetarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as windows_core::Interface>::IID
    }
}
pub trait IXpsPrintJob_Impl: Sized + windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn GetJobStatus(&self, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsPrintJob {}
impl IXpsPrintJob_Vtbl {
    pub const fn new<Identity: IXpsPrintJob_Impl, const OFFSET: isize>() -> IXpsPrintJob_Vtbl {
        unsafe extern "system" fn Cancel<Identity: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPrintJob_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn GetJobStatus<Identity: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPrintJob_Impl::GetJobStatus(this, core::mem::transmute_copy(&jobstatus)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStream_Impl: Sized + super::super::super::System::Com::ISequentialStream_Impl {
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsPrintJobStream {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream_Vtbl {
    pub const fn new<Identity: IXpsPrintJobStream_Impl, const OFFSET: isize>() -> IXpsPrintJobStream_Vtbl {
        unsafe extern "system" fn Close<Identity: IXpsPrintJobStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPrintJobStream_Impl::Close(this).into()
        }
        Self { base__: super::super::super::System::Com::ISequentialStream_Vtbl::new::<Identity, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::ISequentialStream as windows_core::Interface>::IID
    }
}

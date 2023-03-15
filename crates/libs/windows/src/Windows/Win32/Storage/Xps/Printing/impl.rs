#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintDocumentPackageStatusEvent_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IPrintDocumentPackageStatusEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintDocumentPackageStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>() -> IPrintDocumentPackageStatusEvent_Vtbl {
        unsafe extern "system" fn PackageStatusUpdated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PackageStatusUpdated(::core::mem::transmute_copy(&packagestatus)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PackageStatusUpdated: PackageStatusUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageStatusEvent as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"implement\"`*"]
pub trait IPrintDocumentPackageTarget_Impl: Sized {
    fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPackageTarget(&self, guidtargettype: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPrintDocumentPackageTarget {}
impl IPrintDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetPackageTargetTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPackageTargetTypes(::core::mem::transmute_copy(&targetcount), ::core::mem::transmute_copy(&targettypes)).into()
        }
        unsafe extern "system" fn GetPackageTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPackageTarget(::core::mem::transmute_copy(&guidtargettype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtarget)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageTargetTypes: GetPackageTargetTypes::<Identity, Impl, OFFSET>,
            GetPackageTarget: GetPackageTarget::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintDocumentPackageTarget2_Impl: Sized {
    fn GetIsTargetIppPrinter(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetTargetIppPrintDevice(&self, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPrintDocumentPackageTarget2 {}
#[cfg(feature = "Win32_Foundation")]
impl IPrintDocumentPackageTarget2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTarget2_Vtbl {
        unsafe extern "system" fn GetIsTargetIppPrinter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isippprinter: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsTargetIppPrinter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isippprinter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetIppPrintDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTargetIppPrintDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtarget)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsTargetIppPrinter: GetIsTargetIppPrinter::<Identity, Impl, OFFSET>,
            GetTargetIppPrintDevice: GetTargetIppPrintDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageTargetFactory_Impl: Sized {
    fn CreateDocumentPackageTargetForPrintJob(&self, printername: &::windows::core::PCWSTR, jobname: &::windows::core::PCWSTR, joboutputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, jobprintticketstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintDocumentPackageTargetFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageTargetFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTargetFactory_Vtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printername: ::windows::core::PCWSTR, jobname: ::windows::core::PCWSTR, joboutputstream: *mut ::core::ffi::c_void, jobprintticketstream: *mut ::core::ffi::c_void, docpackagetarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentPackageTargetForPrintJob(::core::mem::transmute(&printername), ::core::mem::transmute(&jobname), ::windows::core::from_raw_borrowed(&joboutputstream), ::windows::core::from_raw_borrowed(&jobprintticketstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docpackagetarget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"implement\"`*"]
pub trait IXpsPrintJob_Impl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn GetJobStatus(&self, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXpsPrintJob {}
impl IXpsPrintJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: isize>() -> IXpsPrintJob_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn GetJobStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetJobStatus(::core::mem::transmute_copy(&jobstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetJobStatus: GetJobStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJob as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStream_Impl: Sized + super::super::super::System::Com::ISequentialStream_Impl {
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXpsPrintJobStream {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJobStream_Impl, const OFFSET: isize>() -> IXpsPrintJobStream_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJobStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self { base__: super::super::super::System::Com::ISequentialStream_Vtbl::new::<Identity, Impl, OFFSET>(), Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::ISequentialStream as ::windows::core::ComInterface>::IID
    }
}

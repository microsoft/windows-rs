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
        iid == &<IPrintDocumentPackageStatusEvent as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
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
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPackageTargetTypes: GetPackageTargetTypes::<Identity, Impl, OFFSET>,
            GetPackageTarget: GetPackageTarget::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageTargetFactory_Impl: Sized {
    fn CreateDocumentPackageTargetForPrintJob(&self, printername: &::windows::core::PCWSTR, jobname: &::windows::core::PCWSTR, joboutputstream: &::core::option::Option<super::super::super::System::Com::IStream>, jobprintticketstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintDocumentPackageTargetFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageTargetFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTargetFactory_Vtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printername: ::windows::core::PCWSTR, jobname: ::windows::core::PCWSTR, joboutputstream: *mut ::core::ffi::c_void, jobprintticketstream: *mut ::core::ffi::c_void, docpackagetarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentPackageTargetForPrintJob(::core::mem::transmute(&printername), ::core::mem::transmute(&jobname), ::core::mem::transmute(&joboutputstream), ::core::mem::transmute(&jobprintticketstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docpackagetarget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as ::windows::core::Interface>::IID
    }
}
pub trait IXpsPrintJob_Impl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn GetJobStatus(&self) -> ::windows::core::Result<XPS_JOB_STATUS>;
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
            match this.GetJobStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(jobstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetJobStatus: GetJobStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJob as ::windows::core::Interface>::IID
    }
}
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
        iid == &<IXpsPrintJobStream as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID
    }
}

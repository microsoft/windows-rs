#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintDocumentPackageStatusEvent_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn PackageStatusUpdated(&mut self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintDocumentPackageStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageStatusEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentPackageStatusEvent_Vtbl {
        unsafe extern "system" fn PackageStatusUpdated<Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PackageStatusUpdated(::core::mem::transmute_copy(&packagestatus)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PackageStatusUpdated: PackageStatusUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageStatusEvent as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IPrintDocumentPackageTarget_Impl: Sized {
    fn GetPackageTargetTypes(&mut self, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPackageTarget(&mut self, guidtargettype: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
impl IPrintDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetPackageTargetTypes<Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPackageTargetTypes(::core::mem::transmute_copy(&targetcount), ::core::mem::transmute_copy(&targettypes)).into()
        }
        unsafe extern "system" fn GetPackageTarget<Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPackageTarget(::core::mem::transmute_copy(&guidtargettype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtarget)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPackageTargetTypes: GetPackageTargetTypes::<Impl, IMPL_OFFSET>,
            GetPackageTarget: GetPackageTarget::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPrintDocumentPackageTargetFactory_Impl: Sized {
    fn CreateDocumentPackageTargetForPrintJob(&mut self, printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, joboutputstream: &::core::option::Option<super::super::super::System::Com::IStream>, jobprintticketstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPrintDocumentPackageTargetFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageTargetFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentPackageTargetFactory_Vtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, joboutputstream: ::windows::core::RawPtr, jobprintticketstream: ::windows::core::RawPtr, docpackagetarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentPackageTargetForPrintJob(::core::mem::transmute_copy(&printername), ::core::mem::transmute_copy(&jobname), ::core::mem::transmute(&joboutputstream), ::core::mem::transmute(&jobprintticketstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *docpackagetarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as ::windows::core::Interface>::IID
    }
}
pub trait IXpsPrintJob_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn GetJobStatus(&mut self) -> ::windows::core::Result<XPS_JOB_STATUS>;
}
impl IXpsPrintJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsPrintJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsPrintJob_Vtbl {
        unsafe extern "system" fn Cancel<Impl: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn GetJobStatus<Impl: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *jobstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            GetJobStatus: GetJobStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStream_Impl: Sized + super::super::super::System::Com::ISequentialStream_Impl {
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsPrintJobStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsPrintJobStream_Vtbl {
        unsafe extern "system" fn Close<Impl: IXpsPrintJobStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: super::super::super::System::Com::ISequentialStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID
    }
}

#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintDocumentPackageStatusEventImpl: Sized + IDispatchImpl {
    fn PackageStatusUpdated();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintDocumentPackageStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageStatusEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentPackageStatusEventVtbl {
        unsafe extern "system" fn PackageStatusUpdated<Impl: IPrintDocumentPackageStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, PackageStatusUpdated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageStatusEvent as ::windows::core::Interface>::IID
    }
}
pub trait IPrintDocumentPackageTargetImpl: Sized {
    fn GetPackageTargetTypes();
    fn GetPackageTarget();
    fn Cancel();
}
impl IPrintDocumentPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentPackageTargetVtbl {
        unsafe extern "system" fn GetPackageTargetTypes<Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageTarget<Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPackageTargetTypes::<Impl, IMPL_OFFSET>, GetPackageTarget::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPrintDocumentPackageTargetFactoryImpl: Sized {
    fn CreateDocumentPackageTargetForPrintJob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPrintDocumentPackageTargetFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageTargetFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentPackageTargetFactoryVtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Impl: IPrintDocumentPackageTargetFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, joboutputstream: ::windows::core::RawPtr, jobprintticketstream: ::windows::core::RawPtr, docpackagetarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDocumentPackageTargetForPrintJob::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as ::windows::core::Interface>::IID
    }
}
pub trait IXpsPrintJobImpl: Sized {
    fn Cancel();
    fn GetJobStatus();
}
impl IXpsPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsPrintJobVtbl {
        unsafe extern "system" fn Cancel<Impl: IXpsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJobStatus<Impl: IXpsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, GetJobStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStreamImpl: Sized + ISequentialStreamImpl {
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsPrintJobStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsPrintJobStreamVtbl {
        unsafe extern "system" fn Close<Impl: IXpsPrintJobStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as ::windows::core::Interface>::IID
    }
}

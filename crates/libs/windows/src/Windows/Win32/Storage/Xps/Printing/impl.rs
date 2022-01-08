#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageStatusEventImpl: Sized + IDispatchImpl {
    fn PackageStatusUpdated();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintDocumentPackageStatusEvent {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.Printing.IPrintDocumentPackageStatusEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageStatusEventImpl, const OFFSET: isize>() -> IPrintDocumentPackageStatusEventVtbl {
        unsafe extern "system" fn PackageStatusUpdated<Impl: IPrintDocumentPackageStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageStatusUpdated(&*(&packagestatus as *const <PrintDocumentPackageStatus as ::windows::core::Abi>::Abi as *const <PrintDocumentPackageStatus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintDocumentPackageStatusEvent>, ::windows::core::GetTrustLevel, PackageStatusUpdated::<Impl, OFFSET>)
    }
}
pub trait IPrintDocumentPackageTargetImpl: Sized {
    fn GetPackageTargetTypes();
    fn GetPackageTarget();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IPrintDocumentPackageTarget {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.Printing.IPrintDocumentPackageTarget";
}
impl IPrintDocumentPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>() -> IPrintDocumentPackageTargetVtbl {
        unsafe extern "system" fn GetPackageTargetTypes<Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageTargetTypes(::core::mem::transmute_copy(&targetcount), ::core::mem::transmute_copy(&targettypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageTarget<Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtargettype: &::windows::core::GUID, riid: &::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageTarget(&*(&guidtargettype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvtarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPrintDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintDocumentPackageTarget>, ::windows::core::GetTrustLevel, GetPackageTargetTypes::<Impl, OFFSET>, GetPackageTarget::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IPrintDocumentPackageTargetFactoryImpl: Sized {
    fn CreateDocumentPackageTargetForPrintJob();
}
impl ::windows::core::RuntimeName for IPrintDocumentPackageTargetFactory {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.Printing.IPrintDocumentPackageTargetFactory";
}
impl IPrintDocumentPackageTargetFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentPackageTargetFactoryImpl, const OFFSET: isize>() -> IPrintDocumentPackageTargetFactoryVtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Impl: IPrintDocumentPackageTargetFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, joboutputstream: ::windows::core::RawPtr, jobprintticketstream: ::windows::core::RawPtr, docpackagetarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentPackageTargetForPrintJob(
                &*(&printername as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&jobname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&joboutputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&jobprintticketstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&docpackagetarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintDocumentPackageTargetFactory>, ::windows::core::GetTrustLevel, CreateDocumentPackageTargetForPrintJob::<Impl, OFFSET>)
    }
}
pub trait IXpsPrintJobImpl: Sized {
    fn Cancel();
    fn GetJobStatus();
}
impl ::windows::core::RuntimeName for IXpsPrintJob {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.Printing.IXpsPrintJob";
}
impl IXpsPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsPrintJobImpl, const OFFSET: isize>() -> IXpsPrintJobVtbl {
        unsafe extern "system" fn Cancel<Impl: IXpsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobStatus<Impl: IXpsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobStatus(::core::mem::transmute_copy(&jobstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXpsPrintJob>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, GetJobStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStreamImpl: Sized + ISequentialStreamImpl {
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXpsPrintJobStream {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.Printing.IXpsPrintJobStream";
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsPrintJobStreamImpl, const OFFSET: isize>() -> IXpsPrintJobStreamVtbl {
        unsafe extern "system" fn Close<Impl: IXpsPrintJobStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXpsPrintJobStream>, ::windows::core::GetTrustLevel, Close::<Impl, OFFSET>)
    }
}

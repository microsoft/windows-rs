#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDiagnosticActionResult_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Results(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticActionResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDiagnosticActionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticActionResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticActionResult_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IDiagnosticActionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IDiagnosticActionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDiagnosticActionResult, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            Results: Results::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticActionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Json", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDiagnosticInvoker_Impl: Sized {
    fn RunDiagnosticActionAsync(&mut self, context: &::core::option::Option<super::super::Data::Json::JsonObject>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(all(feature = "Data_Json", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvoker";
}
#[cfg(all(feature = "Data_Json", feature = "Foundation", feature = "implement_exclusive"))]
impl IDiagnosticInvoker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticInvoker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticInvoker_Vtbl {
        unsafe extern "system" fn RunDiagnosticActionAsync<Impl: IDiagnosticInvoker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunDiagnosticActionAsync(&*(&context as *const <super::super::Data::Json::JsonObject as ::windows::core::Abi>::Abi as *const <super::super::Data::Json::JsonObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDiagnosticInvoker, BASE_OFFSET>(),
            RunDiagnosticActionAsync: RunDiagnosticActionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticInvoker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDiagnosticInvoker2_Impl: Sized {
    fn RunDiagnosticActionFromStringAsync(&mut self, context: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDiagnosticInvoker2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvoker2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDiagnosticInvoker2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticInvoker2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticInvoker2_Vtbl {
        unsafe extern "system" fn RunDiagnosticActionFromStringAsync<Impl: IDiagnosticInvoker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunDiagnosticActionFromStringAsync(&*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDiagnosticInvoker2, BASE_OFFSET>(),
            RunDiagnosticActionFromStringAsync: RunDiagnosticActionFromStringAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticInvoker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvokerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<DiagnosticInvoker>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<DiagnosticInvoker>;
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticInvokerStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvokerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticInvokerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticInvokerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticInvokerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IDiagnosticInvokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IDiagnosticInvokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IDiagnosticInvokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDiagnosticInvokerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticInvokerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessCpuUsage_Impl: Sized {
    fn GetReport(&mut self) -> ::windows::core::Result<ProcessCpuUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessCpuUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessCpuUsage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessCpuUsage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessCpuUsage_Vtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessCpuUsage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessCpuUsage, BASE_OFFSET>(), GetReport: GetReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessCpuUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProcessCpuUsageReport_Impl: Sized {
    fn KernelTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessCpuUsageReport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProcessCpuUsageReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessCpuUsageReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessCpuUsageReport_Vtbl {
        unsafe extern "system" fn KernelTime<Impl: IProcessCpuUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KernelTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserTime<Impl: IProcessCpuUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessCpuUsageReport, BASE_OFFSET>(),
            KernelTime: KernelTime::<Impl, IMPL_OFFSET>,
            UserTime: UserTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessCpuUsageReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProcessDiagnosticInfo_Impl: Sized {
    fn ProcessId(&mut self) -> ::windows::core::Result<u32>;
    fn ExecutableFileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Parent(&mut self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
    fn ProcessStartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DiskUsage(&mut self) -> ::windows::core::Result<ProcessDiskUsage>;
    fn MemoryUsage(&mut self) -> ::windows::core::Result<ProcessMemoryUsage>;
    fn CpuUsage(&mut self) -> ::windows::core::Result<ProcessCpuUsage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProcessDiagnosticInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfo_Vtbl {
        unsafe extern "system" fn ProcessId<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecutableFileName<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutableFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessStartTime<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiskUsage<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiskUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUsage<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemoryUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUsage<Impl: IProcessDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CpuUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessDiagnosticInfo, BASE_OFFSET>(),
            ProcessId: ProcessId::<Impl, IMPL_OFFSET>,
            ExecutableFileName: ExecutableFileName::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            ProcessStartTime: ProcessStartTime::<Impl, IMPL_OFFSET>,
            DiskUsage: DiskUsage::<Impl, IMPL_OFFSET>,
            MemoryUsage: MemoryUsage::<Impl, IMPL_OFFSET>,
            CpuUsage: CpuUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProcessDiagnosticInfo2_Impl: Sized {
    fn GetAppDiagnosticInfos(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>>;
    fn IsPackaged(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfo2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProcessDiagnosticInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfo2_Vtbl {
        unsafe extern "system" fn GetAppDiagnosticInfos<Impl: IProcessDiagnosticInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPackaged<Impl: IProcessDiagnosticInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPackaged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessDiagnosticInfo2, BASE_OFFSET>(),
            GetAppDiagnosticInfos: GetAppDiagnosticInfos::<Impl, IMPL_OFFSET>,
            IsPackaged: IsPackaged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProcessDiagnosticInfoStatics_Impl: Sized {
    fn GetForProcesses(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>>;
    fn GetForCurrentProcess(&mut self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfoStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProcessDiagnosticInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfoStatics_Vtbl {
        unsafe extern "system" fn GetForProcesses<Impl: IProcessDiagnosticInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForProcesses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentProcess<Impl: IProcessDiagnosticInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentProcess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessDiagnosticInfoStatics, BASE_OFFSET>(),
            GetForProcesses: GetForProcesses::<Impl, IMPL_OFFSET>,
            GetForCurrentProcess: GetForCurrentProcess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoStatics2_Impl: Sized {
    fn TryGetForProcessId(&mut self, processid: u32) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiagnosticInfoStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfoStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfoStatics2_Vtbl {
        unsafe extern "system" fn TryGetForProcessId<Impl: IProcessDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetForProcessId(processid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessDiagnosticInfoStatics2, BASE_OFFSET>(),
            TryGetForProcessId: TryGetForProcessId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiskUsage_Impl: Sized {
    fn GetReport(&mut self) -> ::windows::core::Result<ProcessDiskUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiskUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiskUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiskUsage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiskUsage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiskUsage_Vtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessDiskUsage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessDiskUsage, BASE_OFFSET>(), GetReport: GetReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiskUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiskUsageReport_Impl: Sized {
    fn ReadOperationCount(&mut self) -> ::windows::core::Result<i64>;
    fn WriteOperationCount(&mut self) -> ::windows::core::Result<i64>;
    fn OtherOperationCount(&mut self) -> ::windows::core::Result<i64>;
    fn BytesReadCount(&mut self) -> ::windows::core::Result<i64>;
    fn BytesWrittenCount(&mut self) -> ::windows::core::Result<i64>;
    fn OtherBytesCount(&mut self) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiskUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiskUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiskUsageReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiskUsageReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiskUsageReport_Vtbl {
        unsafe extern "system" fn ReadOperationCount<Impl: IProcessDiskUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOperationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteOperationCount<Impl: IProcessDiskUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteOperationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherOperationCount<Impl: IProcessDiskUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherOperationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesReadCount<Impl: IProcessDiskUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesReadCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesWrittenCount<Impl: IProcessDiskUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesWrittenCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherBytesCount<Impl: IProcessDiskUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherBytesCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessDiskUsageReport, BASE_OFFSET>(),
            ReadOperationCount: ReadOperationCount::<Impl, IMPL_OFFSET>,
            WriteOperationCount: WriteOperationCount::<Impl, IMPL_OFFSET>,
            OtherOperationCount: OtherOperationCount::<Impl, IMPL_OFFSET>,
            BytesReadCount: BytesReadCount::<Impl, IMPL_OFFSET>,
            BytesWrittenCount: BytesWrittenCount::<Impl, IMPL_OFFSET>,
            OtherBytesCount: OtherBytesCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiskUsageReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryUsage_Impl: Sized {
    fn GetReport(&mut self) -> ::windows::core::Result<ProcessMemoryUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessMemoryUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessMemoryUsage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessMemoryUsage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessMemoryUsage_Vtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessMemoryUsage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessMemoryUsage, BASE_OFFSET>(), GetReport: GetReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessMemoryUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryUsageReport_Impl: Sized {
    fn NonPagedPoolSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PageFaultCount(&mut self) -> ::windows::core::Result<u32>;
    fn PageFileSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PagedPoolSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PeakNonPagedPoolSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PeakPageFileSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PeakPagedPoolSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PeakVirtualMemorySizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PeakWorkingSetSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn PrivatePageCount(&mut self) -> ::windows::core::Result<u64>;
    fn VirtualMemorySizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn WorkingSetSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessMemoryUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessMemoryUsageReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessMemoryUsageReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessMemoryUsageReport_Vtbl {
        unsafe extern "system" fn NonPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonPagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageFaultCount<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageFaultCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageFileSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageFileSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PagedPoolSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakNonPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeakNonPagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakPageFileSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeakPageFileSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeakPagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakVirtualMemorySizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeakVirtualMemorySizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakWorkingSetSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeakWorkingSetSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivatePageCount<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivatePageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualMemorySizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualMemorySizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingSetSizeInBytes<Impl: IProcessMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkingSetSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessMemoryUsageReport, BASE_OFFSET>(),
            NonPagedPoolSizeInBytes: NonPagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PageFaultCount: PageFaultCount::<Impl, IMPL_OFFSET>,
            PageFileSizeInBytes: PageFileSizeInBytes::<Impl, IMPL_OFFSET>,
            PagedPoolSizeInBytes: PagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakNonPagedPoolSizeInBytes: PeakNonPagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakPageFileSizeInBytes: PeakPageFileSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakPagedPoolSizeInBytes: PeakPagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakVirtualMemorySizeInBytes: PeakVirtualMemorySizeInBytes::<Impl, IMPL_OFFSET>,
            PeakWorkingSetSizeInBytes: PeakWorkingSetSizeInBytes::<Impl, IMPL_OFFSET>,
            PrivatePageCount: PrivatePageCount::<Impl, IMPL_OFFSET>,
            VirtualMemorySizeInBytes: VirtualMemorySizeInBytes::<Impl, IMPL_OFFSET>,
            WorkingSetSizeInBytes: WorkingSetSizeInBytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessMemoryUsageReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemCpuUsage_Impl: Sized {
    fn GetReport(&mut self) -> ::windows::core::Result<SystemCpuUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemCpuUsage";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemCpuUsage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemCpuUsage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemCpuUsage_Vtbl {
        unsafe extern "system" fn GetReport<Impl: ISystemCpuUsage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemCpuUsage, BASE_OFFSET>(), GetReport: GetReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemCpuUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemCpuUsageReport_Impl: Sized {
    fn KernelTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IdleTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemCpuUsageReport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemCpuUsageReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemCpuUsageReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemCpuUsageReport_Vtbl {
        unsafe extern "system" fn KernelTime<Impl: ISystemCpuUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KernelTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserTime<Impl: ISystemCpuUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdleTime<Impl: ISystemCpuUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdleTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemCpuUsageReport, BASE_OFFSET>(),
            KernelTime: KernelTime::<Impl, IMPL_OFFSET>,
            UserTime: UserTime::<Impl, IMPL_OFFSET>,
            IdleTime: IdleTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemCpuUsageReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfo_Impl: Sized {
    fn MemoryUsage(&mut self) -> ::windows::core::Result<SystemMemoryUsage>;
    fn CpuUsage(&mut self) -> ::windows::core::Result<SystemCpuUsage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemDiagnosticInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDiagnosticInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDiagnosticInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDiagnosticInfo_Vtbl {
        unsafe extern "system" fn MemoryUsage<Impl: ISystemDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemoryUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUsage<Impl: ISystemDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CpuUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemDiagnosticInfo, BASE_OFFSET>(),
            MemoryUsage: MemoryUsage::<Impl, IMPL_OFFSET>,
            CpuUsage: CpuUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDiagnosticInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoStatics_Impl: Sized {
    fn GetForCurrentSystem(&mut self) -> ::windows::core::Result<SystemDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemDiagnosticInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDiagnosticInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDiagnosticInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDiagnosticInfoStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentSystem<Impl: ISystemDiagnosticInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemDiagnosticInfoStatics, BASE_OFFSET>(),
            GetForCurrentSystem: GetForCurrentSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDiagnosticInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoStatics2_Impl: Sized {
    fn IsArchitectureSupported(&mut self, r#type: super::ProcessorArchitecture) -> ::windows::core::Result<bool>;
    fn PreferredArchitecture(&mut self) -> ::windows::core::Result<super::ProcessorArchitecture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDiagnosticInfoStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDiagnosticInfoStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDiagnosticInfoStatics2_Vtbl {
        unsafe extern "system" fn IsArchitectureSupported<Impl: ISystemDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsArchitectureSupported(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredArchitecture<Impl: ISystemDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ProcessorArchitecture) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredArchitecture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemDiagnosticInfoStatics2, BASE_OFFSET>(),
            IsArchitectureSupported: IsArchitectureSupported::<Impl, IMPL_OFFSET>,
            PreferredArchitecture: PreferredArchitecture::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDiagnosticInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMemoryUsage_Impl: Sized {
    fn GetReport(&mut self) -> ::windows::core::Result<SystemMemoryUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemMemoryUsage";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMemoryUsage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMemoryUsage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMemoryUsage_Vtbl {
        unsafe extern "system" fn GetReport<Impl: ISystemMemoryUsage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMemoryUsage, BASE_OFFSET>(), GetReport: GetReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMemoryUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMemoryUsageReport_Impl: Sized {
    fn TotalPhysicalSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn AvailableSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn CommittedSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemMemoryUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMemoryUsageReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMemoryUsageReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMemoryUsageReport_Vtbl {
        unsafe extern "system" fn TotalPhysicalSizeInBytes<Impl: ISystemMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalPhysicalSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableSizeInBytes<Impl: ISystemMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommittedSizeInBytes<Impl: ISystemMemoryUsageReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommittedSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMemoryUsageReport, BASE_OFFSET>(),
            TotalPhysicalSizeInBytes: TotalPhysicalSizeInBytes::<Impl, IMPL_OFFSET>,
            AvailableSizeInBytes: AvailableSizeInBytes::<Impl, IMPL_OFFSET>,
            CommittedSizeInBytes: CommittedSizeInBytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMemoryUsageReport as ::windows::core::Interface>::IID
    }
}

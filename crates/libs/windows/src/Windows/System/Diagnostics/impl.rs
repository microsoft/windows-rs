#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticActionResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Results(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticActionResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticActionResultVtbl {
    pub const fn new<Impl: IDiagnosticActionResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiagnosticActionResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IDiagnosticActionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IDiagnosticActionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiagnosticActionResult>, base.5, ExtendedError::<Impl, OFFSET>, Results::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvokerImpl: Sized {
    fn RunDiagnosticActionAsync(&self, context: &::core::option::Option<super::super::Data::Json::JsonObject>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvoker";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticInvokerVtbl {
    pub const fn new<Impl: IDiagnosticInvokerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiagnosticInvokerVtbl {
        unsafe extern "system" fn RunDiagnosticActionAsync<Impl: IDiagnosticInvokerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunDiagnosticActionAsync(&*(&context as *const <super::super::Data::Json::JsonObject as ::windows::core::Abi>::Abi as *const <super::super::Data::Json::JsonObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiagnosticInvoker>, base.5, RunDiagnosticActionAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvoker2Impl: Sized {
    fn RunDiagnosticActionFromStringAsync(&self, context: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticInvoker2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvoker2";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticInvoker2Vtbl {
    pub const fn new<Impl: IDiagnosticInvoker2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiagnosticInvoker2Vtbl {
        unsafe extern "system" fn RunDiagnosticActionFromStringAsync<Impl: IDiagnosticInvoker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunDiagnosticActionFromStringAsync(&*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiagnosticInvoker2>, base.5, RunDiagnosticActionFromStringAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvokerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<DiagnosticInvoker>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<DiagnosticInvoker>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiagnosticInvokerStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvokerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDiagnosticInvokerStaticsVtbl {
    pub const fn new<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiagnosticInvokerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiagnosticInvokerStatics>, base.5, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessCpuUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<ProcessCpuUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessCpuUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessCpuUsageVtbl {
    pub const fn new<Impl: IProcessCpuUsageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessCpuUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessCpuUsageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessCpuUsage>, base.5, GetReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessCpuUsageReportImpl: Sized {
    fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessCpuUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessCpuUsageReportVtbl {
    pub const fn new<Impl: IProcessCpuUsageReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessCpuUsageReportVtbl {
        unsafe extern "system" fn KernelTime<Impl: IProcessCpuUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KernelTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserTime<Impl: IProcessCpuUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessCpuUsageReport>, base.5, KernelTime::<Impl, OFFSET>, UserTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoImpl: Sized {
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Parent(&self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
    fn ProcessStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DiskUsage(&self) -> ::windows::core::Result<ProcessDiskUsage>;
    fn MemoryUsage(&self) -> ::windows::core::Result<ProcessMemoryUsage>;
    fn CpuUsage(&self) -> ::windows::core::Result<ProcessCpuUsage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiagnosticInfoVtbl {
    pub const fn new<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessDiagnosticInfoVtbl {
        unsafe extern "system" fn ProcessId<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecutableFileName<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecutableFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessStartTime<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiskUsage<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiskUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUsage<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MemoryUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUsage<Impl: IProcessDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CpuUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfo>, base.5, ProcessId::<Impl, OFFSET>, ExecutableFileName::<Impl, OFFSET>, Parent::<Impl, OFFSET>, ProcessStartTime::<Impl, OFFSET>, DiskUsage::<Impl, OFFSET>, MemoryUsage::<Impl, OFFSET>, CpuUsage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfo2Impl: Sized {
    fn GetAppDiagnosticInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>>;
    fn IsPackaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfo2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiagnosticInfo2Vtbl {
    pub const fn new<Impl: IProcessDiagnosticInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessDiagnosticInfo2Vtbl {
        unsafe extern "system" fn GetAppDiagnosticInfos<Impl: IProcessDiagnosticInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAppDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPackaged<Impl: IProcessDiagnosticInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPackaged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfo2>, base.5, GetAppDiagnosticInfos::<Impl, OFFSET>, IsPackaged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoStaticsImpl: Sized {
    fn GetForProcesses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>>;
    fn GetForCurrentProcess(&self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiagnosticInfoStaticsVtbl {
    pub const fn new<Impl: IProcessDiagnosticInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessDiagnosticInfoStaticsVtbl {
        unsafe extern "system" fn GetForProcesses<Impl: IProcessDiagnosticInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForProcesses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentProcess<Impl: IProcessDiagnosticInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentProcess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfoStatics>, base.5, GetForProcesses::<Impl, OFFSET>, GetForCurrentProcess::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoStatics2Impl: Sized {
    fn TryGetForProcessId(&self, processid: u32) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiagnosticInfoStatics2Vtbl {
    pub const fn new<Impl: IProcessDiagnosticInfoStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessDiagnosticInfoStatics2Vtbl {
        unsafe extern "system" fn TryGetForProcessId<Impl: IProcessDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetForProcessId(processid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfoStatics2>, base.5, TryGetForProcessId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiskUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<ProcessDiskUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiskUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiskUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiskUsageVtbl {
    pub const fn new<Impl: IProcessDiskUsageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessDiskUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessDiskUsageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessDiskUsage>, base.5, GetReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiskUsageReportImpl: Sized {
    fn ReadOperationCount(&self) -> ::windows::core::Result<i64>;
    fn WriteOperationCount(&self) -> ::windows::core::Result<i64>;
    fn OtherOperationCount(&self) -> ::windows::core::Result<i64>;
    fn BytesReadCount(&self) -> ::windows::core::Result<i64>;
    fn BytesWrittenCount(&self) -> ::windows::core::Result<i64>;
    fn OtherBytesCount(&self) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessDiskUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiskUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessDiskUsageReportVtbl {
    pub const fn new<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessDiskUsageReportVtbl {
        unsafe extern "system" fn ReadOperationCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOperationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteOperationCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteOperationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherOperationCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OtherOperationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesReadCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BytesReadCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesWrittenCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BytesWrittenCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherBytesCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OtherBytesCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessDiskUsageReport>, base.5, ReadOperationCount::<Impl, OFFSET>, WriteOperationCount::<Impl, OFFSET>, OtherOperationCount::<Impl, OFFSET>, BytesReadCount::<Impl, OFFSET>, BytesWrittenCount::<Impl, OFFSET>, OtherBytesCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<ProcessMemoryUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessMemoryUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessMemoryUsageVtbl {
    pub const fn new<Impl: IProcessMemoryUsageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessMemoryUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessMemoryUsageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessMemoryUsage>, base.5, GetReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryUsageReportImpl: Sized {
    fn NonPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PageFaultCount(&self) -> ::windows::core::Result<u32>;
    fn PageFileSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakNonPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakPageFileSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakVirtualMemorySizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakWorkingSetSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PrivatePageCount(&self) -> ::windows::core::Result<u64>;
    fn VirtualMemorySizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn WorkingSetSizeInBytes(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessMemoryUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessMemoryUsageReportVtbl {
    pub const fn new<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessMemoryUsageReportVtbl {
        unsafe extern "system" fn NonPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NonPagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageFaultCount<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageFaultCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageFileSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageFileSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakNonPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeakNonPagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakPageFileSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeakPageFileSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeakPagedPoolSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakVirtualMemorySizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeakVirtualMemorySizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakWorkingSetSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeakWorkingSetSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivatePageCount<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivatePageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualMemorySizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VirtualMemorySizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingSetSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WorkingSetSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IProcessMemoryUsageReport>,
            base.5,
            NonPagedPoolSizeInBytes::<Impl, OFFSET>,
            PageFaultCount::<Impl, OFFSET>,
            PageFileSizeInBytes::<Impl, OFFSET>,
            PagedPoolSizeInBytes::<Impl, OFFSET>,
            PeakNonPagedPoolSizeInBytes::<Impl, OFFSET>,
            PeakPageFileSizeInBytes::<Impl, OFFSET>,
            PeakPagedPoolSizeInBytes::<Impl, OFFSET>,
            PeakVirtualMemorySizeInBytes::<Impl, OFFSET>,
            PeakWorkingSetSizeInBytes::<Impl, OFFSET>,
            PrivatePageCount::<Impl, OFFSET>,
            VirtualMemorySizeInBytes::<Impl, OFFSET>,
            WorkingSetSizeInBytes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemCpuUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<SystemCpuUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemCpuUsage";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemCpuUsageVtbl {
    pub const fn new<Impl: ISystemCpuUsageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemCpuUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: ISystemCpuUsageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemCpuUsage>, base.5, GetReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemCpuUsageReportImpl: Sized {
    fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IdleTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemCpuUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemCpuUsageReportVtbl {
    pub const fn new<Impl: ISystemCpuUsageReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemCpuUsageReportVtbl {
        unsafe extern "system" fn KernelTime<Impl: ISystemCpuUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KernelTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserTime<Impl: ISystemCpuUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdleTime<Impl: ISystemCpuUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IdleTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemCpuUsageReport>, base.5, KernelTime::<Impl, OFFSET>, UserTime::<Impl, OFFSET>, IdleTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoImpl: Sized {
    fn MemoryUsage(&self) -> ::windows::core::Result<SystemMemoryUsage>;
    fn CpuUsage(&self) -> ::windows::core::Result<SystemCpuUsage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemDiagnosticInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDiagnosticInfoVtbl {
    pub const fn new<Impl: ISystemDiagnosticInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemDiagnosticInfoVtbl {
        unsafe extern "system" fn MemoryUsage<Impl: ISystemDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MemoryUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUsage<Impl: ISystemDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CpuUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemDiagnosticInfo>, base.5, MemoryUsage::<Impl, OFFSET>, CpuUsage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoStaticsImpl: Sized {
    fn GetForCurrentSystem(&self) -> ::windows::core::Result<SystemDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemDiagnosticInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDiagnosticInfoStaticsVtbl {
    pub const fn new<Impl: ISystemDiagnosticInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemDiagnosticInfoStaticsVtbl {
        unsafe extern "system" fn GetForCurrentSystem<Impl: ISystemDiagnosticInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemDiagnosticInfoStatics>, base.5, GetForCurrentSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoStatics2Impl: Sized {
    fn IsArchitectureSupported(&self, r#type: super::ProcessorArchitecture) -> ::windows::core::Result<bool>;
    fn PreferredArchitecture(&self) -> ::windows::core::Result<super::ProcessorArchitecture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDiagnosticInfoStatics2Vtbl {
    pub const fn new<Impl: ISystemDiagnosticInfoStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemDiagnosticInfoStatics2Vtbl {
        unsafe extern "system" fn IsArchitectureSupported<Impl: ISystemDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsArchitectureSupported(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredArchitecture<Impl: ISystemDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ProcessorArchitecture) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredArchitecture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemDiagnosticInfoStatics2>, base.5, IsArchitectureSupported::<Impl, OFFSET>, PreferredArchitecture::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMemoryUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<SystemMemoryUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemMemoryUsage";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMemoryUsageVtbl {
    pub const fn new<Impl: ISystemMemoryUsageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMemoryUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: ISystemMemoryUsageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMemoryUsage>, base.5, GetReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMemoryUsageReportImpl: Sized {
    fn TotalPhysicalSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn AvailableSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn CommittedSizeInBytes(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemMemoryUsageReport";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMemoryUsageReportVtbl {
    pub const fn new<Impl: ISystemMemoryUsageReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMemoryUsageReportVtbl {
        unsafe extern "system" fn TotalPhysicalSizeInBytes<Impl: ISystemMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalPhysicalSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableSizeInBytes<Impl: ISystemMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AvailableSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommittedSizeInBytes<Impl: ISystemMemoryUsageReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommittedSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMemoryUsageReport>, base.5, TotalPhysicalSizeInBytes::<Impl, OFFSET>, AvailableSizeInBytes::<Impl, OFFSET>, CommittedSizeInBytes::<Impl, OFFSET>)
    }
}

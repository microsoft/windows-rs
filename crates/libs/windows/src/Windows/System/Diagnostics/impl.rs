#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDiagnosticActionResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Results(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticActionResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDiagnosticActionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticActionResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticActionResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IDiagnosticActionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Results<Impl: IDiagnosticActionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiagnosticActionResult>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>, Results::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticActionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Json", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDiagnosticInvokerImpl: Sized {
    fn RunDiagnosticActionAsync(&self, context: &::core::option::Option<super::super::Data::Json::JsonObject>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(all(feature = "Data_Json", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvoker";
}
#[cfg(all(feature = "Data_Json", feature = "Foundation", feature = "implement_exclusive"))]
impl IDiagnosticInvokerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticInvokerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticInvokerVtbl {
        unsafe extern "system" fn RunDiagnosticActionAsync<Impl: IDiagnosticInvokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiagnosticInvoker>, ::windows::core::GetTrustLevel, RunDiagnosticActionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticInvoker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDiagnosticInvoker2Impl: Sized {
    fn RunDiagnosticActionFromStringAsync(&self, context: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDiagnosticInvoker2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IDiagnosticInvoker2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDiagnosticInvoker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticInvoker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticInvoker2Vtbl {
        unsafe extern "system" fn RunDiagnosticActionFromStringAsync<Impl: IDiagnosticInvoker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiagnosticInvoker2>, ::windows::core::GetTrustLevel, RunDiagnosticActionFromStringAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticInvoker2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiagnosticInvokerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiagnosticInvokerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSupported<Impl: IDiagnosticInvokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiagnosticInvokerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>, GetForUser::<Impl, IMPL_OFFSET>, IsSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiagnosticInvokerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessCpuUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessCpuUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessCpuUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessCpuUsage>, ::windows::core::GetTrustLevel, GetReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessCpuUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProcessCpuUsageReportImpl: Sized {
    fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessCpuUsageReport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProcessCpuUsageReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessCpuUsageReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessCpuUsageReportVtbl {
        unsafe extern "system" fn KernelTime<Impl: IProcessCpuUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UserTime<Impl: IProcessCpuUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessCpuUsageReport>, ::windows::core::GetTrustLevel, KernelTime::<Impl, IMPL_OFFSET>, UserTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessCpuUsageReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProcessDiagnosticInfoImpl: Sized {
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Parent(&self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
    fn ProcessStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DiskUsage(&self) -> ::windows::core::Result<ProcessDiskUsage>;
    fn MemoryUsage(&self) -> ::windows::core::Result<ProcessMemoryUsage>;
    fn CpuUsage(&self) -> ::windows::core::Result<ProcessCpuUsage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProcessDiagnosticInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfoVtbl {
        unsafe extern "system" fn ProcessId<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExecutableFileName<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Parent<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProcessStartTime<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DiskUsage<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MemoryUsage<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CpuUsage<Impl: IProcessDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfo>,
            ::windows::core::GetTrustLevel,
            ProcessId::<Impl, IMPL_OFFSET>,
            ExecutableFileName::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            ProcessStartTime::<Impl, IMPL_OFFSET>,
            DiskUsage::<Impl, IMPL_OFFSET>,
            MemoryUsage::<Impl, IMPL_OFFSET>,
            CpuUsage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProcessDiagnosticInfo2Impl: Sized {
    fn GetAppDiagnosticInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>>;
    fn IsPackaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfo2 {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProcessDiagnosticInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfo2Vtbl {
        unsafe extern "system" fn GetAppDiagnosticInfos<Impl: IProcessDiagnosticInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPackaged<Impl: IProcessDiagnosticInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfo2>, ::windows::core::GetTrustLevel, GetAppDiagnosticInfos::<Impl, IMPL_OFFSET>, IsPackaged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProcessDiagnosticInfoStaticsImpl: Sized {
    fn GetForProcesses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>>;
    fn GetForCurrentProcess(&self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.IProcessDiagnosticInfoStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProcessDiagnosticInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfoStaticsVtbl {
        unsafe extern "system" fn GetForProcesses<Impl: IProcessDiagnosticInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForCurrentProcess<Impl: IProcessDiagnosticInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfoStatics>, ::windows::core::GetTrustLevel, GetForProcesses::<Impl, IMPL_OFFSET>, GetForCurrentProcess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfoStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiagnosticInfoStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiagnosticInfoStatics2Vtbl {
        unsafe extern "system" fn TryGetForProcessId<Impl: IProcessDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessDiagnosticInfoStatics2>, ::windows::core::GetTrustLevel, TryGetForProcessId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiagnosticInfoStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiskUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiskUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessDiskUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessDiskUsage>, ::windows::core::GetTrustLevel, GetReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiskUsage as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessDiskUsageReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessDiskUsageReportVtbl {
        unsafe extern "system" fn ReadOperationCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WriteOperationCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OtherOperationCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BytesReadCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BytesWrittenCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OtherBytesCount<Impl: IProcessDiskUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProcessDiskUsageReport>,
            ::windows::core::GetTrustLevel,
            ReadOperationCount::<Impl, IMPL_OFFSET>,
            WriteOperationCount::<Impl, IMPL_OFFSET>,
            OtherOperationCount::<Impl, IMPL_OFFSET>,
            BytesReadCount::<Impl, IMPL_OFFSET>,
            BytesWrittenCount::<Impl, IMPL_OFFSET>,
            OtherBytesCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessDiskUsageReport as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessMemoryUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessMemoryUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: IProcessMemoryUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessMemoryUsage>, ::windows::core::GetTrustLevel, GetReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessMemoryUsage as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessMemoryUsageReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessMemoryUsageReportVtbl {
        unsafe extern "system" fn NonPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PageFaultCount<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PageFileSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeakNonPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeakPageFileSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeakPagedPoolSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeakVirtualMemorySizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeakWorkingSetSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrivatePageCount<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VirtualMemorySizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WorkingSetSizeInBytes<Impl: IProcessMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProcessMemoryUsageReport>,
            ::windows::core::GetTrustLevel,
            NonPagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PageFaultCount::<Impl, IMPL_OFFSET>,
            PageFileSizeInBytes::<Impl, IMPL_OFFSET>,
            PagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakNonPagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakPageFileSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakPagedPoolSizeInBytes::<Impl, IMPL_OFFSET>,
            PeakVirtualMemorySizeInBytes::<Impl, IMPL_OFFSET>,
            PeakWorkingSetSizeInBytes::<Impl, IMPL_OFFSET>,
            PrivatePageCount::<Impl, IMPL_OFFSET>,
            VirtualMemorySizeInBytes::<Impl, IMPL_OFFSET>,
            WorkingSetSizeInBytes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessMemoryUsageReport as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemCpuUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemCpuUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: ISystemCpuUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemCpuUsage>, ::windows::core::GetTrustLevel, GetReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemCpuUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemCpuUsageReportImpl: Sized {
    fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IdleTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ISystemCpuUsageReport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemCpuUsageReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemCpuUsageReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemCpuUsageReportVtbl {
        unsafe extern "system" fn KernelTime<Impl: ISystemCpuUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UserTime<Impl: ISystemCpuUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IdleTime<Impl: ISystemCpuUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemCpuUsageReport>, ::windows::core::GetTrustLevel, KernelTime::<Impl, IMPL_OFFSET>, UserTime::<Impl, IMPL_OFFSET>, IdleTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemCpuUsageReport as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDiagnosticInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDiagnosticInfoVtbl {
        unsafe extern "system" fn MemoryUsage<Impl: ISystemDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CpuUsage<Impl: ISystemDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemDiagnosticInfo>, ::windows::core::GetTrustLevel, MemoryUsage::<Impl, IMPL_OFFSET>, CpuUsage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDiagnosticInfo as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDiagnosticInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDiagnosticInfoStaticsVtbl {
        unsafe extern "system" fn GetForCurrentSystem<Impl: ISystemDiagnosticInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemDiagnosticInfoStatics>, ::windows::core::GetTrustLevel, GetForCurrentSystem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDiagnosticInfoStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDiagnosticInfoStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDiagnosticInfoStatics2Vtbl {
        unsafe extern "system" fn IsArchitectureSupported<Impl: ISystemDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PreferredArchitecture<Impl: ISystemDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ProcessorArchitecture) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemDiagnosticInfoStatics2>, ::windows::core::GetTrustLevel, IsArchitectureSupported::<Impl, IMPL_OFFSET>, PreferredArchitecture::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDiagnosticInfoStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMemoryUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMemoryUsageVtbl {
        unsafe extern "system" fn GetReport<Impl: ISystemMemoryUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemMemoryUsage>, ::windows::core::GetTrustLevel, GetReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMemoryUsage as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMemoryUsageReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMemoryUsageReportVtbl {
        unsafe extern "system" fn TotalPhysicalSizeInBytes<Impl: ISystemMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailableSizeInBytes<Impl: ISystemMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CommittedSizeInBytes<Impl: ISystemMemoryUsageReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemMemoryUsageReport>, ::windows::core::GetTrustLevel, TotalPhysicalSizeInBytes::<Impl, IMPL_OFFSET>, AvailableSizeInBytes::<Impl, IMPL_OFFSET>, CommittedSizeInBytes::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMemoryUsageReport as ::windows::core::Interface>::IID
    }
}

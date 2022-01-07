#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticActionsStaticsImpl: Sized {
    fn IsScenarioEnabled(&self, scenarioid: &::windows::core::GUID) -> ::windows::core::Result<bool>;
    fn TryEscalateScenario(&self, scenarioid: &::windows::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: &::windows::core::HSTRING, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn DownloadLatestSettingsForNamespace(&self, partner: &::windows::core::HSTRING, feature: &::windows::core::HSTRING, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState>;
    fn GetActiveScenarioList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
    fn ForceUpload(&self, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState>;
    fn IsTraceRunning(&self, slottype: PlatformDiagnosticTraceSlotType, scenarioid: &::windows::core::GUID, traceprofilehash: u64) -> ::windows::core::Result<PlatformDiagnosticTraceSlotState>;
    fn GetActiveTraceRuntime(&self, slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<PlatformDiagnosticTraceRuntimeInfo>;
    fn GetKnownTraceList(&self, slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformDiagnosticActionsStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformDiagnosticActionsStaticsVtbl {
    pub const fn new<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlatformDiagnosticActionsStaticsVtbl {
        unsafe extern "system" fn IsScenarioEnabled<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenarioid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScenarioEnabled(&*(&scenarioid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEscalateScenario<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenarioid: ::windows::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryEscalateScenario(
                &*(&scenarioid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                escalationtype,
                &*(&outputdirectory as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                timestampoutputdirectory,
                forceescalationupload,
                &*(&triggers as *const <super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadLatestSettingsForNamespace<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partner: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, feature: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadLatestSettingsForNamespace(&*(&partner as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&feature as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), isscenarionamespace, downloadovercostednetwork, downloadoverbattery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveScenarioList<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetActiveScenarioList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceUpload<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForceUpload(latency, uploadovercostednetwork, uploadoverbattery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTraceRunning<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, slottype: PlatformDiagnosticTraceSlotType, scenarioid: ::windows::core::GUID, traceprofilehash: u64, result__: *mut PlatformDiagnosticTraceSlotState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTraceRunning(slottype, &*(&scenarioid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), traceprofilehash) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveTraceRuntime<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, slottype: PlatformDiagnosticTraceSlotType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetActiveTraceRuntime(slottype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKnownTraceList<Impl: IPlatformDiagnosticActionsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, slottype: PlatformDiagnosticTraceSlotType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKnownTraceList(slottype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlatformDiagnosticActionsStatics>, base.5, IsScenarioEnabled::<Impl, OFFSET>, TryEscalateScenario::<Impl, OFFSET>, DownloadLatestSettingsForNamespace::<Impl, OFFSET>, GetActiveScenarioList::<Impl, OFFSET>, ForceUpload::<Impl, OFFSET>, IsTraceRunning::<Impl, OFFSET>, GetActiveTraceRuntime::<Impl, OFFSET>, GetKnownTraceList::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticTraceInfoImpl: Sized {
    fn ScenarioId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ProfileHash(&self) -> ::windows::core::Result<u64>;
    fn IsExclusive(&self) -> ::windows::core::Result<bool>;
    fn IsAutoLogger(&self) -> ::windows::core::Result<bool>;
    fn MaxTraceDurationFileTime(&self) -> ::windows::core::Result<i64>;
    fn Priority(&self) -> ::windows::core::Result<PlatformDiagnosticTracePriority>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformDiagnosticTraceInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformDiagnosticTraceInfoVtbl {
    pub const fn new<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlatformDiagnosticTraceInfoVtbl {
        unsafe extern "system" fn ScenarioId<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScenarioId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProfileHash<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProfileHash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsExclusive<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoLogger<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAutoLogger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTraceDurationFileTime<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxTraceDurationFileTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IPlatformDiagnosticTraceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PlatformDiagnosticTracePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlatformDiagnosticTraceInfo>, base.5, ScenarioId::<Impl, OFFSET>, ProfileHash::<Impl, OFFSET>, IsExclusive::<Impl, OFFSET>, IsAutoLogger::<Impl, OFFSET>, MaxTraceDurationFileTime::<Impl, OFFSET>, Priority::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticTraceRuntimeInfoImpl: Sized {
    fn RuntimeFileTime(&self) -> ::windows::core::Result<i64>;
    fn EtwRuntimeFileTime(&self) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformDiagnosticTraceRuntimeInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformDiagnosticTraceRuntimeInfoVtbl {
    pub const fn new<Impl: IPlatformDiagnosticTraceRuntimeInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlatformDiagnosticTraceRuntimeInfoVtbl {
        unsafe extern "system" fn RuntimeFileTime<Impl: IPlatformDiagnosticTraceRuntimeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RuntimeFileTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EtwRuntimeFileTime<Impl: IPlatformDiagnosticTraceRuntimeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EtwRuntimeFileTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlatformDiagnosticTraceRuntimeInfo>, base.5, RuntimeFileTime::<Impl, OFFSET>, EtwRuntimeFileTime::<Impl, OFFSET>)
    }
}

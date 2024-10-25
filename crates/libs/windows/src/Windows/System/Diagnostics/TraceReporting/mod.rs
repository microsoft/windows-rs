windows_core::imp::define_interface!(IPlatformDiagnosticActionsStatics, IPlatformDiagnosticActionsStatics_Vtbl, 0xc1145cfa_9292_4267_890a_9ea3ed072312);
impl windows_core::RuntimeType for IPlatformDiagnosticActionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformDiagnosticActionsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsScenarioEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryEscalateScenario: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, PlatformDiagnosticEscalationType, core::mem::MaybeUninit<windows_core::HSTRING>, bool, bool, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryEscalateScenario: usize,
    pub DownloadLatestSettingsForNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool, bool, bool, *mut PlatformDiagnosticActionState) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActiveScenarioList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActiveScenarioList: usize,
    pub ForceUpload: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformDiagnosticEventBufferLatencies, bool, bool, *mut PlatformDiagnosticActionState) -> windows_core::HRESULT,
    pub IsTraceRunning: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformDiagnosticTraceSlotType, windows_core::GUID, u64, *mut PlatformDiagnosticTraceSlotState) -> windows_core::HRESULT,
    pub GetActiveTraceRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformDiagnosticTraceSlotType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKnownTraceList: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformDiagnosticTraceSlotType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKnownTraceList: usize,
}
windows_core::imp::define_interface!(IPlatformDiagnosticTraceInfo, IPlatformDiagnosticTraceInfo_Vtbl, 0xf870ed97_d597_4bf7_88dc_cf5c7dc2a1d2);
impl windows_core::RuntimeType for IPlatformDiagnosticTraceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformDiagnosticTraceInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScenarioId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ProfileHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAutoLogger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxTraceDurationFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformDiagnosticTracePriority) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlatformDiagnosticTraceRuntimeInfo, IPlatformDiagnosticTraceRuntimeInfo_Vtbl, 0x3d4d5e2d_01d8_4768_8554_1eb1ca610986);
impl windows_core::RuntimeType for IPlatformDiagnosticTraceRuntimeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformDiagnosticTraceRuntimeInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RuntimeFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub EtwRuntimeFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub struct PlatformDiagnosticActions;
impl PlatformDiagnosticActions {}
impl windows_core::RuntimeName for PlatformDiagnosticActions {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PlatformDiagnosticTraceInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlatformDiagnosticTraceInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PlatformDiagnosticTraceInfo {
    pub fn ScenarioId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScenarioId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProfileHash(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileHash)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsExclusive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExclusive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAutoLogger(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoLogger)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxTraceDurationFileTime(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxTraceDurationFileTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Priority(&self) -> windows_core::Result<PlatformDiagnosticTracePriority> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlatformDiagnosticTraceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlatformDiagnosticTraceInfo>();
}
unsafe impl windows_core::Interface for PlatformDiagnosticTraceInfo {
    type Vtable = <IPlatformDiagnosticTraceInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlatformDiagnosticTraceInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlatformDiagnosticTraceInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PlatformDiagnosticTraceRuntimeInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlatformDiagnosticTraceRuntimeInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PlatformDiagnosticTraceRuntimeInfo {
    pub fn RuntimeFileTime(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RuntimeFileTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EtwRuntimeFileTime(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EtwRuntimeFileTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlatformDiagnosticTraceRuntimeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlatformDiagnosticTraceRuntimeInfo>();
}
unsafe impl windows_core::Interface for PlatformDiagnosticTraceRuntimeInfo {
    type Vtable = <IPlatformDiagnosticTraceRuntimeInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlatformDiagnosticTraceRuntimeInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlatformDiagnosticTraceRuntimeInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDiagnosticActionState(pub i32);
impl PlatformDiagnosticActionState {
    pub const Success: Self = Self(0i32);
    pub const FreeNetworkNotAvailable: Self = Self(1i32);
    pub const ACPowerNotAvailable: Self = Self(2i32);
}
impl windows_core::TypeKind for PlatformDiagnosticActionState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDiagnosticActionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActionState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: Self = Self(0i32);
    pub const OnFailure: Self = Self(1i32);
}
impl windows_core::TypeKind for PlatformDiagnosticEscalationType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDiagnosticEscalationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEscalationType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: Self = Self(1u32);
    pub const CostDeferred: Self = Self(2u32);
    pub const Realtime: Self = Self(4u32);
}
impl windows_core::TypeKind for PlatformDiagnosticEventBufferLatencies {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDiagnosticEventBufferLatencies {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEventBufferLatencies;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: Self = Self(0i32);
    pub const UserElevated: Self = Self(1i32);
}
impl windows_core::TypeKind for PlatformDiagnosticTracePriority {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDiagnosticTracePriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTracePriority;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Throttled: Self = Self(2i32);
}
impl windows_core::TypeKind for PlatformDiagnosticTraceSlotState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDiagnosticTraceSlotState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: Self = Self(0i32);
    pub const AlwaysOn: Self = Self(1i32);
    pub const Mini: Self = Self(2i32);
}
impl windows_core::TypeKind for PlatformDiagnosticTraceSlotType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDiagnosticTraceSlotType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotType;i4)");
}

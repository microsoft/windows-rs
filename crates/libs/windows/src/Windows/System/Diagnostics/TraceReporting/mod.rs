#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformDiagnosticActionsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticActionsStatics {
    type Vtable = IPlatformDiagnosticActionsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1145cfa_9292_4267_890a_9ea3ed072312);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticActionsStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsScenarioEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenarioid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryEscalateScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenarioid: ::windows::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryEscalateScenario: usize,
    pub DownloadLatestSettingsForNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partner: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, feature: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActiveScenarioList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActiveScenarioList: usize,
    pub ForceUpload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::core::HRESULT,
    pub IsTraceRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slottype: PlatformDiagnosticTraceSlotType, scenarioid: ::windows::core::GUID, traceprofilehash: u64, result__: *mut PlatformDiagnosticTraceSlotState) -> ::windows::core::HRESULT,
    pub GetActiveTraceRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slottype: PlatformDiagnosticTraceSlotType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKnownTraceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slottype: PlatformDiagnosticTraceSlotType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKnownTraceList: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticTraceInfo {
    type Vtable = IPlatformDiagnosticTraceInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf870ed97_d597_4bf7_88dc_cf5c7dc2a1d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ScenarioId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ProfileHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IsExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAutoLogger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxTraceDurationFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlatformDiagnosticTracePriority) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceRuntimeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticTraceRuntimeInfo {
    type Vtable = IPlatformDiagnosticTraceRuntimeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d4d5e2d_01d8_4768_8554_1eb1ca610986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceRuntimeInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RuntimeFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub EtwRuntimeFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformDiagnosticActionState(pub i32);
impl PlatformDiagnosticActionState {
    pub const Success: Self = Self(0i32);
    pub const FreeNetworkNotAvailable: Self = Self(1i32);
    pub const ACPowerNotAvailable: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformDiagnosticActionState {}
impl ::core::clone::Clone for PlatformDiagnosticActionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDiagnosticActionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticActionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDiagnosticActionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticActionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticActionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
pub struct PlatformDiagnosticActions;
impl PlatformDiagnosticActions {
    pub fn IsScenarioEnabled(scenarioid: ::windows::core::GUID) -> ::windows::core::Result<bool> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsScenarioEnabled)(::windows::core::Interface::as_raw(this), scenarioid, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryEscalateScenario<'a, P0, E0>(scenarioid: ::windows::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: &::windows::core::HSTRING, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryEscalateScenario)(::windows::core::Interface::as_raw(this), scenarioid, escalationtype, ::core::mem::transmute_copy(outputdirectory), timestampoutputdirectory, forceescalationupload, triggers.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn DownloadLatestSettingsForNamespace(partner: &::windows::core::HSTRING, feature: &::windows::core::HSTRING, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DownloadLatestSettingsForNamespace)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(partner), ::core::mem::transmute_copy(feature), isscenarionamespace, downloadovercostednetwork, downloadoverbattery, result__.as_mut_ptr()).from_abi::<PlatformDiagnosticActionState>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActiveScenarioList() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::GUID>> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetActiveScenarioList)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>(result__)
        })
    }
    pub fn ForceUpload(latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ForceUpload)(::windows::core::Interface::as_raw(this), latency, uploadovercostednetwork, uploadoverbattery, result__.as_mut_ptr()).from_abi::<PlatformDiagnosticActionState>(result__)
        })
    }
    pub fn IsTraceRunning(slottype: PlatformDiagnosticTraceSlotType, scenarioid: ::windows::core::GUID, traceprofilehash: u64) -> ::windows::core::Result<PlatformDiagnosticTraceSlotState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsTraceRunning)(::windows::core::Interface::as_raw(this), slottype, scenarioid, traceprofilehash, result__.as_mut_ptr()).from_abi::<PlatformDiagnosticTraceSlotState>(result__)
        })
    }
    pub fn GetActiveTraceRuntime(slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<PlatformDiagnosticTraceRuntimeInfo> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetActiveTraceRuntime)(::windows::core::Interface::as_raw(this), slottype, result__.as_mut_ptr()).from_abi::<PlatformDiagnosticTraceRuntimeInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetKnownTraceList(slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetKnownTraceList)(::windows::core::Interface::as_raw(this), slottype, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlatformDiagnosticActionsStatics<R, F: FnOnce(&IPlatformDiagnosticActionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlatformDiagnosticActions, IPlatformDiagnosticActionsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PlatformDiagnosticActions {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions";
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: Self = Self(0i32);
    pub const OnFailure: Self = Self(1i32);
}
impl ::core::marker::Copy for PlatformDiagnosticEscalationType {}
impl ::core::clone::Clone for PlatformDiagnosticEscalationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDiagnosticEscalationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticEscalationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDiagnosticEscalationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticEscalationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticEscalationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEscalationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: Self = Self(1u32);
    pub const CostDeferred: Self = Self(2u32);
    pub const Realtime: Self = Self(4u32);
}
impl ::core::marker::Copy for PlatformDiagnosticEventBufferLatencies {}
impl ::core::clone::Clone for PlatformDiagnosticEventBufferLatencies {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDiagnosticEventBufferLatencies {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticEventBufferLatencies {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDiagnosticEventBufferLatencies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticEventBufferLatencies").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PlatformDiagnosticEventBufferLatencies {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PlatformDiagnosticEventBufferLatencies {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticEventBufferLatencies {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEventBufferLatencies;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceInfo(::windows::core::IUnknown);
impl PlatformDiagnosticTraceInfo {
    pub fn ScenarioId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ScenarioId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ProfileHash(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProfileHash)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn IsExclusive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsExclusive)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAutoLogger(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsAutoLogger)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxTraceDurationFileTime(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxTraceDurationFileTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows::core::Result<PlatformDiagnosticTracePriority> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Priority)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlatformDiagnosticTracePriority>(result__)
        }
    }
}
impl ::core::clone::Clone for PlatformDiagnosticTraceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlatformDiagnosticTraceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlatformDiagnosticTraceInfo {}
impl ::core::fmt::Debug for PlatformDiagnosticTraceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticTraceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo;{f870ed97-d597-4bf7-88dc-cf5c7dc2a1d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlatformDiagnosticTraceInfo {
    type Vtable = IPlatformDiagnosticTraceInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPlatformDiagnosticTraceInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlatformDiagnosticTraceInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo";
}
impl ::core::convert::From<PlatformDiagnosticTraceInfo> for ::windows::core::IUnknown {
    fn from(value: PlatformDiagnosticTraceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceInfo> for ::windows::core::IUnknown {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceInfo> for &::windows::core::IUnknown {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PlatformDiagnosticTraceInfo> for ::windows::core::IInspectable {
    fn from(value: PlatformDiagnosticTraceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceInfo> for ::windows::core::IInspectable {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceInfo> for &::windows::core::IInspectable {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PlatformDiagnosticTraceInfo {}
unsafe impl ::core::marker::Sync for PlatformDiagnosticTraceInfo {}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: Self = Self(0i32);
    pub const UserElevated: Self = Self(1i32);
}
impl ::core::marker::Copy for PlatformDiagnosticTracePriority {}
impl ::core::clone::Clone for PlatformDiagnosticTracePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDiagnosticTracePriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticTracePriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDiagnosticTracePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticTracePriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTracePriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTracePriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceRuntimeInfo(::windows::core::IUnknown);
impl PlatformDiagnosticTraceRuntimeInfo {
    pub fn RuntimeFileTime(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RuntimeFileTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn EtwRuntimeFileTime(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EtwRuntimeFileTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
}
impl ::core::clone::Clone for PlatformDiagnosticTraceRuntimeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlatformDiagnosticTraceRuntimeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlatformDiagnosticTraceRuntimeInfo {}
impl ::core::fmt::Debug for PlatformDiagnosticTraceRuntimeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticTraceRuntimeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceRuntimeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo;{3d4d5e2d-01d8-4768-8554-1eb1ca610986})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlatformDiagnosticTraceRuntimeInfo {
    type Vtable = IPlatformDiagnosticTraceRuntimeInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPlatformDiagnosticTraceRuntimeInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlatformDiagnosticTraceRuntimeInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo";
}
impl ::core::convert::From<PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IUnknown {
    fn from(value: PlatformDiagnosticTraceRuntimeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IUnknown {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for &::windows::core::IUnknown {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IInspectable {
    fn from(value: PlatformDiagnosticTraceRuntimeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IInspectable {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for &::windows::core::IInspectable {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PlatformDiagnosticTraceRuntimeInfo {}
unsafe impl ::core::marker::Sync for PlatformDiagnosticTraceRuntimeInfo {}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Throttled: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformDiagnosticTraceSlotState {}
impl ::core::clone::Clone for PlatformDiagnosticTraceSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDiagnosticTraceSlotState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticTraceSlotState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDiagnosticTraceSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticTraceSlotState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceSlotState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: Self = Self(0i32);
    pub const AlwaysOn: Self = Self(1i32);
    pub const Mini: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformDiagnosticTraceSlotType {}
impl ::core::clone::Clone for PlatformDiagnosticTraceSlotType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDiagnosticTraceSlotType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticTraceSlotType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDiagnosticTraceSlotType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDiagnosticTraceSlotType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceSlotType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

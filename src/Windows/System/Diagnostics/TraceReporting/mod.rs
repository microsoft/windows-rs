#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformDiagnosticActionsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticActionsStatics {
    type Vtable = IPlatformDiagnosticActionsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1145cfa_9292_4267_890a_9ea3ed072312);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticActionsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scenarioid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scenarioid: ::windows::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partner: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, feature: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, slottype: PlatformDiagnosticTraceSlotType, scenarioid: ::windows::core::GUID, traceprofilehash: u64, result__: *mut PlatformDiagnosticTraceSlotState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, slottype: PlatformDiagnosticTraceSlotType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, slottype: PlatformDiagnosticTraceSlotType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticTraceInfo {
    type Vtable = IPlatformDiagnosticTraceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf870ed97_d597_4bf7_88dc_cf5c7dc2a1d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PlatformDiagnosticTracePriority) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceRuntimeInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticTraceRuntimeInfo {
    type Vtable = IPlatformDiagnosticTraceRuntimeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d4d5e2d_01d8_4768_8554_1eb1ca610986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceRuntimeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticActionState(pub i32);
impl PlatformDiagnosticActionState {
    pub const Success: PlatformDiagnosticActionState = PlatformDiagnosticActionState(0i32);
    pub const FreeNetworkNotAvailable: PlatformDiagnosticActionState = PlatformDiagnosticActionState(1i32);
    pub const ACPowerNotAvailable: PlatformDiagnosticActionState = PlatformDiagnosticActionState(2i32);
}
impl ::core::convert::From<i32> for PlatformDiagnosticActionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticActionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticActionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActionState;i4)");
}
impl ::windows::core::DefaultType for PlatformDiagnosticActionState {
    type DefaultType = Self;
}
pub struct PlatformDiagnosticActions {}
impl PlatformDiagnosticActions {
    pub fn IsScenarioEnabled<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(scenarioid: Param0) -> ::windows::core::Result<bool> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), scenarioid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryEscalateScenario<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(scenarioid: Param0, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: Param2, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: Param5) -> ::windows::core::Result<bool> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), scenarioid.into_param().abi(), escalationtype, outputdirectory.into_param().abi(), timestampoutputdirectory, forceescalationupload, triggers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn DownloadLatestSettingsForNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(partner: Param0, feature: Param1, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: PlatformDiagnosticActionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), partner.into_param().abi(), feature.into_param().abi(), isscenarionamespace, downloadovercostednetwork, downloadoverbattery, &mut result__).from_abi::<PlatformDiagnosticActionState>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActiveScenarioList() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::GUID>> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>(result__)
        })
    }
    pub fn ForceUpload(latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: PlatformDiagnosticActionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), latency, uploadovercostednetwork, uploadoverbattery, &mut result__).from_abi::<PlatformDiagnosticActionState>(result__)
        })
    }
    pub fn IsTraceRunning<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(slottype: PlatformDiagnosticTraceSlotType, scenarioid: Param1, traceprofilehash: u64) -> ::windows::core::Result<PlatformDiagnosticTraceSlotState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: PlatformDiagnosticTraceSlotState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), slottype, scenarioid.into_param().abi(), traceprofilehash, &mut result__).from_abi::<PlatformDiagnosticTraceSlotState>(result__)
        })
    }
    pub fn GetActiveTraceRuntime(slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<PlatformDiagnosticTraceRuntimeInfo> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), slottype, &mut result__).from_abi::<PlatformDiagnosticTraceRuntimeInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetKnownTraceList(slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), slottype, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>>(result__)
        })
    }
    pub fn IPlatformDiagnosticActionsStatics<R, F: FnOnce(&IPlatformDiagnosticActionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlatformDiagnosticActions, IPlatformDiagnosticActionsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlatformDiagnosticActions {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: PlatformDiagnosticEscalationType = PlatformDiagnosticEscalationType(0i32);
    pub const OnFailure: PlatformDiagnosticEscalationType = PlatformDiagnosticEscalationType(1i32);
}
impl ::core::convert::From<i32> for PlatformDiagnosticEscalationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticEscalationType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticEscalationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEscalationType;i4)");
}
impl ::windows::core::DefaultType for PlatformDiagnosticEscalationType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(1u32);
    pub const CostDeferred: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(2u32);
    pub const Realtime: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(4u32);
}
impl ::core::convert::From<u32> for PlatformDiagnosticEventBufferLatencies {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticEventBufferLatencies {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticEventBufferLatencies {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEventBufferLatencies;u4)");
}
impl ::windows::core::DefaultType for PlatformDiagnosticEventBufferLatencies {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PlatformDiagnosticEventBufferLatencies {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PlatformDiagnosticEventBufferLatencies {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlatformDiagnosticTraceInfo(pub ::windows::core::IInspectable);
impl PlatformDiagnosticTraceInfo {
    pub fn ScenarioId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ProfileHash(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn IsExclusive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsAutoLogger(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MaxTraceDurationFileTime(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows::core::Result<PlatformDiagnosticTracePriority> {
        let this = self;
        unsafe {
            let mut result__: PlatformDiagnosticTracePriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlatformDiagnosticTracePriority>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo;{f870ed97-d597-4bf7-88dc-cf5c7dc2a1d2})");
}
unsafe impl ::windows::core::Interface for PlatformDiagnosticTraceInfo {
    type Vtable = IPlatformDiagnosticTraceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf870ed97_d597_4bf7_88dc_cf5c7dc2a1d2);
}
impl ::windows::core::RuntimeName for PlatformDiagnosticTraceInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo";
}
impl ::core::convert::From<PlatformDiagnosticTraceInfo> for ::windows::core::IUnknown {
    fn from(value: PlatformDiagnosticTraceInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceInfo> for ::windows::core::IUnknown {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlatformDiagnosticTraceInfo> for ::windows::core::IInspectable {
    fn from(value: PlatformDiagnosticTraceInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceInfo> for ::windows::core::IInspectable {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlatformDiagnosticTraceInfo {}
unsafe impl ::core::marker::Sync for PlatformDiagnosticTraceInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: PlatformDiagnosticTracePriority = PlatformDiagnosticTracePriority(0i32);
    pub const UserElevated: PlatformDiagnosticTracePriority = PlatformDiagnosticTracePriority(1i32);
}
impl ::core::convert::From<i32> for PlatformDiagnosticTracePriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticTracePriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTracePriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTracePriority;i4)");
}
impl ::windows::core::DefaultType for PlatformDiagnosticTracePriority {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlatformDiagnosticTraceRuntimeInfo(pub ::windows::core::IInspectable);
impl PlatformDiagnosticTraceRuntimeInfo {
    pub fn RuntimeFileTime(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn EtwRuntimeFileTime(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceRuntimeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo;{3d4d5e2d-01d8-4768-8554-1eb1ca610986})");
}
unsafe impl ::windows::core::Interface for PlatformDiagnosticTraceRuntimeInfo {
    type Vtable = IPlatformDiagnosticTraceRuntimeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d4d5e2d_01d8_4768_8554_1eb1ca610986);
}
impl ::windows::core::RuntimeName for PlatformDiagnosticTraceRuntimeInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo";
}
impl ::core::convert::From<PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IUnknown {
    fn from(value: PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IUnknown {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IInspectable {
    fn from(value: PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for ::windows::core::IInspectable {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlatformDiagnosticTraceRuntimeInfo {}
unsafe impl ::core::marker::Sync for PlatformDiagnosticTraceRuntimeInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(0i32);
    pub const Running: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(1i32);
    pub const Throttled: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(2i32);
}
impl ::core::convert::From<i32> for PlatformDiagnosticTraceSlotState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticTraceSlotState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceSlotState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotState;i4)");
}
impl ::windows::core::DefaultType for PlatformDiagnosticTraceSlotState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(0i32);
    pub const AlwaysOn: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(1i32);
    pub const Mini: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(2i32);
}
impl ::core::convert::From<i32> for PlatformDiagnosticTraceSlotType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformDiagnosticTraceSlotType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformDiagnosticTraceSlotType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotType;i4)");
}
impl ::windows::core::DefaultType for PlatformDiagnosticTraceSlotType {
    type DefaultType = Self;
}

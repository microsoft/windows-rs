#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformDiagnosticActionsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformDiagnosticActionsStatics {
    type Vtable = IPlatformDiagnosticActionsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3239337210, 37522, 16999, [137, 10, 158, 163, 237, 7, 35, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticActionsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scenarioid: ::windows::runtime::GUID, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scenarioid: ::windows::runtime::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partner: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, feature: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, slottype: PlatformDiagnosticTraceSlotType, scenarioid: ::windows::runtime::GUID, traceprofilehash: u64, result__: *mut PlatformDiagnosticTraceSlotState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, slottype: PlatformDiagnosticTraceSlotType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, slottype: PlatformDiagnosticTraceSlotType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformDiagnosticTraceInfo {
    type Vtable = IPlatformDiagnosticTraceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4168150423, 54679, 19447, [136, 220, 207, 92, 125, 194, 161, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlatformDiagnosticTracePriority) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceRuntimeInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformDiagnosticTraceRuntimeInfo {
    type Vtable = IPlatformDiagnosticTraceRuntimeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1028480557, 472, 18280, [133, 84, 30, 177, 202, 97, 9, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticTraceRuntimeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticActionState(pub i32);
impl PlatformDiagnosticActionState {
    pub const Success: PlatformDiagnosticActionState = PlatformDiagnosticActionState(0i32);
    pub const FreeNetworkNotAvailable: PlatformDiagnosticActionState = PlatformDiagnosticActionState(1i32);
    pub const ACPowerNotAvailable: PlatformDiagnosticActionState = PlatformDiagnosticActionState(2i32);
}
impl ::std::convert::From<i32> for PlatformDiagnosticActionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDiagnosticActionState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticActionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActionState;i4)");
}
impl ::windows::runtime::DefaultType for PlatformDiagnosticActionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
pub struct PlatformDiagnosticActions {}
impl PlatformDiagnosticActions {
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn IsScenarioEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(scenarioid: Param0) -> ::windows::runtime::Result<bool> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), scenarioid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`, `Foundation_Collections`*"]
    pub fn TryEscalateScenario<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(
        scenarioid: Param0,
        escalationtype: PlatformDiagnosticEscalationType,
        outputdirectory: Param2,
        timestampoutputdirectory: bool,
        forceescalationupload: bool,
        triggers: Param5,
    ) -> ::windows::runtime::Result<bool> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), scenarioid.into_param().abi(), escalationtype, outputdirectory.into_param().abi(), timestampoutputdirectory, forceescalationupload, triggers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn DownloadLatestSettingsForNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(partner: Param0, feature: Param1, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool) -> ::windows::runtime::Result<PlatformDiagnosticActionState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: PlatformDiagnosticActionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), partner.into_param().abi(), feature.into_param().abi(), isscenarionamespace, downloadovercostednetwork, downloadoverbattery, &mut result__).from_abi::<PlatformDiagnosticActionState>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`, `Foundation_Collections`*"]
    pub fn GetActiveScenarioList() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::GUID>> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::GUID>>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn ForceUpload(latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool) -> ::windows::runtime::Result<PlatformDiagnosticActionState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: PlatformDiagnosticActionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), latency, uploadovercostednetwork, uploadoverbattery, &mut result__).from_abi::<PlatformDiagnosticActionState>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn IsTraceRunning<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(slottype: PlatformDiagnosticTraceSlotType, scenarioid: Param1, traceprofilehash: u64) -> ::windows::runtime::Result<PlatformDiagnosticTraceSlotState> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: PlatformDiagnosticTraceSlotState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), slottype, scenarioid.into_param().abi(), traceprofilehash, &mut result__).from_abi::<PlatformDiagnosticTraceSlotState>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn GetActiveTraceRuntime(slottype: PlatformDiagnosticTraceSlotType) -> ::windows::runtime::Result<PlatformDiagnosticTraceRuntimeInfo> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), slottype, &mut result__).from_abi::<PlatformDiagnosticTraceRuntimeInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`, `Foundation_Collections`*"]
    pub fn GetKnownTraceList(slottype: PlatformDiagnosticTraceSlotType) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>> {
        Self::IPlatformDiagnosticActionsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), slottype, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>>(result__)
        })
    }
    pub fn IPlatformDiagnosticActionsStatics<R, F: FnOnce(&IPlatformDiagnosticActionsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlatformDiagnosticActions, IPlatformDiagnosticActionsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PlatformDiagnosticActions {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions";
}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: PlatformDiagnosticEscalationType = PlatformDiagnosticEscalationType(0i32);
    pub const OnFailure: PlatformDiagnosticEscalationType = PlatformDiagnosticEscalationType(1i32);
}
impl ::std::convert::From<i32> for PlatformDiagnosticEscalationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDiagnosticEscalationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticEscalationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEscalationType;i4)");
}
impl ::windows::runtime::DefaultType for PlatformDiagnosticEscalationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(1u32);
    pub const CostDeferred: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(2u32);
    pub const Realtime: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(4u32);
}
impl ::std::convert::From<u32> for PlatformDiagnosticEventBufferLatencies {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDiagnosticEventBufferLatencies {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticEventBufferLatencies {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEventBufferLatencies;u4)");
}
impl ::windows::runtime::DefaultType for PlatformDiagnosticEventBufferLatencies {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PlatformDiagnosticEventBufferLatencies {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PlatformDiagnosticEventBufferLatencies {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PlatformDiagnosticEventBufferLatencies {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlatformDiagnosticTraceInfo(pub ::windows::runtime::IInspectable);
impl PlatformDiagnosticTraceInfo {
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn ScenarioId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn ProfileHash(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn IsExclusive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn IsAutoLogger(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn MaxTraceDurationFileTime(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<PlatformDiagnosticTracePriority> {
        let this = self;
        unsafe {
            let mut result__: PlatformDiagnosticTracePriority = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlatformDiagnosticTracePriority>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticTraceInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo;{f870ed97-d597-4bf7-88dc-cf5c7dc2a1d2})");
}
unsafe impl ::windows::runtime::Interface for PlatformDiagnosticTraceInfo {
    type Vtable = IPlatformDiagnosticTraceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4168150423, 54679, 19447, [136, 220, 207, 92, 125, 194, 161, 210]);
}
impl ::windows::runtime::RuntimeName for PlatformDiagnosticTraceInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo";
}
impl ::std::convert::From<PlatformDiagnosticTraceInfo> for ::windows::runtime::IUnknown {
    fn from(value: PlatformDiagnosticTraceInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PlatformDiagnosticTraceInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PlatformDiagnosticTraceInfo> for ::windows::runtime::IInspectable {
    fn from(value: PlatformDiagnosticTraceInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlatformDiagnosticTraceInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PlatformDiagnosticTraceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlatformDiagnosticTraceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PlatformDiagnosticTraceInfo {}
unsafe impl ::std::marker::Sync for PlatformDiagnosticTraceInfo {}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: PlatformDiagnosticTracePriority = PlatformDiagnosticTracePriority(0i32);
    pub const UserElevated: PlatformDiagnosticTracePriority = PlatformDiagnosticTracePriority(1i32);
}
impl ::std::convert::From<i32> for PlatformDiagnosticTracePriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDiagnosticTracePriority {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticTracePriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTracePriority;i4)");
}
impl ::windows::runtime::DefaultType for PlatformDiagnosticTracePriority {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlatformDiagnosticTraceRuntimeInfo(pub ::windows::runtime::IInspectable);
impl PlatformDiagnosticTraceRuntimeInfo {
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn RuntimeFileTime(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
    pub fn EtwRuntimeFileTime(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticTraceRuntimeInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo;{3d4d5e2d-01d8-4768-8554-1eb1ca610986})");
}
unsafe impl ::windows::runtime::Interface for PlatformDiagnosticTraceRuntimeInfo {
    type Vtable = IPlatformDiagnosticTraceRuntimeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1028480557, 472, 18280, [133, 84, 30, 177, 202, 97, 9, 134]);
}
impl ::windows::runtime::RuntimeName for PlatformDiagnosticTraceRuntimeInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo";
}
impl ::std::convert::From<PlatformDiagnosticTraceRuntimeInfo> for ::windows::runtime::IUnknown {
    fn from(value: PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PlatformDiagnosticTraceRuntimeInfo> for ::windows::runtime::IInspectable {
    fn from(value: PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlatformDiagnosticTraceRuntimeInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PlatformDiagnosticTraceRuntimeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlatformDiagnosticTraceRuntimeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PlatformDiagnosticTraceRuntimeInfo {}
unsafe impl ::std::marker::Sync for PlatformDiagnosticTraceRuntimeInfo {}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(0i32);
    pub const Running: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(1i32);
    pub const Throttled: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(2i32);
}
impl ::std::convert::From<i32> for PlatformDiagnosticTraceSlotState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDiagnosticTraceSlotState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticTraceSlotState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotState;i4)");
}
impl ::windows::runtime::DefaultType for PlatformDiagnosticTraceSlotState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Diagnostics_TraceReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(0i32);
    pub const AlwaysOn: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(1i32);
    pub const Mini: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(2i32);
}
impl ::std::convert::From<i32> for PlatformDiagnosticTraceSlotType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDiagnosticTraceSlotType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDiagnosticTraceSlotType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotType;i4)");
}
impl ::windows::runtime::DefaultType for PlatformDiagnosticTraceSlotType {
    type DefaultType = Self;
}

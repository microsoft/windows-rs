#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DiagnosticActionResult(pub ::windows::core::IInspectable);
impl DiagnosticActionResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Results(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DiagnosticActionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DiagnosticActionResult;{c265a296-e73b-4097-b28f-3442f03dd831})");
}
unsafe impl ::windows::core::Interface for DiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc265a296_e73b_4097_b28f_3442f03dd831);
}
impl ::windows::core::RuntimeName for DiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticActionResult";
}
impl ::core::convert::From<DiagnosticActionResult> for ::windows::core::IUnknown {
    fn from(value: DiagnosticActionResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DiagnosticActionResult> for ::windows::core::IUnknown {
    fn from(value: &DiagnosticActionResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiagnosticActionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DiagnosticActionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DiagnosticActionResult> for ::windows::core::IInspectable {
    fn from(value: DiagnosticActionResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DiagnosticActionResult> for ::windows::core::IInspectable {
    fn from(value: &DiagnosticActionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiagnosticActionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DiagnosticActionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DiagnosticActionResult {}
unsafe impl ::core::marker::Sync for DiagnosticActionResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DiagnosticActionState(pub i32);
impl DiagnosticActionState {
    pub const Initializing: DiagnosticActionState = DiagnosticActionState(0i32);
    pub const Downloading: DiagnosticActionState = DiagnosticActionState(1i32);
    pub const VerifyingTrust: DiagnosticActionState = DiagnosticActionState(2i32);
    pub const Detecting: DiagnosticActionState = DiagnosticActionState(3i32);
    pub const Resolving: DiagnosticActionState = DiagnosticActionState(4i32);
    pub const VerifyingResolution: DiagnosticActionState = DiagnosticActionState(5i32);
    pub const Executing: DiagnosticActionState = DiagnosticActionState(6i32);
}
impl ::core::convert::From<i32> for DiagnosticActionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DiagnosticActionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DiagnosticActionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DiagnosticActionState;i4)");
}
impl ::windows::core::DefaultType for DiagnosticActionState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DiagnosticInvoker(pub ::windows::core::IInspectable);
impl DiagnosticInvoker {
    #[cfg(all(feature = "Data_Json", feature = "Foundation"))]
    pub fn RunDiagnosticActionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Json::JsonObject>>(&self, context: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DiagnosticInvoker>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::User>>(user: Param0) -> ::windows::core::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<DiagnosticInvoker>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RunDiagnosticActionFromStringAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = &::windows::core::Interface::cast::<IDiagnosticInvoker2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>(result__)
        }
    }
    pub fn IDiagnosticInvokerStatics<R, F: FnOnce(&IDiagnosticInvokerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiagnosticInvoker, IDiagnosticInvokerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DiagnosticInvoker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DiagnosticInvoker;{187b270a-02e3-4f86-84fc-fdd892b5940f})");
}
unsafe impl ::windows::core::Interface for DiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187b270a_02e3_4f86_84fc_fdd892b5940f);
}
impl ::windows::core::RuntimeName for DiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticInvoker";
}
impl ::core::convert::From<DiagnosticInvoker> for ::windows::core::IUnknown {
    fn from(value: DiagnosticInvoker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DiagnosticInvoker> for ::windows::core::IUnknown {
    fn from(value: &DiagnosticInvoker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiagnosticInvoker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DiagnosticInvoker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DiagnosticInvoker> for ::windows::core::IInspectable {
    fn from(value: DiagnosticInvoker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DiagnosticInvoker> for ::windows::core::IInspectable {
    fn from(value: &DiagnosticInvoker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiagnosticInvoker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DiagnosticInvoker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DiagnosticInvoker {}
unsafe impl ::core::marker::Sync for DiagnosticInvoker {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiagnosticActionResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc265a296_e73b_4097_b28f_3442f03dd831);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticActionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiagnosticInvoker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187b270a_02e3_4f86_84fc_fdd892b5940f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvoker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Data_Json", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Json", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiagnosticInvoker2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiagnosticInvoker2 {
    type Vtable = IDiagnosticInvoker2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bf945c_155a_4b52_a8ec_070c44f95000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvoker2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiagnosticInvokerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiagnosticInvokerStatics {
    type Vtable = IDiagnosticInvokerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cfad8de_f15c_4554_a813_c113c3881b09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvokerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessCpuUsage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessCpuUsage {
    type Vtable = IProcessCpuUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bbb2472_c8bf_423a_a810_b559ae4354e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessCpuUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessCpuUsageReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6d9cac_3987_4e2f_a119_6b5fa214f1b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessCpuUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe830b04b_300e_4ee6_a0ab_5b5f5231b434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessDiagnosticInfo2 {
    type Vtable = IProcessDiagnosticInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9558cb1a_3d0b_49ec_ab70_4f7a112805de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessDiagnosticInfoStatics {
    type Vtable = IProcessDiagnosticInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f41b260_b49f_428c_aa0e_84744f49ca95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessDiagnosticInfoStatics2 {
    type Vtable = IProcessDiagnosticInfoStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a869897_9899_4a44_a29b_091663be09b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessDiskUsage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessDiskUsage {
    type Vtable = IProcessDiskUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad78bfd_7e51_4e53_bfaa_5a6ee1aabbf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiskUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessDiskUsageReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401627fd_535d_4c1f_81b8_da54e1be635e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiskUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessMemoryUsage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf50b229b_827c_42b7_b07c_0e32627e6b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessMemoryUsageReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2c77cba_1951_4685_8532_7e749ecf8eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemCpuUsage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemCpuUsage {
    type Vtable = ISystemCpuUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6037b3ac_02d6_4234_8362_7fe3adc81f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCpuUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemCpuUsageReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c26d0b2_9483_4f62_ab57_82b29d9719b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCpuUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa290fe05_dff3_407f_9a1b_0b2b317ca800);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemDiagnosticInfoStatics {
    type Vtable = ISystemDiagnosticInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd404ac21_fc7d_45f0_9a3f_39203aed9f7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemDiagnosticInfoStatics2 {
    type Vtable = ISystemDiagnosticInfoStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ded189_6af9_4da9_a422_15f73255b3eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ProcessorArchitecture) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMemoryUsage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17ffc595_1702_49cf_aa27_2f0a32591404);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMemoryUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMemoryUsageReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38663c87_2a9f_403a_bd19_2cf3e8169500);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMemoryUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessCpuUsage(pub ::windows::core::IInspectable);
impl ProcessCpuUsage {
    pub fn GetReport(&self) -> ::windows::core::Result<ProcessCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessCpuUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessCpuUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessCpuUsage;{0bbb2472-c8bf-423a-a810-b559ae4354e2})");
}
unsafe impl ::windows::core::Interface for ProcessCpuUsage {
    type Vtable = IProcessCpuUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bbb2472_c8bf_423a_a810_b559ae4354e2);
}
impl ::windows::core::RuntimeName for ProcessCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsage";
}
impl ::core::convert::From<ProcessCpuUsage> for ::windows::core::IUnknown {
    fn from(value: ProcessCpuUsage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessCpuUsage> for ::windows::core::IUnknown {
    fn from(value: &ProcessCpuUsage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessCpuUsage> for ::windows::core::IInspectable {
    fn from(value: ProcessCpuUsage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessCpuUsage> for ::windows::core::IInspectable {
    fn from(value: &ProcessCpuUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessCpuUsage {}
unsafe impl ::core::marker::Sync for ProcessCpuUsage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessCpuUsageReport(pub ::windows::core::IInspectable);
impl ProcessCpuUsageReport {
    #[cfg(feature = "Foundation")]
    pub fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessCpuUsageReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessCpuUsageReport;{8a6d9cac-3987-4e2f-a119-6b5fa214f1b4})");
}
unsafe impl ::windows::core::Interface for ProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6d9cac_3987_4e2f_a119_6b5fa214f1b4);
}
impl ::windows::core::RuntimeName for ProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsageReport";
}
impl ::core::convert::From<ProcessCpuUsageReport> for ::windows::core::IUnknown {
    fn from(value: ProcessCpuUsageReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessCpuUsageReport> for ::windows::core::IUnknown {
    fn from(value: &ProcessCpuUsageReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessCpuUsageReport> for ::windows::core::IInspectable {
    fn from(value: ProcessCpuUsageReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessCpuUsageReport> for ::windows::core::IInspectable {
    fn from(value: &ProcessCpuUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessCpuUsageReport {}
unsafe impl ::core::marker::Sync for ProcessCpuUsageReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessDiagnosticInfo(pub ::windows::core::IInspectable);
impl ProcessDiagnosticInfo {
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<ProcessDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiagnosticInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProcessStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn DiskUsage(&self) -> ::windows::core::Result<ProcessDiskUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiskUsage>(result__)
        }
    }
    pub fn MemoryUsage(&self) -> ::windows::core::Result<ProcessMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryUsage>(result__)
        }
    }
    pub fn CpuUsage(&self) -> ::windows::core::Result<ProcessCpuUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessCpuUsage>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForProcesses() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>>(result__)
        })
    }
    pub fn GetForCurrentProcess() -> ::windows::core::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiagnosticInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppDiagnosticInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>> {
        let this = &::windows::core::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>>(result__)
        }
    }
    pub fn IsPackaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TryGetForProcessId(processid: u32) -> ::windows::core::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), processid, &mut result__).from_abi::<ProcessDiagnosticInfo>(result__)
        })
    }
    pub fn IProcessDiagnosticInfoStatics<R, F: FnOnce(&IProcessDiagnosticInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProcessDiagnosticInfoStatics2<R, F: FnOnce(&IProcessDiagnosticInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessDiagnosticInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiagnosticInfo;{e830b04b-300e-4ee6-a0ab-5b5f5231b434})");
}
unsafe impl ::windows::core::Interface for ProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe830b04b_300e_4ee6_a0ab_5b5f5231b434);
}
impl ::windows::core::RuntimeName for ProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiagnosticInfo";
}
impl ::core::convert::From<ProcessDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: ProcessDiagnosticInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: &ProcessDiagnosticInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: ProcessDiagnosticInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: &ProcessDiagnosticInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessDiagnosticInfo {}
unsafe impl ::core::marker::Sync for ProcessDiagnosticInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessDiskUsage(pub ::windows::core::IInspectable);
impl ProcessDiskUsage {
    pub fn GetReport(&self) -> ::windows::core::Result<ProcessDiskUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiskUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessDiskUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiskUsage;{5ad78bfd-7e51-4e53-bfaa-5a6ee1aabbf8})");
}
unsafe impl ::windows::core::Interface for ProcessDiskUsage {
    type Vtable = IProcessDiskUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad78bfd_7e51_4e53_bfaa_5a6ee1aabbf8);
}
impl ::windows::core::RuntimeName for ProcessDiskUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsage";
}
impl ::core::convert::From<ProcessDiskUsage> for ::windows::core::IUnknown {
    fn from(value: ProcessDiskUsage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessDiskUsage> for ::windows::core::IUnknown {
    fn from(value: &ProcessDiskUsage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessDiskUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessDiskUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessDiskUsage> for ::windows::core::IInspectable {
    fn from(value: ProcessDiskUsage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessDiskUsage> for ::windows::core::IInspectable {
    fn from(value: &ProcessDiskUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessDiskUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessDiskUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessDiskUsage {}
unsafe impl ::core::marker::Sync for ProcessDiskUsage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessDiskUsageReport(pub ::windows::core::IInspectable);
impl ProcessDiskUsageReport {
    pub fn ReadOperationCount(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn WriteOperationCount(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn OtherOperationCount(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn BytesReadCount(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn BytesWrittenCount(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn OtherBytesCount(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessDiskUsageReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiskUsageReport;{401627fd-535d-4c1f-81b8-da54e1be635e})");
}
unsafe impl ::windows::core::Interface for ProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401627fd_535d_4c1f_81b8_da54e1be635e);
}
impl ::windows::core::RuntimeName for ProcessDiskUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsageReport";
}
impl ::core::convert::From<ProcessDiskUsageReport> for ::windows::core::IUnknown {
    fn from(value: ProcessDiskUsageReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessDiskUsageReport> for ::windows::core::IUnknown {
    fn from(value: &ProcessDiskUsageReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessDiskUsageReport> for ::windows::core::IInspectable {
    fn from(value: ProcessDiskUsageReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessDiskUsageReport> for ::windows::core::IInspectable {
    fn from(value: &ProcessDiskUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessDiskUsageReport {}
unsafe impl ::core::marker::Sync for ProcessDiskUsageReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessMemoryUsage(pub ::windows::core::IInspectable);
impl ProcessMemoryUsage {
    pub fn GetReport(&self) -> ::windows::core::Result<ProcessMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessMemoryUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessMemoryUsage;{f50b229b-827c-42b7-b07c-0e32627e6b3e})");
}
unsafe impl ::windows::core::Interface for ProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf50b229b_827c_42b7_b07c_0e32627e6b3e);
}
impl ::windows::core::RuntimeName for ProcessMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsage";
}
impl ::core::convert::From<ProcessMemoryUsage> for ::windows::core::IUnknown {
    fn from(value: ProcessMemoryUsage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessMemoryUsage> for ::windows::core::IUnknown {
    fn from(value: &ProcessMemoryUsage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessMemoryUsage> for ::windows::core::IInspectable {
    fn from(value: ProcessMemoryUsage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessMemoryUsage> for ::windows::core::IInspectable {
    fn from(value: &ProcessMemoryUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryUsage {}
unsafe impl ::core::marker::Sync for ProcessMemoryUsage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessMemoryUsageReport(pub ::windows::core::IInspectable);
impl ProcessMemoryUsageReport {
    pub fn NonPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PageFaultCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PageFileSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PeakNonPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PeakPageFileSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PeakPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PeakVirtualMemorySizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PeakWorkingSetSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PrivatePageCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn VirtualMemorySizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn WorkingSetSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessMemoryUsageReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessMemoryUsageReport;{c2c77cba-1951-4685-8532-7e749ecf8eeb})");
}
unsafe impl ::windows::core::Interface for ProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2c77cba_1951_4685_8532_7e749ecf8eeb);
}
impl ::windows::core::RuntimeName for ProcessMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsageReport";
}
impl ::core::convert::From<ProcessMemoryUsageReport> for ::windows::core::IUnknown {
    fn from(value: ProcessMemoryUsageReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessMemoryUsageReport> for ::windows::core::IUnknown {
    fn from(value: &ProcessMemoryUsageReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessMemoryUsageReport> for ::windows::core::IInspectable {
    fn from(value: ProcessMemoryUsageReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessMemoryUsageReport> for ::windows::core::IInspectable {
    fn from(value: &ProcessMemoryUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryUsageReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryUsageReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemCpuUsage(pub ::windows::core::IInspectable);
impl SystemCpuUsage {
    pub fn GetReport(&self) -> ::windows::core::Result<SystemCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemCpuUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemCpuUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemCpuUsage;{6037b3ac-02d6-4234-8362-7fe3adc81f5f})");
}
unsafe impl ::windows::core::Interface for SystemCpuUsage {
    type Vtable = ISystemCpuUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6037b3ac_02d6_4234_8362_7fe3adc81f5f);
}
impl ::windows::core::RuntimeName for SystemCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsage";
}
impl ::core::convert::From<SystemCpuUsage> for ::windows::core::IUnknown {
    fn from(value: SystemCpuUsage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemCpuUsage> for ::windows::core::IUnknown {
    fn from(value: &SystemCpuUsage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemCpuUsage> for ::windows::core::IInspectable {
    fn from(value: SystemCpuUsage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemCpuUsage> for ::windows::core::IInspectable {
    fn from(value: &SystemCpuUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemCpuUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemCpuUsage {}
unsafe impl ::core::marker::Sync for SystemCpuUsage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemCpuUsageReport(pub ::windows::core::IInspectable);
impl SystemCpuUsageReport {
    #[cfg(feature = "Foundation")]
    pub fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IdleTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemCpuUsageReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemCpuUsageReport;{2c26d0b2-9483-4f62-ab57-82b29d9719b8})");
}
unsafe impl ::windows::core::Interface for SystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c26d0b2_9483_4f62_ab57_82b29d9719b8);
}
impl ::windows::core::RuntimeName for SystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsageReport";
}
impl ::core::convert::From<SystemCpuUsageReport> for ::windows::core::IUnknown {
    fn from(value: SystemCpuUsageReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemCpuUsageReport> for ::windows::core::IUnknown {
    fn from(value: &SystemCpuUsageReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemCpuUsageReport> for ::windows::core::IInspectable {
    fn from(value: SystemCpuUsageReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemCpuUsageReport> for ::windows::core::IInspectable {
    fn from(value: &SystemCpuUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemCpuUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemCpuUsageReport {}
unsafe impl ::core::marker::Sync for SystemCpuUsageReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemDiagnosticInfo(pub ::windows::core::IInspectable);
impl SystemDiagnosticInfo {
    pub fn MemoryUsage(&self) -> ::windows::core::Result<SystemMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMemoryUsage>(result__)
        }
    }
    pub fn CpuUsage(&self) -> ::windows::core::Result<SystemCpuUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemCpuUsage>(result__)
        }
    }
    pub fn GetForCurrentSystem() -> ::windows::core::Result<SystemDiagnosticInfo> {
        Self::ISystemDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemDiagnosticInfo>(result__)
        })
    }
    pub fn IsArchitectureSupported(r#type: super::ProcessorArchitecture) -> ::windows::core::Result<bool> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn PreferredArchitecture() -> ::windows::core::Result<super::ProcessorArchitecture> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: super::ProcessorArchitecture = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ProcessorArchitecture>(result__)
        })
    }
    pub fn ISystemDiagnosticInfoStatics<R, F: FnOnce(&ISystemDiagnosticInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISystemDiagnosticInfoStatics2<R, F: FnOnce(&ISystemDiagnosticInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemDiagnosticInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemDiagnosticInfo;{a290fe05-dff3-407f-9a1b-0b2b317ca800})");
}
unsafe impl ::windows::core::Interface for SystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa290fe05_dff3_407f_9a1b_0b2b317ca800);
}
impl ::windows::core::RuntimeName for SystemDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemDiagnosticInfo";
}
impl ::core::convert::From<SystemDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: SystemDiagnosticInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: &SystemDiagnosticInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: SystemDiagnosticInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: &SystemDiagnosticInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemDiagnosticInfo {}
unsafe impl ::core::marker::Sync for SystemDiagnosticInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMemoryUsage(pub ::windows::core::IInspectable);
impl SystemMemoryUsage {
    pub fn GetReport(&self) -> ::windows::core::Result<SystemMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMemoryUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMemoryUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemMemoryUsage;{17ffc595-1702-49cf-aa27-2f0a32591404})");
}
unsafe impl ::windows::core::Interface for SystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17ffc595_1702_49cf_aa27_2f0a32591404);
}
impl ::windows::core::RuntimeName for SystemMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsage";
}
impl ::core::convert::From<SystemMemoryUsage> for ::windows::core::IUnknown {
    fn from(value: SystemMemoryUsage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMemoryUsage> for ::windows::core::IUnknown {
    fn from(value: &SystemMemoryUsage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMemoryUsage> for ::windows::core::IInspectable {
    fn from(value: SystemMemoryUsage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMemoryUsage> for ::windows::core::IInspectable {
    fn from(value: &SystemMemoryUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMemoryUsage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMemoryUsage {}
unsafe impl ::core::marker::Sync for SystemMemoryUsage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMemoryUsageReport(pub ::windows::core::IInspectable);
impl SystemMemoryUsageReport {
    pub fn TotalPhysicalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn AvailableSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn CommittedSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMemoryUsageReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemMemoryUsageReport;{38663c87-2a9f-403a-bd19-2cf3e8169500})");
}
unsafe impl ::windows::core::Interface for SystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38663c87_2a9f_403a_bd19_2cf3e8169500);
}
impl ::windows::core::RuntimeName for SystemMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsageReport";
}
impl ::core::convert::From<SystemMemoryUsageReport> for ::windows::core::IUnknown {
    fn from(value: SystemMemoryUsageReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMemoryUsageReport> for ::windows::core::IUnknown {
    fn from(value: &SystemMemoryUsageReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMemoryUsageReport> for ::windows::core::IInspectable {
    fn from(value: SystemMemoryUsageReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMemoryUsageReport> for ::windows::core::IInspectable {
    fn from(value: &SystemMemoryUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMemoryUsageReport {}
unsafe impl ::core::marker::Sync for SystemMemoryUsageReport {}

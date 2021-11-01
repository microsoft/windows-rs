#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DiagnosticActionResult(::windows::runtime::IInspectable);
impl DiagnosticActionResult {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation_Collections`*"]
    pub fn Results(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DiagnosticActionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DiagnosticActionResult;{c265a296-e73b-4097-b28f-3442f03dd831})");
}
unsafe impl ::windows::runtime::Interface for DiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3261440662, 59195, 16535, [178, 143, 52, 66, 240, 61, 216, 49]);
}
impl ::windows::runtime::RuntimeName for DiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticActionResult";
}
impl ::std::convert::From<DiagnosticActionResult> for ::windows::runtime::IUnknown {
    fn from(value: DiagnosticActionResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DiagnosticActionResult> for ::windows::runtime::IUnknown {
    fn from(value: &DiagnosticActionResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DiagnosticActionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DiagnosticActionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DiagnosticActionResult> for ::windows::runtime::IInspectable {
    fn from(value: DiagnosticActionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DiagnosticActionResult> for ::windows::runtime::IInspectable {
    fn from(value: &DiagnosticActionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DiagnosticActionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DiagnosticActionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DiagnosticActionResult {}
unsafe impl ::std::marker::Sync for DiagnosticActionResult {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DiagnosticActionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DiagnosticActionState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DiagnosticActionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DiagnosticActionState;i4)");
}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DiagnosticInvoker(::windows::runtime::IInspectable);
impl DiagnosticInvoker {
    #[cfg(all(feature = "Data_Json", feature = "Foundation"))]
    #[doc = "*Required features: `System_Diagnostics`, `Data_Json`, `Foundation`*"]
    pub fn RunDiagnosticActionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Json::JsonObject>>(&self, context: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DiagnosticInvoker>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<DiagnosticInvoker>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn RunDiagnosticActionFromStringAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, context: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = &::windows::runtime::Interface::cast::<IDiagnosticInvoker2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>(result__)
        }
    }
    pub fn IDiagnosticInvokerStatics<R, F: FnOnce(&IDiagnosticInvokerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DiagnosticInvoker, IDiagnosticInvokerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DiagnosticInvoker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DiagnosticInvoker;{187b270a-02e3-4f86-84fc-fdd892b5940f})");
}
unsafe impl ::windows::runtime::Interface for DiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(410724106, 739, 20358, [132, 252, 253, 216, 146, 181, 148, 15]);
}
impl ::windows::runtime::RuntimeName for DiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticInvoker";
}
impl ::std::convert::From<DiagnosticInvoker> for ::windows::runtime::IUnknown {
    fn from(value: DiagnosticInvoker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DiagnosticInvoker> for ::windows::runtime::IUnknown {
    fn from(value: &DiagnosticInvoker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DiagnosticInvoker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DiagnosticInvoker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DiagnosticInvoker> for ::windows::runtime::IInspectable {
    fn from(value: DiagnosticInvoker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DiagnosticInvoker> for ::windows::runtime::IInspectable {
    fn from(value: &DiagnosticInvoker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DiagnosticInvoker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DiagnosticInvoker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DiagnosticInvoker {}
unsafe impl ::std::marker::Sync for DiagnosticInvoker {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDiagnosticActionResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3261440662, 59195, 16535, [178, 143, 52, 66, 240, 61, 216, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticActionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDiagnosticInvoker(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(410724106, 739, 20358, [132, 252, 253, 216, 146, 181, 148, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvoker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Data_Json", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Data_Json", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDiagnosticInvoker2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDiagnosticInvoker2 {
    type Vtable = IDiagnosticInvoker2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820983388, 5466, 19282, [168, 236, 7, 12, 68, 249, 80, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvoker2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDiagnosticInvokerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDiagnosticInvokerStatics {
    type Vtable = IDiagnosticInvokerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1559943390, 61788, 17748, [168, 19, 193, 19, 195, 136, 27, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvokerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessCpuUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessCpuUsage {
    type Vtable = IProcessCpuUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(196813938, 51391, 16954, [168, 16, 181, 89, 174, 67, 84, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessCpuUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessCpuUsageReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2322439340, 14727, 20015, [161, 25, 107, 95, 162, 20, 241, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessCpuUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3895504971, 12302, 20198, [160, 171, 91, 95, 82, 49, 180, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessDiagnosticInfo2 {
    type Vtable = IProcessDiagnosticInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2505624346, 15627, 18924, [171, 112, 79, 122, 17, 40, 5, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessDiagnosticInfoStatics {
    type Vtable = IProcessDiagnosticInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(792834656, 46239, 17036, [170, 14, 132, 116, 79, 73, 202, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessDiagnosticInfoStatics2 {
    type Vtable = IProcessDiagnosticInfoStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1250334871, 39065, 19012, [162, 155, 9, 22, 99, 190, 9, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessDiskUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessDiskUsage {
    type Vtable = IProcessDiskUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1524075517, 32337, 20051, [191, 170, 90, 110, 225, 170, 187, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiskUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessDiskUsageReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1075193853, 21341, 19487, [129, 184, 218, 84, 225, 190, 99, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiskUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessMemoryUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4111147675, 33404, 17079, [176, 124, 14, 50, 98, 126, 107, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProcessMemoryUsageReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3267853498, 6481, 18053, [133, 50, 126, 116, 158, 207, 142, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemCpuUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemCpuUsage {
    type Vtable = ISystemCpuUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1614263212, 726, 16948, [131, 98, 127, 227, 173, 200, 31, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCpuUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemCpuUsageReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(740741298, 38019, 20322, [171, 87, 130, 178, 157, 151, 25, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCpuUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2727411205, 57331, 16511, [154, 27, 11, 43, 49, 124, 168, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemDiagnosticInfoStatics {
    type Vtable = ISystemDiagnosticInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3557076001, 64637, 17904, [154, 63, 57, 32, 58, 237, 159, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemDiagnosticInfoStatics2 {
    type Vtable = ISystemDiagnosticInfoStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2044645769, 27385, 19881, [164, 34, 21, 247, 50, 85, 179, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::ProcessorArchitecture) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemMemoryUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(402638229, 5890, 18895, [170, 39, 47, 10, 50, 89, 20, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMemoryUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISystemMemoryUsageReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(946224263, 10911, 16442, [189, 25, 44, 243, 232, 22, 149, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMemoryUsageReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessCpuUsage(::windows::runtime::IInspectable);
impl ProcessCpuUsage {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetReport(&self) -> ::windows::runtime::Result<ProcessCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessCpuUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessCpuUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessCpuUsage;{0bbb2472-c8bf-423a-a810-b559ae4354e2})");
}
unsafe impl ::windows::runtime::Interface for ProcessCpuUsage {
    type Vtable = IProcessCpuUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(196813938, 51391, 16954, [168, 16, 181, 89, 174, 67, 84, 226]);
}
impl ::windows::runtime::RuntimeName for ProcessCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsage";
}
impl ::std::convert::From<ProcessCpuUsage> for ::windows::runtime::IUnknown {
    fn from(value: ProcessCpuUsage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessCpuUsage> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessCpuUsage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessCpuUsage> for ::windows::runtime::IInspectable {
    fn from(value: ProcessCpuUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessCpuUsage> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessCpuUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessCpuUsage {}
unsafe impl ::std::marker::Sync for ProcessCpuUsage {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessCpuUsageReport(::windows::runtime::IInspectable);
impl ProcessCpuUsageReport {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn KernelTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn UserTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessCpuUsageReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessCpuUsageReport;{8a6d9cac-3987-4e2f-a119-6b5fa214f1b4})");
}
unsafe impl ::windows::runtime::Interface for ProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2322439340, 14727, 20015, [161, 25, 107, 95, 162, 20, 241, 180]);
}
impl ::windows::runtime::RuntimeName for ProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsageReport";
}
impl ::std::convert::From<ProcessCpuUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: ProcessCpuUsageReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessCpuUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessCpuUsageReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessCpuUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: ProcessCpuUsageReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessCpuUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessCpuUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessCpuUsageReport {}
unsafe impl ::std::marker::Sync for ProcessCpuUsageReport {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessDiagnosticInfo(::windows::runtime::IInspectable);
impl ProcessDiagnosticInfo {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn ProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn ExecutableFileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<ProcessDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiagnosticInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn ProcessStartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn DiskUsage(&self) -> ::windows::runtime::Result<ProcessDiskUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiskUsage>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn MemoryUsage(&self) -> ::windows::runtime::Result<ProcessMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryUsage>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn CpuUsage(&self) -> ::windows::runtime::Result<ProcessCpuUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessCpuUsage>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation_Collections`*"]
    pub fn GetForProcesses() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetForCurrentProcess() -> ::windows::runtime::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiagnosticInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation_Collections`*"]
    pub fn GetAppDiagnosticInfos(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>> {
        let this = &::windows::runtime::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn IsPackaged(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn TryGetForProcessId(processid: u32) -> ::windows::runtime::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), processid, &mut result__).from_abi::<ProcessDiagnosticInfo>(result__)
        })
    }
    pub fn IProcessDiagnosticInfoStatics<R, F: FnOnce(&IProcessDiagnosticInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProcessDiagnosticInfoStatics2<R, F: FnOnce(&IProcessDiagnosticInfoStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessDiagnosticInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiagnosticInfo;{e830b04b-300e-4ee6-a0ab-5b5f5231b434})");
}
unsafe impl ::windows::runtime::Interface for ProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3895504971, 12302, 20198, [160, 171, 91, 95, 82, 49, 180, 52]);
}
impl ::windows::runtime::RuntimeName for ProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiagnosticInfo";
}
impl ::std::convert::From<ProcessDiagnosticInfo> for ::windows::runtime::IUnknown {
    fn from(value: ProcessDiagnosticInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessDiagnosticInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessDiagnosticInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessDiagnosticInfo> for ::windows::runtime::IInspectable {
    fn from(value: ProcessDiagnosticInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessDiagnosticInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessDiagnosticInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessDiagnosticInfo {}
unsafe impl ::std::marker::Sync for ProcessDiagnosticInfo {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessDiskUsage(::windows::runtime::IInspectable);
impl ProcessDiskUsage {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetReport(&self) -> ::windows::runtime::Result<ProcessDiskUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessDiskUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessDiskUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiskUsage;{5ad78bfd-7e51-4e53-bfaa-5a6ee1aabbf8})");
}
unsafe impl ::windows::runtime::Interface for ProcessDiskUsage {
    type Vtable = IProcessDiskUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1524075517, 32337, 20051, [191, 170, 90, 110, 225, 170, 187, 248]);
}
impl ::windows::runtime::RuntimeName for ProcessDiskUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsage";
}
impl ::std::convert::From<ProcessDiskUsage> for ::windows::runtime::IUnknown {
    fn from(value: ProcessDiskUsage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessDiskUsage> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessDiskUsage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessDiskUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessDiskUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessDiskUsage> for ::windows::runtime::IInspectable {
    fn from(value: ProcessDiskUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessDiskUsage> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessDiskUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessDiskUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessDiskUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessDiskUsage {}
unsafe impl ::std::marker::Sync for ProcessDiskUsage {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessDiskUsageReport(::windows::runtime::IInspectable);
impl ProcessDiskUsageReport {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn ReadOperationCount(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn WriteOperationCount(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn OtherOperationCount(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn BytesReadCount(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn BytesWrittenCount(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn OtherBytesCount(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessDiskUsageReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiskUsageReport;{401627fd-535d-4c1f-81b8-da54e1be635e})");
}
unsafe impl ::windows::runtime::Interface for ProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1075193853, 21341, 19487, [129, 184, 218, 84, 225, 190, 99, 94]);
}
impl ::windows::runtime::RuntimeName for ProcessDiskUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsageReport";
}
impl ::std::convert::From<ProcessDiskUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: ProcessDiskUsageReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessDiskUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessDiskUsageReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessDiskUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: ProcessDiskUsageReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessDiskUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessDiskUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessDiskUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessDiskUsageReport {}
unsafe impl ::std::marker::Sync for ProcessDiskUsageReport {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessMemoryUsage(::windows::runtime::IInspectable);
impl ProcessMemoryUsage {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetReport(&self) -> ::windows::runtime::Result<ProcessMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessMemoryUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessMemoryUsage;{f50b229b-827c-42b7-b07c-0e32627e6b3e})");
}
unsafe impl ::windows::runtime::Interface for ProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4111147675, 33404, 17079, [176, 124, 14, 50, 98, 126, 107, 62]);
}
impl ::windows::runtime::RuntimeName for ProcessMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsage";
}
impl ::std::convert::From<ProcessMemoryUsage> for ::windows::runtime::IUnknown {
    fn from(value: ProcessMemoryUsage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessMemoryUsage> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessMemoryUsage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessMemoryUsage> for ::windows::runtime::IInspectable {
    fn from(value: ProcessMemoryUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessMemoryUsage> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessMemoryUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessMemoryUsage {}
unsafe impl ::std::marker::Sync for ProcessMemoryUsage {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessMemoryUsageReport(::windows::runtime::IInspectable);
impl ProcessMemoryUsageReport {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn NonPagedPoolSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PageFaultCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PageFileSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PagedPoolSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PeakNonPagedPoolSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PeakPageFileSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PeakPagedPoolSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PeakVirtualMemorySizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PeakWorkingSetSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PrivatePageCount(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn VirtualMemorySizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn WorkingSetSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessMemoryUsageReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessMemoryUsageReport;{c2c77cba-1951-4685-8532-7e749ecf8eeb})");
}
unsafe impl ::windows::runtime::Interface for ProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3267853498, 6481, 18053, [133, 50, 126, 116, 158, 207, 142, 235]);
}
impl ::windows::runtime::RuntimeName for ProcessMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsageReport";
}
impl ::std::convert::From<ProcessMemoryUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: ProcessMemoryUsageReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessMemoryUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessMemoryUsageReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessMemoryUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: ProcessMemoryUsageReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessMemoryUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessMemoryUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessMemoryUsageReport {}
unsafe impl ::std::marker::Sync for ProcessMemoryUsageReport {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemCpuUsage(::windows::runtime::IInspectable);
impl SystemCpuUsage {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetReport(&self) -> ::windows::runtime::Result<SystemCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemCpuUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemCpuUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemCpuUsage;{6037b3ac-02d6-4234-8362-7fe3adc81f5f})");
}
unsafe impl ::windows::runtime::Interface for SystemCpuUsage {
    type Vtable = ISystemCpuUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1614263212, 726, 16948, [131, 98, 127, 227, 173, 200, 31, 95]);
}
impl ::windows::runtime::RuntimeName for SystemCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsage";
}
impl ::std::convert::From<SystemCpuUsage> for ::windows::runtime::IUnknown {
    fn from(value: SystemCpuUsage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemCpuUsage> for ::windows::runtime::IUnknown {
    fn from(value: &SystemCpuUsage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SystemCpuUsage> for ::windows::runtime::IInspectable {
    fn from(value: SystemCpuUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemCpuUsage> for ::windows::runtime::IInspectable {
    fn from(value: &SystemCpuUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemCpuUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemCpuUsage {}
unsafe impl ::std::marker::Sync for SystemCpuUsage {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemCpuUsageReport(::windows::runtime::IInspectable);
impl SystemCpuUsageReport {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn KernelTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn UserTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Diagnostics`, `Foundation`*"]
    pub fn IdleTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemCpuUsageReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemCpuUsageReport;{2c26d0b2-9483-4f62-ab57-82b29d9719b8})");
}
unsafe impl ::windows::runtime::Interface for SystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(740741298, 38019, 20322, [171, 87, 130, 178, 157, 151, 25, 184]);
}
impl ::windows::runtime::RuntimeName for SystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsageReport";
}
impl ::std::convert::From<SystemCpuUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: SystemCpuUsageReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemCpuUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: &SystemCpuUsageReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SystemCpuUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: SystemCpuUsageReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemCpuUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: &SystemCpuUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemCpuUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemCpuUsageReport {}
unsafe impl ::std::marker::Sync for SystemCpuUsageReport {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemDiagnosticInfo(::windows::runtime::IInspectable);
impl SystemDiagnosticInfo {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn MemoryUsage(&self) -> ::windows::runtime::Result<SystemMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemMemoryUsage>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn CpuUsage(&self) -> ::windows::runtime::Result<SystemCpuUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemCpuUsage>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetForCurrentSystem() -> ::windows::runtime::Result<SystemDiagnosticInfo> {
        Self::ISystemDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemDiagnosticInfo>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn IsArchitectureSupported(r#type: super::ProcessorArchitecture) -> ::windows::runtime::Result<bool> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn PreferredArchitecture() -> ::windows::runtime::Result<super::ProcessorArchitecture> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: super::ProcessorArchitecture = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::ProcessorArchitecture>(result__)
        })
    }
    pub fn ISystemDiagnosticInfoStatics<R, F: FnOnce(&ISystemDiagnosticInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISystemDiagnosticInfoStatics2<R, F: FnOnce(&ISystemDiagnosticInfoStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemDiagnosticInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemDiagnosticInfo;{a290fe05-dff3-407f-9a1b-0b2b317ca800})");
}
unsafe impl ::windows::runtime::Interface for SystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2727411205, 57331, 16511, [154, 27, 11, 43, 49, 124, 168, 0]);
}
impl ::windows::runtime::RuntimeName for SystemDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemDiagnosticInfo";
}
impl ::std::convert::From<SystemDiagnosticInfo> for ::windows::runtime::IUnknown {
    fn from(value: SystemDiagnosticInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemDiagnosticInfo> for ::windows::runtime::IUnknown {
    fn from(value: &SystemDiagnosticInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SystemDiagnosticInfo> for ::windows::runtime::IInspectable {
    fn from(value: SystemDiagnosticInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemDiagnosticInfo> for ::windows::runtime::IInspectable {
    fn from(value: &SystemDiagnosticInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemDiagnosticInfo {}
unsafe impl ::std::marker::Sync for SystemDiagnosticInfo {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemMemoryUsage(::windows::runtime::IInspectable);
impl SystemMemoryUsage {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn GetReport(&self) -> ::windows::runtime::Result<SystemMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemMemoryUsageReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMemoryUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemMemoryUsage;{17ffc595-1702-49cf-aa27-2f0a32591404})");
}
unsafe impl ::windows::runtime::Interface for SystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(402638229, 5890, 18895, [170, 39, 47, 10, 50, 89, 20, 4]);
}
impl ::windows::runtime::RuntimeName for SystemMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsage";
}
impl ::std::convert::From<SystemMemoryUsage> for ::windows::runtime::IUnknown {
    fn from(value: SystemMemoryUsage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemMemoryUsage> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMemoryUsage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SystemMemoryUsage> for ::windows::runtime::IInspectable {
    fn from(value: SystemMemoryUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemMemoryUsage> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMemoryUsage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMemoryUsage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemMemoryUsage {}
unsafe impl ::std::marker::Sync for SystemMemoryUsage {}
#[doc = "*Required features: `System_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemMemoryUsageReport(::windows::runtime::IInspectable);
impl SystemMemoryUsageReport {
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn TotalPhysicalSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn AvailableSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics`*"]
    pub fn CommittedSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMemoryUsageReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemMemoryUsageReport;{38663c87-2a9f-403a-bd19-2cf3e8169500})");
}
unsafe impl ::windows::runtime::Interface for SystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(946224263, 10911, 16442, [189, 25, 44, 243, 232, 22, 149, 0]);
}
impl ::windows::runtime::RuntimeName for SystemMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsageReport";
}
impl ::std::convert::From<SystemMemoryUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: SystemMemoryUsageReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemMemoryUsageReport> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMemoryUsageReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SystemMemoryUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: SystemMemoryUsageReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemMemoryUsageReport> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMemoryUsageReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMemoryUsageReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemMemoryUsageReport {}
unsafe impl ::std::marker::Sync for SystemMemoryUsageReport {}

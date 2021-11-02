#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
pub struct AutomationAnnotationTypeRegistration {
    pub LocalId: i32,
}
impl AutomationAnnotationTypeRegistration {}
impl ::std::default::Default for AutomationAnnotationTypeRegistration {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AutomationAnnotationTypeRegistration {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AutomationAnnotationTypeRegistration").field("LocalId", &self.LocalId).finish()
    }
}
impl ::std::cmp::PartialEq for AutomationAnnotationTypeRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.LocalId == other.LocalId
    }
}
impl ::std::cmp::Eq for AutomationAnnotationTypeRegistration {}
unsafe impl ::windows::runtime::Abi for AutomationAnnotationTypeRegistration {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationAnnotationTypeRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
pub struct AutomationRemoteOperationOperandId {
    pub Value: i32,
}
impl AutomationRemoteOperationOperandId {}
impl ::std::default::Default for AutomationRemoteOperationOperandId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AutomationRemoteOperationOperandId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AutomationRemoteOperationOperandId").field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for AutomationRemoteOperationOperandId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for AutomationRemoteOperationOperandId {}
unsafe impl ::windows::runtime::Abi for AutomationRemoteOperationOperandId {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationRemoteOperationOperandId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId;i4)");
}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AutomationRemoteOperationResult(::windows::runtime::IInspectable);
impl AutomationRemoteOperationResult {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AutomationRemoteOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: AutomationRemoteOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AutomationRemoteOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn ErrorLocation(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn HasOperand<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), operandid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn GetOperand<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), operandid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationRemoteOperationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult;{e0f80c42-4a67-5534-bf5a-09e8a99b36b1})");
}
unsafe impl ::windows::runtime::Interface for AutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3774352450, 19047, 21812, [191, 90, 9, 232, 169, 155, 54, 177]);
}
impl ::windows::runtime::RuntimeName for AutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
}
impl ::std::convert::From<AutomationRemoteOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: AutomationRemoteOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AutomationRemoteOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &AutomationRemoteOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AutomationRemoteOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: AutomationRemoteOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AutomationRemoteOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: &AutomationRemoteOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AutomationRemoteOperationResult {}
unsafe impl ::std::marker::Sync for AutomationRemoteOperationResult {}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(0i32);
    pub const MalformedBytecode: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(1i32);
    pub const InstructionLimitExceeded: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(2i32);
    pub const UnhandledException: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(3i32);
    pub const ExecutionFailure: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(4i32);
}
impl ::std::convert::From<i32> for AutomationRemoteOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationRemoteOperationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationRemoteOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus;i4)");
}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
pub struct CoreAutomationRegistrar {}
impl CoreAutomationRegistrar {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn RegisterAnnotationType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(guid: Param0) -> ::windows::runtime::Result<AutomationAnnotationTypeRegistration> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe {
            let mut result__: AutomationAnnotationTypeRegistration = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), guid.into_param().abi(), &mut result__).from_abi::<AutomationAnnotationTypeRegistration>(result__)
        })
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn UnregisterAnnotationType<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationAnnotationTypeRegistration>>(registration: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), registration.into_param().abi()).ok() })
    }
    pub fn ICoreAutomationRegistrarStatics<R, F: FnOnce(&ICoreAutomationRegistrarStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreAutomationRegistrar, ICoreAutomationRegistrarStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CoreAutomationRegistrar {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRegistrar";
}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreAutomationRemoteOperation(::windows::runtime::IInspectable);
impl CoreAutomationRemoteOperation {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreAutomationRemoteOperation, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn IsOpcodeSupported(&self, opcode: u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), opcode, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn ImportElement<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::runtime::IntoParam<'a, super::AutomationElement>>(&self, operandid: Param0, element: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), operandid.into_param().abi(), element.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn ImportTextRange<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::runtime::IntoParam<'a, super::AutomationTextRange>>(&self, operandid: Param0, textrange: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), operandid.into_param().abi(), textrange.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn AddToResults<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), operandid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn Execute(&self, bytecodebuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<AutomationRemoteOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), bytecodebuffer.len() as u32, ::std::mem::transmute(bytecodebuffer.as_ptr()), &mut result__).from_abi::<AutomationRemoteOperationResult>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn ImportConnectionBoundObject<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::runtime::IntoParam<'a, super::AutomationConnectionBoundObject>>(&self, operandid: Param0, connectionboundobject: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreAutomationRemoteOperation2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), operandid.into_param().abi(), connectionboundobject.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreAutomationRemoteOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation;{3ac656f4-e2bc-5c6e-b8e7-b224fb74b060})");
}
unsafe impl ::windows::runtime::Interface for CoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(986076916, 58044, 23662, [184, 231, 178, 36, 251, 116, 176, 96]);
}
impl ::windows::runtime::RuntimeName for CoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
}
impl ::std::convert::From<CoreAutomationRemoteOperation> for ::windows::runtime::IUnknown {
    fn from(value: CoreAutomationRemoteOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreAutomationRemoteOperation> for ::windows::runtime::IUnknown {
    fn from(value: &CoreAutomationRemoteOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreAutomationRemoteOperation> for ::windows::runtime::IInspectable {
    fn from(value: CoreAutomationRemoteOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreAutomationRemoteOperation> for ::windows::runtime::IInspectable {
    fn from(value: &CoreAutomationRemoteOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreAutomationRemoteOperation {}
unsafe impl ::std::marker::Sync for CoreAutomationRemoteOperation {}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreAutomationRemoteOperationContext(::windows::runtime::IInspectable);
impl CoreAutomationRemoteOperationContext {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn GetOperand<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, id: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn SetOperand<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, id: Param0, operand: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id.into_param().abi(), operand.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn SetOperand2<'a, Param0: ::windows::runtime::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, id: Param0, operand: Param1, operandinterfaceid: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), id.into_param().abi(), operand.into_param().abi(), operandinterfaceid.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreAutomationRemoteOperationContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext;{b9af9cbb-3d3e-5918-a16b-7861626a3aeb})");
}
unsafe impl ::windows::runtime::Interface for CoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3115293883, 15678, 22808, [161, 107, 120, 97, 98, 106, 58, 235]);
}
impl ::windows::runtime::RuntimeName for CoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
}
impl ::std::convert::From<CoreAutomationRemoteOperationContext> for ::windows::runtime::IUnknown {
    fn from(value: CoreAutomationRemoteOperationContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreAutomationRemoteOperationContext> for ::windows::runtime::IUnknown {
    fn from(value: &CoreAutomationRemoteOperationContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreAutomationRemoteOperationContext> for ::windows::runtime::IInspectable {
    fn from(value: CoreAutomationRemoteOperationContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreAutomationRemoteOperationContext> for ::windows::runtime::IInspectable {
    fn from(value: &CoreAutomationRemoteOperationContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreAutomationRemoteOperationContext {}
unsafe impl ::std::marker::Sync for CoreAutomationRemoteOperationContext {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAutomationRemoteOperationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3774352450, 19047, 21812, [191, 90, 9, 232, 169, 155, 54, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationRemoteOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AutomationRemoteOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operandid: AutomationRemoteOperationOperandId, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operandid: AutomationRemoteOperationOperandId, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
pub struct ICoreAutomationConnectionBoundObjectProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAutomationConnectionBoundObjectProvider {
    type Vtable = ICoreAutomationConnectionBoundObjectProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(102808420, 38422, 21907, [190, 58, 235, 142, 109, 174, 179, 250]);
}
impl ICoreAutomationConnectionBoundObjectProvider {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn IsComThreadingRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICoreAutomationConnectionBoundObjectProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{0620bb64-9616-5593-be3a-eb8e6daeb3fa}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationConnectionBoundObjectProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICoreAutomationRegistrarStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAutomationRegistrarStatics {
    type Vtable = ICoreAutomationRegistrarStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1045435035, 55004, 22144, [181, 128, 255, 255, 120, 48, 3, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRegistrarStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: ::windows::runtime::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, registration: AutomationAnnotationTypeRegistration) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(986076916, 58044, 23662, [184, 231, 178, 36, 251, 116, 176, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opcode: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operandid: AutomationRemoteOperationOperandId, element: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operandid: AutomationRemoteOperationOperandId, textrange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operandid: AutomationRemoteOperationOperandId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAutomationRemoteOperation2 {
    type Vtable = ICoreAutomationRemoteOperation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009425007, 59731, 20633, [140, 233, 220, 168, 19, 72, 43, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operandid: AutomationRemoteOperationOperandId, connectionboundobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3115293883, 15678, 22808, [161, 107, 120, 97, 98, 106, 58, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: AutomationRemoteOperationOperandId, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: AutomationRemoteOperationOperandId, operand: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: AutomationRemoteOperationOperandId, operand: ::windows::runtime::RawPtr, operandinterfaceid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
pub struct ICoreAutomationRemoteOperationExtensionProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAutomationRemoteOperationExtensionProvider {
    type Vtable = ICoreAutomationRemoteOperationExtensionProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2297773671, 56425, 21819, [160, 170, 112, 71, 126, 114, 77, 168]);
}
impl ICoreAutomationRemoteOperationExtensionProvider {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn CallExtension<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, CoreAutomationRemoteOperationContext>>(&self, extensionid: Param0, context: Param1, operandids: &[<AutomationRemoteOperationOperandId as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), extensionid.into_param().abi(), context.into_param().abi(), operandids.len() as u32, ::std::mem::transmute(operandids.as_ptr())).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn IsExtensionSupported<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, extensionid: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), extensionid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICoreAutomationRemoteOperationExtensionProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{88f53e67-dc69-553b-a0aa-70477e724da8}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationExtensionProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extensionid: ::windows::runtime::GUID, context: ::windows::runtime::RawPtr, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extensionid: ::windows::runtime::GUID, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1552550173, 38092, 23347, [175, 219, 103, 140, 222, 210, 189, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSessionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteAutomationClientSessionFactory {
    type Vtable = IRemoteAutomationClientSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4065338941, 24663, 21363, [165, 165, 237, 114, 101, 254, 3, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteAutomationConnectionRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3929217448, 58280, 24006, [173, 248, 4, 78, 70, 177, 74, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationConnectionRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteAutomationDisconnectedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3149085245, 23952, 23608, [158, 178, 221, 157, 204, 27, 46, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationDisconnectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteAutomationServerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteAutomationServerStatics {
    type Vtable = IRemoteAutomationServerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3874002014, 3089, 20520, [154, 227, 194, 119, 18, 136, 182, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationServerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteAutomationWindow(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2086696585, 18797, 20778, [155, 213, 192, 80, 207, 175, 20, 40]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RemoteAutomationClientSession(::windows::runtime::IInspectable);
impl RemoteAutomationClientSession {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_UIAutomation_Core`, `Foundation`*"]
    pub fn CreateWindowAsync<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), remotewindowid, remoteprocessid, parentautomationelement.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn SessionId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_UIAutomation_Core`, `Foundation`*"]
    pub fn ConnectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_UIAutomation_Core`, `Foundation`*"]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_UIAutomation_Core`, `Foundation`*"]
    pub fn Disconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_UIAutomation_Core`, `Foundation`*"]
    pub fn RemoveDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<RemoteAutomationClientSession>(result__)
        })
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn CreateInstance2<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(name: Param0, sessionid: Param1) -> ::windows::runtime::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), sessionid.into_param().abi(), &mut result__).from_abi::<RemoteAutomationClientSession>(result__)
        })
    }
    pub fn IRemoteAutomationClientSessionFactory<R, F: FnOnce(&IRemoteAutomationClientSessionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteAutomationClientSession, IRemoteAutomationClientSessionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteAutomationClientSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationClientSession;{5c8a091d-94cc-5b33-afdb-678cded2bd54})");
}
unsafe impl ::windows::runtime::Interface for RemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1552550173, 38092, 23347, [175, 219, 103, 140, 222, 210, 189, 84]);
}
impl ::windows::runtime::RuntimeName for RemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
}
impl ::std::convert::From<RemoteAutomationClientSession> for ::windows::runtime::IUnknown {
    fn from(value: RemoteAutomationClientSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteAutomationClientSession> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteAutomationClientSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RemoteAutomationClientSession> for ::windows::runtime::IInspectable {
    fn from(value: RemoteAutomationClientSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteAutomationClientSession> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteAutomationClientSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteAutomationClientSession {}
unsafe impl ::std::marker::Sync for RemoteAutomationClientSession {}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RemoteAutomationConnectionRequestedEventArgs(::windows::runtime::IInspectable);
impl RemoteAutomationConnectionRequestedEventArgs {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn LocalPipeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn RemoteProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteAutomationConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs;{ea3319a8-e3a8-5dc6-adf8-044e46b14af5})");
}
unsafe impl ::windows::runtime::Interface for RemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3929217448, 58280, 24006, [173, 248, 4, 78, 70, 177, 74, 245]);
}
impl ::windows::runtime::RuntimeName for RemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
}
impl ::std::convert::From<RemoteAutomationConnectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteAutomationConnectionRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteAutomationConnectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteAutomationConnectionRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RemoteAutomationConnectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteAutomationConnectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteAutomationConnectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteAutomationConnectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteAutomationConnectionRequestedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteAutomationConnectionRequestedEventArgs {}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RemoteAutomationDisconnectedEventArgs(::windows::runtime::IInspectable);
impl RemoteAutomationDisconnectedEventArgs {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn LocalPipeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteAutomationDisconnectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs;{bbb33a3d-5d90-5c38-9eb2-dd9dcc1b2e3f})");
}
unsafe impl ::windows::runtime::Interface for RemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3149085245, 23952, 23608, [158, 178, 221, 157, 204, 27, 46, 63]);
}
impl ::windows::runtime::RuntimeName for RemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
}
impl ::std::convert::From<RemoteAutomationDisconnectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteAutomationDisconnectedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteAutomationDisconnectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteAutomationDisconnectedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RemoteAutomationDisconnectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteAutomationDisconnectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteAutomationDisconnectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteAutomationDisconnectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteAutomationDisconnectedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteAutomationDisconnectedEventArgs {}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
pub struct RemoteAutomationServer {}
impl RemoteAutomationServer {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn ReportSession<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(sessionid: Param0) -> ::windows::runtime::Result<()> {
        Self::IRemoteAutomationServerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), sessionid.into_param().abi()).ok() })
    }
    pub fn IRemoteAutomationServerStatics<R, F: FnOnce(&IRemoteAutomationServerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteAutomationServer, IRemoteAutomationServerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RemoteAutomationServer {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationServer";
}
#[doc = "*Required features: `UI_UIAutomation_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RemoteAutomationWindow(::windows::runtime::IInspectable);
impl RemoteAutomationWindow {
    #[doc = "*Required features: `UI_UIAutomation_Core`*"]
    pub fn AutomationProvider(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_UIAutomation_Core`, `Foundation`*"]
    pub fn UnregisterAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteAutomationWindow {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationWindow;{7c607689-496d-512a-9bd5-c050cfaf1428})");
}
unsafe impl ::windows::runtime::Interface for RemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2086696585, 18797, 20778, [155, 213, 192, 80, 207, 175, 20, 40]);
}
impl ::windows::runtime::RuntimeName for RemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
}
impl ::std::convert::From<RemoteAutomationWindow> for ::windows::runtime::IUnknown {
    fn from(value: RemoteAutomationWindow) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteAutomationWindow> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteAutomationWindow) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteAutomationWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteAutomationWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RemoteAutomationWindow> for ::windows::runtime::IInspectable {
    fn from(value: RemoteAutomationWindow) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteAutomationWindow> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteAutomationWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteAutomationWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteAutomationWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteAutomationWindow {}
unsafe impl ::std::marker::Sync for RemoteAutomationWindow {}

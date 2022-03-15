#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct AutomationAnnotationTypeRegistration {
    pub LocalId: i32,
}
impl ::core::marker::Copy for AutomationAnnotationTypeRegistration {}
impl ::core::clone::Clone for AutomationAnnotationTypeRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AutomationAnnotationTypeRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutomationAnnotationTypeRegistration").field("LocalId", &self.LocalId).finish()
    }
}
unsafe impl ::windows::core::Abi for AutomationAnnotationTypeRegistration {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationAnnotationTypeRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for AutomationAnnotationTypeRegistration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AutomationAnnotationTypeRegistration>()) == 0 }
    }
}
impl ::core::cmp::Eq for AutomationAnnotationTypeRegistration {}
impl ::core::default::Default for AutomationAnnotationTypeRegistration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct AutomationRemoteOperationOperandId {
    pub Value: i32,
}
impl ::core::marker::Copy for AutomationRemoteOperationOperandId {}
impl ::core::clone::Clone for AutomationRemoteOperationOperandId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AutomationRemoteOperationOperandId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutomationRemoteOperationOperandId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for AutomationRemoteOperationOperandId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationRemoteOperationOperandId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for AutomationRemoteOperationOperandId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AutomationRemoteOperationOperandId>()) == 0 }
    }
}
impl ::core::cmp::Eq for AutomationRemoteOperationOperandId {}
impl ::core::default::Default for AutomationRemoteOperationOperandId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct AutomationRemoteOperationResult(::windows::core::IUnknown);
impl AutomationRemoteOperationResult {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AutomationRemoteOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: AutomationRemoteOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationRemoteOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn ErrorLocation(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn HasOperand<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasOperand)(::core::mem::transmute_copy(this), operandid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn GetOperand<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOperand)(::core::mem::transmute_copy(this), operandid.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationRemoteOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationRemoteOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationRemoteOperationResult {}
impl ::core::fmt::Debug for AutomationRemoteOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationRemoteOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationRemoteOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult;{e0f80c42-4a67-5534-bf5a-09e8a99b36b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_Vtbl;
    const IID: ::windows::core::GUID = <IAutomationRemoteOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
}
impl ::core::convert::From<AutomationRemoteOperationResult> for ::windows::core::IUnknown {
    fn from(value: AutomationRemoteOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationRemoteOperationResult> for ::windows::core::IUnknown {
    fn from(value: &AutomationRemoteOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationRemoteOperationResult> for ::windows::core::IInspectable {
    fn from(value: AutomationRemoteOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationRemoteOperationResult> for ::windows::core::IInspectable {
    fn from(value: &AutomationRemoteOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationRemoteOperationResult {}
unsafe impl ::core::marker::Sync for AutomationRemoteOperationResult {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const MalformedBytecode: Self = Self(1i32);
    pub const InstructionLimitExceeded: Self = Self(2i32);
    pub const UnhandledException: Self = Self(3i32);
    pub const ExecutionFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationRemoteOperationStatus {}
impl ::core::clone::Clone for AutomationRemoteOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationRemoteOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutomationRemoteOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationRemoteOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationRemoteOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationRemoteOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct CoreAutomationRegistrar {}
impl CoreAutomationRegistrar {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn RegisterAnnotationType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(guid: Param0) -> ::windows::core::Result<AutomationAnnotationTypeRegistration> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe {
            let mut result__: AutomationAnnotationTypeRegistration = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterAnnotationType)(::core::mem::transmute_copy(this), guid.into_param().abi(), &mut result__).from_abi::<AutomationAnnotationTypeRegistration>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn UnregisterAnnotationType<'a, Param0: ::windows::core::IntoParam<'a, AutomationAnnotationTypeRegistration>>(registration: Param0) -> ::windows::core::Result<()> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe { (::windows::core::Interface::vtable(this).UnregisterAnnotationType)(::core::mem::transmute_copy(this), registration.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICoreAutomationRegistrarStatics<R, F: FnOnce(&ICoreAutomationRegistrarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreAutomationRegistrar, ICoreAutomationRegistrarStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CoreAutomationRegistrar {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRegistrar";
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct CoreAutomationRemoteOperation(::windows::core::IUnknown);
impl CoreAutomationRemoteOperation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreAutomationRemoteOperation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn IsOpcodeSupported(&self, opcode: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOpcodeSupported)(::core::mem::transmute_copy(this), opcode, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn ImportElement<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::core::IntoParam<'a, super::AutomationElement>>(&self, operandid: Param0, element: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ImportElement)(::core::mem::transmute_copy(this), operandid.into_param().abi(), element.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn ImportTextRange<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::core::IntoParam<'a, super::AutomationTextRange>>(&self, operandid: Param0, textrange: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ImportTextRange)(::core::mem::transmute_copy(this), operandid.into_param().abi(), textrange.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn AddToResults<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToResults)(::core::mem::transmute_copy(this), operandid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn Execute(&self, bytecodebuffer: &[u8]) -> ::windows::core::Result<AutomationRemoteOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Execute)(::core::mem::transmute_copy(this), bytecodebuffer.len() as u32, ::core::mem::transmute(bytecodebuffer.as_ptr()), &mut result__).from_abi::<AutomationRemoteOperationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn ImportConnectionBoundObject<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::core::IntoParam<'a, super::AutomationConnectionBoundObject>>(&self, operandid: Param0, connectionboundobject: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreAutomationRemoteOperation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ImportConnectionBoundObject)(::core::mem::transmute_copy(this), operandid.into_param().abi(), connectionboundobject.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreAutomationRemoteOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreAutomationRemoteOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAutomationRemoteOperation {}
impl ::core::fmt::Debug for CoreAutomationRemoteOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAutomationRemoteOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreAutomationRemoteOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation;{3ac656f4-e2bc-5c6e-b8e7-b224fb74b060})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_Vtbl;
    const IID: ::windows::core::GUID = <ICoreAutomationRemoteOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
}
impl ::core::convert::From<CoreAutomationRemoteOperation> for ::windows::core::IUnknown {
    fn from(value: CoreAutomationRemoteOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperation> for ::windows::core::IUnknown {
    fn from(value: &CoreAutomationRemoteOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreAutomationRemoteOperation> for ::windows::core::IInspectable {
    fn from(value: CoreAutomationRemoteOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperation> for ::windows::core::IInspectable {
    fn from(value: &CoreAutomationRemoteOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAutomationRemoteOperation {}
unsafe impl ::core::marker::Sync for CoreAutomationRemoteOperation {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct CoreAutomationRemoteOperationContext(::windows::core::IUnknown);
impl CoreAutomationRemoteOperationContext {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn GetOperand<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, id: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOperand)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn SetOperand<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, id: Param0, operand: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOperand)(::core::mem::transmute_copy(this), id.into_param().abi(), operand.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn SetOperand2<'a, Param0: ::windows::core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0, operand: Param1, operandinterfaceid: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOperand2)(::core::mem::transmute_copy(this), id.into_param().abi(), operand.into_param().abi(), operandinterfaceid.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreAutomationRemoteOperationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreAutomationRemoteOperationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAutomationRemoteOperationContext {}
impl ::core::fmt::Debug for CoreAutomationRemoteOperationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAutomationRemoteOperationContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreAutomationRemoteOperationContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext;{b9af9cbb-3d3e-5918-a16b-7861626a3aeb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_Vtbl;
    const IID: ::windows::core::GUID = <ICoreAutomationRemoteOperationContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
}
impl ::core::convert::From<CoreAutomationRemoteOperationContext> for ::windows::core::IUnknown {
    fn from(value: CoreAutomationRemoteOperationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperationContext> for ::windows::core::IUnknown {
    fn from(value: &CoreAutomationRemoteOperationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreAutomationRemoteOperationContext> for ::windows::core::IInspectable {
    fn from(value: CoreAutomationRemoteOperationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperationContext> for ::windows::core::IInspectable {
    fn from(value: &CoreAutomationRemoteOperationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAutomationRemoteOperationContext {}
unsafe impl ::core::marker::Sync for CoreAutomationRemoteOperationContext {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationRemoteOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f80c42_4a67_5534_bf5a_09e8a99b36b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationRemoteOperationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutomationRemoteOperationStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ErrorLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub HasOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct ICoreAutomationConnectionBoundObjectProvider(::windows::core::IUnknown);
impl ICoreAutomationConnectionBoundObjectProvider {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn IsComThreadingRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComThreadingRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICoreAutomationConnectionBoundObjectProvider> for ::windows::core::IUnknown {
    fn from(value: ICoreAutomationConnectionBoundObjectProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationConnectionBoundObjectProvider> for ::windows::core::IUnknown {
    fn from(value: &ICoreAutomationConnectionBoundObjectProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreAutomationConnectionBoundObjectProvider> for ::windows::core::IInspectable {
    fn from(value: ICoreAutomationConnectionBoundObjectProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationConnectionBoundObjectProvider> for ::windows::core::IInspectable {
    fn from(value: &ICoreAutomationConnectionBoundObjectProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreAutomationConnectionBoundObjectProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreAutomationConnectionBoundObjectProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreAutomationConnectionBoundObjectProvider {}
impl ::core::fmt::Debug for ICoreAutomationConnectionBoundObjectProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreAutomationConnectionBoundObjectProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreAutomationConnectionBoundObjectProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0620bb64-9616-5593-be3a-eb8e6daeb3fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICoreAutomationConnectionBoundObjectProvider {
    type Vtable = ICoreAutomationConnectionBoundObjectProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0620bb64_9616_5593_be3a_eb8e6daeb3fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsComThreadingRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRegistrarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRegistrarStatics {
    type Vtable = ICoreAutomationRegistrarStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e50129b_d6dc_5680_b580_ffff78300304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRegistrarStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RegisterAnnotationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT,
    pub UnregisterAnnotationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ac656f4_e2bc_5c6e_b8e7_b224fb74b060);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsOpcodeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opcode: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ImportElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ImportTextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, textrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddToResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperation2 {
    type Vtable = ICoreAutomationRemoteOperation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefaf86f_e953_5099_8ce9_dca813482ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ImportConnectionBoundObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, connectionboundobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9af9cbb_3d3e_5918_a16b_7861626a3aeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationContext_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOperand2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationExtensionProvider(::windows::core::IUnknown);
impl ICoreAutomationRemoteOperationExtensionProvider {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn CallExtension<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, CoreAutomationRemoteOperationContext>>(&self, extensionid: Param0, context: Param1, operandids: &[AutomationRemoteOperationOperandId]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CallExtension)(::core::mem::transmute_copy(this), extensionid.into_param().abi(), context.into_param().abi(), operandids.len() as u32, ::core::mem::transmute(operandids.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn IsExtensionSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, extensionid: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsExtensionSupported)(::core::mem::transmute_copy(this), extensionid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICoreAutomationRemoteOperationExtensionProvider> for ::windows::core::IUnknown {
    fn from(value: ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationRemoteOperationExtensionProvider> for ::windows::core::IUnknown {
    fn from(value: &ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreAutomationRemoteOperationExtensionProvider> for ::windows::core::IInspectable {
    fn from(value: ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationRemoteOperationExtensionProvider> for ::windows::core::IInspectable {
    fn from(value: &ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreAutomationRemoteOperationExtensionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreAutomationRemoteOperationExtensionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreAutomationRemoteOperationExtensionProvider {}
impl ::core::fmt::Debug for ICoreAutomationRemoteOperationExtensionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreAutomationRemoteOperationExtensionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreAutomationRemoteOperationExtensionProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{88f53e67-dc69-553b-a0aa-70477e724da8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperationExtensionProvider {
    type Vtable = ICoreAutomationRemoteOperationExtensionProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f53e67_dc69_553b_a0aa_70477e724da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CallExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, context: ::windows::core::RawPtr, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT,
    pub IsExtensionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationClientSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8a091d_94cc_5b33_afdb_678cded2bd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWindowAsync: usize,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationClientSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationClientSessionFactory {
    type Vtable = IRemoteAutomationClientSessionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf250263d_6057_5373_a5a5_ed7265fe0376);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSessionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationConnectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3319a8_e3a8_5dc6_adf8_044e46b14af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationConnectionRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationDisconnectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbb33a3d_5d90_5c38_9eb2_dd9dcc1b2e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationDisconnectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationServerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationServerStatics {
    type Vtable = IRemoteAutomationServerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6e8945e_0c11_5028_9ae3_c2771288b6b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationServerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ReportSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c607689_496d_512a_9bd5_c050cfaf1428);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationWindow_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AutomationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAsync: usize,
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationClientSession(::windows::core::IUnknown);
impl RemoteAutomationClientSession {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWindowAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWindowAsync)(::core::mem::transmute_copy(this), remotewindowid, remoteprocessid, parentautomationelement.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Disconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Disconnected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDisconnected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<RemoteAutomationClientSession>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn CreateInstance2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(name: Param0, sessionid: Param1) -> ::windows::core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance2)(::core::mem::transmute_copy(this), name.into_param().abi(), sessionid.into_param().abi(), &mut result__).from_abi::<RemoteAutomationClientSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteAutomationClientSessionFactory<R, F: FnOnce(&IRemoteAutomationClientSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteAutomationClientSession, IRemoteAutomationClientSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteAutomationClientSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationClientSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationClientSession {}
impl ::core::fmt::Debug for RemoteAutomationClientSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationClientSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteAutomationClientSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationClientSession;{5c8a091d-94cc-5b33-afdb-678cded2bd54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_Vtbl;
    const IID: ::windows::core::GUID = <IRemoteAutomationClientSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
}
impl ::core::convert::From<RemoteAutomationClientSession> for ::windows::core::IUnknown {
    fn from(value: RemoteAutomationClientSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationClientSession> for ::windows::core::IUnknown {
    fn from(value: &RemoteAutomationClientSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationClientSession> for ::windows::core::IInspectable {
    fn from(value: RemoteAutomationClientSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationClientSession> for ::windows::core::IInspectable {
    fn from(value: &RemoteAutomationClientSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoteAutomationClientSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationClientSession {}
unsafe impl ::core::marker::Sync for RemoteAutomationClientSession {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationConnectionRequestedEventArgs(::windows::core::IUnknown);
impl RemoteAutomationConnectionRequestedEventArgs {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalPipeName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn RemoteProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteProcessId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteAutomationConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationConnectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationConnectionRequestedEventArgs {}
impl ::core::fmt::Debug for RemoteAutomationConnectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationConnectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteAutomationConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs;{ea3319a8-e3a8-5dc6-adf8-044e46b14af5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRemoteAutomationConnectionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
}
impl ::core::convert::From<RemoteAutomationConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RemoteAutomationConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RemoteAutomationConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RemoteAutomationConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RemoteAutomationConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteAutomationConnectionRequestedEventArgs {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationDisconnectedEventArgs(::windows::core::IUnknown);
impl RemoteAutomationDisconnectedEventArgs {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalPipeName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteAutomationDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationDisconnectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationDisconnectedEventArgs {}
impl ::core::fmt::Debug for RemoteAutomationDisconnectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationDisconnectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteAutomationDisconnectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs;{bbb33a3d-5d90-5c38-9eb2-dd9dcc1b2e3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRemoteAutomationDisconnectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
}
impl ::core::convert::From<RemoteAutomationDisconnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RemoteAutomationDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationDisconnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RemoteAutomationDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationDisconnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RemoteAutomationDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationDisconnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RemoteAutomationDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteAutomationDisconnectedEventArgs {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct RemoteAutomationServer {}
impl RemoteAutomationServer {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn ReportSession<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(sessionid: Param0) -> ::windows::core::Result<()> {
        Self::IRemoteAutomationServerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ReportSession)(::core::mem::transmute_copy(this), sessionid.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IRemoteAutomationServerStatics<R, F: FnOnce(&IRemoteAutomationServerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteAutomationServer, IRemoteAutomationServerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RemoteAutomationServer {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationServer";
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationWindow(::windows::core::IUnknown);
impl RemoteAutomationWindow {
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
    pub fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutomationProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_UIAutomation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnregisterAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteAutomationWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationWindow {}
impl ::core::fmt::Debug for RemoteAutomationWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteAutomationWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationWindow;{7c607689-496d-512a-9bd5-c050cfaf1428})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_Vtbl;
    const IID: ::windows::core::GUID = <IRemoteAutomationWindow as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
}
impl ::core::convert::From<RemoteAutomationWindow> for ::windows::core::IUnknown {
    fn from(value: RemoteAutomationWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationWindow> for ::windows::core::IUnknown {
    fn from(value: &RemoteAutomationWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteAutomationWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoteAutomationWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationWindow> for ::windows::core::IInspectable {
    fn from(value: RemoteAutomationWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationWindow> for ::windows::core::IInspectable {
    fn from(value: &RemoteAutomationWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteAutomationWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoteAutomationWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationWindow {}
unsafe impl ::core::marker::Sync for RemoteAutomationWindow {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

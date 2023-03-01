#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationRemoteOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_Vtbl;
}
impl ::core::clone::Clone for IAutomationRemoteOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAutomationRemoteOperationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f80c42_4a67_5534_bf5a_09e8a99b36b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationRemoteOperationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
    pub fn IsComThreadingRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsComThreadingRequired)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ICoreAutomationConnectionBoundObjectProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for ICoreAutomationConnectionBoundObjectProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{0620bb64-9616-5593-be3a-eb8e6daeb3fa}");
}
unsafe impl ::windows::core::Interface for ICoreAutomationConnectionBoundObjectProvider {
    type Vtable = ICoreAutomationConnectionBoundObjectProvider_Vtbl;
}
impl ::core::clone::Clone for ICoreAutomationConnectionBoundObjectProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAutomationConnectionBoundObjectProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0620bb64_9616_5593_be3a_eb8e6daeb3fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsComThreadingRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRegistrarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRegistrarStatics {
    type Vtable = ICoreAutomationRegistrarStatics_Vtbl;
}
impl ::core::clone::Clone for ICoreAutomationRegistrarStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAutomationRegistrarStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e50129b_d6dc_5680_b580_ffff78300304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRegistrarStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RegisterAnnotationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT,
    pub UnregisterAnnotationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_Vtbl;
}
impl ::core::clone::Clone for ICoreAutomationRemoteOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAutomationRemoteOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ac656f4_e2bc_5c6e_b8e7_b224fb74b060);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsOpcodeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opcode: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ImportElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, element: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ImportTextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, textrange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperation2 {
    type Vtable = ICoreAutomationRemoteOperation2_Vtbl;
}
impl ::core::clone::Clone for ICoreAutomationRemoteOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAutomationRemoteOperation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefaf86f_e953_5099_8ce9_dca813482ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ImportConnectionBoundObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, connectionboundobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_Vtbl;
}
impl ::core::clone::Clone for ICoreAutomationRemoteOperationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAutomationRemoteOperationContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9af9cbb_3d3e_5918_a16b_7861626a3aeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOperand2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationExtensionProvider(::windows::core::IUnknown);
impl ICoreAutomationRemoteOperationExtensionProvider {
    pub fn CallExtension(&self, extensionid: ::windows::core::GUID, context: &CoreAutomationRemoteOperationContext, operandids: &[AutomationRemoteOperationOperandId]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CallExtension)(::windows::core::Interface::as_raw(this), extensionid, ::core::mem::transmute_copy(context), operandids.len() as u32, operandids.as_ptr()).ok() }
    }
    pub fn IsExtensionSupported(&self, extensionid: ::windows::core::GUID) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsExtensionSupported)(::windows::core::Interface::as_raw(this), extensionid, &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ICoreAutomationRemoteOperationExtensionProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for ICoreAutomationRemoteOperationExtensionProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{88f53e67-dc69-553b-a0aa-70477e724da8}");
}
unsafe impl ::windows::core::Interface for ICoreAutomationRemoteOperationExtensionProvider {
    type Vtable = ICoreAutomationRemoteOperationExtensionProvider_Vtbl;
}
impl ::core::clone::Clone for ICoreAutomationRemoteOperationExtensionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAutomationRemoteOperationExtensionProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f53e67_dc69_553b_a0aa_70477e724da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CallExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, context: *mut ::core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT,
    pub IsExtensionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationClientSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_Vtbl;
}
impl ::core::clone::Clone for IRemoteAutomationClientSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteAutomationClientSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8a091d_94cc_5b33_afdb_678cded2bd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWindowAsync: usize,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IRemoteAutomationClientSessionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteAutomationClientSessionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf250263d_6057_5373_a5a5_ed7265fe0376);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSessionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, sessionid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationConnectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRemoteAutomationConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteAutomationConnectionRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3319a8_e3a8_5dc6_adf8_044e46b14af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationConnectionRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationDisconnectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRemoteAutomationDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteAutomationDisconnectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbb33a3d_5d90_5c38_9eb2_dd9dcc1b2e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationDisconnectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationServerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationServerStatics {
    type Vtable = IRemoteAutomationServerStatics_Vtbl;
}
impl ::core::clone::Clone for IRemoteAutomationServerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteAutomationServerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6e8945e_0c11_5028_9ae3_c2771288b6b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationServerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_Vtbl;
}
impl ::core::clone::Clone for IRemoteAutomationWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteAutomationWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c607689_496d_512a_9bd5_c050cfaf1428);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationWindow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AutomationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAsync: usize,
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct AutomationRemoteOperationResult(::windows::core::IUnknown);
impl AutomationRemoteOperationResult {
    pub fn Status(&self) -> ::windows::core::Result<AutomationRemoteOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AutomationRemoteOperationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorLocation(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ErrorLocation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasOperand(&self, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasOperand)(::windows::core::Interface::as_raw(this), operandid, &mut result__).from_abi(result__)
        }
    }
    pub fn GetOperand(&self, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).GetOperand)(::windows::core::Interface::as_raw(this), operandid, &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for AutomationRemoteOperationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult;{e0f80c42-4a67-5534-bf5a-09e8a99b36b1})");
}
impl ::core::clone::Clone for AutomationRemoteOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AutomationRemoteOperationResult {
    const IID: ::windows::core::GUID = <IAutomationRemoteOperationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
}
::windows::imp::interface_hierarchy!(AutomationRemoteOperationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AutomationRemoteOperationResult {}
unsafe impl ::core::marker::Sync for AutomationRemoteOperationResult {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct CoreAutomationRegistrar;
impl CoreAutomationRegistrar {
    pub fn RegisterAnnotationType(guid: ::windows::core::GUID) -> ::windows::core::Result<AutomationAnnotationTypeRegistration> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AutomationAnnotationTypeRegistration>();
            (::windows::core::Interface::vtable(this).RegisterAnnotationType)(::windows::core::Interface::as_raw(this), guid, &mut result__).from_abi(result__)
        })
    }
    pub fn UnregisterAnnotationType(registration: AutomationAnnotationTypeRegistration) -> ::windows::core::Result<()> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe { (::windows::core::Interface::vtable(this).UnregisterAnnotationType)(::windows::core::Interface::as_raw(this), registration).ok() })
    }
    #[doc(hidden)]
    pub fn ICoreAutomationRegistrarStatics<R, F: FnOnce(&ICoreAutomationRegistrarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreAutomationRegistrar, ICoreAutomationRegistrarStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreAutomationRemoteOperation, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsOpcodeSupported(&self, opcode: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsOpcodeSupported)(::windows::core::Interface::as_raw(this), opcode, &mut result__).from_abi(result__)
        }
    }
    pub fn ImportElement(&self, operandid: AutomationRemoteOperationOperandId, element: &super::AutomationElement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ImportElement)(::windows::core::Interface::as_raw(this), operandid, ::core::mem::transmute_copy(element)).ok() }
    }
    pub fn ImportTextRange(&self, operandid: AutomationRemoteOperationOperandId, textrange: &super::AutomationTextRange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ImportTextRange)(::windows::core::Interface::as_raw(this), operandid, ::core::mem::transmute_copy(textrange)).ok() }
    }
    pub fn AddToResults(&self, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToResults)(::windows::core::Interface::as_raw(this), operandid).ok() }
    }
    pub fn Execute(&self, bytecodebuffer: &[u8]) -> ::windows::core::Result<AutomationRemoteOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AutomationRemoteOperationResult>();
            (::windows::core::Interface::vtable(this).Execute)(::windows::core::Interface::as_raw(this), bytecodebuffer.len() as u32, bytecodebuffer.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn ImportConnectionBoundObject(&self, operandid: AutomationRemoteOperationOperandId, connectionboundobject: &super::AutomationConnectionBoundObject) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreAutomationRemoteOperation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ImportConnectionBoundObject)(::windows::core::Interface::as_raw(this), operandid, ::core::mem::transmute_copy(connectionboundobject)).ok() }
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
impl ::windows::core::RuntimeType for CoreAutomationRemoteOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation;{3ac656f4-e2bc-5c6e-b8e7-b224fb74b060})");
}
impl ::core::clone::Clone for CoreAutomationRemoteOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreAutomationRemoteOperation {
    const IID: ::windows::core::GUID = <ICoreAutomationRemoteOperation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
}
::windows::imp::interface_hierarchy!(CoreAutomationRemoteOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreAutomationRemoteOperation {}
unsafe impl ::core::marker::Sync for CoreAutomationRemoteOperation {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct CoreAutomationRemoteOperationContext(::windows::core::IUnknown);
impl CoreAutomationRemoteOperationContext {
    pub fn GetOperand(&self, id: AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).GetOperand)(::windows::core::Interface::as_raw(this), id, &mut result__).from_abi(result__)
        }
    }
    pub fn SetOperand<P0>(&self, id: AutomationRemoteOperationOperandId, operand: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOperand)(::windows::core::Interface::as_raw(this), id, operand.into_param().abi()).ok() }
    }
    pub fn SetOperand2<P0>(&self, id: AutomationRemoteOperationOperandId, operand: P0, operandinterfaceid: ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOperand2)(::windows::core::Interface::as_raw(this), id, operand.into_param().abi(), operandinterfaceid).ok() }
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
impl ::windows::core::RuntimeType for CoreAutomationRemoteOperationContext {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext;{b9af9cbb-3d3e-5918-a16b-7861626a3aeb})");
}
impl ::core::clone::Clone for CoreAutomationRemoteOperationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreAutomationRemoteOperationContext {
    const IID: ::windows::core::GUID = <ICoreAutomationRemoteOperationContext as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
}
::windows::imp::interface_hierarchy!(CoreAutomationRemoteOperationContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreAutomationRemoteOperationContext {}
unsafe impl ::core::marker::Sync for CoreAutomationRemoteOperationContext {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationClientSession(::windows::core::IUnknown);
impl RemoteAutomationClientSession {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWindowAsync<P0>(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>();
            (::windows::core::Interface::vtable(this).CreateWindowAsync)(::windows::core::Interface::as_raw(this), remotewindowid, remoteprocessid, parentautomationelement.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SessionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ConnectionRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Disconnected(&self, handler: &super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Disconnected)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateInstance(name: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<RemoteAutomationClientSession>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInstance2(name: &::windows::core::HSTRING, sessionid: ::windows::core::GUID) -> ::windows::core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<RemoteAutomationClientSession>();
            (::windows::core::Interface::vtable(this).CreateInstance2)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), sessionid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteAutomationClientSessionFactory<R, F: FnOnce(&IRemoteAutomationClientSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RemoteAutomationClientSession, IRemoteAutomationClientSessionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for RemoteAutomationClientSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationClientSession;{5c8a091d-94cc-5b33-afdb-678cded2bd54})");
}
impl ::core::clone::Clone for RemoteAutomationClientSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RemoteAutomationClientSession {
    const IID: ::windows::core::GUID = <IRemoteAutomationClientSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
}
::windows::imp::interface_hierarchy!(RemoteAutomationClientSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteAutomationClientSession {}
unsafe impl ::core::marker::Sync for RemoteAutomationClientSession {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationConnectionRequestedEventArgs(::windows::core::IUnknown);
impl RemoteAutomationConnectionRequestedEventArgs {
    pub fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalPipeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).RemoteProcessId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for RemoteAutomationConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs;{ea3319a8-e3a8-5dc6-adf8-044e46b14af5})");
}
impl ::core::clone::Clone for RemoteAutomationConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RemoteAutomationConnectionRequestedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteAutomationConnectionRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(RemoteAutomationConnectionRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteAutomationConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteAutomationConnectionRequestedEventArgs {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationDisconnectedEventArgs(::windows::core::IUnknown);
impl RemoteAutomationDisconnectedEventArgs {
    pub fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalPipeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for RemoteAutomationDisconnectedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs;{bbb33a3d-5d90-5c38-9eb2-dd9dcc1b2e3f})");
}
impl ::core::clone::Clone for RemoteAutomationDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RemoteAutomationDisconnectedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteAutomationDisconnectedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
}
::windows::imp::interface_hierarchy!(RemoteAutomationDisconnectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteAutomationDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteAutomationDisconnectedEventArgs {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct RemoteAutomationServer;
impl RemoteAutomationServer {
    pub fn ReportSession(sessionid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        Self::IRemoteAutomationServerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ReportSession)(::windows::core::Interface::as_raw(this), sessionid).ok() })
    }
    #[doc(hidden)]
    pub fn IRemoteAutomationServerStatics<R, F: FnOnce(&IRemoteAutomationServerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RemoteAutomationServer, IRemoteAutomationServerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for RemoteAutomationServer {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationServer";
}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct RemoteAutomationWindow(::windows::core::IUnknown);
impl RemoteAutomationWindow {
    pub fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).AutomationProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).UnregisterAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for RemoteAutomationWindow {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationWindow;{7c607689-496d-512a-9bd5-c050cfaf1428})");
}
impl ::core::clone::Clone for RemoteAutomationWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RemoteAutomationWindow {
    const IID: ::windows::core::GUID = <IRemoteAutomationWindow as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
}
::windows::imp::interface_hierarchy!(RemoteAutomationWindow, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteAutomationWindow {}
unsafe impl ::core::marker::Sync for RemoteAutomationWindow {}
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for AutomationRemoteOperationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AutomationRemoteOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationRemoteOperationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AutomationRemoteOperationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus;i4)");
}
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
impl ::windows::core::TypeKind for AutomationAnnotationTypeRegistration {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for AutomationAnnotationTypeRegistration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration;i4)");
}
impl ::core::cmp::PartialEq for AutomationAnnotationTypeRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.LocalId == other.LocalId
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
impl ::windows::core::TypeKind for AutomationRemoteOperationOperandId {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for AutomationRemoteOperationOperandId {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId;i4)");
}
impl ::core::cmp::PartialEq for AutomationRemoteOperationOperandId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for AutomationRemoteOperationOperandId {}
impl ::core::default::Default for AutomationRemoteOperationOperandId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

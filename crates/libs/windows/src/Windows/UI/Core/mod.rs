#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcceleratorKeyEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAcceleratorKeyEventArgs {
    type Vtable = IAcceleratorKeyEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAcceleratorKeyEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAcceleratorKeyEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff1c4c4a_9287_470b_836e_9086e3126ade);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcceleratorKeyEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreAcceleratorKeyEventType) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcceleratorKeyEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAcceleratorKeyEventArgs2 {
    type Vtable = IAcceleratorKeyEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IAcceleratorKeyEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAcceleratorKeyEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd300a9f6_2f7e_4873_a555_166e596ee1c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcceleratorKeyEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationProviderRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationProviderRequestedEventArgs {
    type Vtable = IAutomationProviderRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAutomationProviderRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAutomationProviderRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x961ff258_21bf_4b42_a298_fa479d4c52e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProviderRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AutomationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAutomationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackRequestedEventArgs {
    type Vtable = IBackRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBackRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd603d28a_e411_4a4e_ba41_6a327a8675bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICharacterReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICharacterReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc584659f_99b2_4bcc_bd33_04e63f42902e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub KeyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClosestInteractiveBoundsRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClosestInteractiveBoundsRequestedEventArgs {
    type Vtable = IClosestInteractiveBoundsRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IClosestInteractiveBoundsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClosestInteractiveBoundsRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x347c11d7_f6f8_40e3_b29f_ae50d3e86486);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosestInteractiveBoundsRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PointerPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPosition: usize,
    #[cfg(feature = "Foundation")]
    pub SearchBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchBounds: usize,
    #[cfg(feature = "Foundation")]
    pub ClosestInteractiveBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClosestInteractiveBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetClosestInteractiveBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetClosestInteractiveBounds: usize,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICoreAcceleratorKeys(::windows::core::IUnknown);
impl ICoreAcceleratorKeys {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyActivated(&self, handler: &super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AcceleratorKeyActivated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAcceleratorKeyActivated)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICoreAcceleratorKeys, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ICoreAcceleratorKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreAcceleratorKeys {}
impl ::core::fmt::Debug for ICoreAcceleratorKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreAcceleratorKeys").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICoreAcceleratorKeys {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{9ffdf7f5-b8c9-4ef0-b7d2-1de626561fc8}");
}
unsafe impl ::windows::core::Interface for ICoreAcceleratorKeys {
    type Vtable = ICoreAcceleratorKeys_Vtbl;
}
impl ::core::clone::Clone for ICoreAcceleratorKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreAcceleratorKeys {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ffdf7f5_b8c9_4ef0_b7d2_1de626561fc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAcceleratorKeys_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AcceleratorKeyActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceleratorKeyActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAcceleratorKeyActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAcceleratorKeyActivated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreClosestInteractiveBoundsRequested(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreClosestInteractiveBoundsRequested {
    type Vtable = ICoreClosestInteractiveBoundsRequested_Vtbl;
}
impl ::core::clone::Clone for ICoreClosestInteractiveBoundsRequested {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreClosestInteractiveBoundsRequested {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf303043a_e8bf_4e8e_ae69_c9dadd57a114);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreClosestInteractiveBoundsRequested_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClosestInteractiveBoundsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosestInteractiveBoundsRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreComponentFocusable(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreComponentFocusable {
    type Vtable = ICoreComponentFocusable_Vtbl;
}
impl ::core::clone::Clone for ICoreComponentFocusable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreComponentFocusable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52f96fa3_8742_4411_ae69_79a85f29ac8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreComponentFocusable_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreCursor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreCursor {
    type Vtable = ICoreCursor_Vtbl;
}
impl ::core::clone::Clone for ICoreCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreCursor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96893acf_111d_442c_8a77_b87992f8e2d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreCursor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreCursorType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreCursorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreCursorFactory {
    type Vtable = ICoreCursorFactory_Vtbl;
}
impl ::core::clone::Clone for ICoreCursorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreCursorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6359621_a79d_4ed3_8c32_a9ef9d6b76a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreCursorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreCursorType, id: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDispatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreDispatcher {
    type Vtable = ICoreDispatcher_Vtbl;
}
impl ::core::clone::Clone for ICoreDispatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreDispatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60db2fa8_b705_4fde_a7d6_ebbb1891d39e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDispatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ProcessEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: CoreProcessEventsOption) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: CoreDispatcherPriority, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunIdleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunIdleAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDispatcher2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreDispatcher2 {
    type Vtable = ICoreDispatcher2_Vtbl;
}
impl ::core::clone::Clone for ICoreDispatcher2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreDispatcher2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f5e63c7_e3aa_4eae_b0e0_dcf321ca4b2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDispatcher2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryRunAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: CoreDispatcherPriority, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRunAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRunIdleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRunIdleAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDispatcherWithTaskPriority(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreDispatcherWithTaskPriority {
    type Vtable = ICoreDispatcherWithTaskPriority_Vtbl;
}
impl ::core::clone::Clone for ICoreDispatcherWithTaskPriority {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreDispatcherWithTaskPriority {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafaecad_484d_41be_ba80_1d58c65263ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDispatcherWithTaskPriority_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CurrentPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreDispatcherPriority) -> ::windows::core::HRESULT,
    pub SetCurrentPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreDispatcherPriority) -> ::windows::core::HRESULT,
    pub ShouldYield: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShouldYieldToPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: CoreDispatcherPriority, result__: *mut bool) -> ::windows::core::HRESULT,
    pub StopProcessEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreIndependentInputSourceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreIndependentInputSourceController {
    type Vtable = ICoreIndependentInputSourceController_Vtbl;
}
impl ::core::clone::Clone for ICoreIndependentInputSourceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreIndependentInputSourceController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0963261c_84fe_578a_83ca_6425309ccde4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIndependentInputSourceController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsTransparentForUncontrolledInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsTransparentForUncontrolledInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPalmRejectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPalmRejectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetControlledInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputtypes: CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    pub SetControlledInputWithFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreIndependentInputSourceControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreIndependentInputSourceControllerStatics {
    type Vtable = ICoreIndependentInputSourceControllerStatics_Vtbl;
}
impl ::core::clone::Clone for ICoreIndependentInputSourceControllerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreIndependentInputSourceControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3edc4e20_9a8a_5691_8586_fca4cb57526d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIndependentInputSourceControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateForIVisualElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForIVisualElement: usize,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICoreInputSourceBase(::windows::core::IUnknown);
impl ICoreInputSourceBase {
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreDispatcher>();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsInputEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInputEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).InputEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputEnabled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICoreInputSourceBase, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ICoreInputSourceBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreInputSourceBase {}
impl ::core::fmt::Debug for ICoreInputSourceBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreInputSourceBase").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICoreInputSourceBase {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{9f488807-4580-4be8-be68-92a9311713bb}");
}
unsafe impl ::windows::core::Interface for ICoreInputSourceBase {
    type Vtable = ICoreInputSourceBase_Vtbl;
}
impl ::core::clone::Clone for ICoreInputSourceBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreInputSourceBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f488807_4580_4be8_be68_92a9311713bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputSourceBase_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputEnabled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreKeyboardInputSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreKeyboardInputSource {
    type Vtable = ICoreKeyboardInputSource_Vtbl;
}
impl ::core::clone::Clone for ICoreKeyboardInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreKeyboardInputSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x231c9088_e469_4df1_b208_6e490d71cb90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreKeyboardInputSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetCurrentKeyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetCurrentKeyState: usize,
    #[cfg(feature = "Foundation")]
    pub CharacterReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCharacterReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreKeyboardInputSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreKeyboardInputSource2 {
    type Vtable = ICoreKeyboardInputSource2_Vtbl;
}
impl ::core::clone::Clone for ICoreKeyboardInputSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreKeyboardInputSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa24cb94_f963_47a5_8778_207c482b0afd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreKeyboardInputSource2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentKeyEventDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICorePointerInputSource(::windows::core::IUnknown);
impl ICorePointerInputSource {
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasCapture)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).PointerCursor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerCursor(&self, value: &CoreCursor) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCursor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerEntered)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerExited(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerExited)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerPressed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICorePointerInputSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ICorePointerInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorePointerInputSource {}
impl ::core::fmt::Debug for ICorePointerInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorePointerInputSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICorePointerInputSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{bbf1bb18-e47a-48eb-8807-f8f8d3ea4551}");
}
unsafe impl ::windows::core::Interface for ICorePointerInputSource {
    type Vtable = ICorePointerInputSource_Vtbl;
}
impl ::core::clone::Clone for ICorePointerInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorePointerInputSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf1bb18_e47a_48eb_8807_f8f8d3ea4551);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePointerInputSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReleasePointerCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPointerCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PointerPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPosition: usize,
    pub PointerCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPointerCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PointerCaptureLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerCaptureLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub PointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub PointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub PointerMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub PointerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PointerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub PointerWheelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerWheelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerWheelChanged: usize,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICorePointerInputSource2(::windows::core::IUnknown);
impl ICorePointerInputSource2 {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::DispatcherQueue>();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasCapture)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).PointerCursor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerCursor(&self, value: &CoreCursor) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCursor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerEntered)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerExited(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerExited)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerPressed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICorePointerInputSource2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<ICorePointerInputSource> for ICorePointerInputSource2 {}
impl ::core::cmp::PartialEq for ICorePointerInputSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorePointerInputSource2 {}
impl ::core::fmt::Debug for ICorePointerInputSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorePointerInputSource2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICorePointerInputSource2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d703708a-4516-4786-b1e5-2751d563f997}");
}
unsafe impl ::windows::core::Interface for ICorePointerInputSource2 {
    type Vtable = ICorePointerInputSource2_Vtbl;
}
impl ::core::clone::Clone for ICorePointerInputSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorePointerInputSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd703708a_4516_4786_b1e5_2751d563f997);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePointerInputSource2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICorePointerRedirector(::windows::core::IUnknown);
impl ICorePointerRedirector {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedAway(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedAway)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedAway(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedAway)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedTo(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedTo(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedTo)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedReleased(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICorePointerRedirector, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ICorePointerRedirector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorePointerRedirector {}
impl ::core::fmt::Debug for ICorePointerRedirector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorePointerRedirector").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICorePointerRedirector {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{8f9d0c94-5688-4b0c-a9f1-f931f7fa3dc3}");
}
unsafe impl ::windows::core::Interface for ICorePointerRedirector {
    type Vtable = ICorePointerRedirector_Vtbl;
}
impl ::core::clone::Clone for ICorePointerRedirector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorePointerRedirector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f9d0c94_5688_4b0c_a9f1_f931f7fa3dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePointerRedirector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PointerRoutedAway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerRoutedAway: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerRoutedAway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerRoutedAway: usize,
    #[cfg(feature = "Foundation")]
    pub PointerRoutedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerRoutedTo: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerRoutedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerRoutedTo: usize,
    #[cfg(feature = "Foundation")]
    pub PointerRoutedReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerRoutedReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerRoutedReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerRoutedReleased: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTouchHitTesting(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTouchHitTesting {
    type Vtable = ICoreTouchHitTesting_Vtbl;
}
impl ::core::clone::Clone for ICoreTouchHitTesting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreTouchHitTesting {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1d8a289_3acf_4124_9fa3_ea8aba353c21);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTouchHitTesting_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TouchHitTesting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TouchHitTesting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTouchHitTesting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTouchHitTesting: usize,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICoreWindow(::windows::core::IUnknown);
impl ICoreWindow {
    pub fn AutomationHostProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).AutomationHostProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).Bounds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).CustomProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreDispatcher>();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<CoreWindowFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowFlowDirection>();
            (::windows::core::Interface::vtable(this).FlowDirection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFlowDirection)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsInputEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInputEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).PointerCursor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerCursor(&self, value: &CoreCursor) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCursor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Visible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Activate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Activate)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreVirtualKeyStates>();
            (::windows::core::Interface::vtable(this).GetAsyncKeyState)(::windows::core::Interface::as_raw(this), virtualkey, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreVirtualKeyStates>();
            (::windows::core::Interface::vtable(this).GetKeyState)(::windows::core::Interface::as_raw(this), virtualkey, &mut result__).from_abi(result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Activated(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Activated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActivated)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutomationProviderRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AutomationProviderRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutomationProviderRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAutomationProviderRequested)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CharacterReceived(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CharacterReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCharacterReceived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCharacterReceived)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).InputEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputEnabled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyDown(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).KeyDown)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyDown)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyUp(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).KeyUp)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyUp)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerEntered)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerExited(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerExited)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerPressed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TouchHitTesting(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TouchHitTesting)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTouchHitTesting(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTouchHitTesting)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SizeChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSizeChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).VisibilityChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVisibilityChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICoreWindow, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ICoreWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindow {}
impl ::core::fmt::Debug for ICoreWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindow").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICoreWindow {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{79b9d5f2-879e-4b89-b798-79e47598030c}");
}
unsafe impl ::windows::core::Interface for ICoreWindow {
    type Vtable = ICoreWindow_Vtbl;
}
impl ::core::clone::Clone for ICoreWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79b9d5f2_879e_4b89_b798_79e47598030c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AutomationHostProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowFlowDirection) -> ::windows::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreWindowFlowDirection) -> ::windows::core::HRESULT,
    pub IsInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PointerCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPointerCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PointerPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPosition: usize,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetAsyncKeyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetAsyncKeyState: usize,
    #[cfg(feature = "System")]
    pub GetKeyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetKeyState: usize,
    pub ReleasePointerCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPointerCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(feature = "Foundation")]
    pub AutomationProviderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutomationProviderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutomationProviderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutomationProviderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CharacterReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCharacterReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub InputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub PointerCaptureLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerCaptureLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub PointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub PointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub PointerMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub PointerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PointerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub TouchHitTesting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TouchHitTesting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTouchHitTesting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTouchHitTesting: usize,
    #[cfg(feature = "Foundation")]
    pub PointerWheelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerWheelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibilityChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindow2 {
    type Vtable = ICoreWindow2_Vtbl;
}
impl ::core::clone::Clone for ICoreWindow2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindow2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c2b1b85_6917_4361_9c02_0d9e3a420b95);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetPointerPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPointerPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindow3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindow3 {
    type Vtable = ICoreWindow3_Vtbl;
}
impl ::core::clone::Clone for ICoreWindow3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindow3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32c20dd8_faef_4375_a2ab_32640e4815c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClosestInteractiveBoundsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosestInteractiveBoundsRequested: usize,
    pub GetCurrentKeyEventDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindow4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindow4 {
    type Vtable = ICoreWindow4_Vtbl;
}
impl ::core::clone::Clone for ICoreWindow4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindow4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35caf0d0_47f0_436c_af97_0dd88f6f5f02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ResizeStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResizeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResizeStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResizeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ResizeCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResizeCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResizeCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResizeCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindow5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindow5 {
    type Vtable = ICoreWindow5_Vtbl;
}
impl ::core::clone::Clone for ICoreWindow5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindow5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b4ae1e1_2e6d_4eaa_bda1_1c5cc1bee141);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub ActivationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowActivationMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowDialog(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowDialog {
    type Vtable = ICoreWindowDialog_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowDialog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7392ce0_c78d_427e_8b2c_01ff420c69d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowDialog_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSize: usize,
    #[cfg(feature = "Foundation")]
    pub MinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSize: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsInteractionDelayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetIsInteractionDelayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub BackButtonCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    BackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetBackButtonCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetBackButtonCommand: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowDialogFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowDialogFactory {
    type Vtable = ICoreWindowDialogFactory_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowDialogFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowDialogFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfb2a855_1c59_4b13_b1e5_16e29805f7c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowDialogFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ICoreWindowEventArgs(::windows::core::IUnknown);
impl ICoreWindowEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICoreWindowEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ICoreWindowEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowEventArgs {}
impl ::core::fmt::Debug for ICoreWindowEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICoreWindowEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{272b1ef3-c633-4da5-a26c-c6d0f56b29da}");
}
unsafe impl ::windows::core::Interface for ICoreWindowEventArgs {
    type Vtable = ICoreWindowEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272b1ef3_c633_4da5_a26c_c6d0f56b29da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowFlyout(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowFlyout {
    type Vtable = ICoreWindowFlyout_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowFlyout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowFlyout {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe89d854d_2050_40bb_b344_f6f355eeb314);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowFlyout_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSize: usize,
    #[cfg(feature = "Foundation")]
    pub MinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSize: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsInteractionDelayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetIsInteractionDelayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub BackButtonCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    BackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetBackButtonCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetBackButtonCommand: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowFlyoutFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowFlyoutFactory {
    type Vtable = ICoreWindowFlyoutFactory_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowFlyoutFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowFlyoutFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdec4c6c4_93e8_4f7c_be27_cefaa1af68a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowFlyoutFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Point, title: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithTitle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowPopupShowingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowPopupShowingEventArgs {
    type Vtable = ICoreWindowPopupShowingEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowPopupShowingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowPopupShowingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26155fa2_5ba5_4ea4_a3b4_2dc7d63c8e26);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowPopupShowingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetDesiredSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowResizeManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowResizeManager {
    type Vtable = ICoreWindowResizeManager_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowResizeManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowResizeManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b925_b350_48b3_a198_5c1a84700243);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowResizeManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NotifyLayoutCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowResizeManagerLayoutCapability(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowResizeManagerLayoutCapability {
    type Vtable = ICoreWindowResizeManagerLayoutCapability_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowResizeManagerLayoutCapability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowResizeManagerLayoutCapability {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb74f27b_a544_4301_80e6_0ae033ef4536);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowResizeManagerLayoutCapability_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetShouldWaitForLayoutCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShouldWaitForLayoutCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowResizeManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowResizeManagerStatics {
    type Vtable = ICoreWindowResizeManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowResizeManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowResizeManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae4a9045_6d70_49db_8e68_46ffbd17d38d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowResizeManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowStatic {
    type Vtable = ICoreWindowStatic_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowStatic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowStatic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d239005_3c2a_41b1_9022_536bb9cf93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowStatic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowWithContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowWithContext {
    type Vtable = ICoreWindowWithContext_Vtbl;
}
impl ::core::clone::Clone for ICoreWindowWithContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoreWindowWithContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ac40241_3575_4c3b_af66_e8c529d4d06c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowWithContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIdleDispatchedHandlerArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIdleDispatchedHandlerArgs {
    type Vtable = IIdleDispatchedHandlerArgs_Vtbl;
}
impl ::core::clone::Clone for IIdleDispatchedHandlerArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IIdleDispatchedHandlerArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98bb6a24_dc1c_43cb_b4ed_d1c0eb2391f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdleDispatchedHandlerArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsDispatcherIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct IInitializeWithCoreWindow(::windows::core::IUnknown);
impl IInitializeWithCoreWindow {
    pub fn Initialize(&self, window: &CoreWindow) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Initialize)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(window)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IInitializeWithCoreWindow, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IInitializeWithCoreWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithCoreWindow {}
impl ::core::fmt::Debug for IInitializeWithCoreWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithCoreWindow").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IInitializeWithCoreWindow {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{188f20d6-9873-464a-ace5-57e010f465e6}");
}
unsafe impl ::windows::core::Interface for IInitializeWithCoreWindow {
    type Vtable = IInitializeWithCoreWindow_Vtbl;
}
impl ::core::clone::Clone for IInitializeWithCoreWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInitializeWithCoreWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x188f20d6_9873_464a_ace5_57e010f465e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithCoreWindow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputEnabledEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputEnabledEventArgs {
    type Vtable = IInputEnabledEventArgs_Vtbl;
}
impl ::core::clone::Clone for IInputEnabledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInputEnabledEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80371d4f_2fd8_4c24_aa86_3163a87b4e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputEnabledEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyEventArgs {
    type Vtable = IKeyEventArgs_Vtbl;
}
impl ::core::clone::Clone for IKeyEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IKeyEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ff5e930_2544_4a17_bd78_1f2fdebb106b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyEventArgs2 {
    type Vtable = IKeyEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IKeyEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IKeyEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x583add98_0790_4571_9b12_645ef9d79e42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPointerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPointerEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x920d9cb1_a5fc_4a21_8c09_49dfe6ffe25f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    CurrentPoint: usize,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemNavigationManager {
    type Vtable = ISystemNavigationManager_Vtbl;
}
impl ::core::clone::Clone for ISystemNavigationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemNavigationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93023118_cf50_42a6_9706_69107fa122e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BackRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemNavigationManager2 {
    type Vtable = ISystemNavigationManager2_Vtbl;
}
impl ::core::clone::Clone for ISystemNavigationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemNavigationManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c510401_67be_49ae_9509_671c1e54a389);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppViewBackButtonVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppViewBackButtonVisibility) -> ::windows::core::HRESULT,
    pub SetAppViewBackButtonVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppViewBackButtonVisibility) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemNavigationManagerStatics {
    type Vtable = ISystemNavigationManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemNavigationManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemNavigationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc52b5ce_bee0_4305_8c54_68228ed683b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITouchHitTestingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITouchHitTestingEventArgs {
    type Vtable = ITouchHitTestingEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITouchHitTestingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITouchHitTestingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f3b823_0b7c_424e_9df7_33d4f962931b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITouchHitTestingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ProximityEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProximityEvaluation: usize,
    #[cfg(feature = "Foundation")]
    pub SetProximityEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProximityEvaluation: usize,
    #[cfg(feature = "Foundation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point: usize,
    #[cfg(feature = "Foundation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingBox: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateProximityToRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlboundingbox: super::super::Foundation::Rect, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateProximityToRect: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateProximityToPolygon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlVertices_array_size: u32, controlvertices: *const super::super::Foundation::Point, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateProximityToPolygon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisibilityChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisibilityChangedEventArgs {
    type Vtable = IVisibilityChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVisibilityChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf9918ea_d801_4564_a495_b1e84f8ad085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisibilityChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowActivatedEventArgs {
    type Vtable = IWindowActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWindowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x179d65e7_4658_4cb6_aa13_41d094ea255e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WindowActivationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowActivationState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowSizeChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowSizeChangedEventArgs {
    type Vtable = IWindowSizeChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWindowSizeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowSizeChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a200ec7_0426_47dc_b86c_6f475915e451);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowSizeChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct AcceleratorKeyEventArgs(::windows::core::IUnknown);
impl AcceleratorKeyEventArgs {
    pub fn EventType(&self) -> ::windows::core::Result<CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreAcceleratorKeyEventType>();
            (::windows::core::Interface::vtable(this).EventType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::VirtualKey>();
            (::windows::core::Interface::vtable(this).VirtualKey)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CorePhysicalKeyStatus>();
            (::windows::core::Interface::vtable(this).KeyStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAcceleratorKeyEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AcceleratorKeyEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AcceleratorKeyEventArgs {}
impl ::core::fmt::Debug for AcceleratorKeyEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AcceleratorKeyEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AcceleratorKeyEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AcceleratorKeyEventArgs;{ff1c4c4a-9287-470b-836e-9086e3126ade})");
}
impl ::core::clone::Clone for AcceleratorKeyEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AcceleratorKeyEventArgs {
    type Vtable = IAcceleratorKeyEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AcceleratorKeyEventArgs {
    const IID: ::windows::core::GUID = <IAcceleratorKeyEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AcceleratorKeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.AcceleratorKeyEventArgs";
}
::windows::imp::interface_hierarchy!(AcceleratorKeyEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for AcceleratorKeyEventArgs {}
unsafe impl ::core::marker::Send for AcceleratorKeyEventArgs {}
unsafe impl ::core::marker::Sync for AcceleratorKeyEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct AutomationProviderRequestedEventArgs(::windows::core::IUnknown);
impl AutomationProviderRequestedEventArgs {
    pub fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).AutomationProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutomationProvider<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutomationProvider)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AutomationProviderRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationProviderRequestedEventArgs {}
impl ::core::fmt::Debug for AutomationProviderRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationProviderRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AutomationProviderRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AutomationProviderRequestedEventArgs;{961ff258-21bf-4b42-a298-fa479d4c52e2})");
}
impl ::core::clone::Clone for AutomationProviderRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AutomationProviderRequestedEventArgs {
    type Vtable = IAutomationProviderRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AutomationProviderRequestedEventArgs {
    const IID: ::windows::core::GUID = <IAutomationProviderRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AutomationProviderRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.AutomationProviderRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(AutomationProviderRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for AutomationProviderRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct BackRequestedEventArgs(::windows::core::IUnknown);
impl BackRequestedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for BackRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackRequestedEventArgs {}
impl ::core::fmt::Debug for BackRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.BackRequestedEventArgs;{d603d28a-e411-4a4e-ba41-6a327a8675bc})");
}
impl ::core::clone::Clone for BackRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackRequestedEventArgs {
    type Vtable = IBackRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackRequestedEventArgs {
    const IID: ::windows::core::GUID = <IBackRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.BackRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(BackRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BackRequestedEventArgs {}
unsafe impl ::core::marker::Sync for BackRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CharacterReceivedEventArgs(::windows::core::IUnknown);
impl CharacterReceivedEventArgs {
    pub fn KeyCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).KeyCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CorePhysicalKeyStatus>();
            (::windows::core::Interface::vtable(this).KeyStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CharacterReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterReceivedEventArgs {}
impl ::core::fmt::Debug for CharacterReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CharacterReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CharacterReceivedEventArgs;{c584659f-99b2-4bcc-bd33-04e63f42902e})");
}
impl ::core::clone::Clone for CharacterReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CharacterReceivedEventArgs {
    const IID: ::windows::core::GUID = <ICharacterReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CharacterReceivedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CharacterReceivedEventArgs";
}
::windows::imp::interface_hierarchy!(CharacterReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for CharacterReceivedEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct ClosestInteractiveBoundsRequestedEventArgs(::windows::core::IUnknown);
impl ClosestInteractiveBoundsRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).SearchBounds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClosestInteractiveBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).ClosestInteractiveBounds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetClosestInteractiveBounds(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClosestInteractiveBounds)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ClosestInteractiveBoundsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClosestInteractiveBoundsRequestedEventArgs {}
impl ::core::fmt::Debug for ClosestInteractiveBoundsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosestInteractiveBoundsRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ClosestInteractiveBoundsRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs;{347c11d7-f6f8-40e3-b29f-ae50d3e86486})");
}
impl ::core::clone::Clone for ClosestInteractiveBoundsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ClosestInteractiveBoundsRequestedEventArgs {
    type Vtable = IClosestInteractiveBoundsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ClosestInteractiveBoundsRequestedEventArgs {
    const IID: ::windows::core::GUID = <IClosestInteractiveBoundsRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ClosestInteractiveBoundsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(ClosestInteractiveBoundsRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreAcceleratorKeys(::windows::core::IUnknown);
impl CoreAcceleratorKeys {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyActivated(&self, handler: &super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AcceleratorKeyActivated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAcceleratorKeyActivated)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreAcceleratorKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAcceleratorKeys {}
impl ::core::fmt::Debug for CoreAcceleratorKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAcceleratorKeys").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreAcceleratorKeys {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreAcceleratorKeys;{9ffdf7f5-b8c9-4ef0-b7d2-1de626561fc8})");
}
impl ::core::clone::Clone for CoreAcceleratorKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreAcceleratorKeys {
    type Vtable = ICoreAcceleratorKeys_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreAcceleratorKeys {
    const IID: ::windows::core::GUID = <ICoreAcceleratorKeys as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.CoreAcceleratorKeys";
}
::windows::imp::interface_hierarchy!(CoreAcceleratorKeys, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreAcceleratorKeys> for CoreAcceleratorKeys {}
unsafe impl ::core::marker::Send for CoreAcceleratorKeys {}
unsafe impl ::core::marker::Sync for CoreAcceleratorKeys {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreComponentInputSource(::windows::core::IUnknown);
impl CoreComponentInputSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClosestInteractiveBoundsRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreClosestInteractiveBoundsRequested>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ClosestInteractiveBoundsRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosestInteractiveBoundsRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreClosestInteractiveBoundsRequested>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosestInteractiveBoundsRequested)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn HasFocus(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasFocus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).GotFocus)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreComponentFocusable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGotFocus)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).LostFocus)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreComponentFocusable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLostFocus)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreDispatcher>();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsInputEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInputEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).InputEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputEnabled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetCurrentKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreVirtualKeyStates>();
            (::windows::core::Interface::vtable(this).GetCurrentKeyState)(::windows::core::Interface::as_raw(this), virtualkey, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CharacterReceived(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CharacterReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CharacterReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCharacterReceived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCharacterReceived)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyDown(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).KeyDown)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyDown)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyUp(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).KeyUp)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyUp)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ICoreKeyboardInputSource2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetCurrentKeyEventDeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasCapture)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).PointerCursor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerCursor(&self, value: &CoreCursor) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCursor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerEntered)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerExited(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerExited)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerPressed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::DispatcherQueue>();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TouchHitTesting(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TouchHitTestingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreTouchHitTesting>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TouchHitTesting)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTouchHitTesting(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreTouchHitTesting>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTouchHitTesting)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreComponentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreComponentInputSource {}
impl ::core::fmt::Debug for CoreComponentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreComponentInputSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreComponentInputSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreComponentInputSource;{9f488807-4580-4be8-be68-92a9311713bb})");
}
impl ::core::clone::Clone for CoreComponentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreComponentInputSource {
    type Vtable = ICoreInputSourceBase_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreComponentInputSource {
    const IID: ::windows::core::GUID = <ICoreInputSourceBase as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreComponentInputSource {
    const NAME: &'static str = "Windows.UI.Core.CoreComponentInputSource";
}
::windows::imp::interface_hierarchy!(CoreComponentInputSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreInputSourceBase> for CoreComponentInputSource {}
impl ::windows::core::CanTryInto<ICorePointerInputSource> for CoreComponentInputSource {}
impl ::windows::core::CanTryInto<ICorePointerInputSource2> for CoreComponentInputSource {}
unsafe impl ::core::marker::Send for CoreComponentInputSource {}
unsafe impl ::core::marker::Sync for CoreComponentInputSource {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreCursor(::windows::core::IUnknown);
impl CoreCursor {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<CoreCursorType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursorType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateCursor(r#type: CoreCursorType, id: u32) -> ::windows::core::Result<CoreCursor> {
        Self::ICoreCursorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).CreateCursor)(::windows::core::Interface::as_raw(this), r#type, id, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreCursorFactory<R, F: FnOnce(&ICoreCursorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreCursor, ICoreCursorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreCursor {}
impl ::core::fmt::Debug for CoreCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreCursor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreCursor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreCursor;{96893acf-111d-442c-8a77-b87992f8e2d6})");
}
impl ::core::clone::Clone for CoreCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreCursor {
    type Vtable = ICoreCursor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreCursor {
    const IID: ::windows::core::GUID = <ICoreCursor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreCursor {
    const NAME: &'static str = "Windows.UI.Core.CoreCursor";
}
::windows::imp::interface_hierarchy!(CoreCursor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreCursor {}
unsafe impl ::core::marker::Sync for CoreCursor {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreDispatcher(::windows::core::IUnknown);
impl CoreDispatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyActivated(&self, handler: &super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreAcceleratorKeys>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AcceleratorKeyActivated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreAcceleratorKeys>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAcceleratorKeyActivated)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn HasThreadAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasThreadAccess)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessEvents(&self, options: CoreProcessEventsOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessEvents)(::windows::core::Interface::as_raw(this), options).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunAsync(&self, priority: CoreDispatcherPriority, agilecallback: &DispatchedHandler) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).RunAsync)(::windows::core::Interface::as_raw(this), priority, ::core::mem::transmute_copy(agilecallback), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunIdleAsync(&self, agilecallback: &IdleDispatchedHandler) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).RunIdleAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(agilecallback), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRunAsync(&self, priority: CoreDispatcherPriority, agilecallback: &DispatchedHandler) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcher2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryRunAsync)(::windows::core::Interface::as_raw(this), priority, ::core::mem::transmute_copy(agilecallback), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRunIdleAsync(&self, agilecallback: &IdleDispatchedHandler) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcher2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryRunIdleAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(agilecallback), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentPriority(&self) -> ::windows::core::Result<CoreDispatcherPriority> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreDispatcherPriority>();
            (::windows::core::Interface::vtable(this).CurrentPriority)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCurrentPriority(&self, value: CoreDispatcherPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCurrentPriority)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShouldYield(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ShouldYield)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldYieldToPriority(&self, priority: CoreDispatcherPriority) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ShouldYieldToPriority)(::windows::core::Interface::as_raw(this), priority, &mut result__).from_abi(result__)
        }
    }
    pub fn StopProcessEvents(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopProcessEvents)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreDispatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDispatcher {}
impl ::core::fmt::Debug for CoreDispatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDispatcher").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreDispatcher {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreDispatcher;{60db2fa8-b705-4fde-a7d6-ebbb1891d39e})");
}
impl ::core::clone::Clone for CoreDispatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreDispatcher {
    type Vtable = ICoreDispatcher_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreDispatcher {
    const IID: ::windows::core::GUID = <ICoreDispatcher as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreDispatcher {
    const NAME: &'static str = "Windows.UI.Core.CoreDispatcher";
}
::windows::imp::interface_hierarchy!(CoreDispatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreAcceleratorKeys> for CoreDispatcher {}
unsafe impl ::core::marker::Send for CoreDispatcher {}
unsafe impl ::core::marker::Sync for CoreDispatcher {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreIndependentInputSource(::windows::core::IUnknown);
impl CoreIndependentInputSource {
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreDispatcher>();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsInputEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInputEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).InputEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputEnabled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasCapture)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).PointerCursor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerCursor(&self, value: &CoreCursor) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCursor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerEntered)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerExited(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerExited)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerPressed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerInputSource2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::DispatcherQueue>();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedAway(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedAway)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedAway(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedAway)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedTo(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedTo(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedTo)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedReleased(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreIndependentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreIndependentInputSource {}
impl ::core::fmt::Debug for CoreIndependentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIndependentInputSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreIndependentInputSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreIndependentInputSource;{9f488807-4580-4be8-be68-92a9311713bb})");
}
impl ::core::clone::Clone for CoreIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreIndependentInputSource {
    type Vtable = ICoreInputSourceBase_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreIndependentInputSource {
    const IID: ::windows::core::GUID = <ICoreInputSourceBase as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Core.CoreIndependentInputSource";
}
::windows::imp::interface_hierarchy!(CoreIndependentInputSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreInputSourceBase> for CoreIndependentInputSource {}
impl ::windows::core::CanTryInto<ICorePointerInputSource> for CoreIndependentInputSource {}
impl ::windows::core::CanTryInto<ICorePointerInputSource2> for CoreIndependentInputSource {}
impl ::windows::core::CanTryInto<ICorePointerRedirector> for CoreIndependentInputSource {}
unsafe impl ::core::marker::Send for CoreIndependentInputSource {}
unsafe impl ::core::marker::Sync for CoreIndependentInputSource {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreIndependentInputSourceController(::windows::core::IUnknown);
impl CoreIndependentInputSourceController {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn IsTransparentForUncontrolledInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsTransparentForUncontrolledInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsTransparentForUncontrolledInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTransparentForUncontrolledInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPalmRejectionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPalmRejectionEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPalmRejectionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPalmRejectionEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<CoreIndependentInputSource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreIndependentInputSource>();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetControlledInput(&self, inputtypes: CoreInputDeviceTypes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetControlledInput)(::windows::core::Interface::as_raw(this), inputtypes).ok() }
    }
    pub fn SetControlledInputWithFilters(&self, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetControlledInputWithFilters)(::windows::core::Interface::as_raw(this), inputtypes, required, excluded).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisual<P0>(visual: P0) -> ::windows::core::Result<CoreIndependentInputSourceController>
    where
        P0: ::windows::core::TryIntoParam<super::Composition::Visual>,
    {
        Self::ICoreIndependentInputSourceControllerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreIndependentInputSourceController>();
            (::windows::core::Interface::vtable(this).CreateForVisual)(::windows::core::Interface::as_raw(this), visual.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForIVisualElement<P0>(visualelement: P0) -> ::windows::core::Result<CoreIndependentInputSourceController>
    where
        P0: ::windows::core::TryIntoParam<super::Composition::IVisualElement>,
    {
        Self::ICoreIndependentInputSourceControllerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreIndependentInputSourceController>();
            (::windows::core::Interface::vtable(this).CreateForIVisualElement)(::windows::core::Interface::as_raw(this), visualelement.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreIndependentInputSourceControllerStatics<R, F: FnOnce(&ICoreIndependentInputSourceControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreIndependentInputSourceController, ICoreIndependentInputSourceControllerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreIndependentInputSourceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreIndependentInputSourceController {}
impl ::core::fmt::Debug for CoreIndependentInputSourceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIndependentInputSourceController").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreIndependentInputSourceController {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreIndependentInputSourceController;{0963261c-84fe-578a-83ca-6425309ccde4})");
}
impl ::core::clone::Clone for CoreIndependentInputSourceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreIndependentInputSourceController {
    type Vtable = ICoreIndependentInputSourceController_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreIndependentInputSourceController {
    const IID: ::windows::core::GUID = <ICoreIndependentInputSourceController as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreIndependentInputSourceController {
    const NAME: &'static str = "Windows.UI.Core.CoreIndependentInputSourceController";
}
::windows::imp::interface_hierarchy!(CoreIndependentInputSourceController, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for CoreIndependentInputSourceController {}
unsafe impl ::core::marker::Send for CoreIndependentInputSourceController {}
unsafe impl ::core::marker::Sync for CoreIndependentInputSourceController {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindow(::windows::core::IUnknown);
impl CoreWindow {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedAway(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedAway)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedAway(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedAway)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedTo(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedTo(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedTo)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedReleased(&self, handler: &super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerRoutedReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerRoutedReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AutomationHostProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).AutomationHostProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).Bounds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).CustomProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreDispatcher>();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<CoreWindowFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowFlowDirection>();
            (::windows::core::Interface::vtable(this).FlowDirection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFlowDirection)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsInputEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInputEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreCursor>();
            (::windows::core::Interface::vtable(this).PointerCursor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerCursor(&self, value: &CoreCursor) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCursor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).PointerPosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Visible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Activate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Activate)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreVirtualKeyStates>();
            (::windows::core::Interface::vtable(this).GetAsyncKeyState)(::windows::core::Interface::as_raw(this), virtualkey, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreVirtualKeyStates>();
            (::windows::core::Interface::vtable(this).GetKeyState)(::windows::core::Interface::as_raw(this), virtualkey, &mut result__).from_abi(result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerCapture)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Activated(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Activated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActivated)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutomationProviderRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AutomationProviderRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutomationProviderRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAutomationProviderRequested)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CharacterReceived(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CharacterReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCharacterReceived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCharacterReceived)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).InputEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputEnabled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyDown(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).KeyDown)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyDown)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyUp(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).KeyUp)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyUp)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerEntered)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerExited(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerExited)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerPressed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerReleased)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerReleased)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TouchHitTesting(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TouchHitTesting)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTouchHitTesting(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTouchHitTesting)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SizeChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSizeChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).VisibilityChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVisibilityChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPointerPosition(&self, value: super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPointerPosition)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClosestInteractiveBoundsRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ClosestInteractiveBoundsRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosestInteractiveBoundsRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosestInteractiveBoundsRequested)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetCurrentKeyEventDeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResizeStarted(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ResizeStarted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResizeStarted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveResizeStarted)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResizeCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ResizeCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResizeCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveResizeCompleted)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::DispatcherQueue>();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivationMode(&self) -> ::windows::core::Result<CoreWindowActivationMode> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindow5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowActivationMode>();
            (::windows::core::Interface::vtable(this).ActivationMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetForCurrentThread() -> ::windows::core::Result<CoreWindow> {
        Self::ICoreWindowStatic(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindow>();
            (::windows::core::Interface::vtable(this).GetForCurrentThread)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UIContext(&self) -> ::windows::core::Result<super::UIContext> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowWithContext>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::UIContext>();
            (::windows::core::Interface::vtable(this).UIContext)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICoreWindowStatic<R, F: FnOnce(&ICoreWindowStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreWindow, ICoreWindowStatic> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindow {}
impl ::core::fmt::Debug for CoreWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindow").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindow {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindow;{79b9d5f2-879e-4b89-b798-79e47598030c})");
}
impl ::core::clone::Clone for CoreWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreWindow {
    type Vtable = ICoreWindow_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreWindow {
    const IID: ::windows::core::GUID = <ICoreWindow as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindow {
    const NAME: &'static str = "Windows.UI.Core.CoreWindow";
}
::windows::imp::interface_hierarchy!(CoreWindow, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICorePointerRedirector> for CoreWindow {}
impl ::windows::core::CanTryInto<ICoreWindow> for CoreWindow {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowDialog(::windows::core::IUnknown);
impl CoreWindowDialog {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreWindowDialog, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Showing(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Showing)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShowing(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShowing)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).MaxSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).MinSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).IsInteractionDelayed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInteractionDelayed)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>();
            (::windows::core::Interface::vtable(this).Commands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).DefaultCommandIndex)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultCommandIndex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).CancelCommandIndex)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancelCommandIndex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Popups::UICommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).BackButtonCommand)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetBackButtonCommand(&self, value: &super::Popups::UICommandInvokedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackButtonCommand)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>();
            (::windows::core::Interface::vtable(this).ShowAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWithTitle(title: &::windows::core::HSTRING) -> ::windows::core::Result<CoreWindowDialog> {
        Self::ICoreWindowDialogFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowDialog>();
            (::windows::core::Interface::vtable(this).CreateWithTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(title), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowDialogFactory<R, F: FnOnce(&ICoreWindowDialogFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreWindowDialog, ICoreWindowDialogFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreWindowDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowDialog {}
impl ::core::fmt::Debug for CoreWindowDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowDialog").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowDialog {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowDialog;{e7392ce0-c78d-427e-8b2c-01ff420c69d5})");
}
impl ::core::clone::Clone for CoreWindowDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreWindowDialog {
    type Vtable = ICoreWindowDialog_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreWindowDialog {
    const IID: ::windows::core::GUID = <ICoreWindowDialog as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowDialog {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowDialog";
}
::windows::imp::interface_hierarchy!(CoreWindowDialog, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowEventArgs(::windows::core::IUnknown);
impl CoreWindowEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreWindowEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowEventArgs {}
impl ::core::fmt::Debug for CoreWindowEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowEventArgs;{272b1ef3-c633-4da5-a26c-c6d0f56b29da})");
}
impl ::core::clone::Clone for CoreWindowEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreWindowEventArgs {
    type Vtable = ICoreWindowEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreWindowEventArgs {
    const IID: ::windows::core::GUID = <ICoreWindowEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowEventArgs";
}
::windows::imp::interface_hierarchy!(CoreWindowEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for CoreWindowEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowFlyout(::windows::core::IUnknown);
impl CoreWindowFlyout {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Showing(&self, handler: &super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Showing)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShowing(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShowing)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).MaxSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).MinSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).IsInteractionDelayed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInteractionDelayed)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>();
            (::windows::core::Interface::vtable(this).Commands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).DefaultCommandIndex)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultCommandIndex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Popups::UICommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).BackButtonCommand)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetBackButtonCommand(&self, value: &super::Popups::UICommandInvokedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackButtonCommand)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>();
            (::windows::core::Interface::vtable(this).ShowAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(position: super::super::Foundation::Point) -> ::windows::core::Result<CoreWindowFlyout> {
        Self::ICoreWindowFlyoutFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowFlyout>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithTitle(position: super::super::Foundation::Point, title: &::windows::core::HSTRING) -> ::windows::core::Result<CoreWindowFlyout> {
        Self::ICoreWindowFlyoutFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowFlyout>();
            (::windows::core::Interface::vtable(this).CreateWithTitle)(::windows::core::Interface::as_raw(this), position, ::core::mem::transmute_copy(title), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowFlyoutFactory<R, F: FnOnce(&ICoreWindowFlyoutFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreWindowFlyout, ICoreWindowFlyoutFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreWindowFlyout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowFlyout {}
impl ::core::fmt::Debug for CoreWindowFlyout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowFlyout").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowFlyout {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowFlyout;{e89d854d-2050-40bb-b344-f6f355eeb314})");
}
impl ::core::clone::Clone for CoreWindowFlyout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreWindowFlyout {
    type Vtable = ICoreWindowFlyout_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreWindowFlyout {
    const IID: ::windows::core::GUID = <ICoreWindowFlyout as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowFlyout {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowFlyout";
}
::windows::imp::interface_hierarchy!(CoreWindowFlyout, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowPopupShowingEventArgs(::windows::core::IUnknown);
impl CoreWindowPopupShowingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredSize(&self, value: super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreWindowPopupShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowPopupShowingEventArgs {}
impl ::core::fmt::Debug for CoreWindowPopupShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowPopupShowingEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowPopupShowingEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowPopupShowingEventArgs;{26155fa2-5ba5-4ea4-a3b4-2dc7d63c8e26})");
}
impl ::core::clone::Clone for CoreWindowPopupShowingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreWindowPopupShowingEventArgs {
    type Vtable = ICoreWindowPopupShowingEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreWindowPopupShowingEventArgs {
    const IID: ::windows::core::GUID = <ICoreWindowPopupShowingEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowPopupShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowPopupShowingEventArgs";
}
::windows::imp::interface_hierarchy!(CoreWindowPopupShowingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowResizeManager(::windows::core::IUnknown);
impl CoreWindowResizeManager {
    pub fn NotifyLayoutCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyLayoutCompleted)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetShouldWaitForLayoutCompletion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowResizeManagerLayoutCapability>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetShouldWaitForLayoutCompletion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShouldWaitForLayoutCompletion(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowResizeManagerLayoutCapability>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ShouldWaitForLayoutCompletion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreWindowResizeManager> {
        Self::ICoreWindowResizeManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowResizeManager>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowResizeManagerStatics<R, F: FnOnce(&ICoreWindowResizeManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CoreWindowResizeManager, ICoreWindowResizeManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreWindowResizeManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowResizeManager {}
impl ::core::fmt::Debug for CoreWindowResizeManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowResizeManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowResizeManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowResizeManager;{b8f0b925-b350-48b3-a198-5c1a84700243})");
}
impl ::core::clone::Clone for CoreWindowResizeManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CoreWindowResizeManager {
    type Vtable = ICoreWindowResizeManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CoreWindowResizeManager {
    const IID: ::windows::core::GUID = <ICoreWindowResizeManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowResizeManager {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowResizeManager";
}
::windows::imp::interface_hierarchy!(CoreWindowResizeManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreWindowResizeManager {}
unsafe impl ::core::marker::Sync for CoreWindowResizeManager {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct IdleDispatchedHandlerArgs(::windows::core::IUnknown);
impl IdleDispatchedHandlerArgs {
    pub fn IsDispatcherIdle(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDispatcherIdle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for IdleDispatchedHandlerArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IdleDispatchedHandlerArgs {}
impl ::core::fmt::Debug for IdleDispatchedHandlerArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IdleDispatchedHandlerArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IdleDispatchedHandlerArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.IdleDispatchedHandlerArgs;{98bb6a24-dc1c-43cb-b4ed-d1c0eb2391f3})");
}
impl ::core::clone::Clone for IdleDispatchedHandlerArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IdleDispatchedHandlerArgs {
    type Vtable = IIdleDispatchedHandlerArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for IdleDispatchedHandlerArgs {
    const IID: ::windows::core::GUID = <IIdleDispatchedHandlerArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for IdleDispatchedHandlerArgs {
    const NAME: &'static str = "Windows.UI.Core.IdleDispatchedHandlerArgs";
}
::windows::imp::interface_hierarchy!(IdleDispatchedHandlerArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct InputEnabledEventArgs(::windows::core::IUnknown);
impl InputEnabledEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).InputEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InputEnabledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputEnabledEventArgs {}
impl ::core::fmt::Debug for InputEnabledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputEnabledEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InputEnabledEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.InputEnabledEventArgs;{80371d4f-2fd8-4c24-aa86-3163a87b4e5a})");
}
impl ::core::clone::Clone for InputEnabledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InputEnabledEventArgs {
    type Vtable = IInputEnabledEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InputEnabledEventArgs {
    const IID: ::windows::core::GUID = <IInputEnabledEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InputEnabledEventArgs {
    const NAME: &'static str = "Windows.UI.Core.InputEnabledEventArgs";
}
::windows::imp::interface_hierarchy!(InputEnabledEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for InputEnabledEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct KeyEventArgs(::windows::core::IUnknown);
impl KeyEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::VirtualKey>();
            (::windows::core::Interface::vtable(this).VirtualKey)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CorePhysicalKeyStatus>();
            (::windows::core::Interface::vtable(this).KeyStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IKeyEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for KeyEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyEventArgs {}
impl ::core::fmt::Debug for KeyEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for KeyEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.KeyEventArgs;{5ff5e930-2544-4a17-bd78-1f2fdebb106b})");
}
impl ::core::clone::Clone for KeyEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for KeyEventArgs {
    type Vtable = IKeyEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for KeyEventArgs {
    const IID: ::windows::core::GUID = <IKeyEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for KeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.KeyEventArgs";
}
::windows::imp::interface_hierarchy!(KeyEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for KeyEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct PointerEventArgs(::windows::core::IUnknown);
impl PointerEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn CurrentPoint(&self) -> ::windows::core::Result<super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Input::PointerPoint>();
            (::windows::core::Interface::vtable(this).CurrentPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn KeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::VirtualKeyModifiers>();
            (::windows::core::Interface::vtable(this).KeyModifiers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>>();
            (::windows::core::Interface::vtable(this).GetIntermediatePoints)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PointerEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerEventArgs {}
impl ::core::fmt::Debug for PointerEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PointerEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.PointerEventArgs;{920d9cb1-a5fc-4a21-8c09-49dfe6ffe25f})");
}
impl ::core::clone::Clone for PointerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PointerEventArgs {
    const IID: ::windows::core::GUID = <IPointerEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Windows.UI.Core.PointerEventArgs";
}
::windows::imp::interface_hierarchy!(PointerEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for PointerEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct SystemNavigationManager(::windows::core::IUnknown);
impl SystemNavigationManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackRequested(&self, handler: &super::super::Foundation::EventHandler<BackRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).BackRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBackRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn AppViewBackButtonVisibility(&self) -> ::windows::core::Result<AppViewBackButtonVisibility> {
        let this = &::windows::core::ComInterface::cast::<ISystemNavigationManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppViewBackButtonVisibility>();
            (::windows::core::Interface::vtable(this).AppViewBackButtonVisibility)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppViewBackButtonVisibility(&self, value: AppViewBackButtonVisibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISystemNavigationManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAppViewBackButtonVisibility)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<SystemNavigationManager> {
        Self::ISystemNavigationManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SystemNavigationManager>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemNavigationManagerStatics<R, F: FnOnce(&ISystemNavigationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SystemNavigationManager, ISystemNavigationManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SystemNavigationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemNavigationManager {}
impl ::core::fmt::Debug for SystemNavigationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemNavigationManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SystemNavigationManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.SystemNavigationManager;{93023118-cf50-42a6-9706-69107fa122e1})");
}
impl ::core::clone::Clone for SystemNavigationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SystemNavigationManager {
    type Vtable = ISystemNavigationManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SystemNavigationManager {
    const IID: ::windows::core::GUID = <ISystemNavigationManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SystemNavigationManager {
    const NAME: &'static str = "Windows.UI.Core.SystemNavigationManager";
}
::windows::imp::interface_hierarchy!(SystemNavigationManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SystemNavigationManager {}
unsafe impl ::core::marker::Sync for SystemNavigationManager {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct TouchHitTestingEventArgs(::windows::core::IUnknown);
impl TouchHitTestingEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProximityEvaluation(&self) -> ::windows::core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreProximityEvaluation>();
            (::windows::core::Interface::vtable(this).ProximityEvaluation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProximityEvaluation(&self, value: CoreProximityEvaluation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProximityEvaluation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).Point)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingBox)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EvaluateProximityToRect(&self, controlboundingbox: super::super::Foundation::Rect) -> ::windows::core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreProximityEvaluation>();
            (::windows::core::Interface::vtable(this).EvaluateProximityToRect)(::windows::core::Interface::as_raw(this), controlboundingbox, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EvaluateProximityToPolygon(&self, controlvertices: &[super::super::Foundation::Point]) -> ::windows::core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreProximityEvaluation>();
            (::windows::core::Interface::vtable(this).EvaluateProximityToPolygon)(::windows::core::Interface::as_raw(this), controlvertices.len() as u32, controlvertices.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TouchHitTestingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TouchHitTestingEventArgs {}
impl ::core::fmt::Debug for TouchHitTestingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TouchHitTestingEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TouchHitTestingEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.TouchHitTestingEventArgs;{22f3b823-0b7c-424e-9df7-33d4f962931b})");
}
impl ::core::clone::Clone for TouchHitTestingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TouchHitTestingEventArgs {
    type Vtable = ITouchHitTestingEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TouchHitTestingEventArgs {
    const IID: ::windows::core::GUID = <ITouchHitTestingEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TouchHitTestingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.TouchHitTestingEventArgs";
}
::windows::imp::interface_hierarchy!(TouchHitTestingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for TouchHitTestingEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct VisibilityChangedEventArgs(::windows::core::IUnknown);
impl VisibilityChangedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Visible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VisibilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisibilityChangedEventArgs {}
impl ::core::fmt::Debug for VisibilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisibilityChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VisibilityChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.VisibilityChangedEventArgs;{bf9918ea-d801-4564-a495-b1e84f8ad085})");
}
impl ::core::clone::Clone for VisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VisibilityChangedEventArgs {
    type Vtable = IVisibilityChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VisibilityChangedEventArgs {
    const IID: ::windows::core::GUID = <IVisibilityChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.VisibilityChangedEventArgs";
}
::windows::imp::interface_hierarchy!(VisibilityChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for VisibilityChangedEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct WindowActivatedEventArgs(::windows::core::IUnknown);
impl WindowActivatedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn WindowActivationState(&self) -> ::windows::core::Result<CoreWindowActivationState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CoreWindowActivationState>();
            (::windows::core::Interface::vtable(this).WindowActivationState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WindowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowActivatedEventArgs {}
impl ::core::fmt::Debug for WindowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.WindowActivatedEventArgs;{179d65e7-4658-4cb6-aa13-41d094ea255e})");
}
impl ::core::clone::Clone for WindowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WindowActivatedEventArgs {
    type Vtable = IWindowActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WindowActivatedEventArgs {
    const IID: ::windows::core::GUID = <IWindowActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WindowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.WindowActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(WindowActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for WindowActivatedEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct WindowSizeChangedEventArgs(::windows::core::IUnknown);
impl WindowSizeChangedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WindowSizeChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowSizeChangedEventArgs {}
impl ::core::fmt::Debug for WindowSizeChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowSizeChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowSizeChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.WindowSizeChangedEventArgs;{5a200ec7-0426-47dc-b86c-6f475915e451})");
}
impl ::core::clone::Clone for WindowSizeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WindowSizeChangedEventArgs {
    type Vtable = IWindowSizeChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WindowSizeChangedEventArgs {
    const IID: ::windows::core::GUID = <IWindowSizeChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WindowSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.WindowSizeChangedEventArgs";
}
::windows::imp::interface_hierarchy!(WindowSizeChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ICoreWindowEventArgs> for WindowSizeChangedEventArgs {}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppViewBackButtonVisibility(pub i32);
impl AppViewBackButtonVisibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for AppViewBackButtonVisibility {}
impl ::core::clone::Clone for AppViewBackButtonVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppViewBackButtonVisibility {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AppViewBackButtonVisibility {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AppViewBackButtonVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppViewBackButtonVisibility").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppViewBackButtonVisibility {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AppViewBackButtonVisibility;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreAcceleratorKeyEventType(pub i32);
impl CoreAcceleratorKeyEventType {
    pub const Character: Self = Self(2i32);
    pub const DeadCharacter: Self = Self(3i32);
    pub const KeyDown: Self = Self(0i32);
    pub const KeyUp: Self = Self(1i32);
    pub const SystemCharacter: Self = Self(6i32);
    pub const SystemDeadCharacter: Self = Self(7i32);
    pub const SystemKeyDown: Self = Self(4i32);
    pub const SystemKeyUp: Self = Self(5i32);
    pub const UnicodeCharacter: Self = Self(8i32);
}
impl ::core::marker::Copy for CoreAcceleratorKeyEventType {}
impl ::core::clone::Clone for CoreAcceleratorKeyEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreAcceleratorKeyEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreAcceleratorKeyEventType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreAcceleratorKeyEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAcceleratorKeyEventType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreAcceleratorKeyEventType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreAcceleratorKeyEventType;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreCursorType(pub i32);
impl CoreCursorType {
    pub const Arrow: Self = Self(0i32);
    pub const Cross: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Hand: Self = Self(3i32);
    pub const Help: Self = Self(4i32);
    pub const IBeam: Self = Self(5i32);
    pub const SizeAll: Self = Self(6i32);
    pub const SizeNortheastSouthwest: Self = Self(7i32);
    pub const SizeNorthSouth: Self = Self(8i32);
    pub const SizeNorthwestSoutheast: Self = Self(9i32);
    pub const SizeWestEast: Self = Self(10i32);
    pub const UniversalNo: Self = Self(11i32);
    pub const UpArrow: Self = Self(12i32);
    pub const Wait: Self = Self(13i32);
    pub const Pin: Self = Self(14i32);
    pub const Person: Self = Self(15i32);
}
impl ::core::marker::Copy for CoreCursorType {}
impl ::core::clone::Clone for CoreCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreCursorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreCursorType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreCursorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreCursorType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreCursorType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreCursorType;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreDispatcherPriority(pub i32);
impl CoreDispatcherPriority {
    pub const Idle: Self = Self(-2i32);
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreDispatcherPriority {}
impl ::core::clone::Clone for CoreDispatcherPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreDispatcherPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreDispatcherPriority {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreDispatcherPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDispatcherPriority").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreDispatcherPriority {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreDispatcherPriority;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreIndependentInputFilters(pub u32);
impl CoreIndependentInputFilters {
    pub const None: Self = Self(0u32);
    pub const MouseButton: Self = Self(1u32);
    pub const MouseWheel: Self = Self(2u32);
    pub const MouseHover: Self = Self(4u32);
    pub const PenWithBarrel: Self = Self(8u32);
    pub const PenInverted: Self = Self(16u32);
}
impl ::core::marker::Copy for CoreIndependentInputFilters {}
impl ::core::clone::Clone for CoreIndependentInputFilters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreIndependentInputFilters {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreIndependentInputFilters {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreIndependentInputFilters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIndependentInputFilters").field(&self.0).finish()
    }
}
impl CoreIndependentInputFilters {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreIndependentInputFilters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreIndependentInputFilters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreIndependentInputFilters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreIndependentInputFilters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreIndependentInputFilters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for CoreIndependentInputFilters {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreIndependentInputFilters;u4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreInputDeviceTypes(pub u32);
impl CoreInputDeviceTypes {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
impl ::core::marker::Copy for CoreInputDeviceTypes {}
impl ::core::clone::Clone for CoreInputDeviceTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreInputDeviceTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreInputDeviceTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreInputDeviceTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputDeviceTypes").field(&self.0).finish()
    }
}
impl CoreInputDeviceTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreInputDeviceTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreInputDeviceTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreInputDeviceTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreInputDeviceTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreInputDeviceTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for CoreInputDeviceTypes {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreInputDeviceTypes;u4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreProcessEventsOption(pub i32);
impl CoreProcessEventsOption {
    pub const ProcessOneAndAllPending: Self = Self(0i32);
    pub const ProcessOneIfPresent: Self = Self(1i32);
    pub const ProcessUntilQuit: Self = Self(2i32);
    pub const ProcessAllIfPresent: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreProcessEventsOption {}
impl ::core::clone::Clone for CoreProcessEventsOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreProcessEventsOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreProcessEventsOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreProcessEventsOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreProcessEventsOption").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreProcessEventsOption {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreProcessEventsOption;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreProximityEvaluationScore(pub i32);
impl CoreProximityEvaluationScore {
    pub const Closest: Self = Self(0i32);
    pub const Farthest: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for CoreProximityEvaluationScore {}
impl ::core::clone::Clone for CoreProximityEvaluationScore {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreProximityEvaluationScore {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreProximityEvaluationScore {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreProximityEvaluationScore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreProximityEvaluationScore").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreProximityEvaluationScore {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreProximityEvaluationScore;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreVirtualKeyStates(pub u32);
impl CoreVirtualKeyStates {
    pub const None: Self = Self(0u32);
    pub const Down: Self = Self(1u32);
    pub const Locked: Self = Self(2u32);
}
impl ::core::marker::Copy for CoreVirtualKeyStates {}
impl ::core::clone::Clone for CoreVirtualKeyStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreVirtualKeyStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreVirtualKeyStates {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreVirtualKeyStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreVirtualKeyStates").field(&self.0).finish()
    }
}
impl CoreVirtualKeyStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreVirtualKeyStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreVirtualKeyStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreVirtualKeyStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreVirtualKeyStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreVirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for CoreVirtualKeyStates {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreVirtualKeyStates;u4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWindowActivationMode(pub i32);
impl CoreWindowActivationMode {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWindowActivationMode {}
impl ::core::clone::Clone for CoreWindowActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWindowActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreWindowActivationMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreWindowActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowActivationMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowActivationMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowActivationMode;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWindowActivationState(pub i32);
impl CoreWindowActivationState {
    pub const CodeActivated: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const PointerActivated: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWindowActivationState {}
impl ::core::clone::Clone for CoreWindowActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWindowActivationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreWindowActivationState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreWindowActivationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowActivationState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowActivationState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowActivationState;i4)");
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWindowFlowDirection(pub i32);
impl CoreWindowFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWindowFlowDirection {}
impl ::core::clone::Clone for CoreWindowFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWindowFlowDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CoreWindowFlowDirection {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CoreWindowFlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowFlowDirection").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CoreWindowFlowDirection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowFlowDirection;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Core\"`*"]
pub struct CorePhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl ::core::marker::Copy for CorePhysicalKeyStatus {}
impl ::core::clone::Clone for CorePhysicalKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CorePhysicalKeyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CorePhysicalKeyStatus").field("RepeatCount", &self.RepeatCount).field("ScanCode", &self.ScanCode).field("IsExtendedKey", &self.IsExtendedKey).field("IsMenuKeyDown", &self.IsMenuKeyDown).field("WasKeyDown", &self.WasKeyDown).field("IsKeyReleased", &self.IsKeyReleased).finish()
    }
}
impl ::windows::core::TypeKind for CorePhysicalKeyStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for CorePhysicalKeyStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CorePhysicalKeyStatus;u4;u4;b1;b1;b1;b1)");
}
impl ::core::cmp::PartialEq for CorePhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount && self.ScanCode == other.ScanCode && self.IsExtendedKey == other.IsExtendedKey && self.IsMenuKeyDown == other.IsMenuKeyDown && self.WasKeyDown == other.WasKeyDown && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::core::cmp::Eq for CorePhysicalKeyStatus {}
impl ::core::default::Default for CorePhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Core\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct CoreProximityEvaluation {
    pub Score: i32,
    pub AdjustedPoint: super::super::Foundation::Point,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for CoreProximityEvaluation {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for CoreProximityEvaluation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreProximityEvaluation").field("Score", &self.Score).field("AdjustedPoint", &self.AdjustedPoint).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::TypeKind for CoreProximityEvaluation {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeType for CoreProximityEvaluation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CoreProximityEvaluation;i4;struct(Windows.Foundation.Point;f4;f4))");
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for CoreProximityEvaluation {
    fn eq(&self, other: &Self) -> bool {
        self.Score == other.Score && self.AdjustedPoint == other.AdjustedPoint
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for CoreProximityEvaluation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct DispatchedHandler(pub ::windows::core::IUnknown);
impl DispatchedHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DispatchedHandlerBox::<F> { vtable: &DispatchedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct DispatchedHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DispatchedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DispatchedHandlerBox<F> {
    const VTABLE: DispatchedHandler_Vtbl = DispatchedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DispatchedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::cmp::PartialEq for DispatchedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatchedHandler {}
impl ::core::fmt::Debug for DispatchedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatchedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DispatchedHandler {
    type Vtable = DispatchedHandler_Vtbl;
}
impl ::core::clone::Clone for DispatchedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for DispatchedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f276c4_98d8_4636_bf49_eb79507548e9);
}
impl ::windows::core::RuntimeType for DispatchedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d1f276c4-98d8-4636-bf49-eb79507548e9}");
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatchedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct IdleDispatchedHandler(pub ::windows::core::IUnknown);
impl IdleDispatchedHandler {
    pub fn new<F: FnMut(::core::option::Option<&IdleDispatchedHandlerArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = IdleDispatchedHandlerBox::<F> { vtable: &IdleDispatchedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, e: &IdleDispatchedHandlerArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(e)).ok() }
    }
}
#[repr(C)]
struct IdleDispatchedHandlerBox<F: FnMut(::core::option::Option<&IdleDispatchedHandlerArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const IdleDispatchedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&IdleDispatchedHandlerArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> IdleDispatchedHandlerBox<F> {
    const VTABLE: IdleDispatchedHandler_Vtbl = IdleDispatchedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<IdleDispatchedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&e)).into()
    }
}
impl ::core::cmp::PartialEq for IdleDispatchedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IdleDispatchedHandler {}
impl ::core::fmt::Debug for IdleDispatchedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IdleDispatchedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IdleDispatchedHandler {
    type Vtable = IdleDispatchedHandler_Vtbl;
}
impl ::core::clone::Clone for IdleDispatchedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IdleDispatchedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa42b0c24_7f21_4abc_99c1_8f01007f0880);
}
impl ::windows::core::RuntimeType for IdleDispatchedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a42b0c24-7f21-4abc-99c1-8f01007f0880}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IdleDispatchedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

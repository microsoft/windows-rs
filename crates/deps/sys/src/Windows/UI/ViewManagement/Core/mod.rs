#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreFrameworkInputView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewHidingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: CoreInputViewKind = CoreInputViewKind(0i32);
    pub const Keyboard: CoreInputViewKind = CoreInputViewKind(1i32);
    pub const Handwriting: CoreInputViewKind = CoreInputViewKind(2i32);
    pub const Emoji: CoreInputViewKind = CoreInputViewKind(3i32);
    pub const Symbols: CoreInputViewKind = CoreInputViewKind(4i32);
    pub const Clipboard: CoreInputViewKind = CoreInputViewKind(5i32);
    pub const Dictation: CoreInputViewKind = CoreInputViewKind(6i32);
}
#[repr(transparent)]
pub struct CoreInputViewOcclusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: CoreInputViewOcclusionKind = CoreInputViewOcclusionKind(0i32);
    pub const Floating: CoreInputViewOcclusionKind = CoreInputViewOcclusionKind(1i32);
    pub const Overlay: CoreInputViewOcclusionKind = CoreInputViewOcclusionKind(2i32);
}
#[repr(transparent)]
pub struct CoreInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewTransferringXYFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(0i32);
    pub const Right: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(1i32);
    pub const Down: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(2i32);
    pub const Left: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(3i32);
}
#[repr(transparent)]
pub struct ICoreFrameworkInputView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreFrameworkInputViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputView4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputView5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewHidingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewOcclusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputViewTransferringXYFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettingsController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettingsControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UISettingsController(pub *mut ::core::ffi::c_void);

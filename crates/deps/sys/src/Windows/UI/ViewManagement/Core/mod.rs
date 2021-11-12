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
    pub const Default: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Handwriting: Self = Self(2i32);
    pub const Emoji: Self = Self(3i32);
    pub const Symbols: Self = Self(4i32);
    pub const Clipboard: Self = Self(5i32);
    pub const Dictation: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreInputViewKind {}
impl ::core::clone::Clone for CoreInputViewKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputViewOcclusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: Self = Self(0i32);
    pub const Floating: Self = Self(1i32);
    pub const Overlay: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreInputViewOcclusionKind {}
impl ::core::clone::Clone for CoreInputViewOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Up: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Down: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreInputViewXYFocusTransferDirection {}
impl ::core::clone::Clone for CoreInputViewXYFocusTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
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

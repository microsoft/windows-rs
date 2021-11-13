#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreFrameworkInputView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreFrameworkInputView {}
impl ::core::clone::Clone for CoreFrameworkInputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreFrameworkInputViewAnimationStartingEventArgs {}
impl ::core::clone::Clone for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
impl ::core::clone::Clone for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInputView {}
impl ::core::clone::Clone for CoreInputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInputViewAnimationStartingEventArgs {}
impl ::core::clone::Clone for CoreInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputViewHidingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInputViewHidingEventArgs {}
impl ::core::clone::Clone for CoreInputViewHidingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CoreInputViewOcclusion {}
impl ::core::clone::Clone for CoreInputViewOcclusion {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CoreInputViewOcclusionsChangedEventArgs {}
impl ::core::clone::Clone for CoreInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputViewShowingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInputViewShowingEventArgs {}
impl ::core::clone::Clone for CoreInputViewShowingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputViewTransferringXYFocusEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInputViewTransferringXYFocusEventArgs {}
impl ::core::clone::Clone for CoreInputViewTransferringXYFocusEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ICoreFrameworkInputView {}
impl ::core::clone::Clone for ICoreFrameworkInputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreFrameworkInputViewAnimationStartingEventArgs {}
impl ::core::clone::Clone for ICoreFrameworkInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreFrameworkInputViewOcclusionsChangedEventArgs {}
impl ::core::clone::Clone for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreFrameworkInputViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreFrameworkInputViewStatics {}
impl ::core::clone::Clone for ICoreFrameworkInputViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputView {}
impl ::core::clone::Clone for ICoreInputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputView2 {}
impl ::core::clone::Clone for ICoreInputView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputView3 {}
impl ::core::clone::Clone for ICoreInputView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputView4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputView4 {}
impl ::core::clone::Clone for ICoreInputView4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputView5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputView5 {}
impl ::core::clone::Clone for ICoreInputView5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewAnimationStartingEventArgs {}
impl ::core::clone::Clone for ICoreInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewHidingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewHidingEventArgs {}
impl ::core::clone::Clone for ICoreInputViewHidingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewOcclusion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewOcclusion {}
impl ::core::clone::Clone for ICoreInputViewOcclusion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewOcclusionsChangedEventArgs {}
impl ::core::clone::Clone for ICoreInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewShowingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewShowingEventArgs {}
impl ::core::clone::Clone for ICoreInputViewShowingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewStatics {}
impl ::core::clone::Clone for ICoreInputViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewStatics2 {}
impl ::core::clone::Clone for ICoreInputViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputViewTransferringXYFocusEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputViewTransferringXYFocusEventArgs {}
impl ::core::clone::Clone for ICoreInputViewTransferringXYFocusEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUISettingsController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUISettingsController {}
impl ::core::clone::Clone for IUISettingsController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUISettingsControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUISettingsControllerStatics {}
impl ::core::clone::Clone for IUISettingsControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UISettingsController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UISettingsController {}
impl ::core::clone::Clone for UISettingsController {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types)]
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
pub struct CoreInputViewKind(i32);
#[repr(transparent)]
pub struct CoreInputViewOcclusion(pub *mut ::core::ffi::c_void);
pub struct CoreInputViewOcclusionKind(i32);
#[repr(transparent)]
pub struct CoreInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputViewTransferringXYFocusEventArgs(pub *mut ::core::ffi::c_void);
pub struct CoreInputViewXYFocusTransferDirection(i32);
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

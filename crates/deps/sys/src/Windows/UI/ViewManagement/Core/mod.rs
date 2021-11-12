#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CoreFrameworkInputView(i32);
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(i32);
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(i32);
pub struct CoreInputView(i32);
pub struct CoreInputViewAnimationStartingEventArgs(i32);
pub struct CoreInputViewHidingEventArgs(i32);
pub struct CoreInputViewKind(i32);
pub struct CoreInputViewOcclusion(i32);
pub struct CoreInputViewOcclusionKind(i32);
pub struct CoreInputViewOcclusionsChangedEventArgs(i32);
pub struct CoreInputViewShowingEventArgs(i32);
pub struct CoreInputViewTransferringXYFocusEventArgs(i32);
pub struct CoreInputViewXYFocusTransferDirection(i32);
pub struct ICoreFrameworkInputView(pub *mut ::core::ffi::c_void);
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreFrameworkInputViewStatics(pub *mut ::core::ffi::c_void);
pub struct ICoreInputView(pub *mut ::core::ffi::c_void);
pub struct ICoreInputView2(pub *mut ::core::ffi::c_void);
pub struct ICoreInputView3(pub *mut ::core::ffi::c_void);
pub struct ICoreInputView4(pub *mut ::core::ffi::c_void);
pub struct ICoreInputView5(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewAnimationStartingEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewHidingEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewOcclusion(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewOcclusionsChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewShowingEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewStatics(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewStatics2(pub *mut ::core::ffi::c_void);
pub struct ICoreInputViewTransferringXYFocusEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUISettingsController(pub *mut ::core::ffi::c_void);
pub struct IUISettingsControllerStatics(pub *mut ::core::ffi::c_void);
pub struct UISettingsController(i32);

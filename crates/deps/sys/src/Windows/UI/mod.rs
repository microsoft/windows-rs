#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Accessibility")]
pub mod Accessibility;
#[cfg(feature = "UI_ApplicationSettings")]
pub mod ApplicationSettings;
#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Core")]
pub mod Core;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Notifications")]
pub mod Notifications;
#[cfg(feature = "UI_Popups")]
pub mod Popups;
#[cfg(feature = "UI_Shell")]
pub mod Shell;
#[cfg(feature = "UI_StartScreen")]
pub mod StartScreen;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_UIAutomation")]
pub mod UIAutomation;
#[cfg(feature = "UI_ViewManagement")]
pub mod ViewManagement;
#[cfg(feature = "UI_WebUI")]
pub mod WebUI;
#[cfg(feature = "UI_WindowManagement")]
pub mod WindowManagement;
#[cfg(feature = "UI_Xaml")]
pub mod Xaml;
#[link(name = "windows")]
extern "system" {}
pub struct Color(i32);
#[repr(transparent)]
pub struct ColorHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Colors(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorHelperStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColors(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIContentRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UIContentRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UIContext(pub *mut ::core::ffi::c_void);
pub struct WindowId(i32);

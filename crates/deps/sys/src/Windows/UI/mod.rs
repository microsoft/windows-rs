#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(C)]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl ::core::marker::Copy for Color {}
impl ::core::clone::Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorHelper {}
impl ::core::clone::Clone for ColorHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Colors(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Colors {}
impl ::core::clone::Clone for Colors {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorHelper {}
impl ::core::clone::Clone for IColorHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorHelperStatics {}
impl ::core::clone::Clone for IColorHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorHelperStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorHelperStatics2 {}
impl ::core::clone::Clone for IColorHelperStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColors(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColors {}
impl ::core::clone::Clone for IColors {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorsStatics {}
impl ::core::clone::Clone for IColorsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIContentRoot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIContentRoot {}
impl ::core::clone::Clone for IUIContentRoot {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIContext {}
impl ::core::clone::Clone for IUIContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UIContentRoot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UIContentRoot {}
impl ::core::clone::Clone for UIContentRoot {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UIContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UIContext {}
impl ::core::clone::Clone for UIContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}

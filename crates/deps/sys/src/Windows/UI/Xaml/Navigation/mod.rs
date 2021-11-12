#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FrameNavigationOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameNavigationOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameNavigationOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigatingCancelEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigatingCancelEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPageStackEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPageStackEntryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPageStackEntryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoadCompletedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigatedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigatingCancelEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigatingCancelEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: NavigationCacheMode = NavigationCacheMode(0i32);
    pub const Required: NavigationCacheMode = NavigationCacheMode(1i32);
    pub const Enabled: NavigationCacheMode = NavigationCacheMode(2i32);
}
#[repr(transparent)]
pub struct NavigationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationFailedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: NavigationMode = NavigationMode(0i32);
    pub const Back: NavigationMode = NavigationMode(1i32);
    pub const Forward: NavigationMode = NavigationMode(2i32);
    pub const Refresh: NavigationMode = NavigationMode(3i32);
}
#[repr(transparent)]
pub struct NavigationStoppedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PageStackEntry(pub *mut ::core::ffi::c_void);

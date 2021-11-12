#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct FrameNavigationOptions(i32);
pub struct IFrameNavigationOptions(pub *mut ::core::ffi::c_void);
pub struct IFrameNavigationOptionsFactory(pub *mut ::core::ffi::c_void);
pub struct INavigatingCancelEventArgs(pub *mut ::core::ffi::c_void);
pub struct INavigatingCancelEventArgs2(pub *mut ::core::ffi::c_void);
pub struct INavigationEventArgs(pub *mut ::core::ffi::c_void);
pub struct INavigationEventArgs2(pub *mut ::core::ffi::c_void);
pub struct INavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPageStackEntry(pub *mut ::core::ffi::c_void);
pub struct IPageStackEntryFactory(pub *mut ::core::ffi::c_void);
pub struct IPageStackEntryStatics(pub *mut ::core::ffi::c_void);
pub struct LoadCompletedEventHandler(pub *mut ::core::ffi::c_void);
pub struct NavigatedEventHandler(pub *mut ::core::ffi::c_void);
pub struct NavigatingCancelEventArgs(i32);
pub struct NavigatingCancelEventHandler(pub *mut ::core::ffi::c_void);
pub struct NavigationCacheMode(i32);
pub struct NavigationEventArgs(i32);
pub struct NavigationFailedEventArgs(i32);
pub struct NavigationFailedEventHandler(pub *mut ::core::ffi::c_void);
pub struct NavigationMode(i32);
pub struct NavigationStoppedEventHandler(pub *mut ::core::ffi::c_void);
pub struct PageStackEntry(i32);

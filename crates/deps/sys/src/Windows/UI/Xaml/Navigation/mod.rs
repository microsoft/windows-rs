#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FrameNavigationOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameNavigationOptions {}
impl ::core::clone::Clone for FrameNavigationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameNavigationOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameNavigationOptions {}
impl ::core::clone::Clone for IFrameNavigationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameNavigationOptionsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameNavigationOptionsFactory {}
impl ::core::clone::Clone for IFrameNavigationOptionsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigatingCancelEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigatingCancelEventArgs {}
impl ::core::clone::Clone for INavigatingCancelEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigatingCancelEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigatingCancelEventArgs2 {}
impl ::core::clone::Clone for INavigatingCancelEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationEventArgs {}
impl ::core::clone::Clone for INavigationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationEventArgs2 {}
impl ::core::clone::Clone for INavigationEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationFailedEventArgs {}
impl ::core::clone::Clone for INavigationFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPageStackEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPageStackEntry {}
impl ::core::clone::Clone for IPageStackEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPageStackEntryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPageStackEntryFactory {}
impl ::core::clone::Clone for IPageStackEntryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPageStackEntryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPageStackEntryStatics {}
impl ::core::clone::Clone for IPageStackEntryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoadCompletedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoadCompletedEventHandler {}
impl ::core::clone::Clone for LoadCompletedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigatedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigatedEventHandler {}
impl ::core::clone::Clone for NavigatedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigatingCancelEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigatingCancelEventArgs {}
impl ::core::clone::Clone for NavigatingCancelEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigatingCancelEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigatingCancelEventHandler {}
impl ::core::clone::Clone for NavigatingCancelEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: Self = Self(0i32);
    pub const Required: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationCacheMode {}
impl ::core::clone::Clone for NavigationCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationEventArgs {}
impl ::core::clone::Clone for NavigationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationFailedEventArgs {}
impl ::core::clone::Clone for NavigationFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationFailedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationFailedEventHandler {}
impl ::core::clone::Clone for NavigationFailedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: Self = Self(0i32);
    pub const Back: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Refresh: Self = Self(3i32);
}
impl ::core::marker::Copy for NavigationMode {}
impl ::core::clone::Clone for NavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationStoppedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationStoppedEventHandler {}
impl ::core::clone::Clone for NavigationStoppedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PageStackEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PageStackEntry {}
impl ::core::clone::Clone for PageStackEntry {
    fn clone(&self) -> Self {
        *self
    }
}

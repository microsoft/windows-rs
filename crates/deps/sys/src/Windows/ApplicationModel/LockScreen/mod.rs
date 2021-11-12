#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILockApplicationHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockApplicationHost {}
impl ::core::clone::Clone for ILockApplicationHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockApplicationHostStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockApplicationHostStatics {}
impl ::core::clone::Clone for ILockApplicationHostStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenBadge(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenBadge {}
impl ::core::clone::Clone for ILockScreenBadge {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenInfo {}
impl ::core::clone::Clone for ILockScreenInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenUnlockingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenUnlockingDeferral {}
impl ::core::clone::Clone for ILockScreenUnlockingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenUnlockingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenUnlockingEventArgs {}
impl ::core::clone::Clone for ILockScreenUnlockingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockApplicationHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockApplicationHost {}
impl ::core::clone::Clone for LockApplicationHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenBadge(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenBadge {}
impl ::core::clone::Clone for LockScreenBadge {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenInfo {}
impl ::core::clone::Clone for LockScreenInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenUnlockingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenUnlockingDeferral {}
impl ::core::clone::Clone for LockScreenUnlockingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenUnlockingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenUnlockingEventArgs {}
impl ::core::clone::Clone for LockScreenUnlockingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

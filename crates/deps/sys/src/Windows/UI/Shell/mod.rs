#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IAdaptiveCard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveCard {}
impl ::core::clone::Clone for IAdaptiveCard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveCardBuilderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveCardBuilderStatics {}
impl ::core::clone::Clone for IAdaptiveCardBuilderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecurityAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecurityAppManager {}
impl ::core::clone::Clone for ISecurityAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareWindowCommandEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareWindowCommandEventArgs {}
impl ::core::clone::Clone for IShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareWindowCommandSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareWindowCommandSource {}
impl ::core::clone::Clone for IShareWindowCommandSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareWindowCommandSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareWindowCommandSourceStatics {}
impl ::core::clone::Clone for IShareWindowCommandSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskbarManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskbarManager {}
impl ::core::clone::Clone for ITaskbarManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskbarManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskbarManager2 {}
impl ::core::clone::Clone for ITaskbarManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskbarManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskbarManagerStatics {}
impl ::core::clone::Clone for ITaskbarManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
impl ::core::marker::Copy for SecurityAppKind {}
impl ::core::clone::Clone for SecurityAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SecurityAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SecurityAppManager {}
impl ::core::clone::Clone for SecurityAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for SecurityAppState {}
impl ::core::clone::Clone for SecurityAppState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
impl ::core::marker::Copy for SecurityAppSubstatus {}
impl ::core::clone::Clone for SecurityAppSubstatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareWindowCommand {}
impl ::core::clone::Clone for ShareWindowCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareWindowCommandEventArgs {}
impl ::core::clone::Clone for ShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareWindowCommandSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareWindowCommandSource {}
impl ::core::clone::Clone for ShareWindowCommandSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TaskbarManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TaskbarManager {}
impl ::core::clone::Clone for TaskbarManager {
    fn clone(&self) -> Self {
        *self
    }
}

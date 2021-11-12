#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameListCategory(pub i32);
impl GameListCategory {
    pub const Candidate: Self = Self(0i32);
    pub const ConfirmedBySystem: Self = Self(1i32);
    pub const ConfirmedByUser: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GameListChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameListEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameListEntryLaunchableState(pub i32);
impl GameListEntryLaunchableState {
    pub const NotLaunchable: Self = Self(0i32);
    pub const ByLastRunningFullPath: Self = Self(1i32);
    pub const ByUserProvidedPath: Self = Self(2i32);
    pub const ByTile: Self = Self(3i32);
}
#[repr(transparent)]
pub struct GameListRemovedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameModeConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameModeUserConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameListEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameListEntry2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameListStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameListStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameModeConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameModeUserConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameModeUserConfigurationStatics(pub *mut ::core::ffi::c_void);

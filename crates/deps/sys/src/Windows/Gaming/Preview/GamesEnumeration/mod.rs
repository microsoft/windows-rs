#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GameListCategory(i32);
#[repr(transparent)]
pub struct GameListChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameListEntry(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GameListEntryLaunchableState(i32);
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

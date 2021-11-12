#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GameList(i32);
pub struct GameListCategory(i32);
pub struct GameListChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct GameListEntry(i32);
pub struct GameListEntryLaunchableState(i32);
pub struct GameListRemovedEventHandler(pub *mut ::core::ffi::c_void);
pub struct GameModeConfiguration(i32);
pub struct GameModeUserConfiguration(i32);
pub struct IGameListEntry(pub *mut ::core::ffi::c_void);
pub struct IGameListEntry2(pub *mut ::core::ffi::c_void);
pub struct IGameListStatics(pub *mut ::core::ffi::c_void);
pub struct IGameListStatics2(pub *mut ::core::ffi::c_void);
pub struct IGameModeConfiguration(pub *mut ::core::ffi::c_void);
pub struct IGameModeUserConfiguration(pub *mut ::core::ffi::c_void);
pub struct IGameModeUserConfigurationStatics(pub *mut ::core::ffi::c_void);

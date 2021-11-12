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
impl ::core::marker::Copy for GameListCategory {}
impl ::core::clone::Clone for GameListCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameListChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameListChangedEventHandler {}
impl ::core::clone::Clone for GameListChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameListEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameListEntry {}
impl ::core::clone::Clone for GameListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameListEntryLaunchableState(pub i32);
impl GameListEntryLaunchableState {
    pub const NotLaunchable: Self = Self(0i32);
    pub const ByLastRunningFullPath: Self = Self(1i32);
    pub const ByUserProvidedPath: Self = Self(2i32);
    pub const ByTile: Self = Self(3i32);
}
impl ::core::marker::Copy for GameListEntryLaunchableState {}
impl ::core::clone::Clone for GameListEntryLaunchableState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameListRemovedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameListRemovedEventHandler {}
impl ::core::clone::Clone for GameListRemovedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameModeConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameModeConfiguration {}
impl ::core::clone::Clone for GameModeConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameModeUserConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameModeUserConfiguration {}
impl ::core::clone::Clone for GameModeUserConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameListEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameListEntry {}
impl ::core::clone::Clone for IGameListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameListEntry2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameListEntry2 {}
impl ::core::clone::Clone for IGameListEntry2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameListStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameListStatics {}
impl ::core::clone::Clone for IGameListStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameListStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameListStatics2 {}
impl ::core::clone::Clone for IGameListStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameModeConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameModeConfiguration {}
impl ::core::clone::Clone for IGameModeConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameModeUserConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameModeUserConfiguration {}
impl ::core::clone::Clone for IGameModeUserConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameModeUserConfigurationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameModeUserConfigurationStatics {}
impl ::core::clone::Clone for IGameModeUserConfigurationStatics {
    fn clone(&self) -> Self {
        *self
    }
}

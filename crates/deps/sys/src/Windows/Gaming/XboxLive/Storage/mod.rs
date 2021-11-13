#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameSaveBlobGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveBlobGetResult {}
impl ::core::clone::Clone for GameSaveBlobGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveBlobInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveBlobInfo {}
impl ::core::clone::Clone for GameSaveBlobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveBlobInfoGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveBlobInfoGetResult {}
impl ::core::clone::Clone for GameSaveBlobInfoGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveBlobInfoQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveBlobInfoQuery {}
impl ::core::clone::Clone for GameSaveBlobInfoQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveContainer {}
impl ::core::clone::Clone for GameSaveContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveContainerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveContainerInfo {}
impl ::core::clone::Clone for GameSaveContainerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveContainerInfoGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveContainerInfoGetResult {}
impl ::core::clone::Clone for GameSaveContainerInfoGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveContainerInfoQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveContainerInfoQuery {}
impl ::core::clone::Clone for GameSaveContainerInfoQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Abort: Self = Self(-2147467260i32);
    pub const InvalidContainerName: Self = Self(-2138898431i32);
    pub const NoAccess: Self = Self(-2138898430i32);
    pub const OutOfLocalStorage: Self = Self(-2138898429i32);
    pub const UserCanceled: Self = Self(-2138898428i32);
    pub const UpdateTooBig: Self = Self(-2138898427i32);
    pub const QuotaExceeded: Self = Self(-2138898426i32);
    pub const ProvidedBufferTooSmall: Self = Self(-2138898425i32);
    pub const BlobNotFound: Self = Self(-2138898424i32);
    pub const NoXboxLiveInfo: Self = Self(-2138898423i32);
    pub const ContainerNotInSync: Self = Self(-2138898422i32);
    pub const ContainerSyncFailed: Self = Self(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: Self = Self(-2138898420i32);
    pub const ObjectExpired: Self = Self(-2138898419i32);
}
impl ::core::marker::Copy for GameSaveErrorStatus {}
impl ::core::clone::Clone for GameSaveErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveOperationResult {}
impl ::core::clone::Clone for GameSaveOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveProvider {}
impl ::core::clone::Clone for GameSaveProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameSaveProviderGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameSaveProviderGetResult {}
impl ::core::clone::Clone for GameSaveProviderGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveBlobGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveBlobGetResult {}
impl ::core::clone::Clone for IGameSaveBlobGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveBlobInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveBlobInfo {}
impl ::core::clone::Clone for IGameSaveBlobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveBlobInfoGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveBlobInfoGetResult {}
impl ::core::clone::Clone for IGameSaveBlobInfoGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveBlobInfoQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveBlobInfoQuery {}
impl ::core::clone::Clone for IGameSaveBlobInfoQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveContainer {}
impl ::core::clone::Clone for IGameSaveContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveContainerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveContainerInfo {}
impl ::core::clone::Clone for IGameSaveContainerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveContainerInfoGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveContainerInfoGetResult {}
impl ::core::clone::Clone for IGameSaveContainerInfoGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveContainerInfoQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveContainerInfoQuery {}
impl ::core::clone::Clone for IGameSaveContainerInfoQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveOperationResult {}
impl ::core::clone::Clone for IGameSaveOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveProvider {}
impl ::core::clone::Clone for IGameSaveProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveProviderGetResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveProviderGetResult {}
impl ::core::clone::Clone for IGameSaveProviderGetResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameSaveProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameSaveProviderStatics {}
impl ::core::clone::Clone for IGameSaveProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}

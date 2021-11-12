#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameSaveBlobGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveBlobInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveBlobInfoGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveBlobInfoQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveContainerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveContainerInfoGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveContainerInfoQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: GameSaveErrorStatus = GameSaveErrorStatus(0i32);
    pub const Abort: GameSaveErrorStatus = GameSaveErrorStatus(-2147467260i32);
    pub const InvalidContainerName: GameSaveErrorStatus = GameSaveErrorStatus(-2138898431i32);
    pub const NoAccess: GameSaveErrorStatus = GameSaveErrorStatus(-2138898430i32);
    pub const OutOfLocalStorage: GameSaveErrorStatus = GameSaveErrorStatus(-2138898429i32);
    pub const UserCanceled: GameSaveErrorStatus = GameSaveErrorStatus(-2138898428i32);
    pub const UpdateTooBig: GameSaveErrorStatus = GameSaveErrorStatus(-2138898427i32);
    pub const QuotaExceeded: GameSaveErrorStatus = GameSaveErrorStatus(-2138898426i32);
    pub const ProvidedBufferTooSmall: GameSaveErrorStatus = GameSaveErrorStatus(-2138898425i32);
    pub const BlobNotFound: GameSaveErrorStatus = GameSaveErrorStatus(-2138898424i32);
    pub const NoXboxLiveInfo: GameSaveErrorStatus = GameSaveErrorStatus(-2138898423i32);
    pub const ContainerNotInSync: GameSaveErrorStatus = GameSaveErrorStatus(-2138898422i32);
    pub const ContainerSyncFailed: GameSaveErrorStatus = GameSaveErrorStatus(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: GameSaveErrorStatus = GameSaveErrorStatus(-2138898420i32);
    pub const ObjectExpired: GameSaveErrorStatus = GameSaveErrorStatus(-2138898419i32);
}
#[repr(transparent)]
pub struct GameSaveOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameSaveProviderGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveBlobGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveBlobInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveBlobInfoGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveBlobInfoQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveContainerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveContainerInfoGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveContainerInfoQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveProviderGetResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameSaveProviderStatics(pub *mut ::core::ffi::c_void);

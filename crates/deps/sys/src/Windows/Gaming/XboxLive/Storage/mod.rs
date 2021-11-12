#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GameSaveBlobGetResult(i32);
pub struct GameSaveBlobInfo(i32);
pub struct GameSaveBlobInfoGetResult(i32);
pub struct GameSaveBlobInfoQuery(i32);
pub struct GameSaveContainer(i32);
pub struct GameSaveContainerInfo(i32);
pub struct GameSaveContainerInfoGetResult(i32);
pub struct GameSaveContainerInfoQuery(i32);
pub struct GameSaveErrorStatus(i32);
pub struct GameSaveOperationResult(i32);
pub struct GameSaveProvider(i32);
pub struct GameSaveProviderGetResult(i32);
pub struct IGameSaveBlobGetResult(pub *mut ::core::ffi::c_void);
pub struct IGameSaveBlobInfo(pub *mut ::core::ffi::c_void);
pub struct IGameSaveBlobInfoGetResult(pub *mut ::core::ffi::c_void);
pub struct IGameSaveBlobInfoQuery(pub *mut ::core::ffi::c_void);
pub struct IGameSaveContainer(pub *mut ::core::ffi::c_void);
pub struct IGameSaveContainerInfo(pub *mut ::core::ffi::c_void);
pub struct IGameSaveContainerInfoGetResult(pub *mut ::core::ffi::c_void);
pub struct IGameSaveContainerInfoQuery(pub *mut ::core::ffi::c_void);
pub struct IGameSaveOperationResult(pub *mut ::core::ffi::c_void);
pub struct IGameSaveProvider(pub *mut ::core::ffi::c_void);
pub struct IGameSaveProviderGetResult(pub *mut ::core::ffi::c_void);
pub struct IGameSaveProviderStatics(pub *mut ::core::ffi::c_void);

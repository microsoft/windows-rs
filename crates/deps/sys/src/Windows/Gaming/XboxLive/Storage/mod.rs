#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct GameSaveErrorStatus(i32);
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

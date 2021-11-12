#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameService(pub *mut ::core::ffi::c_void);
pub struct GameServiceGameOutcome(i32);
#[repr(transparent)]
pub struct GameServicePropertyCollection(pub *mut ::core::ffi::c_void);
pub struct GameServiceScoreKind(i32);
#[repr(transparent)]
pub struct IGameService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameServicePropertyCollection(pub *mut ::core::ffi::c_void);

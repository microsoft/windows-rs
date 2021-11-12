#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GameService(i32);
pub struct GameServiceGameOutcome(i32);
pub struct GameServicePropertyCollection(i32);
pub struct GameServiceScoreKind(i32);
pub struct IGameService(pub *mut ::core::ffi::c_void);
pub struct IGameService2(pub *mut ::core::ffi::c_void);
pub struct IGameServicePropertyCollection(pub *mut ::core::ffi::c_void);

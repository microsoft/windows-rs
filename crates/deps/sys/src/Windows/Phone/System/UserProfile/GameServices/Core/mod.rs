#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: GameServiceGameOutcome = GameServiceGameOutcome(0i32);
    pub const Win: GameServiceGameOutcome = GameServiceGameOutcome(1i32);
    pub const Loss: GameServiceGameOutcome = GameServiceGameOutcome(2i32);
    pub const Tie: GameServiceGameOutcome = GameServiceGameOutcome(3i32);
}
#[repr(transparent)]
pub struct GameServicePropertyCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: GameServiceScoreKind = GameServiceScoreKind(0i32);
    pub const Time: GameServiceScoreKind = GameServiceScoreKind(1i32);
}
#[repr(transparent)]
pub struct IGameService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameServicePropertyCollection(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILampArrayBitmapEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayBitmapEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayBlinkEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayBlinkEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayColorRampEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayColorRampEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayCustomEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayCustomEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayEffectPlaylist(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayEffectPlaylistStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArraySolidEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArraySolidEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayBitmapEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayBlinkEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayColorRampEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayCustomEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: Self = Self(0i32);
    pub const KeepState: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectCompletionBehavior {}
impl ::core::clone::Clone for LampArrayEffectCompletionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayEffectPlaylist(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: Self = Self(0i32);
    pub const Simultaneous: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectStartMode {}
impl ::core::clone::Clone for LampArrayEffectStartMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayRepetitionMode {}
impl ::core::clone::Clone for LampArrayRepetitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArraySolidEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);

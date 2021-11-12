#![allow(non_snake_case, non_camel_case_types)]
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
pub struct LampArrayEffectCompletionBehavior(i32);
#[repr(transparent)]
pub struct LampArrayEffectPlaylist(pub *mut ::core::ffi::c_void);
pub struct LampArrayEffectStartMode(i32);
pub struct LampArrayRepetitionMode(i32);
#[repr(transparent)]
pub struct LampArraySolidEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);

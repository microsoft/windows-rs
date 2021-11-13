#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILampArrayBitmapEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayBitmapEffect {}
impl ::core::clone::Clone for ILampArrayBitmapEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayBitmapEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayBitmapEffectFactory {}
impl ::core::clone::Clone for ILampArrayBitmapEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayBitmapRequestedEventArgs {}
impl ::core::clone::Clone for ILampArrayBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayBlinkEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayBlinkEffect {}
impl ::core::clone::Clone for ILampArrayBlinkEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayBlinkEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayBlinkEffectFactory {}
impl ::core::clone::Clone for ILampArrayBlinkEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayColorRampEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayColorRampEffect {}
impl ::core::clone::Clone for ILampArrayColorRampEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayColorRampEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayColorRampEffectFactory {}
impl ::core::clone::Clone for ILampArrayColorRampEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayCustomEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayCustomEffect {}
impl ::core::clone::Clone for ILampArrayCustomEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayCustomEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayCustomEffectFactory {}
impl ::core::clone::Clone for ILampArrayCustomEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayEffect {}
impl ::core::clone::Clone for ILampArrayEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayEffectPlaylist(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayEffectPlaylist {}
impl ::core::clone::Clone for ILampArrayEffectPlaylist {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayEffectPlaylistStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayEffectPlaylistStatics {}
impl ::core::clone::Clone for ILampArrayEffectPlaylistStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArraySolidEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArraySolidEffect {}
impl ::core::clone::Clone for ILampArraySolidEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArraySolidEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArraySolidEffectFactory {}
impl ::core::clone::Clone for ILampArraySolidEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILampArrayUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILampArrayUpdateRequestedEventArgs {}
impl ::core::clone::Clone for ILampArrayUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayBitmapEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LampArrayBitmapEffect {}
impl ::core::clone::Clone for LampArrayBitmapEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LampArrayBitmapRequestedEventArgs {}
impl ::core::clone::Clone for LampArrayBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayBlinkEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LampArrayBlinkEffect {}
impl ::core::clone::Clone for LampArrayBlinkEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayColorRampEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LampArrayColorRampEffect {}
impl ::core::clone::Clone for LampArrayColorRampEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayCustomEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LampArrayCustomEffect {}
impl ::core::clone::Clone for LampArrayCustomEffect {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LampArrayEffectPlaylist {}
impl ::core::clone::Clone for LampArrayEffectPlaylist {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LampArraySolidEffect {}
impl ::core::clone::Clone for LampArraySolidEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LampArrayUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LampArrayUpdateRequestedEventArgs {}
impl ::core::clone::Clone for LampArrayUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

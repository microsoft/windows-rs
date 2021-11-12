#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioRenderCategory(pub i32);
impl AudioRenderCategory {
    pub const Other: AudioRenderCategory = AudioRenderCategory(0i32);
    pub const ForegroundOnlyMedia: AudioRenderCategory = AudioRenderCategory(1i32);
    pub const BackgroundCapableMedia: AudioRenderCategory = AudioRenderCategory(2i32);
    pub const Communications: AudioRenderCategory = AudioRenderCategory(3i32);
    pub const Alerts: AudioRenderCategory = AudioRenderCategory(4i32);
    pub const SoundEffects: AudioRenderCategory = AudioRenderCategory(5i32);
    pub const GameEffects: AudioRenderCategory = AudioRenderCategory(6i32);
    pub const GameMedia: AudioRenderCategory = AudioRenderCategory(7i32);
    pub const GameChat: AudioRenderCategory = AudioRenderCategory(8i32);
    pub const Speech: AudioRenderCategory = AudioRenderCategory(9i32);
    pub const Movie: AudioRenderCategory = AudioRenderCategory(10i32);
    pub const Media: AudioRenderCategory = AudioRenderCategory(11i32);
}

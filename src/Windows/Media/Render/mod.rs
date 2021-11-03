#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_Render`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for AudioRenderCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioRenderCategory {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioRenderCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Render.AudioRenderCategory;i4)");
}
impl ::windows::runtime::DefaultType for AudioRenderCategory {
    type DefaultType = Self;
}

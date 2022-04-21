#[doc = "*Required features: `\"Media_Render\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioRenderCategory(pub i32);
impl AudioRenderCategory {
    pub const Other: Self = Self(0i32);
    pub const ForegroundOnlyMedia: Self = Self(1i32);
    pub const BackgroundCapableMedia: Self = Self(2i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for AudioRenderCategory {}
impl ::core::clone::Clone for AudioRenderCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioRenderCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioRenderCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioRenderCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRenderCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioRenderCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Render.AudioRenderCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

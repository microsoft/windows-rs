#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AudioRenderCategory(pub i32);
impl AudioRenderCategory {
    pub const Other: Self = Self(0);
    pub const ForegroundOnlyMedia: Self = Self(1);
    pub const BackgroundCapableMedia: Self = Self(2);
    pub const Communications: Self = Self(3);
    pub const Alerts: Self = Self(4);
    pub const SoundEffects: Self = Self(5);
    pub const GameEffects: Self = Self(6);
    pub const GameMedia: Self = Self(7);
    pub const GameChat: Self = Self(8);
    pub const Speech: Self = Self(9);
    pub const Movie: Self = Self(10);
    pub const Media: Self = Self(11);
}
impl windows_core::TypeKind for AudioRenderCategory {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AudioRenderCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Render.AudioRenderCategory;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Render.AudioRenderCategory");
}

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[deprecated(
    note = "PlayToConnectionState may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionState."
)]
pub struct PlayToConnectionState(pub i32);
#[allow(deprecated)]
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
#[allow(deprecated)]
impl windows_core::TypeKind for PlayToConnectionState {
    type TypeKind = windows_core::CopyType;
}
#[allow(deprecated)]
impl windows_core::RuntimeType for PlayToConnectionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Media.PlayTo.PlayToConnectionState;i4)",
    );
}

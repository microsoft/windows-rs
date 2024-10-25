pub const SMEXF_HANDLER: STDMSHLFLAGS = 2i32;
pub const SMEXF_SERVER: STDMSHLFLAGS = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STDMSHLFLAGS(pub i32);
impl windows_core::TypeKind for STDMSHLFLAGS {
    type TypeKind = windows_core::CopyType;
}

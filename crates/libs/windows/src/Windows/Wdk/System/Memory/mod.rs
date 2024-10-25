pub const ViewShare: SECTION_INHERIT = 1i32;
pub const ViewUnmap: SECTION_INHERIT = 2i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SECTION_INHERIT(pub i32);
impl windows_core::TypeKind for SECTION_INHERIT {
    type TypeKind = windows_core::CopyType;
}

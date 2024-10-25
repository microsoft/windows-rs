pub const CpAicLaunchAdminProcess: CreateProcessMethod = 2i32;
pub const CpCreateProcess: CreateProcessMethod = 0i32;
pub const CpCreateProcessAsUser: CreateProcessMethod = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateProcessMethod(pub i32);
impl windows_core::TypeKind for CreateProcessMethod {
    type TypeKind = windows_core::CopyType;
}

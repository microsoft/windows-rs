pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = 1u32;
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = 1u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPERATION_END_PARAMETERS_FLAGS(pub u32);
impl windows_core::TypeKind for OPERATION_END_PARAMETERS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPERATION_START_FLAGS(pub u32);
impl windows_core::TypeKind for OPERATION_START_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPERATION_END_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_END_PARAMETERS_FLAGS,
}
impl Default for OPERATION_END_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPERATION_END_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPERATION_START_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_START_FLAGS,
}
impl Default for OPERATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPERATION_START_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}

pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = 2i32;
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = 1i32;
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = 31i32;
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = 1i32;
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = 2i32;
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = 0i32;
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = 4i32;
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = 8i32;
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = 16i32;
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = 6i32;
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = 2i32;
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = 0i32;
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = 4i32;
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = 1i32;
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = 2i32;
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = 4i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CALLFRAME_COPY(pub i32);
impl windows_core::TypeKind for CALLFRAME_COPY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CALLFRAME_FREE(pub i32);
impl windows_core::TypeKind for CALLFRAME_FREE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CALLFRAME_NULL(pub i32);
impl windows_core::TypeKind for CALLFRAME_NULL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CALLFRAME_WALK(pub i32);
impl windows_core::TypeKind for CALLFRAME_WALK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: windows_core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
impl Default for CALLFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CALLFRAMEINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
impl Default for CALLFRAMEPARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CALLFRAMEPARAMINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub punkReserved: Option<windows_core::IUnknown>,
    pub guidTransferSyntax: windows_core::GUID,
}
impl Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CALLFRAME_MARSHALCONTEXT {
    type TypeKind = windows_core::CloneType;
}

#[cfg(feature = "Win32_winnt")]
pub type FDQUERYCONTEXT = super::winnt::DWORDLONG;
pub const FD_LONGHORN: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IFunctionDiscoveryProviderRefresh(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IFunctionInstanceCollectionQuery2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IFunctionInstanceCollectionQueryCollection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IFunctionInstanceQuery2(pub u8);
pub type PropertyConstraint = i32;
pub const QCT_LAYERED: QueryCategoryType = 1;
pub const QCT_PROVIDER: QueryCategoryType = 0;
pub const QC_CONTAINS: PropertyConstraint = 9;
pub const QC_DOESNOTEXIST: PropertyConstraint = 8;
pub const QC_EQUALS: PropertyConstraint = 0;
pub const QC_EXISTS: PropertyConstraint = 7;
pub const QC_GREATERTHAN: PropertyConstraint = 4;
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = 5;
pub const QC_LESSTHAN: PropertyConstraint = 2;
pub const QC_LESSTHANOREQUAL: PropertyConstraint = 3;
pub const QC_NOTEQUAL: PropertyConstraint = 1;
pub const QC_STARTSWITH: PropertyConstraint = 6;
pub const QUA_ADD: QueryUpdateAction = 0;
pub const QUA_CHANGE: QueryUpdateAction = 2;
pub const QUA_REMOVE: QueryUpdateAction = 1;
pub type QueryCategoryType = i32;
pub type QueryUpdateAction = i32;
pub const SVF_SYSTEM: SystemVisibilityFlags = 0;
pub const SVF_USER: SystemVisibilityFlags = 1;
pub type SystemVisibilityFlags = i32;

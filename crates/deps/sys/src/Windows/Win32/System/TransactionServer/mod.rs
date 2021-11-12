#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct Catalog(i32);
#[repr(C)]
pub struct CatalogCollection(i32);
#[repr(C)]
pub struct CatalogObject(i32);
#[repr(C)]
pub struct ComponentUtil(i32);
#[repr(transparent)]
pub struct ICatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentUtil(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageUtil(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteComponentUtil(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoleAssociationUtil(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageUtil(i32);
#[repr(C)]
pub struct RemoteComponentUtil(i32);
#[repr(C)]
pub struct RoleAssociationUtil(i32);
#[repr(C)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(i32);
#[repr(C)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(i32);
#[repr(C)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0003(i32);

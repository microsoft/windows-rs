#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Catalog(i32);
pub struct CatalogCollection(i32);
pub struct CatalogObject(i32);
pub struct ComponentUtil(i32);
pub struct ICatalog(pub *mut ::core::ffi::c_void);
pub struct IComponentUtil(pub *mut ::core::ffi::c_void);
pub struct IPackageUtil(pub *mut ::core::ffi::c_void);
pub struct IRemoteComponentUtil(pub *mut ::core::ffi::c_void);
pub struct IRoleAssociationUtil(pub *mut ::core::ffi::c_void);
pub struct PackageUtil(i32);
pub struct RemoteComponentUtil(i32);
pub struct RoleAssociationUtil(i32);
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(i32);
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(i32);
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0003(i32);

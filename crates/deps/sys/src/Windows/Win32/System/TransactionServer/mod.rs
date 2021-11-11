#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Catalog();
    fn CatalogCollection();
    fn CatalogObject();
    fn ComponentUtil();
    fn ICatalog();
    fn IComponentUtil();
    fn IPackageUtil();
    fn IRemoteComponentUtil();
    fn IRoleAssociationUtil();
    fn PackageUtil();
    fn RemoteComponentUtil();
    fn RoleAssociationUtil();
    fn __MIDL___MIDL_itf_mtxadmin_0107_0001();
    fn __MIDL___MIDL_itf_mtxadmin_0107_0002();
    fn __MIDL___MIDL_itf_mtxadmin_0107_0003();
}

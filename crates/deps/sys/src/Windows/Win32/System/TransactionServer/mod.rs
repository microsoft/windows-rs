#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(pub i32);
pub const mtsInstallUsers: __MIDL___MIDL_itf_mtxadmin_0107_0001 = __MIDL___MIDL_itf_mtxadmin_0107_0001(1i32);
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(pub i32);
pub const mtsExportUsers: __MIDL___MIDL_itf_mtxadmin_0107_0002 = __MIDL___MIDL_itf_mtxadmin_0107_0002(1i32);
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0003(pub i32);
pub const mtsErrObjectErrors: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368511i32);
pub const mtsErrObjectInvalid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368510i32);
pub const mtsErrKeyMissing: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368509i32);
pub const mtsErrAlreadyInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368508i32);
pub const mtsErrDownloadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368507i32);
pub const mtsErrPDFWriteFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368505i32);
pub const mtsErrPDFReadFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368504i32);
pub const mtsErrPDFVersion: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368503i32);
pub const mtsErrCoReqCompInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
pub const mtsErrBadPath: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368502i32);
pub const mtsErrPackageExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368501i32);
pub const mtsErrRoleExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368500i32);
pub const mtsErrCantCopyFile: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368499i32);
pub const mtsErrNoTypeLib: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368498i32);
pub const mtsErrNoUser: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368497i32);
pub const mtsErrInvalidUserids: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
pub const mtsErrNoRegistryCLSID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368495i32);
pub const mtsErrBadRegistryProgID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368494i32);
pub const mtsErrAuthenticationLevel: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368493i32);
pub const mtsErrUserPasswdNotValid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368492i32);
pub const mtsErrNoRegistryRead: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368491i32);
pub const mtsErrNoRegistryWrite: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368490i32);
pub const mtsErrNoRegistryRepair: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368489i32);
pub const mtsErrCLSIDOrIIDMismatch: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368488i32);
pub const mtsErrRemoteInterface: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368487i32);
pub const mtsErrDllRegisterServer: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368486i32);
pub const mtsErrNoServerShare: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368485i32);
pub const mtsErrNoAccessToUNC: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368484i32);
pub const mtsErrDllLoadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368483i32);
pub const mtsErrBadRegistryLibID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368482i32);
pub const mtsErrPackDirNotFound: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368481i32);
pub const mtsErrTreatAs: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368480i32);
pub const mtsErrBadForward: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368479i32);
pub const mtsErrBadIID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368478i32);
pub const mtsErrRegistrarFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368477i32);
pub const mtsErrCompFileDoesNotExist: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368476i32);
pub const mtsErrCompFileLoadDLLFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368475i32);
pub const mtsErrCompFileGetClassObj: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368474i32);
pub const mtsErrCompFileClassNotAvail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368473i32);
pub const mtsErrCompFileBadTLB: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368472i32);
pub const mtsErrCompFileNotInstallable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368471i32);
pub const mtsErrNotChangeable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368470i32);
pub const mtsErrNotDeletable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368469i32);
pub const mtsErrSession: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368468i32);
pub const mtsErrCompFileNoRegistrar: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368460i32);

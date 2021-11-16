#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const Catalog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169537, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const CatalogCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169539, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const CatalogObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169538, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const ComponentUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169540, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
#[repr(transparent)]
pub struct ICatalog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICatalog {}
impl ::core::clone::Clone for ICatalog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponentUtil(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentUtil {}
impl ::core::clone::Clone for IComponentUtil {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageUtil(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageUtil {}
impl ::core::clone::Clone for IPackageUtil {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteComponentUtil(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteComponentUtil {}
impl ::core::clone::Clone for IRemoteComponentUtil {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRoleAssociationUtil(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRoleAssociationUtil {}
impl ::core::clone::Clone for IRoleAssociationUtil {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PackageUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169541, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const RemoteComponentUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169542, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const RoleAssociationUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169543, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const mtsInstallUsers: i32 = 1i32;
pub const mtsExportUsers: i32 = 1i32;
pub const mtsErrObjectErrors: i32 = -2146368511i32;
pub const mtsErrObjectInvalid: i32 = -2146368510i32;
pub const mtsErrKeyMissing: i32 = -2146368509i32;
pub const mtsErrAlreadyInstalled: i32 = -2146368508i32;
pub const mtsErrDownloadFailed: i32 = -2146368507i32;
pub const mtsErrPDFWriteFail: i32 = -2146368505i32;
pub const mtsErrPDFReadFail: i32 = -2146368504i32;
pub const mtsErrPDFVersion: i32 = -2146368503i32;
pub const mtsErrCoReqCompInstalled: i32 = -2146368496i32;
pub const mtsErrBadPath: i32 = -2146368502i32;
pub const mtsErrPackageExists: i32 = -2146368501i32;
pub const mtsErrRoleExists: i32 = -2146368500i32;
pub const mtsErrCantCopyFile: i32 = -2146368499i32;
pub const mtsErrNoTypeLib: i32 = -2146368498i32;
pub const mtsErrNoUser: i32 = -2146368497i32;
pub const mtsErrInvalidUserids: i32 = -2146368496i32;
pub const mtsErrNoRegistryCLSID: i32 = -2146368495i32;
pub const mtsErrBadRegistryProgID: i32 = -2146368494i32;
pub const mtsErrAuthenticationLevel: i32 = -2146368493i32;
pub const mtsErrUserPasswdNotValid: i32 = -2146368492i32;
pub const mtsErrNoRegistryRead: i32 = -2146368491i32;
pub const mtsErrNoRegistryWrite: i32 = -2146368490i32;
pub const mtsErrNoRegistryRepair: i32 = -2146368489i32;
pub const mtsErrCLSIDOrIIDMismatch: i32 = -2146368488i32;
pub const mtsErrRemoteInterface: i32 = -2146368487i32;
pub const mtsErrDllRegisterServer: i32 = -2146368486i32;
pub const mtsErrNoServerShare: i32 = -2146368485i32;
pub const mtsErrNoAccessToUNC: i32 = -2146368484i32;
pub const mtsErrDllLoadFailed: i32 = -2146368483i32;
pub const mtsErrBadRegistryLibID: i32 = -2146368482i32;
pub const mtsErrPackDirNotFound: i32 = -2146368481i32;
pub const mtsErrTreatAs: i32 = -2146368480i32;
pub const mtsErrBadForward: i32 = -2146368479i32;
pub const mtsErrBadIID: i32 = -2146368478i32;
pub const mtsErrRegistrarFailed: i32 = -2146368477i32;
pub const mtsErrCompFileDoesNotExist: i32 = -2146368476i32;
pub const mtsErrCompFileLoadDLLFail: i32 = -2146368475i32;
pub const mtsErrCompFileGetClassObj: i32 = -2146368474i32;
pub const mtsErrCompFileClassNotAvail: i32 = -2146368473i32;
pub const mtsErrCompFileBadTLB: i32 = -2146368472i32;
pub const mtsErrCompFileNotInstallable: i32 = -2146368471i32;
pub const mtsErrNotChangeable: i32 = -2146368470i32;
pub const mtsErrNotDeletable: i32 = -2146368469i32;
pub const mtsErrSession: i32 = -2146368468i32;
pub const mtsErrCompFileNoRegistrar: i32 = -2146368460i32;

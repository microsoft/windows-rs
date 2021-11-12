#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AddPackageByAppInstallerOptions(i32);
#[repr(transparent)]
pub struct AddPackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoUpdateSettingsOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateSharedPackageContainerResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeleteSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeleteSharedPackageContainerResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeploymentOptions(i32);
#[repr(C)]
pub struct DeploymentProgress(i32);
#[repr(C)]
pub struct DeploymentProgressState(i32);
#[repr(transparent)]
pub struct DeploymentResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FindSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAddPackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallerManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoUpdateSettingsOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoUpdateSettingsOptionsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateSharedPackageContainerResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeleteSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeleteSharedPackageContainerResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeploymentResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeploymentResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFindSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageAllUserProvisioningOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager10(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManager9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageManagerDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageUserInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageVolume2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisterPackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPackageContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPackageContainerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPackageContainerManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPackageContainerMember(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPackageContainerMemberFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStagePackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSharedPackageContainerResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageAllUserProvisioningOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageInstallState(i32);
#[repr(transparent)]
pub struct PackageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageManagerDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageState(i32);
#[repr(C)]
pub struct PackageStatus(i32);
#[repr(C)]
pub struct PackageStubPreference(i32);
#[repr(C)]
pub struct PackageTypes(i32);
#[repr(transparent)]
pub struct PackageUserInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RegisterPackageOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RemovalOptions(i32);
#[repr(transparent)]
pub struct SharedPackageContainer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SharedPackageContainerContract(i32);
#[repr(C)]
pub struct SharedPackageContainerCreationCollisionOptions(i32);
#[repr(transparent)]
pub struct SharedPackageContainerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedPackageContainerMember(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SharedPackageContainerOperationStatus(i32);
#[repr(transparent)]
pub struct StagePackageOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StubPackageOption(i32);
#[repr(transparent)]
pub struct UpdateSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UpdateSharedPackageContainerResult(pub *mut ::core::ffi::c_void);

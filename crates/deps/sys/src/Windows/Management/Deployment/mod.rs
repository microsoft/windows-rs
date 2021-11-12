#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(0u32);
    pub const InstallAllResources: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(32u32);
    pub const ForceTargetAppShutdown: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(64u32);
    pub const RequiredContentGroupOnly: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(256u32);
    pub const LimitToExistingPackages: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(512u32);
}
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
#[repr(transparent)]
pub struct DeploymentOptions(pub u32);
impl DeploymentOptions {
    pub const None: DeploymentOptions = DeploymentOptions(0u32);
    pub const ForceApplicationShutdown: DeploymentOptions = DeploymentOptions(1u32);
    pub const DevelopmentMode: DeploymentOptions = DeploymentOptions(2u32);
    pub const InstallAllResources: DeploymentOptions = DeploymentOptions(32u32);
    pub const ForceTargetApplicationShutdown: DeploymentOptions = DeploymentOptions(64u32);
    pub const RequiredContentGroupOnly: DeploymentOptions = DeploymentOptions(256u32);
    pub const ForceUpdateFromAnyVersion: DeploymentOptions = DeploymentOptions(262144u32);
    pub const RetainFilesOnFailure: DeploymentOptions = DeploymentOptions(2097152u32);
    pub const StageInPlace: DeploymentOptions = DeploymentOptions(4194304u32);
}
#[repr(C)]
pub struct DeploymentProgress(i32);
#[repr(transparent)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: DeploymentProgressState = DeploymentProgressState(0i32);
    pub const Processing: DeploymentProgressState = DeploymentProgressState(1i32);
}
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
#[repr(transparent)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: PackageInstallState = PackageInstallState(0i32);
    pub const Staged: PackageInstallState = PackageInstallState(1i32);
    pub const Installed: PackageInstallState = PackageInstallState(2i32);
    pub const Paused: PackageInstallState = PackageInstallState(6i32);
}
#[repr(transparent)]
pub struct PackageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageManagerDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: PackageState = PackageState(0i32);
    pub const LicenseInvalid: PackageState = PackageState(1i32);
    pub const Modified: PackageState = PackageState(2i32);
    pub const Tampered: PackageState = PackageState(3i32);
}
#[repr(transparent)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: PackageStatus = PackageStatus(0u32);
    pub const LicenseIssue: PackageStatus = PackageStatus(1u32);
    pub const Modified: PackageStatus = PackageStatus(2u32);
    pub const Tampered: PackageStatus = PackageStatus(4u32);
    pub const Disabled: PackageStatus = PackageStatus(8u32);
}
#[repr(transparent)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: PackageStubPreference = PackageStubPreference(0i32);
    pub const Stub: PackageStubPreference = PackageStubPreference(1i32);
}
#[repr(transparent)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: PackageTypes = PackageTypes(0u32);
    pub const Main: PackageTypes = PackageTypes(1u32);
    pub const Framework: PackageTypes = PackageTypes(2u32);
    pub const Resource: PackageTypes = PackageTypes(4u32);
    pub const Bundle: PackageTypes = PackageTypes(8u32);
    pub const Xap: PackageTypes = PackageTypes(16u32);
    pub const Optional: PackageTypes = PackageTypes(32u32);
    pub const All: PackageTypes = PackageTypes(4294967295u32);
}
#[repr(transparent)]
pub struct PackageUserInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RegisterPackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: RemovalOptions = RemovalOptions(0u32);
    pub const PreserveApplicationData: RemovalOptions = RemovalOptions(4096u32);
    pub const PreserveRoamableApplicationData: RemovalOptions = RemovalOptions(128u32);
    pub const RemoveForAllUsers: RemovalOptions = RemovalOptions(524288u32);
}
#[repr(transparent)]
pub struct SharedPackageContainer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SharedPackageContainerContract(i32);
#[repr(transparent)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(0i32);
    pub const MergeWithExisting: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(1i32);
    pub const ReplaceExisting: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(2i32);
}
#[repr(transparent)]
pub struct SharedPackageContainerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedPackageContainerMember(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(0i32);
    pub const BlockedByPolicy: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(1i32);
    pub const AlreadyExists: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(2i32);
    pub const PackageFamilyExistsInAnotherContainer: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(3i32);
    pub const NotFound: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(4i32);
    pub const UnknownFailure: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(5i32);
}
#[repr(transparent)]
pub struct StagePackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: StubPackageOption = StubPackageOption(0i32);
    pub const InstallFull: StubPackageOption = StubPackageOption(1i32);
    pub const InstallStub: StubPackageOption = StubPackageOption(2i32);
    pub const UsePreference: StubPackageOption = StubPackageOption(3i32);
}
#[repr(transparent)]
pub struct UpdateSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UpdateSharedPackageContainerResult(pub *mut ::core::ffi::c_void);

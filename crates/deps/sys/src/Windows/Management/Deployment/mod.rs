#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: Self = Self(0u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetAppShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const LimitToExistingPackages: Self = Self(512u32);
}
impl ::core::marker::Copy for AddPackageByAppInstallerOptions {}
impl ::core::clone::Clone for AddPackageByAppInstallerOptions {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const None: Self = Self(0u32);
    pub const ForceApplicationShutdown: Self = Self(1u32);
    pub const DevelopmentMode: Self = Self(2u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetApplicationShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const ForceUpdateFromAnyVersion: Self = Self(262144u32);
    pub const RetainFilesOnFailure: Self = Self(2097152u32);
    pub const StageInPlace: Self = Self(4194304u32);
}
impl ::core::marker::Copy for DeploymentOptions {}
impl ::core::clone::Clone for DeploymentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl ::core::marker::Copy for DeploymentProgress {}
impl ::core::clone::Clone for DeploymentProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
}
impl ::core::marker::Copy for DeploymentProgressState {}
impl ::core::clone::Clone for DeploymentProgressState {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const NotInstalled: Self = Self(0i32);
    pub const Staged: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for PackageInstallState {}
impl ::core::clone::Clone for PackageInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageManagerDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: Self = Self(0i32);
    pub const LicenseInvalid: Self = Self(1i32);
    pub const Modified: Self = Self(2i32);
    pub const Tampered: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageState {}
impl ::core::clone::Clone for PackageState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: Self = Self(0u32);
    pub const LicenseIssue: Self = Self(1u32);
    pub const Modified: Self = Self(2u32);
    pub const Tampered: Self = Self(4u32);
    pub const Disabled: Self = Self(8u32);
}
impl ::core::marker::Copy for PackageStatus {}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: Self = Self(0i32);
    pub const Stub: Self = Self(1i32);
}
impl ::core::marker::Copy for PackageStubPreference {}
impl ::core::clone::Clone for PackageStubPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: Self = Self(0u32);
    pub const Main: Self = Self(1u32);
    pub const Framework: Self = Self(2u32);
    pub const Resource: Self = Self(4u32);
    pub const Bundle: Self = Self(8u32);
    pub const Xap: Self = Self(16u32);
    pub const Optional: Self = Self(32u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PackageTypes {}
impl ::core::clone::Clone for PackageTypes {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const None: Self = Self(0u32);
    pub const PreserveApplicationData: Self = Self(4096u32);
    pub const PreserveRoamableApplicationData: Self = Self(128u32);
    pub const RemoveForAllUsers: Self = Self(524288u32);
}
impl ::core::marker::Copy for RemovalOptions {}
impl ::core::clone::Clone for RemovalOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SharedPackageContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: Self = Self(0i32);
    pub const MergeWithExisting: Self = Self(1i32);
    pub const ReplaceExisting: Self = Self(2i32);
}
impl ::core::marker::Copy for SharedPackageContainerCreationCollisionOptions {}
impl ::core::clone::Clone for SharedPackageContainerCreationCollisionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SharedPackageContainerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedPackageContainerMember(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const BlockedByPolicy: Self = Self(1i32);
    pub const AlreadyExists: Self = Self(2i32);
    pub const PackageFamilyExistsInAnotherContainer: Self = Self(3i32);
    pub const NotFound: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for SharedPackageContainerOperationStatus {}
impl ::core::clone::Clone for SharedPackageContainerOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StagePackageOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: Self = Self(0i32);
    pub const InstallFull: Self = Self(1i32);
    pub const InstallStub: Self = Self(2i32);
    pub const UsePreference: Self = Self(3i32);
}
impl ::core::marker::Copy for StubPackageOption {}
impl ::core::clone::Clone for StubPackageOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UpdateSharedPackageContainerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UpdateSharedPackageContainerResult(pub *mut ::core::ffi::c_void);

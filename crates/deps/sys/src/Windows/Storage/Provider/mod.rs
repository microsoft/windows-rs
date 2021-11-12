#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: CachedFileOptions = CachedFileOptions(0u32);
    pub const RequireUpdateOnAccess: CachedFileOptions = CachedFileOptions(1u32);
    pub const UseCachedFileWhenOffline: CachedFileOptions = CachedFileOptions(2u32);
    pub const DenyAccessWhenOffline: CachedFileOptions = CachedFileOptions(4u32);
}
#[repr(transparent)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: CachedFileTarget = CachedFileTarget(0i32);
    pub const Remote: CachedFileTarget = CachedFileTarget(1i32);
}
#[repr(transparent)]
pub struct CachedFileUpdaterUI(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CloudFilesContract(i32);
#[repr(transparent)]
pub struct FileUpdateRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileUpdateRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: FileUpdateStatus = FileUpdateStatus(0i32);
    pub const Complete: FileUpdateStatus = FileUpdateStatus(1i32);
    pub const UserInputNeeded: FileUpdateStatus = FileUpdateStatus(2i32);
    pub const CurrentlyUnavailable: FileUpdateStatus = FileUpdateStatus(3i32);
    pub const Failed: FileUpdateStatus = FileUpdateStatus(4i32);
    pub const CompleteAndRenamed: FileUpdateStatus = FileUpdateStatus(5i32);
}
#[repr(transparent)]
pub struct ICachedFileUpdaterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICachedFileUpdaterUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICachedFileUpdaterUI2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileUpdateRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileUpdateRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileUpdateRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderErrorCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderErrorCommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderErrorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderGetContentInfoForPathResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderGetPathForContentUriResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderHandlerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderItemPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderItemProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderItemPropertyDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderItemPropertySource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderPropertyCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderStatusFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderStatusSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderUriSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: ReadActivationMode = ReadActivationMode(0i32);
    pub const BeforeAccess: ReadActivationMode = ReadActivationMode(1i32);
}
#[repr(transparent)]
pub struct StorageProviderError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderErrorCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderFileTypeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderGetContentInfoForPathResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderGetPathForContentUriResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: StorageProviderHardlinkPolicy = StorageProviderHardlinkPolicy(0u32);
    pub const Allowed: StorageProviderHardlinkPolicy = StorageProviderHardlinkPolicy(1u32);
}
#[repr(transparent)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(0i32);
    pub const Progressive: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(1i32);
    pub const Full: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(2i32);
    pub const AlwaysFull: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(3i32);
}
#[repr(transparent)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(0u32);
    pub const ValidationRequired: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(1u32);
    pub const StreamingAllowed: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(2u32);
    pub const AutoDehydrationAllowed: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(4u32);
    pub const AllowFullRestartHydration: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(8u32);
}
#[repr(transparent)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(0u32);
    pub const FileCreationTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(1u32);
    pub const FileReadOnlyAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(2u32);
    pub const FileHiddenAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(4u32);
    pub const FileSystemAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(8u32);
    pub const DirectoryCreationTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(16u32);
    pub const DirectoryReadOnlyAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(32u32);
    pub const DirectoryHiddenAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(64u32);
    pub const DirectorySystemAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(128u32);
    pub const FileLastWriteTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(256u32);
    pub const DirectoryLastWriteTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(512u32);
    pub const PreserveInsyncForSyncEngine: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(2147483648u32);
}
#[repr(transparent)]
pub struct StorageProviderItemProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderItemPropertyDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: StorageProviderPopulationPolicy = StorageProviderPopulationPolicy(1i32);
    pub const AlwaysFull: StorageProviderPopulationPolicy = StorageProviderPopulationPolicy(2i32);
}
#[repr(transparent)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: StorageProviderProtectionMode = StorageProviderProtectionMode(0i32);
    pub const Personal: StorageProviderProtectionMode = StorageProviderProtectionMode(1i32);
}
#[repr(transparent)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: StorageProviderState = StorageProviderState(0i32);
    pub const Syncing: StorageProviderState = StorageProviderState(1i32);
    pub const Paused: StorageProviderState = StorageProviderState(2i32);
    pub const Error: StorageProviderState = StorageProviderState(3i32);
    pub const Warning: StorageProviderState = StorageProviderState(4i32);
    pub const Offline: StorageProviderState = StorageProviderState(5i32);
}
#[repr(transparent)]
pub struct StorageProviderStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderSyncRootInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: StorageProviderUriSourceStatus = StorageProviderUriSourceStatus(0i32);
    pub const NoSyncRoot: StorageProviderUriSourceStatus = StorageProviderUriSourceStatus(1i32);
    pub const FileNotFound: StorageProviderUriSourceStatus = StorageProviderUriSourceStatus(2i32);
}
#[repr(transparent)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: UIStatus = UIStatus(0i32);
    pub const Hidden: UIStatus = UIStatus(1i32);
    pub const Visible: UIStatus = UIStatus(2i32);
    pub const Complete: UIStatus = UIStatus(3i32);
}
#[repr(transparent)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: WriteActivationMode = WriteActivationMode(0i32);
    pub const NotNeeded: WriteActivationMode = WriteActivationMode(1i32);
    pub const AfterWrite: WriteActivationMode = WriteActivationMode(2i32);
}

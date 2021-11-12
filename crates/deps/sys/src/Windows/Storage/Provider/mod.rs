#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl ::core::marker::Copy for CachedFileOptions {}
impl ::core::clone::Clone for CachedFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl ::core::marker::Copy for CachedFileTarget {}
impl ::core::clone::Clone for CachedFileTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CachedFileUpdaterUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CachedFileUpdaterUI {}
impl ::core::clone::Clone for CachedFileUpdaterUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileUpdateRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileUpdateRequest {}
impl ::core::clone::Clone for FileUpdateRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileUpdateRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileUpdateRequestDeferral {}
impl ::core::clone::Clone for FileUpdateRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileUpdateRequestedEventArgs {}
impl ::core::clone::Clone for FileUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl ::core::marker::Copy for FileUpdateStatus {}
impl ::core::clone::Clone for FileUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileUpdaterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileUpdaterStatics {}
impl ::core::clone::Clone for ICachedFileUpdaterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileUpdaterUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileUpdaterUI {}
impl ::core::clone::Clone for ICachedFileUpdaterUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileUpdaterUI2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileUpdaterUI2 {}
impl ::core::clone::Clone for ICachedFileUpdaterUI2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileUpdateRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileUpdateRequest {}
impl ::core::clone::Clone for IFileUpdateRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileUpdateRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileUpdateRequest2 {}
impl ::core::clone::Clone for IFileUpdateRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileUpdateRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileUpdateRequestDeferral {}
impl ::core::clone::Clone for IFileUpdateRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileUpdateRequestedEventArgs {}
impl ::core::clone::Clone for IFileUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderError {}
impl ::core::clone::Clone for IStorageProviderError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderErrorCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderErrorCommand {}
impl ::core::clone::Clone for IStorageProviderErrorCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderErrorCommandFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderErrorCommandFactory {}
impl ::core::clone::Clone for IStorageProviderErrorCommandFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderErrorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderErrorFactory {}
impl ::core::clone::Clone for IStorageProviderErrorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderFileTypeInfo {}
impl ::core::clone::Clone for IStorageProviderFileTypeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderFileTypeInfoFactory {}
impl ::core::clone::Clone for IStorageProviderFileTypeInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderGetContentInfoForPathResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderGetContentInfoForPathResult {}
impl ::core::clone::Clone for IStorageProviderGetContentInfoForPathResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderGetPathForContentUriResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderGetPathForContentUriResult {}
impl ::core::clone::Clone for IStorageProviderGetPathForContentUriResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderHandlerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderHandlerFactory {}
impl ::core::clone::Clone for IStorageProviderHandlerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderItemPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderItemPropertiesStatics {}
impl ::core::clone::Clone for IStorageProviderItemPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderItemProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderItemProperty {}
impl ::core::clone::Clone for IStorageProviderItemProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderItemPropertyDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderItemPropertyDefinition {}
impl ::core::clone::Clone for IStorageProviderItemPropertyDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderItemPropertySource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderItemPropertySource {}
impl ::core::clone::Clone for IStorageProviderItemPropertySource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderPropertyCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderPropertyCapabilities {}
impl ::core::clone::Clone for IStorageProviderPropertyCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderStatus {}
impl ::core::clone::Clone for IStorageProviderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderStatusFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderStatusFactory {}
impl ::core::clone::Clone for IStorageProviderStatusFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderStatusSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderStatusSource {}
impl ::core::clone::Clone for IStorageProviderStatusSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderSyncRootInfo {}
impl ::core::clone::Clone for IStorageProviderSyncRootInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderSyncRootInfo2 {}
impl ::core::clone::Clone for IStorageProviderSyncRootInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderSyncRootInfo3 {}
impl ::core::clone::Clone for IStorageProviderSyncRootInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderSyncRootManagerStatics {}
impl ::core::clone::Clone for IStorageProviderSyncRootManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderSyncRootManagerStatics2 {}
impl ::core::clone::Clone for IStorageProviderSyncRootManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProviderUriSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProviderUriSource {}
impl ::core::clone::Clone for IStorageProviderUriSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl ::core::marker::Copy for ReadActivationMode {}
impl ::core::clone::Clone for ReadActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderError {}
impl ::core::clone::Clone for StorageProviderError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderErrorCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderErrorCommand {}
impl ::core::clone::Clone for StorageProviderErrorCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderFileTypeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderFileTypeInfo {}
impl ::core::clone::Clone for StorageProviderFileTypeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderGetContentInfoForPathResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderGetContentInfoForPathResult {}
impl ::core::clone::Clone for StorageProviderGetContentInfoForPathResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderGetPathForContentUriResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderGetPathForContentUriResult {}
impl ::core::clone::Clone for StorageProviderGetPathForContentUriResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl ::core::marker::Copy for StorageProviderHardlinkPolicy {}
impl ::core::clone::Clone for StorageProviderHardlinkPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicy {}
impl ::core::clone::Clone for StorageProviderHydrationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicyModifier {}
impl ::core::clone::Clone for StorageProviderHydrationPolicyModifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for StorageProviderInSyncPolicy {}
impl ::core::clone::Clone for StorageProviderInSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderItemProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderItemProperty {}
impl ::core::clone::Clone for StorageProviderItemProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderItemPropertyDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderItemPropertyDefinition {}
impl ::core::clone::Clone for StorageProviderItemPropertyDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderPopulationPolicy {}
impl ::core::clone::Clone for StorageProviderPopulationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageProviderProtectionMode {}
impl ::core::clone::Clone for StorageProviderProtectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl ::core::marker::Copy for StorageProviderState {}
impl ::core::clone::Clone for StorageProviderState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderStatus {}
impl ::core::clone::Clone for StorageProviderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderSyncRootInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProviderSyncRootInfo {}
impl ::core::clone::Clone for StorageProviderSyncRootInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderUriSourceStatus {}
impl ::core::clone::Clone for StorageProviderUriSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl ::core::marker::Copy for UIStatus {}
impl ::core::clone::Clone for UIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for WriteActivationMode {}
impl ::core::clone::Clone for WriteActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CachedFileOptions(i32);
pub struct CachedFileTarget(i32);
#[repr(transparent)]
pub struct CachedFileUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CachedFileUpdaterUI(pub *mut ::core::ffi::c_void);
pub struct CloudFilesContract(i32);
#[repr(transparent)]
pub struct FileUpdateRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileUpdateRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileUpdateRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct FileUpdateStatus(i32);
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
pub struct ReadActivationMode(i32);
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
pub struct StorageProviderHardlinkPolicy(i32);
pub struct StorageProviderHydrationPolicy(i32);
pub struct StorageProviderHydrationPolicyModifier(i32);
pub struct StorageProviderInSyncPolicy(i32);
#[repr(transparent)]
pub struct StorageProviderItemProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderItemProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderItemPropertyDefinition(pub *mut ::core::ffi::c_void);
pub struct StorageProviderPopulationPolicy(i32);
pub struct StorageProviderProtectionMode(i32);
pub struct StorageProviderState(i32);
#[repr(transparent)]
pub struct StorageProviderStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderSyncRootInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageProviderSyncRootManager(pub *mut ::core::ffi::c_void);
pub struct StorageProviderUriSourceStatus(i32);
pub struct UIStatus(i32);
pub struct WriteActivationMode(i32);

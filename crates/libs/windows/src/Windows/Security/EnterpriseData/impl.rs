#[cfg(feature = "implement_exclusive")]
pub trait IBufferProtectUnprotectResultImpl: Sized {
    fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ProtectionInfo(&self) -> ::windows::core::Result<DataProtectionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionInfoImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DataProtectionStatus>;
    fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionManagerStaticsImpl: Sized {
    fn ProtectAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>, identity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>;
    fn UnprotectAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>;
    fn ProtectStreamAsync(&self, unprotectedstream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, identity: &::windows::core::HSTRING, protectedstream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
    fn UnprotectStreamAsync(&self, protectedstream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, unprotectedstream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
    fn GetProtectionInfoAsync(&self, protecteddata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
    fn GetStreamProtectionInfoAsync(&self, protectedstream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionInfoImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<FileProtectionStatus>;
    fn IsRoamable(&self) -> ::windows::core::Result<bool>;
    fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionInfo2Impl: Sized {
    fn IsProtectWhileOpenSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionManagerStaticsImpl: Sized {
    fn ProtectAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>, identity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
    fn CopyProtectionAsync(&self, source: &::core::option::Option<super::super::Storage::IStorageItem>, target: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetProtectionInfoAsync(&self, source: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
    fn SaveFileAsContainerAsync(&self, protectedfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>;
    fn LoadFileFromContainerAsync(&self, containerfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>;
    fn LoadFileFromContainerWithTargetAsync(&self, containerfile: &::core::option::Option<super::super::Storage::IStorageFile>, target: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>;
    fn CreateProtectedAndOpenAsync(&self, parentfolder: &::core::option::Option<super::super::Storage::IStorageFolder>, desiredname: &::windows::core::HSTRING, identity: &::windows::core::HSTRING, collisionoption: super::super::Storage::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionManagerStatics2Impl: Sized {
    fn IsContainerAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync(&self, containerfile: &::core::option::Option<super::super::Storage::IStorageFile>, target: &::core::option::Option<super::super::Storage::IStorageItem>, collisionoption: super::super::Storage::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>;
    fn SaveFileAsContainerWithSharingAsync(&self, protectedfile: &::core::option::Option<super::super::Storage::IStorageFile>, sharedwithidentities: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionManagerStatics3Impl: Sized {
    fn UnprotectAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
    fn UnprotectWithOptionsAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>, options: &::core::option::Option<FileUnprotectOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IFileRevocationManagerStaticsImpl: Sized {
    fn ProtectAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>, enterpriseidentity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>;
    fn CopyProtectionAsync(&self, sourcestorageitem: &::core::option::Option<super::super::Storage::IStorageItem>, targetstorageitem: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Revoke(&self, enterpriseidentity: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetStatusAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUnprotectOptionsImpl: Sized {
    fn SetAudit(&self, value: bool) -> ::windows::core::Result<()>;
    fn Audit(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUnprotectOptionsFactoryImpl: Sized {
    fn Create(&self, audit: bool) -> ::windows::core::Result<FileUnprotectOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedAccessResumedEventArgsImpl: Sized {
    fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedAccessSuspendingEventArgsImpl: Sized {
    fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedContainerExportResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ProtectedImportExportStatus>;
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedContainerImportResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ProtectedImportExportStatus>;
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedContentRevokedEventArgsImpl: Sized {
    fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedFileCreateResultImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn Stream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn ProtectionInfo(&self) -> ::windows::core::Result<FileProtectionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyAuditInfoImpl: Sized {
    fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows::core::Result<()>;
    fn Action(&self) -> ::windows::core::Result<ProtectionPolicyAuditAction>;
    fn SetDataDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DataDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSourceDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyAuditInfoFactoryImpl: Sized {
    fn Create(&self, action: ProtectionPolicyAuditAction, datadescription: &::windows::core::HSTRING, sourcedescription: &::windows::core::HSTRING, targetdescription: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyAuditInfo>;
    fn CreateWithActionAndDataDescription(&self, action: ProtectionPolicyAuditAction, datadescription: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyAuditInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerImpl: Sized {
    fn SetIdentity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManager2Impl: Sized {
    fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowEnterpriseIndicator(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStaticsImpl: Sized {
    fn IsIdentityManaged(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryApplyProcessUIPolicy(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn ClearProcessUIPolicy(&self) -> ::windows::core::Result<()>;
    fn CreateCurrentThreadNetworkContext(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ThreadNetworkContext>;
    fn GetPrimaryManagedIdentityForNetworkEndpointAsync(&self, endpointhost: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RevokeContent(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<ProtectionPolicyManager>;
    fn ProtectedAccessSuspending(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectedAccessSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProtectedAccessResumed(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectedAccessResumed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProtectedContentRevoked(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectedContentRevoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CheckAccess(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyEvaluationResult>;
    fn RequestAccessAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStatics2Impl: Sized {
    fn HasContentBeenRevokedSince(&self, identity: &::windows::core::HSTRING, since: &super::super::Foundation::DateTime) -> ::windows::core::Result<bool>;
    fn CheckAccessForApp(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyEvaluationResult>;
    fn RequestAccessForAppAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn GetEnforcementLevel(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<EnforcementLevel>;
    fn IsUserDecryptionAllowed(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsProtectionUnderLockRequired(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn PolicyChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePolicyChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsProtectionEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStatics3Impl: Sized {
    fn RequestAccessWithAuditingInfoAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessWithMessageAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessForAppWithAuditingInfoAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessForAppWithMessageAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn LogAuditEvent(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStatics4Impl: Sized {
    fn IsRoamableProtectionEnabled(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn RequestAccessWithBehaviorAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessForAppWithBehaviorAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForAppAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForProcessAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, processid: u32, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, processid: u32, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn IsFileProtectionRequiredAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>, identity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IsFileProtectionRequiredForNewFileAsync(&self, parentfolder: &::core::option::Option<super::super::Storage::IStorageFolder>, identity: &::windows::core::HSTRING, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn PrimaryManagedIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPrimaryManagedIdentityForIdentity(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThreadNetworkContextImpl: Sized {}

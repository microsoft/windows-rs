pub trait IAppxBlockMapBlockImpl: Sized {
    fn GetHash();
    fn GetCompressedSize();
}
pub trait IAppxBlockMapBlocksEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxBlockMapFileImpl: Sized {
    fn GetBlocks();
    fn GetLocalFileHeaderSize();
    fn GetName();
    fn GetUncompressedSize();
    fn ValidateFileHash();
}
pub trait IAppxBlockMapFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxBlockMapReaderImpl: Sized {
    fn GetFile();
    fn GetFiles();
    fn GetHashMethod();
    fn GetStream();
}
pub trait IAppxBundleFactoryImpl: Sized {
    fn CreateBundleWriter();
    fn CreateBundleReader();
    fn CreateBundleManifestReader();
}
pub trait IAppxBundleManifestOptionalBundleInfoImpl: Sized {
    fn GetPackageId();
    fn GetFileName();
    fn GetPackageInfoItems();
}
pub trait IAppxBundleManifestOptionalBundleInfoEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxBundleManifestPackageInfoImpl: Sized {
    fn GetPackageType();
    fn GetPackageId();
    fn GetFileName();
    fn GetOffset();
    fn GetSize();
    fn GetResources();
}
pub trait IAppxBundleManifestPackageInfo2Impl: Sized {
    fn GetIsPackageReference();
    fn GetIsNonQualifiedResourcePackage();
    fn GetIsDefaultApplicablePackage();
}
pub trait IAppxBundleManifestPackageInfo3Impl: Sized {
    fn GetTargetDeviceFamilies();
}
pub trait IAppxBundleManifestPackageInfo4Impl: Sized {
    fn GetIsStub();
}
pub trait IAppxBundleManifestPackageInfoEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxBundleManifestReaderImpl: Sized {
    fn GetPackageId();
    fn GetPackageInfoItems();
    fn GetStream();
}
pub trait IAppxBundleManifestReader2Impl: Sized {
    fn GetOptionalBundles();
}
pub trait IAppxBundleReaderImpl: Sized {
    fn GetFootprintFile();
    fn GetBlockMap();
    fn GetManifest();
    fn GetPayloadPackages();
    fn GetPayloadPackage();
}
pub trait IAppxBundleWriterImpl: Sized {
    fn AddPayloadPackage();
    fn Close();
}
pub trait IAppxBundleWriter2Impl: Sized {
    fn AddExternalPackageReference();
}
pub trait IAppxBundleWriter3Impl: Sized {
    fn AddPackageReference();
    fn Close();
}
pub trait IAppxBundleWriter4Impl: Sized {
    fn AddPayloadPackage();
    fn AddPackageReference();
    fn AddExternalPackageReference();
}
pub trait IAppxContentGroupImpl: Sized {
    fn GetName();
    fn GetFiles();
}
pub trait IAppxContentGroupFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxContentGroupMapReaderImpl: Sized {
    fn GetRequiredGroup();
    fn GetAutomaticGroups();
}
pub trait IAppxContentGroupMapWriterImpl: Sized {
    fn AddAutomaticGroup();
    fn AddAutomaticFile();
    fn Close();
}
pub trait IAppxContentGroupsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxEncryptedBundleWriterImpl: Sized {
    fn AddPayloadPackageEncrypted();
    fn Close();
}
pub trait IAppxEncryptedBundleWriter2Impl: Sized {
    fn AddExternalPackageReference();
}
pub trait IAppxEncryptedBundleWriter3Impl: Sized {
    fn AddPayloadPackageEncrypted();
    fn AddExternalPackageReference();
}
pub trait IAppxEncryptedPackageWriterImpl: Sized {
    fn AddPayloadFileEncrypted();
    fn Close();
}
pub trait IAppxEncryptedPackageWriter2Impl: Sized {
    fn AddPayloadFilesEncrypted();
}
pub trait IAppxEncryptionFactoryImpl: Sized {
    fn EncryptPackage();
    fn DecryptPackage();
    fn CreateEncryptedPackageWriter();
    fn CreateEncryptedPackageReader();
    fn EncryptBundle();
    fn DecryptBundle();
    fn CreateEncryptedBundleWriter();
    fn CreateEncryptedBundleReader();
}
pub trait IAppxEncryptionFactory2Impl: Sized {
    fn CreateEncryptedPackageWriter();
}
pub trait IAppxEncryptionFactory3Impl: Sized {
    fn EncryptPackage();
    fn CreateEncryptedPackageWriter();
    fn EncryptBundle();
    fn CreateEncryptedBundleWriter();
}
pub trait IAppxEncryptionFactory4Impl: Sized {
    fn EncryptPackage();
}
pub trait IAppxFactoryImpl: Sized {
    fn CreatePackageWriter();
    fn CreatePackageReader();
    fn CreateManifestReader();
    fn CreateBlockMapReader();
    fn CreateValidatedBlockMapReader();
}
pub trait IAppxFactory2Impl: Sized {
    fn CreateContentGroupMapReader();
    fn CreateSourceContentGroupMapReader();
    fn CreateContentGroupMapWriter();
}
pub trait IAppxFileImpl: Sized {
    fn GetCompressionOption();
    fn GetContentType();
    fn GetName();
    fn GetSize();
    fn GetStream();
}
pub trait IAppxFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestApplicationImpl: Sized {
    fn GetStringValue();
    fn GetAppUserModelId();
}
pub trait IAppxManifestApplicationsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestCapabilitiesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestDeviceCapabilitiesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestDriverConstraintImpl: Sized {
    fn GetName();
    fn GetMinVersion();
    fn GetMinDate();
}
pub trait IAppxManifestDriverConstraintsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestDriverDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestDriverDependencyImpl: Sized {
    fn GetDriverConstraints();
}
pub trait IAppxManifestHostRuntimeDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestHostRuntimeDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetMinVersion();
}
pub trait IAppxManifestHostRuntimeDependency2Impl: Sized {
    fn GetPackageFamilyName();
}
pub trait IAppxManifestMainPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestMainPackageDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetPackageFamilyName();
}
pub trait IAppxManifestOSPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestOSPackageDependencyImpl: Sized {
    fn GetName();
    fn GetVersion();
}
pub trait IAppxManifestOptionalPackageInfoImpl: Sized {
    fn GetIsOptionalPackage();
    fn GetMainPackageName();
}
pub trait IAppxManifestPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestPackageDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetMinVersion();
}
pub trait IAppxManifestPackageDependency2Impl: Sized + IAppxManifestPackageDependencyImpl {
    fn GetMaxMajorVersionTested();
}
pub trait IAppxManifestPackageDependency3Impl: Sized {
    fn GetIsOptional();
}
pub trait IAppxManifestPackageIdImpl: Sized {
    fn GetName();
    fn GetArchitecture();
    fn GetPublisher();
    fn GetVersion();
    fn GetResourceId();
    fn ComparePublisher();
    fn GetPackageFullName();
    fn GetPackageFamilyName();
}
pub trait IAppxManifestPackageId2Impl: Sized + IAppxManifestPackageIdImpl {
    fn GetArchitecture2();
}
pub trait IAppxManifestPropertiesImpl: Sized {
    fn GetBoolValue();
    fn GetStringValue();
}
pub trait IAppxManifestQualifiedResourceImpl: Sized {
    fn GetLanguage();
    fn GetScale();
    fn GetDXFeatureLevel();
}
pub trait IAppxManifestQualifiedResourcesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestReaderImpl: Sized {
    fn GetPackageId();
    fn GetProperties();
    fn GetPackageDependencies();
    fn GetCapabilities();
    fn GetResources();
    fn GetDeviceCapabilities();
    fn GetPrerequisite();
    fn GetApplications();
    fn GetStream();
}
pub trait IAppxManifestReader2Impl: Sized + IAppxManifestReaderImpl {
    fn GetQualifiedResources();
}
pub trait IAppxManifestReader3Impl: Sized + IAppxManifestReader2Impl + IAppxManifestReaderImpl {
    fn GetCapabilitiesByCapabilityClass();
    fn GetTargetDeviceFamilies();
}
pub trait IAppxManifestReader4Impl: Sized + IAppxManifestReader3Impl + IAppxManifestReader2Impl + IAppxManifestReaderImpl {
    fn GetOptionalPackageInfo();
}
pub trait IAppxManifestReader5Impl: Sized {
    fn GetMainPackageDependencies();
}
pub trait IAppxManifestReader6Impl: Sized {
    fn GetIsNonQualifiedResourcePackage();
}
pub trait IAppxManifestReader7Impl: Sized {
    fn GetDriverDependencies();
    fn GetOSPackageDependencies();
    fn GetHostRuntimeDependencies();
}
pub trait IAppxManifestResourcesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestTargetDeviceFamiliesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
pub trait IAppxManifestTargetDeviceFamilyImpl: Sized {
    fn GetName();
    fn GetMinVersion();
    fn GetMaxVersionTested();
}
pub trait IAppxPackageEditorImpl: Sized {
    fn SetWorkingDirectory();
    fn CreateDeltaPackage();
    fn CreateDeltaPackageUsingBaselineBlockMap();
    fn UpdatePackage();
    fn UpdateEncryptedPackage();
    fn UpdatePackageManifest();
}
pub trait IAppxPackageReaderImpl: Sized {
    fn GetBlockMap();
    fn GetFootprintFile();
    fn GetPayloadFile();
    fn GetPayloadFiles();
    fn GetManifest();
}
pub trait IAppxPackageWriterImpl: Sized {
    fn AddPayloadFile();
    fn Close();
}
pub trait IAppxPackageWriter2Impl: Sized {
    fn Close();
}
pub trait IAppxPackageWriter3Impl: Sized {
    fn AddPayloadFiles();
}
pub trait IAppxPackagingDiagnosticEventSinkImpl: Sized {
    fn ReportContextChange();
    fn ReportError();
}
pub trait IAppxPackagingDiagnosticEventSinkManagerImpl: Sized {
    fn SetSinkForProcess();
}
pub trait IAppxSourceContentGroupMapReaderImpl: Sized {
    fn GetRequiredGroup();
    fn GetAutomaticGroups();
}

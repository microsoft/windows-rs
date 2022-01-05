#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkplaceId(&self) -> ::windows::core::Result<i32>;
    fn EnrollmentValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EnrollmentValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Status(&self) -> ::windows::core::Result<EnterpriseStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseEnrollmentManagerImpl: Sized {
    fn EnrolledEnterprises(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Enterprise>>;
    fn CurrentEnterprise(&self) -> ::windows::core::Result<Enterprise>;
    fn ValidateEnterprisesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RequestEnrollmentAsync(&self, enrollmenttoken: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<EnterpriseEnrollmentResult>>;
    fn RequestUnenrollmentAsync(&self, enterprise: &::core::option::Option<Enterprise>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseEnrollmentResultImpl: Sized {
    fn EnrolledEnterprise(&self) -> ::windows::core::Result<Enterprise>;
    fn Status(&self) -> ::windows::core::Result<EnterpriseEnrollmentStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstallationManagerStaticsImpl: Sized {
    fn AddPackageAsync(&self, title: &::windows::core::HSTRING, sourcelocation: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn AddPackagePreloadedAsync(&self, title: &::windows::core::HSTRING, sourcelocation: &::core::option::Option<super::super::super::Foundation::Uri>, instanceid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, license: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn GetPendingPackageInstalls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>>;
    fn FindPackagesForCurrentPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>;
    fn FindPackages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstallationManagerStatics2Impl: Sized {
    fn RemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING, removaloptions: super::super::super::Management::Deployment::RemovalOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn RegisterPackageAsync(&self, manifesturi: &::core::option::Option<super::super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Uri>>, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>;
    fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::super::super::ApplicationModel::Package>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallResultImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallState(&self) -> ::windows::core::Result<super::super::super::Management::Deployment::PackageInstallState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallResult2Impl: Sized {
    fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}

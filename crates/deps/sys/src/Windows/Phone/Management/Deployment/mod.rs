#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Enterprise();
    fn EnterpriseEnrollmentManager();
    fn EnterpriseEnrollmentResult();
    fn EnterpriseEnrollmentStatus();
    fn EnterpriseStatus();
    fn IEnterprise();
    fn IEnterpriseEnrollmentManager();
    fn IEnterpriseEnrollmentResult();
    fn IInstallationManagerStatics();
    fn IInstallationManagerStatics2();
    fn IPackageInstallResult();
    fn IPackageInstallResult2();
    fn InstallationManager();
    fn PackageInstallResult();
}

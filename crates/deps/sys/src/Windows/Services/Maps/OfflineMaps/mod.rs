#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IOfflineMapPackage();
    fn IOfflineMapPackageQueryResult();
    fn IOfflineMapPackageStartDownloadResult();
    fn IOfflineMapPackageStatics();
    fn OfflineMapPackage();
    fn OfflineMapPackageQueryResult();
    fn OfflineMapPackageQueryStatus();
    fn OfflineMapPackageStartDownloadResult();
    fn OfflineMapPackageStartDownloadStatus();
    fn OfflineMapPackageStatus();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ID_DOCUMENTPACKAGETARGET_MSXPS();
    fn ID_DOCUMENTPACKAGETARGET_OPENXPS();
    fn ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D();
    fn IPrintDocumentPackageStatusEvent();
    fn IPrintDocumentPackageTarget();
    fn IPrintDocumentPackageTargetFactory();
    fn IXpsPrintJob();
    fn IXpsPrintJobStream();
    fn PrintDocumentPackageCompletion();
    fn PrintDocumentPackageStatus();
    fn PrintDocumentPackageTarget();
    fn PrintDocumentPackageTargetFactory();
    fn StartXpsPrintJob();
    fn StartXpsPrintJob1();
    fn XPS_JOB_COMPLETION();
    fn XPS_JOB_STATUS();
}

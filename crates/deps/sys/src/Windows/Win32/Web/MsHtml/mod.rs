#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ComputeInvCMAP();
    fn CreateDDrawSurfaceOnDIB();
    fn CreateMIMEMap();
    fn DecodeImage();
    fn DecodeImageEx();
    fn DitherTo8();
    fn DoPrivacyDlg();
    fn GetMaxMIMEIDBytes();
    fn IdentifyMIMEType();
    fn RatingAccessDeniedDialog();
    fn RatingAccessDeniedDialog2();
    fn RatingAccessDeniedDialog2W();
    fn RatingAccessDeniedDialogW();
    fn RatingAddToApprovedSites();
    fn RatingCheckUserAccess();
    fn RatingCheckUserAccessW();
    fn RatingClickedOnPRFInternal();
    fn RatingClickedOnRATInternal();
    fn RatingEnable();
    fn RatingEnableW();
    fn RatingEnabledQuery();
    fn RatingFreeDetails();
    fn RatingInit();
    fn RatingObtainCancel();
    fn RatingObtainQuery();
    fn RatingObtainQueryW();
    fn RatingSetupUI();
    fn RatingSetupUIW();
    fn SniffStream();
}

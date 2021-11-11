#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn COMDLG_FILTERSPEC();
    fn DEVICE_SCALE_FACTOR();
    fn IObjectArray();
    fn IObjectCollection();
    fn ITEMIDLIST();
    fn PERCEIVED();
    fn PERCEIVEDFLAG_GDIPLUS();
    fn PERCEIVEDFLAG_HARDCODED();
    fn PERCEIVEDFLAG_NATIVESUPPORT();
    fn PERCEIVEDFLAG_SOFTCODED();
    fn PERCEIVEDFLAG_UNDEFINED();
    fn PERCEIVEDFLAG_WMSDK();
    fn PERCEIVEDFLAG_ZIPFOLDER();
    fn SHCOLSTATE();
    fn SHELLDETAILS();
    fn SHITEMID();
    fn STRRET();
    fn STRRET_TYPE();
}

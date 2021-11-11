#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IQuickLink();
    fn IShareOperation();
    fn IShareOperation2();
    fn IShareOperation3();
    fn QuickLink();
    fn ShareOperation();
}

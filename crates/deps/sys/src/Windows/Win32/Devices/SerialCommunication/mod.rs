#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ComDBClaimNextFreePort();
    fn ComDBClaimPort();
    fn ComDBClose();
    fn ComDBGetCurrentPortUsage();
    fn ComDBOpen();
    fn ComDBReleasePort();
    fn ComDBResizeDatabase();
}

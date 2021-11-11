#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DSCreateISecurityInfoObject();
    fn DSCreateISecurityInfoObjectEx();
    fn DSCreateSecurityPage();
    fn DSEditSecurity();
    fn DSSI_IS_ROOT();
    fn DSSI_NO_ACCESS_CHECK();
    fn DSSI_NO_EDIT_OWNER();
    fn DSSI_NO_EDIT_SACL();
    fn DSSI_NO_FILTER();
    fn DSSI_NO_READONLY_MESSAGE();
    fn DSSI_READ_ONLY();
    fn PFNDSCREATEISECINFO();
    fn PFNDSCREATEISECINFOEX();
    fn PFNDSCREATESECPAGE();
    fn PFNDSEDITSECURITY();
    fn PFNREADOBJECTSECURITY();
    fn PFNWRITEOBJECTSECURITY();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DSCreateISecurityInfoObject();
    fn DSCreateISecurityInfoObjectEx();
    fn DSCreateSecurityPage();
    fn DSEditSecurity();
}

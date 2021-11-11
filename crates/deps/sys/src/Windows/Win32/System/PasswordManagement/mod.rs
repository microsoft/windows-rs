#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CYPHER_BLOCK();
    fn ENCRYPTED_LM_OWF_PASSWORD();
    fn LM_OWF_PASSWORD();
    fn MSChapSrvChangePassword();
    fn MSChapSrvChangePassword2();
    fn SAMPR_ENCRYPTED_USER_PASSWORD();
}

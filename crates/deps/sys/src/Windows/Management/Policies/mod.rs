#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn INamedPolicyData();
    fn INamedPolicyStatics();
    fn NamedPolicy();
    fn NamedPolicyData();
    fn NamedPolicyKind();
}

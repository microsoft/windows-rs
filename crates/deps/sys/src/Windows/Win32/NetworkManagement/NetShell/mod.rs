#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn MatchEnumTag();
    fn MatchToken();
    fn PreprocessCommand();
    fn PrintError();
    fn PrintMessage();
    fn PrintMessageFromModule();
    fn RegisterContext();
    fn RegisterHelper();
}

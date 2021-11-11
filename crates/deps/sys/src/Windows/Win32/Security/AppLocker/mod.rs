#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn SaferCloseLevel();
    fn SaferComputeTokenFromLevel();
    fn SaferCreateLevel();
    fn SaferGetLevelInformation();
    fn SaferGetPolicyInformation();
    fn SaferIdentifyLevel();
    fn SaferRecordEventLogEntry();
    fn SaferSetLevelInformation();
    fn SaferSetPolicyInformation();
    fn SaferiIsExecutableFileType();
}

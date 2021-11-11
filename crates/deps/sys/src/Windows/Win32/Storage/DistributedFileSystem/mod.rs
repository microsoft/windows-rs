#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn NetDfsAdd();
    fn NetDfsAddFtRoot();
    fn NetDfsAddRootTarget();
    fn NetDfsAddStdRoot();
    fn NetDfsEnum();
    fn NetDfsGetClientInfo();
    fn NetDfsGetFtContainerSecurity();
    fn NetDfsGetInfo();
    fn NetDfsGetSecurity();
    fn NetDfsGetStdContainerSecurity();
    fn NetDfsGetSupportedNamespaceVersion();
    fn NetDfsMove();
    fn NetDfsRemove();
    fn NetDfsRemoveFtRoot();
    fn NetDfsRemoveFtRootForced();
    fn NetDfsRemoveRootTarget();
    fn NetDfsRemoveStdRoot();
    fn NetDfsSetClientInfo();
    fn NetDfsSetFtContainerSecurity();
    fn NetDfsSetInfo();
    fn NetDfsSetSecurity();
    fn NetDfsSetStdContainerSecurity();
}

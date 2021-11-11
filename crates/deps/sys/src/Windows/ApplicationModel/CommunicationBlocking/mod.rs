#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CommunicationBlockingAccessManager();
    fn CommunicationBlockingAppManager();
    fn CommunicationBlockingContract();
    fn ICommunicationBlockingAccessManagerStatics();
    fn ICommunicationBlockingAppManagerStatics();
    fn ICommunicationBlockingAppManagerStatics2();
}

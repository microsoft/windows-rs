#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FilterAttach();
    fn FilterAttachAtAltitude();
    fn FilterClose();
    fn FilterConnectCommunicationPort();
    fn FilterCreate();
    fn FilterDetach();
    fn FilterFindClose();
    fn FilterFindFirst();
    fn FilterFindNext();
    fn FilterGetDosName();
    fn FilterGetInformation();
    fn FilterGetMessage();
    fn FilterInstanceClose();
    fn FilterInstanceCreate();
    fn FilterInstanceFindClose();
    fn FilterInstanceFindFirst();
    fn FilterInstanceFindNext();
    fn FilterInstanceGetInformation();
    fn FilterLoad();
    fn FilterReplyMessage();
    fn FilterSendMessage();
    fn FilterUnload();
    fn FilterVolumeFindClose();
    fn FilterVolumeFindFirst();
    fn FilterVolumeFindNext();
    fn FilterVolumeInstanceFindClose();
    fn FilterVolumeInstanceFindFirst();
    fn FilterVolumeInstanceFindNext();
}

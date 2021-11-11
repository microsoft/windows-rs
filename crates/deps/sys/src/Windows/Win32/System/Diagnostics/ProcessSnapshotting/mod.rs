#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn PssCaptureSnapshot();
    fn PssDuplicateSnapshot();
    fn PssFreeSnapshot();
    fn PssQuerySnapshot();
    fn PssWalkMarkerCreate();
    fn PssWalkMarkerFree();
    fn PssWalkMarkerGetPosition();
    fn PssWalkMarkerSeekToBeginning();
    fn PssWalkMarkerSetPosition();
    fn PssWalkSnapshot();
}

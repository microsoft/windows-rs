#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3DPERF_BeginEvent();
    fn D3DPERF_EndEvent();
    fn D3DPERF_GetStatus();
    fn D3DPERF_QueryRepeatFrame();
    fn D3DPERF_SetMarker();
    fn D3DPERF_SetOptions();
    fn D3DPERF_SetRegion();
    fn Direct3DCreate9();
    fn Direct3DCreate9Ex();
}

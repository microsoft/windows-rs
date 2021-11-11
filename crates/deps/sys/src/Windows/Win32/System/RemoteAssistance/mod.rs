#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DISPID_EVENT_ON_CONTEXT_DATA();
    fn DISPID_EVENT_ON_SEND_ERROR();
    fn DISPID_EVENT_ON_STATE_CHANGED();
    fn DISPID_EVENT_ON_TERMINATION();
    fn DRendezvousSessionEvents();
    fn IRendezvousApplication();
    fn IRendezvousSession();
    fn RENDEZVOUS_SESSION_FLAGS();
    fn RENDEZVOUS_SESSION_STATE();
    fn RendezvousApplication();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn KsCreateAllocator();
    fn KsCreateAllocator2();
    fn KsCreateClock();
    fn KsCreateClock2();
    fn KsCreatePin();
    fn KsCreatePin2();
    fn KsCreateTopologyNode();
    fn KsCreateTopologyNode2();
}

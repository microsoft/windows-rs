#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPreallocatedWorkItem();
    fn IPreallocatedWorkItemFactory();
    fn ISignalNotifier();
    fn ISignalNotifierStatics();
    fn PreallocatedWorkItem();
    fn SignalHandler();
    fn SignalNotifier();
}

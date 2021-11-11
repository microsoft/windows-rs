#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPnpObject();
    fn IPnpObjectStatics();
    fn IPnpObjectUpdate();
    fn IPnpObjectWatcher();
    fn PnpObject();
    fn PnpObjectCollection();
    fn PnpObjectType();
    fn PnpObjectUpdate();
    fn PnpObjectWatcher();
}

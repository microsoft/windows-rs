#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DevCloseObjectQuery();
    fn DevCreateObjectQuery();
    fn DevCreateObjectQueryEx();
    fn DevCreateObjectQueryFromId();
    fn DevCreateObjectQueryFromIdEx();
    fn DevCreateObjectQueryFromIds();
    fn DevCreateObjectQueryFromIdsEx();
    fn DevFindProperty();
    fn DevFreeObjectProperties();
    fn DevFreeObjects();
    fn DevGetObjectProperties();
    fn DevGetObjectPropertiesEx();
    fn DevGetObjects();
    fn DevGetObjectsEx();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DEVPROP_FILTER_EXPRESSION();
    fn DEVPROP_OPERATOR();
    fn DEV_OBJECT();
    fn DEV_OBJECT_TYPE();
    fn DEV_QUERY_FLAGS();
    fn DEV_QUERY_PARAMETER();
    fn DEV_QUERY_RESULT_ACTION();
    fn DEV_QUERY_RESULT_ACTION_DATA();
    fn DEV_QUERY_STATE();
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
    fn HDEVQUERY__();
    fn PDEV_QUERY_RESULT_CALLBACK();
}

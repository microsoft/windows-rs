#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CloseIMsgSession();
    fn GetAttribIMsgOnIStg();
    fn MapStorageSCode();
    fn OpenIMsgOnIStg();
    fn OpenIMsgSession();
    fn SetAttribIMsgOnIStg();
}

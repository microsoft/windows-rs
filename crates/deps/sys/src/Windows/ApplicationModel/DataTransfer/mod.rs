#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
#[link(name = "windows")]
extern "system" {}

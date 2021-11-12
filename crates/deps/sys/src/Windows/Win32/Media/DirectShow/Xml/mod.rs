#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_XMLGraphBuilder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 464542049, data2: 24511, data3: 4562, data4: [165, 33, 68, 223, 7, 193, 0, 0] };
pub struct IXMLGraphBuilder(i32);

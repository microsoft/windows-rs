use com::interfaces::iunknown::IUnknown;
use std::ffi::c_void;

#[com::com_interface("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")]
pub trait IInspectable: IUnknown {
    unsafe fn get_iids(&self, iid_count: *mut u32, iids: *mut *mut com::sys::IID);
    unsafe fn get_runtime_class_name(&self, class_name: *mut crate::String);
    unsafe fn get_trust_level(&self, trust_level: *mut c_void);
}

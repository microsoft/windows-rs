windows_core::imp::define_interface!(IThing, IThing_Vtbl);
#[repr(C)]
pub struct IThing_Vtbl {
    Value: usize,
}

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IGuidHelperStatics,
    IGuidHelperStatics_Vtbl,
    0x59c7966b_ae52_5283_ad7f_a1b9e9678add
);
windows_core::imp::interface_hierarchy!(
    IGuidHelperStatics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IGuidHelperStatics {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IGuidHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGuidHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
pub struct GuidHelper;
impl GuidHelper {}
impl windows_core::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}

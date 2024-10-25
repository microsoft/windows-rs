windows_core::imp::define_interface!(IHtmlUtilities, IHtmlUtilities_Vtbl, 0xfec00add_2399_4fac_b5a7_05e9acd7181d);
impl windows_core::RuntimeType for IHtmlUtilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHtmlUtilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConvertToText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
pub struct HtmlUtilities;
impl HtmlUtilities {}
impl windows_core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}

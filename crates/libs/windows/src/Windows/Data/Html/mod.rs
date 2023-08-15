#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlUtilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHtmlUtilities {
    type Vtable = IHtmlUtilities_Vtbl;
}
impl ::core::clone::Clone for IHtmlUtilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHtmlUtilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfec00add_2399_4fac_b5a7_05e9acd7181d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlUtilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConvertToText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, html: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Data_Html\"`*"]
pub struct HtmlUtilities;
impl HtmlUtilities {
    pub fn ConvertToText(html: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertToText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(html), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HtmlUtilities, IHtmlUtilities> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}

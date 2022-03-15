#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Data_Html\"`*"]
pub struct HtmlUtilities {}
impl HtmlUtilities {
    #[doc = "*Required features: `\"Data_Html\"`*"]
    pub fn ConvertToText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(html: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertToText)(::core::mem::transmute_copy(this), html.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HtmlUtilities, IHtmlUtilities> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlUtilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHtmlUtilities {
    type Vtable = IHtmlUtilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfec00add_2399_4fac_b5a7_05e9acd7181d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlUtilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ConvertToText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, html: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

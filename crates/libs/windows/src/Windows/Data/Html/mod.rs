#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlUtilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHtmlUtilities {
    type Vtable = IHtmlUtilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IHtmlUtilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfec00add_2399_4fac_b5a7_05e9acd7181d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlUtilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ConvertToText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, html: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Html\"`*"]
pub struct HtmlUtilities;
impl HtmlUtilities {
    pub fn ConvertToText(html: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertToText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(html), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HtmlUtilities, IHtmlUtilities> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

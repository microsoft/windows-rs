#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Data_Html`*"]
pub struct HtmlUtilities {}
impl HtmlUtilities {
    #[doc = "*Required features: `Data_Html`*"]
    pub fn ConvertToText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(html: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), html.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HtmlUtilities, IHtmlUtilities> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHtmlUtilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHtmlUtilities {
    type Vtable = IHtmlUtilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfec00add_2399_4fac_b5a7_05e9acd7181d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlUtilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, html: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);

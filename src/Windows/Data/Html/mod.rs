#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Data_Html`*"]
pub struct HtmlUtilities {}
impl HtmlUtilities {
    #[doc = "*Required features: `Data_Html`*"]
    pub fn ConvertToText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(html: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), html.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HtmlUtilities, IHtmlUtilities> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHtmlUtilities(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHtmlUtilities {
    type Vtable = IHtmlUtilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfec00add_2399_4fac_b5a7_05e9acd7181d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlUtilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, html: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);

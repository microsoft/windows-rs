#[cfg(feature = "implement_exclusive")]
pub trait IHtmlUtilitiesImpl: Sized {
    fn ConvertToText(&mut self, html: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.IHtmlUtilities";
}
#[cfg(feature = "implement_exclusive")]
impl IHtmlUtilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHtmlUtilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHtmlUtilitiesVtbl {
        unsafe extern "system" fn ConvertToText<Impl: IHtmlUtilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, html: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertToText(&*(&html as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHtmlUtilities, BASE_OFFSET>(), ConvertToText: ConvertToText::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHtmlUtilities as ::windows::core::Interface>::IID
    }
}

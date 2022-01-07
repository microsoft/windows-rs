#[cfg(feature = "implement_exclusive")]
pub trait IHtmlUtilitiesImpl: Sized {
    fn ConvertToText(&self, html: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.IHtmlUtilities";
}
#[cfg(feature = "implement_exclusive")]
impl IHtmlUtilitiesVtbl {
    pub const fn new<Impl: IHtmlUtilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHtmlUtilitiesVtbl {
        unsafe extern "system" fn ConvertToText<Impl: IHtmlUtilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, html: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertToText(&*(&html as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHtmlUtilities>, base.5, ConvertToText::<Impl, OFFSET>)
    }
}

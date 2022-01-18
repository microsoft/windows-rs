pub trait ITextElementOverrides_Impl: Sized {
    fn OnDisconnectVisualChildren(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for ITextElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementOverrides";
}
impl ITextElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextElementOverrides_Vtbl {
        unsafe extern "system" fn OnDisconnectVisualChildren<Impl: ITextElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnectVisualChildren().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextElementOverrides, BASE_OFFSET>(),
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextElementOverrides as ::windows::core::Interface>::IID
    }
}

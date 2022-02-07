pub trait IChatItem_Impl: Sized {
    fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind>;
}
impl ::windows::core::RuntimeName for IChatItem {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatItem";
}
impl IChatItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatItem_Impl, const OFFSET: isize>() -> IChatItem_Vtbl {
        unsafe extern "system" fn ItemKind<Identity: ::windows::core::IUnknownImpl, Impl: IChatItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatItemKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatItem, OFFSET>(), ItemKind: ItemKind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatItem as ::windows::core::Interface>::IID
    }
}

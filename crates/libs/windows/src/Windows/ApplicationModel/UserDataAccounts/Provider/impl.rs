pub trait IUserDataAccountProviderOperation_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<UserDataAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IUserDataAccountProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation";
}
impl IUserDataAccountProviderOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountProviderOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataAccountProviderOperation_Vtbl {
        unsafe extern "system" fn Kind<Impl: IUserDataAccountProviderOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataAccountProviderOperation, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataAccountProviderOperation as ::windows::core::Interface>::IID
    }
}

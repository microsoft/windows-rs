pub trait IWebAccount_Impl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<WebAccountState>;
}
impl ::windows::core::RuntimeName for IWebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccount";
}
impl IWebAccount_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>() -> IWebAccount_Vtbl {
        unsafe extern "system" fn WebAccountProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WebAccountProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccount, OFFSET>(),
            WebAccountProvider: WebAccountProvider::<Identity, Impl, OFFSET>,
            UserName: UserName::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccount as ::windows::core::Interface>::IID
    }
}

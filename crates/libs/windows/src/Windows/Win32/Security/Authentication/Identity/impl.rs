pub trait ICcgDomainAuthCredentials_Impl: Sized {
    fn GetPasswordCredentials(&self, plugininput: &::windows::core::PCWSTR, domainname: *mut ::windows::core::PWSTR, username: *mut ::windows::core::PWSTR, password: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICcgDomainAuthCredentials {}
impl ICcgDomainAuthCredentials_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICcgDomainAuthCredentials_Impl, const OFFSET: isize>() -> ICcgDomainAuthCredentials_Vtbl {
        unsafe extern "system" fn GetPasswordCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICcgDomainAuthCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugininput: ::windows::core::PCWSTR, domainname: *mut ::windows::core::PWSTR, username: *mut ::windows::core::PWSTR, password: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPasswordCredentials(::core::mem::transmute(&plugininput), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPasswordCredentials: GetPasswordCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICcgDomainAuthCredentials as ::windows::core::Interface>::IID
    }
}

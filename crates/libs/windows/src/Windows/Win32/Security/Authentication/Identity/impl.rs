#[cfg(feature = "Win32_Foundation")]
pub trait ICcgDomainAuthCredentialsImpl: Sized {
    fn GetPasswordCredentials();
}
#[cfg(feature = "Win32_Foundation")]
impl ICcgDomainAuthCredentialsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICcgDomainAuthCredentialsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICcgDomainAuthCredentialsVtbl {
        unsafe extern "system" fn GetPasswordCredentials<Impl: ICcgDomainAuthCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugininput: super::super::super::Foundation::PWSTR, domainname: *mut super::super::super::Foundation::PWSTR, username: *mut super::super::super::Foundation::PWSTR, password: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPasswordCredentials: GetPasswordCredentials::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICcgDomainAuthCredentials as ::windows::core::Interface>::IID
    }
}

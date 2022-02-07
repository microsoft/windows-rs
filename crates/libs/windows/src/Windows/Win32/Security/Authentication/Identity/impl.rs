#[cfg(feature = "Win32_Foundation")]
pub trait ICcgDomainAuthCredentials_Impl: Sized {
    fn GetPasswordCredentials(&self, plugininput: super::super::super::Foundation::PWSTR, domainname: *mut super::super::super::Foundation::PWSTR, username: *mut super::super::super::Foundation::PWSTR, password: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICcgDomainAuthCredentials_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICcgDomainAuthCredentials_Impl, const OFFSET: isize>() -> ICcgDomainAuthCredentials_Vtbl {
        unsafe extern "system" fn GetPasswordCredentials<Identity: ::windows::core::IUnknownImpl, Impl: ICcgDomainAuthCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugininput: super::super::super::Foundation::PWSTR, domainname: *mut super::super::super::Foundation::PWSTR, username: *mut super::super::super::Foundation::PWSTR, password: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPasswordCredentials(::core::mem::transmute_copy(&plugininput), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPasswordCredentials: GetPasswordCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICcgDomainAuthCredentials as ::windows::core::Interface>::IID
    }
}

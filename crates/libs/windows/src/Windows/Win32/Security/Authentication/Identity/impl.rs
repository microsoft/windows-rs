pub trait ICcgDomainAuthCredentialsImpl: Sized {
    fn GetPasswordCredentials();
}
impl ::windows::core::RuntimeName for ICcgDomainAuthCredentials {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.ICcgDomainAuthCredentials";
}
impl ICcgDomainAuthCredentialsVtbl {
    pub const fn new<Impl: ICcgDomainAuthCredentialsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICcgDomainAuthCredentialsVtbl {
        unsafe extern "system" fn GetPasswordCredentials<Impl: ICcgDomainAuthCredentialsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plugininput: super::super::super::Foundation::PWSTR, domainname: *mut super::super::super::Foundation::PWSTR, username: *mut super::super::super::Foundation::PWSTR, password: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPasswordCredentials(&*(&plugininput as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICcgDomainAuthCredentials>, base.5, GetPasswordCredentials::<Impl, OFFSET>)
    }
}

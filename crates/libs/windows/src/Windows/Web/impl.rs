#[doc = "*Required features: `\"Web\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IUriToStreamResolver_Impl: Sized {
    fn UriToStreamAsync(&self, uri: ::core::option::Option<&super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IUriToStreamResolver {
    const NAME: &'static str = "Windows.Web.IUriToStreamResolver";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IUriToStreamResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriToStreamResolver_Impl, const OFFSET: isize>() -> IUriToStreamResolver_Vtbl {
        unsafe extern "system" fn UriToStreamAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriToStreamResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UriToStreamAsync(::windows::core::from_raw_borrowed(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUriToStreamResolver, OFFSET>(),
            UriToStreamAsync: UriToStreamAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriToStreamResolver as ::windows::core::ComInterface>::IID
    }
}

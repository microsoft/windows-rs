pub trait ICustomXamlResourceLoaderOverrides_Impl: Sized {
    fn GetResource(&self, resourceid: &::windows::core::HSTRING, objecttype: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING, propertytype: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides";
}
impl ICustomXamlResourceLoaderOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomXamlResourceLoaderOverrides_Impl, const OFFSET: isize>() -> ICustomXamlResourceLoaderOverrides_Vtbl {
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl, Impl: ICustomXamlResourceLoaderOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, objecttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResource(::core::mem::transmute(&resourceid), ::core::mem::transmute(&objecttype), ::core::mem::transmute(&propertyname), ::core::mem::transmute(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomXamlResourceLoaderOverrides, OFFSET>(),
            GetResource: GetResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomXamlResourceLoaderOverrides as ::windows::core::Interface>::IID
    }
}

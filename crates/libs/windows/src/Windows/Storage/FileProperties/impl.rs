#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemExtraProperties_Impl: Sized {
    fn RetrievePropertiesAsync(&self, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SavePropertiesAsync(&self, propertiestosave: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageItemExtraProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.IStorageItemExtraProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageItemExtraProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>() -> IStorageItemExtraProperties_Vtbl {
        unsafe extern "system" fn RetrievePropertiesAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrievePropertiesAsync(::core::mem::transmute(&propertiestoretrieve)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestosave: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SavePropertiesAsync(::core::mem::transmute(&propertiestosave)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsyncOverloadDefault<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SavePropertiesAsyncOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemExtraProperties, OFFSET>(),
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Identity, Impl, OFFSET>,
            SavePropertiesAsync: SavePropertiesAsync::<Identity, Impl, OFFSET>,
            SavePropertiesAsyncOverloadDefault: SavePropertiesAsyncOverloadDefault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemExtraProperties as ::windows::core::Interface>::IID
    }
}

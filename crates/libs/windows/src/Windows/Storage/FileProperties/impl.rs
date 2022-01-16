#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IStorageItemExtraProperties_Impl: Sized {
    fn RetrievePropertiesAsync(&mut self, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SavePropertiesAsync(&mut self, propertiestosave: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IStorageItemExtraProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.IStorageItemExtraProperties";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IStorageItemExtraProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemExtraProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemExtraProperties_Vtbl {
        unsafe extern "system" fn RetrievePropertiesAsync<Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrievePropertiesAsync(&*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsync<Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestosave: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SavePropertiesAsync(&*(&propertiestosave as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsyncOverloadDefault<Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemExtraProperties, BASE_OFFSET>(),
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Impl, IMPL_OFFSET>,
            SavePropertiesAsync: SavePropertiesAsync::<Impl, IMPL_OFFSET>,
            SavePropertiesAsyncOverloadDefault: SavePropertiesAsyncOverloadDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemExtraProperties as ::windows::core::Interface>::IID
    }
}

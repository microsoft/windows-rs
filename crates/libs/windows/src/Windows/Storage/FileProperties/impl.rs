#[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemExtraProperties_Impl: Sized {
    fn RetrievePropertiesAsync(&self, propertiestoretrieve: ::core::option::Option<&super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>>;
    fn SavePropertiesAsync(&self, propertiestosave: ::core::option::Option<&super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IStorageItemExtraProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.IStorageItemExtraProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageItemExtraProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>() -> IStorageItemExtraProperties_Vtbl {
        unsafe extern "system" fn RetrievePropertiesAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrievePropertiesAsync(::windows_core::from_raw_borrowed(&propertiestoretrieve)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestosave: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SavePropertiesAsync(::windows_core::from_raw_borrowed(&propertiestosave)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsyncOverloadDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SavePropertiesAsyncOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemExtraProperties, OFFSET>(),
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Identity, Impl, OFFSET>,
            SavePropertiesAsync: SavePropertiesAsync::<Identity, Impl, OFFSET>,
            SavePropertiesAsyncOverloadDefault: SavePropertiesAsyncOverloadDefault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStorageItemExtraProperties as ::windows_core::ComInterface>::IID
    }
}

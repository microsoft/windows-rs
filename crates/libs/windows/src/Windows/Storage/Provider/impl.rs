pub trait IStorageProviderHandlerFactory_Impl: Sized {
    fn GetStatusSource(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusSource>;
}
impl ::windows::core::RuntimeName for IStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderHandlerFactory";
}
impl IStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderHandlerFactory_Impl, const OFFSET: isize>() -> IStorageProviderHandlerFactory_Vtbl {
        unsafe extern "system" fn GetStatusSource<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatusSource(::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderHandlerFactory, OFFSET>(),
            GetStatusSource: GetStatusSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderHandlerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderItemPropertySource_Impl: Sized {
    fn GetItemProperties(&self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageProviderItemPropertySource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertySource";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageProviderItemPropertySource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>() -> IStorageProviderItemPropertySource_Vtbl {
        unsafe extern "system" fn GetItemProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itempath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemProperties(::core::mem::transmute(&itempath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderItemPropertySource, OFFSET>(),
            GetItemProperties: GetItemProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertySource as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderPropertyCapabilities_Impl: Sized {
    fn IsPropertySupported(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageProviderPropertyCapabilities {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderPropertyCapabilities";
}
impl IStorageProviderPropertyCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>() -> IStorageProviderPropertyCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertySupported<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPropertySupported(::core::mem::transmute(&propertycanonicalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderPropertyCapabilities, OFFSET>(),
            IsPropertySupported: IsPropertySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderPropertyCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageProviderStatusSource_Impl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<StorageProviderStatus>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageProviderStatusSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusSource";
}
#[cfg(feature = "Foundation")]
impl IStorageProviderStatusSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>() -> IStorageProviderStatusSource_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Changed(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderStatusSource, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Changed: Changed::<Identity, Impl, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatusSource as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderUriSource_Impl: Sized {
    fn GetPathForContentUri(&self, contenturi: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetPathForContentUriResult>) -> ::windows::core::Result<()>;
    fn GetContentInfoForPath(&self, path: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetContentInfoForPathResult>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStorageProviderUriSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUriSource";
}
impl IStorageProviderUriSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>() -> IStorageProviderUriSource_Vtbl {
        unsafe extern "system" fn GetPathForContentUri<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenturi: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPathForContentUri(::core::mem::transmute(&contenturi), ::core::mem::transmute(&result)).into()
        }
        unsafe extern "system" fn GetContentInfoForPath<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContentInfoForPath(::core::mem::transmute(&path), ::core::mem::transmute(&result)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderUriSource, OFFSET>(),
            GetPathForContentUri: GetPathForContentUri::<Identity, Impl, OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderUriSource as ::windows::core::Interface>::IID
    }
}

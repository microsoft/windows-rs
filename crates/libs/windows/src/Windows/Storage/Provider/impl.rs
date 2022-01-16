pub trait IStorageProviderHandlerFactory_Impl: Sized {
    fn GetStatusSource(&mut self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusSource>;
}
impl ::windows::core::RuntimeName for IStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderHandlerFactory";
}
impl IStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderHandlerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderHandlerFactory_Vtbl {
        unsafe extern "system" fn GetStatusSource<Impl: IStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusSource(&*(&syncrootid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderHandlerFactory, BASE_OFFSET>(),
            GetStatusSource: GetStatusSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderHandlerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderItemPropertySource_Impl: Sized {
    fn GetItemProperties(&mut self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageProviderItemPropertySource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertySource";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageProviderItemPropertySource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertySource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderItemPropertySource_Vtbl {
        unsafe extern "system" fn GetItemProperties<Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itempath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemProperties(&*(&itempath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderItemPropertySource, BASE_OFFSET>(),
            GetItemProperties: GetItemProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertySource as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderPropertyCapabilities_Impl: Sized {
    fn IsPropertySupported(&mut self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageProviderPropertyCapabilities {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderPropertyCapabilities";
}
impl IStorageProviderPropertyCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderPropertyCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderPropertyCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertySupported<Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertySupported(&*(&propertycanonicalname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderPropertyCapabilities, BASE_OFFSET>(),
            IsPropertySupported: IsPropertySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderPropertyCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageProviderStatusSource_Impl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<StorageProviderStatus>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageProviderStatusSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusSource";
}
#[cfg(feature = "Foundation")]
impl IStorageProviderStatusSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderStatusSource_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IStorageProviderStatusSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderStatusSource, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatusSource as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderUriSource_Impl: Sized {
    fn GetPathForContentUri(&mut self, contenturi: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetPathForContentUriResult>) -> ::windows::core::Result<()>;
    fn GetContentInfoForPath(&mut self, path: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetContentInfoForPathResult>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStorageProviderUriSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUriSource";
}
impl IStorageProviderUriSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderUriSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderUriSource_Vtbl {
        unsafe extern "system" fn GetPathForContentUri<Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenturi: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPathForContentUri(&*(&contenturi as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&result as *const <StorageProviderGetPathForContentUriResult as ::windows::core::Abi>::Abi as *const <StorageProviderGetPathForContentUriResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetContentInfoForPath<Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentInfoForPath(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&result as *const <StorageProviderGetContentInfoForPathResult as ::windows::core::Abi>::Abi as *const <StorageProviderGetContentInfoForPathResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderUriSource, BASE_OFFSET>(),
            GetPathForContentUri: GetPathForContentUri::<Impl, IMPL_OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderUriSource as ::windows::core::Interface>::IID
    }
}

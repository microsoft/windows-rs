#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderImpl: Sized {
    fn GetString(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoader";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderImpl, const OFFSET: isize>() -> IResourceLoaderVtbl {
        unsafe extern "system" fn GetString<Impl: IResourceLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoader>, ::windows::core::GetTrustLevel, GetString::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoader2Impl: Sized {
    fn GetStringForUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoader2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoader2";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoader2Impl, const OFFSET: isize>() -> IResourceLoader2Vtbl {
        unsafe extern "system" fn GetStringForUri<Impl: IResourceLoader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringForUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoader2>, ::windows::core::GetTrustLevel, GetStringForUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderFactoryImpl: Sized {
    fn CreateResourceLoaderByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderFactoryImpl, const OFFSET: isize>() -> IResourceLoaderFactoryVtbl {
        unsafe extern "system" fn CreateResourceLoaderByName<Impl: IResourceLoaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResourceLoaderByName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoaderFactory>, ::windows::core::GetTrustLevel, CreateResourceLoaderByName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStaticsImpl: Sized {
    fn GetStringForReference(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStaticsImpl, const OFFSET: isize>() -> IResourceLoaderStaticsVtbl {
        unsafe extern "system" fn GetStringForReference<Impl: IResourceLoaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringForReference(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics>, ::windows::core::GetTrustLevel, GetStringForReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics2Impl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ResourceLoader>;
    fn GetForCurrentViewWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
    fn GetForViewIndependentUse(&self) -> ::windows::core::Result<ResourceLoader>;
    fn GetForViewIndependentUseWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics2Impl, const OFFSET: isize>() -> IResourceLoaderStatics2Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IResourceLoaderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentViewWithName<Impl: IResourceLoaderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentViewWithName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForViewIndependentUse<Impl: IResourceLoaderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForViewIndependentUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForViewIndependentUseWithName<Impl: IResourceLoaderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForViewIndependentUseWithName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics2>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, OFFSET>, GetForCurrentViewWithName::<Impl, OFFSET>, GetForViewIndependentUse::<Impl, OFFSET>, GetForViewIndependentUseWithName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics3Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::UI::UIContext>) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics3Impl, const OFFSET: isize>() -> IResourceLoaderStatics3Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IResourceLoaderStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::UI::UIContext as ::windows::core::Abi>::Abi as *const <super::super::UI::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics3>, ::windows::core::GetTrustLevel, GetForUIContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics4Impl: Sized {
    fn GetDefaultPriPath(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics4Impl, const OFFSET: isize>() -> IResourceLoaderStatics4Vtbl {
        unsafe extern "system" fn GetDefaultPriPath<Impl: IResourceLoaderStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultPriPath(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics4>, ::windows::core::GetTrustLevel, GetDefaultPriPath::<Impl, OFFSET>)
    }
}

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
    pub const fn new<Impl: IResourceLoaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoaderVtbl {
        unsafe extern "system" fn GetString<Impl: IResourceLoaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetString(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoader>, base.5, GetString::<Impl, OFFSET>)
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
    pub const fn new<Impl: IResourceLoader2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoader2Vtbl {
        unsafe extern "system" fn GetStringForUri<Impl: IResourceLoader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringForUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoader2>, base.5, GetStringForUri::<Impl, OFFSET>)
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
    pub const fn new<Impl: IResourceLoaderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoaderFactoryVtbl {
        unsafe extern "system" fn CreateResourceLoaderByName<Impl: IResourceLoaderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResourceLoaderByName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoaderFactory>, base.5, CreateResourceLoaderByName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IResourceLoaderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoaderStaticsVtbl {
        unsafe extern "system" fn GetStringForReference<Impl: IResourceLoaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringForReference(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics>, base.5, GetStringForReference::<Impl, OFFSET>)
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
    pub const fn new<Impl: IResourceLoaderStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoaderStatics2Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IResourceLoaderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentViewWithName<Impl: IResourceLoaderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentViewWithName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForViewIndependentUse<Impl: IResourceLoaderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForViewIndependentUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForViewIndependentUseWithName<Impl: IResourceLoaderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForViewIndependentUseWithName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics2>, base.5, GetForCurrentView::<Impl, OFFSET>, GetForCurrentViewWithName::<Impl, OFFSET>, GetForViewIndependentUse::<Impl, OFFSET>, GetForViewIndependentUseWithName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IResourceLoaderStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoaderStatics3Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IResourceLoaderStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::UI::UIContext as ::windows::core::Abi>::Abi as *const <super::super::UI::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics3>, base.5, GetForUIContext::<Impl, OFFSET>)
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
    pub const fn new<Impl: IResourceLoaderStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceLoaderStatics4Vtbl {
        unsafe extern "system" fn GetDefaultPriPath<Impl: IResourceLoaderStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultPriPath(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceLoaderStatics4>, base.5, GetDefaultPriPath::<Impl, OFFSET>)
    }
}

#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoader_Impl: Sized {
    fn GetString(&mut self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoader";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoader_Vtbl {
        unsafe extern "system" fn GetString<Impl: IResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoader, BASE_OFFSET>(), GetString: GetString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IResourceLoader2_Impl: Sized {
    fn GetStringForUri(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceLoader2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoader2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IResourceLoader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoader2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoader2_Vtbl {
        unsafe extern "system" fn GetStringForUri<Impl: IResourceLoader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoader2, BASE_OFFSET>(), GetStringForUri: GetStringForUri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderFactory_Impl: Sized {
    fn CreateResourceLoaderByName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoaderFactory_Vtbl {
        unsafe extern "system" fn CreateResourceLoaderByName<Impl: IResourceLoaderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoaderFactory, BASE_OFFSET>(),
            CreateResourceLoaderByName: CreateResourceLoaderByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IResourceLoaderStatics_Impl: Sized {
    fn GetStringForReference(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceLoaderStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IResourceLoaderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoaderStatics_Vtbl {
        unsafe extern "system" fn GetStringForReference<Impl: IResourceLoaderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoaderStatics, BASE_OFFSET>(),
            GetStringForReference: GetStringForReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoaderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics2_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<ResourceLoader>;
    fn GetForCurrentViewWithName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
    fn GetForViewIndependentUse(&mut self) -> ::windows::core::Result<ResourceLoader>;
    fn GetForViewIndependentUseWithName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoaderStatics2_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IResourceLoaderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForCurrentViewWithName<Impl: IResourceLoaderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForViewIndependentUse<Impl: IResourceLoaderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForViewIndependentUseWithName<Impl: IResourceLoaderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoaderStatics2, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            GetForCurrentViewWithName: GetForCurrentViewWithName::<Impl, IMPL_OFFSET>,
            GetForViewIndependentUse: GetForViewIndependentUse::<Impl, IMPL_OFFSET>,
            GetForViewIndependentUseWithName: GetForViewIndependentUseWithName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoaderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IResourceLoaderStatics3_Impl: Sized {
    fn GetForUIContext(&mut self, context: &::core::option::Option<super::super::UI::UIContext>) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceLoaderStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics3";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IResourceLoaderStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoaderStatics3_Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IResourceLoaderStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoaderStatics3, BASE_OFFSET>(),
            GetForUIContext: GetForUIContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoaderStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics4_Impl: Sized {
    fn GetDefaultPriPath(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceLoaderStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.IResourceLoaderStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceLoaderStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceLoaderStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceLoaderStatics4_Vtbl {
        unsafe extern "system" fn GetDefaultPriPath<Impl: IResourceLoaderStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceLoaderStatics4, BASE_OFFSET>(),
            GetDefaultPriPath: GetDefaultPriPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceLoaderStatics4 as ::windows::core::Interface>::IID
    }
}

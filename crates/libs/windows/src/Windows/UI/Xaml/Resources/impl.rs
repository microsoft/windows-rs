#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoader_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoader {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoader";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomXamlResourceLoader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomXamlResourceLoader_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomXamlResourceLoader, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomXamlResourceLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CustomXamlResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomXamlResourceLoaderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomXamlResourceLoaderFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICustomXamlResourceLoaderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomXamlResourceLoaderFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomXamlResourceLoaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderOverrides_Impl: Sized {
    fn GetResource(&mut self, resourceid: &::windows::core::HSTRING, objecttype: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING, propertytype: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomXamlResourceLoaderOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomXamlResourceLoaderOverrides_Vtbl {
        unsafe extern "system" fn GetResource<Impl: ICustomXamlResourceLoaderOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, objecttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(
                &*(&resourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&objecttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&propertytype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomXamlResourceLoaderOverrides, BASE_OFFSET>(),
            GetResource: GetResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomXamlResourceLoaderOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<CustomXamlResourceLoader>;
    fn SetCurrent(&mut self, value: &::core::option::Option<CustomXamlResourceLoader>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomXamlResourceLoaderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomXamlResourceLoaderStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: ICustomXamlResourceLoaderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrent<Impl: ICustomXamlResourceLoaderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrent(&*(&value as *const <CustomXamlResourceLoader as ::windows::core::Abi>::Abi as *const <CustomXamlResourceLoader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomXamlResourceLoaderStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            SetCurrent: SetCurrent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomXamlResourceLoaderStatics as ::windows::core::Interface>::IID
    }
}

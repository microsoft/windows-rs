#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoader {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoader";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderVtbl {
    pub const fn new<Impl: ICustomXamlResourceLoaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomXamlResourceLoaderVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomXamlResourceLoader>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CustomXamlResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderFactoryVtbl {
    pub const fn new<Impl: ICustomXamlResourceLoaderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomXamlResourceLoaderFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICustomXamlResourceLoaderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomXamlResourceLoaderFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderOverridesImpl: Sized {
    fn GetResource(&self, resourceid: &::windows::core::HSTRING, objecttype: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING, propertytype: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderOverridesVtbl {
    pub const fn new<Impl: ICustomXamlResourceLoaderOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomXamlResourceLoaderOverridesVtbl {
        unsafe extern "system" fn GetResource<Impl: ICustomXamlResourceLoaderOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, objecttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomXamlResourceLoaderOverrides>, base.5, GetResource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<CustomXamlResourceLoader>;
    fn SetCurrent(&self, value: &::core::option::Option<CustomXamlResourceLoader>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomXamlResourceLoaderStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomXamlResourceLoaderStaticsVtbl {
    pub const fn new<Impl: ICustomXamlResourceLoaderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomXamlResourceLoaderStaticsVtbl {
        unsafe extern "system" fn Current<Impl: ICustomXamlResourceLoaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrent<Impl: ICustomXamlResourceLoaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurrent(&*(&value as *const <CustomXamlResourceLoader as ::windows::core::Abi>::Abi as *const <CustomXamlResourceLoader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomXamlResourceLoaderStatics>, base.5, Current::<Impl, OFFSET>, SetCurrent::<Impl, OFFSET>)
    }
}

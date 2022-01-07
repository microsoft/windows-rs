#[cfg(feature = "implement_exclusive")]
pub trait IBindingImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<super::PropertyPath>;
    fn SetPath(&self, value: &::core::option::Option<super::PropertyPath>) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<BindingMode>;
    fn SetMode(&self, value: BindingMode) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RelativeSource(&self) -> ::windows::core::Result<RelativeSource>;
    fn SetRelativeSource(&self, value: &::core::option::Option<RelativeSource>) -> ::windows::core::Result<()>;
    fn ElementName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetElementName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Converter(&self) -> ::windows::core::Result<IValueConverter>;
    fn SetConverter(&self, value: &::core::option::Option<IValueConverter>) -> ::windows::core::Result<()>;
    fn ConverterParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetConverterParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ConverterLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetConverterLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBinding {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBinding";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingVtbl {
    pub const fn new<Impl: IBindingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingVtbl {
        unsafe extern "system" fn Path<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::PropertyPath as ::windows::core::Abi>::Abi as *const <super::PropertyPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Mode<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BindingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BindingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Source<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeSource<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeSource<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelativeSource(&*(&value as *const <RelativeSource as ::windows::core::Abi>::Abi as *const <RelativeSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ElementName<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ElementName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementName<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetElementName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Converter<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Converter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConverter<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetConverter(&*(&value as *const <IValueConverter as ::windows::core::Abi>::Abi as *const <IValueConverter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConverterParameter<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConverterParameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConverterParameter<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetConverterParameter(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConverterLanguage<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConverterLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConverterLanguage<Impl: IBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetConverterLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBinding>,
            base.5,
            Path::<Impl, OFFSET>,
            SetPath::<Impl, OFFSET>,
            Mode::<Impl, OFFSET>,
            SetMode::<Impl, OFFSET>,
            Source::<Impl, OFFSET>,
            SetSource::<Impl, OFFSET>,
            RelativeSource::<Impl, OFFSET>,
            SetRelativeSource::<Impl, OFFSET>,
            ElementName::<Impl, OFFSET>,
            SetElementName::<Impl, OFFSET>,
            Converter::<Impl, OFFSET>,
            SetConverter::<Impl, OFFSET>,
            ConverterParameter::<Impl, OFFSET>,
            SetConverterParameter::<Impl, OFFSET>,
            ConverterLanguage::<Impl, OFFSET>,
            SetConverterLanguage::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBinding2Impl: Sized {
    fn FallbackValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetFallbackValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TargetNullValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTargetNullValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn UpdateSourceTrigger(&self) -> ::windows::core::Result<UpdateSourceTrigger>;
    fn SetUpdateSourceTrigger(&self, value: UpdateSourceTrigger) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBinding2 {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBinding2";
}
#[cfg(feature = "implement_exclusive")]
impl IBinding2Vtbl {
    pub const fn new<Impl: IBinding2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBinding2Vtbl {
        unsafe extern "system" fn FallbackValue<Impl: IBinding2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FallbackValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackValue<Impl: IBinding2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFallbackValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetNullValue<Impl: IBinding2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNullValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetNullValue<Impl: IBinding2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetNullValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateSourceTrigger<Impl: IBinding2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UpdateSourceTrigger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateSourceTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateSourceTrigger<Impl: IBinding2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: UpdateSourceTrigger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUpdateSourceTrigger(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBinding2>, base.5, FallbackValue::<Impl, OFFSET>, SetFallbackValue::<Impl, OFFSET>, TargetNullValue::<Impl, OFFSET>, SetTargetNullValue::<Impl, OFFSET>, UpdateSourceTrigger::<Impl, OFFSET>, SetUpdateSourceTrigger::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingBase {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingBase";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingBaseVtbl {
    pub const fn new<Impl: IBindingBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingBaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingBase>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BindingBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingBaseFactoryVtbl {
    pub const fn new<Impl: IBindingBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBindingBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionImpl: Sized {
    fn DataItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ParentBinding(&self) -> ::windows::core::Result<Binding>;
    fn UpdateSource(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingExpression {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingExpression";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingExpressionVtbl {
    pub const fn new<Impl: IBindingExpressionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingExpressionVtbl {
        unsafe extern "system" fn DataItem<Impl: IBindingExpressionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentBinding<Impl: IBindingExpressionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParentBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSource<Impl: IBindingExpressionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateSource().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingExpression>, base.5, DataItem::<Impl, OFFSET>, ParentBinding::<Impl, OFFSET>, UpdateSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingExpressionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingExpressionBase";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingExpressionBaseVtbl {
    pub const fn new<Impl: IBindingExpressionBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingExpressionBaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingExpressionBase>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingExpressionBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingExpressionBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingExpressionBaseFactoryVtbl {
    pub const fn new<Impl: IBindingExpressionBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingExpressionBaseFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingExpressionBaseFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingExpressionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingExpressionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingExpressionFactoryVtbl {
    pub const fn new<Impl: IBindingExpressionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingExpressionFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingExpressionFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Binding>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingFactoryVtbl {
    pub const fn new<Impl: IBindingFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBindingFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingOperationsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingOperations {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingOperations";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingOperationsVtbl {
    pub const fn new<Impl: IBindingOperationsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingOperationsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingOperations>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingOperationsStaticsImpl: Sized {
    fn SetBinding(&self, target: &::core::option::Option<super::DependencyObject>, dp: &::core::option::Option<super::DependencyProperty>, binding: &::core::option::Option<BindingBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingOperationsStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IBindingOperationsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingOperationsStaticsVtbl {
    pub const fn new<Impl: IBindingOperationsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingOperationsStaticsVtbl {
        unsafe extern "system" fn SetBinding<Impl: IBindingOperationsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, dp: ::windows::core::RawPtr, binding: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBinding(&*(&target as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&dp as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&binding as *const <BindingBase as ::windows::core::Abi>::Abi as *const <BindingBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingOperationsStatics>, base.5, SetBinding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICollectionViewImpl: Sized + IIterableImpl<::windows::core::IInspectable> + IObservableVectorImpl<::windows::core::IInspectable> + IVectorImpl<::windows::core::IInspectable> {
    fn CurrentItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CurrentPosition(&self) -> ::windows::core::Result<i32>;
    fn IsCurrentAfterLast(&self) -> ::windows::core::Result<bool>;
    fn IsCurrentBeforeFirst(&self) -> ::windows::core::Result<bool>;
    fn CollectionGroups(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
    fn HasMoreItems(&self) -> ::windows::core::Result<bool>;
    fn CurrentChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentChanging(&self, handler: &::core::option::Option<CurrentChangingEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MoveCurrentTo(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn MoveCurrentToPosition(&self, index: i32) -> ::windows::core::Result<bool>;
    fn MoveCurrentToFirst(&self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToLast(&self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToNext(&self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToPrevious(&self) -> ::windows::core::Result<bool>;
    fn LoadMoreItemsAsync(&self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICollectionView {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionView";
}
#[cfg(feature = "Foundation_Collections")]
impl ICollectionViewVtbl {
    pub const fn new<Impl: ICollectionViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICollectionViewVtbl {
        unsafe extern "system" fn CurrentItem<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPosition<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAfterLast<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCurrentAfterLast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentBeforeFirst<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCurrentBeforeFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionGroups<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollectionGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentChanged(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCurrentChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentChanging<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentChanging(&*(&handler as *const <CurrentChangingEventHandler as ::windows::core::Abi>::Abi as *const <CurrentChangingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanging<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCurrentChanging(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MoveCurrentTo<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveCurrentTo(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPosition<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToPosition(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToFirst<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToLast<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToLast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToNext<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPrevious<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToPrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMoreItemsAsync<Impl: ICollectionViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICollectionView>,
            base.5,
            CurrentItem::<Impl, OFFSET>,
            CurrentPosition::<Impl, OFFSET>,
            IsCurrentAfterLast::<Impl, OFFSET>,
            IsCurrentBeforeFirst::<Impl, OFFSET>,
            CollectionGroups::<Impl, OFFSET>,
            HasMoreItems::<Impl, OFFSET>,
            CurrentChanged::<Impl, OFFSET>,
            RemoveCurrentChanged::<Impl, OFFSET>,
            CurrentChanging::<Impl, OFFSET>,
            RemoveCurrentChanging::<Impl, OFFSET>,
            MoveCurrentTo::<Impl, OFFSET>,
            MoveCurrentToPosition::<Impl, OFFSET>,
            MoveCurrentToFirst::<Impl, OFFSET>,
            MoveCurrentToLast::<Impl, OFFSET>,
            MoveCurrentToNext::<Impl, OFFSET>,
            MoveCurrentToPrevious::<Impl, OFFSET>,
            LoadMoreItemsAsync::<Impl, OFFSET>,
        )
    }
}
pub trait ICollectionViewFactoryImpl: Sized {
    fn CreateView(&self) -> ::windows::core::Result<ICollectionView>;
}
impl ::windows::core::RuntimeName for ICollectionViewFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewFactory";
}
impl ICollectionViewFactoryVtbl {
    pub const fn new<Impl: ICollectionViewFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICollectionViewFactoryVtbl {
        unsafe extern "system" fn CreateView<Impl: ICollectionViewFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICollectionViewFactory>, base.5, CreateView::<Impl, OFFSET>)
    }
}
pub trait ICollectionViewGroupImpl: Sized {
    fn Group(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GroupItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
}
impl ::windows::core::RuntimeName for ICollectionViewGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewGroup";
}
impl ICollectionViewGroupVtbl {
    pub const fn new<Impl: ICollectionViewGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICollectionViewGroupVtbl {
        unsafe extern "system" fn Group<Impl: ICollectionViewGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupItems<Impl: ICollectionViewGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GroupItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICollectionViewGroup>, base.5, Group::<Impl, OFFSET>, GroupItems::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICollectionViewSourceImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn View(&self) -> ::windows::core::Result<ICollectionView>;
    fn IsSourceGrouped(&self) -> ::windows::core::Result<bool>;
    fn SetIsSourceGrouped(&self, value: bool) -> ::windows::core::Result<()>;
    fn ItemsPath(&self) -> ::windows::core::Result<super::PropertyPath>;
    fn SetItemsPath(&self, value: &::core::option::Option<super::PropertyPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICollectionViewSource {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewSource";
}
#[cfg(feature = "implement_exclusive")]
impl ICollectionViewSourceVtbl {
    pub const fn new<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICollectionViewSourceVtbl {
        unsafe extern "system" fn Source<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn View<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).View() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSourceGrouped<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSourceGrouped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSourceGrouped<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsSourceGrouped(value).into()
        }
        unsafe extern "system" fn ItemsPath<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemsPath<Impl: ICollectionViewSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItemsPath(&*(&value as *const <super::PropertyPath as ::windows::core::Abi>::Abi as *const <super::PropertyPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICollectionViewSource>, base.5, Source::<Impl, OFFSET>, SetSource::<Impl, OFFSET>, View::<Impl, OFFSET>, IsSourceGrouped::<Impl, OFFSET>, SetIsSourceGrouped::<Impl, OFFSET>, ItemsPath::<Impl, OFFSET>, SetItemsPath::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICollectionViewSourceStaticsImpl: Sized {
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSourceGroupedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemsPathProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICollectionViewSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICollectionViewSourceStaticsVtbl {
    pub const fn new<Impl: ICollectionViewSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICollectionViewSourceStaticsVtbl {
        unsafe extern "system" fn SourceProperty<Impl: ICollectionViewSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewProperty<Impl: ICollectionViewSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSourceGroupedProperty<Impl: ICollectionViewSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSourceGroupedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsPathProperty<Impl: ICollectionViewSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsPathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICollectionViewSourceStatics>, base.5, SourceProperty::<Impl, OFFSET>, ViewProperty::<Impl, OFFSET>, IsSourceGroupedProperty::<Impl, OFFSET>, ItemsPathProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentChangingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCancelable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrentChangingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICurrentChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrentChangingEventArgsVtbl {
    pub const fn new<Impl: ICurrentChangingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICurrentChangingEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: ICurrentChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: ICurrentChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn IsCancelable<Impl: ICurrentChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCancelable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICurrentChangingEventArgs>, base.5, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>, IsCancelable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentChangingEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CurrentChangingEventArgs>;
    fn CreateWithCancelableParameter(&self, iscancelable: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CurrentChangingEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrentChangingEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICurrentChangingEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrentChangingEventArgsFactoryVtbl {
    pub const fn new<Impl: ICurrentChangingEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICurrentChangingEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICurrentChangingEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithCancelableParameter<Impl: ICurrentChangingEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iscancelable: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithCancelableParameter(iscancelable, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICurrentChangingEventArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateWithCancelableParameter::<Impl, OFFSET>)
    }
}
pub trait ICustomPropertyImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetValue(&self, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, target: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetIndexedValue(&self, target: &::core::option::Option<::windows::core::IInspectable>, index: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetIndexedValue(&self, target: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>, index: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn CanWrite(&self) -> ::windows::core::Result<bool>;
    fn CanRead(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICustomProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICustomProperty";
}
impl ICustomPropertyVtbl {
    pub const fn new<Impl: ICustomPropertyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomPropertyVtbl {
        unsafe extern "system" fn Type<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIndexedValue<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndexedValue(&*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&index as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexedValue<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetIndexedValue(
                    &*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&index as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CanWrite<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Impl: ICustomPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomProperty>, base.5, Type::<Impl, OFFSET>, Name::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, GetIndexedValue::<Impl, OFFSET>, SetIndexedValue::<Impl, OFFSET>, CanWrite::<Impl, OFFSET>, CanRead::<Impl, OFFSET>)
    }
}
pub trait ICustomPropertyProviderImpl: Sized {
    fn GetCustomProperty(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ICustomProperty>;
    fn GetIndexedProperty(&self, name: &::windows::core::HSTRING, r#type: &super::Interop::TypeName) -> ::windows::core::Result<ICustomProperty>;
    fn GetStringRepresentation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Type(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
impl ::windows::core::RuntimeName for ICustomPropertyProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICustomPropertyProvider";
}
impl ICustomPropertyProviderVtbl {
    pub const fn new<Impl: ICustomPropertyProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomPropertyProviderVtbl {
        unsafe extern "system" fn GetCustomProperty<Impl: ICustomPropertyProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomProperty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexedProperty<Impl: ICustomPropertyProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndexedProperty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&r#type as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringRepresentation<Impl: ICustomPropertyProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICustomPropertyProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomPropertyProvider>, base.5, GetCustomProperty::<Impl, OFFSET>, GetIndexedProperty::<Impl, OFFSET>, GetStringRepresentation::<Impl, OFFSET>, Type::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemIndexRangeImpl: Sized {
    fn FirstIndex(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn LastIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemIndexRange {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IItemIndexRange";
}
#[cfg(feature = "implement_exclusive")]
impl IItemIndexRangeVtbl {
    pub const fn new<Impl: IItemIndexRangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemIndexRangeVtbl {
        unsafe extern "system" fn FirstIndex<Impl: IItemIndexRangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FirstIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IItemIndexRangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastIndex<Impl: IItemIndexRangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemIndexRange>, base.5, FirstIndex::<Impl, OFFSET>, Length::<Impl, OFFSET>, LastIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemIndexRangeFactoryImpl: Sized {
    fn CreateInstance(&self, firstindex: i32, length: u32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemIndexRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemIndexRangeFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IItemIndexRangeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IItemIndexRangeFactoryVtbl {
    pub const fn new<Impl: IItemIndexRangeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemIndexRangeFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IItemIndexRangeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, firstindex: i32, length: u32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(firstindex, length, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemIndexRangeFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IItemsRangeInfoImpl: Sized + IClosableImpl {
    fn RangesChanged(&self, visiblerange: &::core::option::Option<ItemIndexRange>, trackeditems: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IItemsRangeInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IItemsRangeInfo";
}
#[cfg(feature = "Foundation")]
impl IItemsRangeInfoVtbl {
    pub const fn new<Impl: IItemsRangeInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemsRangeInfoVtbl {
        unsafe extern "system" fn RangesChanged<Impl: IItemsRangeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visiblerange: ::windows::core::RawPtr, trackeditems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RangesChanged(&*(&visiblerange as *const <ItemIndexRange as ::windows::core::Abi>::Abi as *const <ItemIndexRange as ::windows::core::DefaultType>::DefaultType), &*(&trackeditems as *const <super::super::super::Foundation::Collections::IVectorView<ItemIndexRange> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<ItemIndexRange> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemsRangeInfo>, base.5, RangesChanged::<Impl, OFFSET>)
    }
}
pub trait INotifyPropertyChangedImpl: Sized {
    fn PropertyChanged(&self, handler: &::core::option::Option<PropertyChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INotifyPropertyChanged {
    const NAME: &'static str = "Windows.UI.Xaml.Data.INotifyPropertyChanged";
}
impl INotifyPropertyChangedVtbl {
    pub const fn new<Impl: INotifyPropertyChangedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotifyPropertyChangedVtbl {
        unsafe extern "system" fn PropertyChanged<Impl: INotifyPropertyChangedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyChanged(&*(&handler as *const <PropertyChangedEventHandler as ::windows::core::Abi>::Abi as *const <PropertyChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<Impl: INotifyPropertyChangedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePropertyChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotifyPropertyChanged>, base.5, PropertyChanged::<Impl, OFFSET>, RemovePropertyChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyChangedEventArgsImpl: Sized {
    fn PropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IPropertyChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyChangedEventArgsVtbl {
    pub const fn new<Impl: IPropertyChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyChangedEventArgsVtbl {
        unsafe extern "system" fn PropertyName<Impl: IPropertyChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyChangedEventArgs>, base.5, PropertyName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyChangedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, name: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyChangedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyChangedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IPropertyChangedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyChangedEventArgsFactoryVtbl {
    pub const fn new<Impl: IPropertyChangedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyChangedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPropertyChangedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyChangedEventArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativeSourceImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<RelativeSourceMode>;
    fn SetMode(&self, value: RelativeSourceMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRelativeSource {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IRelativeSource";
}
#[cfg(feature = "implement_exclusive")]
impl IRelativeSourceVtbl {
    pub const fn new<Impl: IRelativeSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRelativeSourceVtbl {
        unsafe extern "system" fn Mode<Impl: IRelativeSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RelativeSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IRelativeSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: RelativeSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRelativeSource>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativeSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RelativeSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRelativeSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IRelativeSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRelativeSourceFactoryVtbl {
    pub const fn new<Impl: IRelativeSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRelativeSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRelativeSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRelativeSourceFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait ISelectionInfoImpl: Sized {
    fn SelectRange(&self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows::core::Result<()>;
    fn DeselectRange(&self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows::core::Result<()>;
    fn IsSelected(&self, index: i32) -> ::windows::core::Result<bool>;
    fn GetSelectedRanges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>;
}
impl ::windows::core::RuntimeName for ISelectionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ISelectionInfo";
}
impl ISelectionInfoVtbl {
    pub const fn new<Impl: ISelectionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectionInfoVtbl {
        unsafe extern "system" fn SelectRange<Impl: ISelectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SelectRange(&*(&itemindexrange as *const <ItemIndexRange as ::windows::core::Abi>::Abi as *const <ItemIndexRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeselectRange<Impl: ISelectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DeselectRange(&*(&itemindexrange as *const <ItemIndexRange as ::windows::core::Abi>::Abi as *const <ItemIndexRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSelected<Impl: ISelectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSelected(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedRanges<Impl: ISelectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSelectedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectionInfo>, base.5, SelectRange::<Impl, OFFSET>, DeselectRange::<Impl, OFFSET>, IsSelected::<Impl, OFFSET>, GetSelectedRanges::<Impl, OFFSET>)
    }
}
pub trait ISupportIncrementalLoadingImpl: Sized {
    fn LoadMoreItemsAsync(&self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
    fn HasMoreItems(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ISupportIncrementalLoading {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ISupportIncrementalLoading";
}
impl ISupportIncrementalLoadingVtbl {
    pub const fn new<Impl: ISupportIncrementalLoadingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISupportIncrementalLoadingVtbl {
        unsafe extern "system" fn LoadMoreItemsAsync<Impl: ISupportIncrementalLoadingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Impl: ISupportIncrementalLoadingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISupportIncrementalLoading>, base.5, LoadMoreItemsAsync::<Impl, OFFSET>, HasMoreItems::<Impl, OFFSET>)
    }
}
pub trait IValueConverterImpl: Sized {
    fn Convert(&self, value: &::core::option::Option<::windows::core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, language: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ConvertBack(&self, value: &::core::option::Option<::windows::core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, language: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IValueConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IValueConverter";
}
impl IValueConverterVtbl {
    pub const fn new<Impl: IValueConverterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IValueConverterVtbl {
        unsafe extern "system" fn Convert<Impl: IValueConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Convert(
                &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&targettype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBack<Impl: IValueConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertBack(
                &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&targettype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IValueConverter>, base.5, Convert::<Impl, OFFSET>, ConvertBack::<Impl, OFFSET>)
    }
}

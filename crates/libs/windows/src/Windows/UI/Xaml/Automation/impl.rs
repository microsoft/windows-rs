#[cfg(feature = "implement_exclusive")]
pub trait IAnnotationPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnnotationPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IAnnotationPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAnnotationPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnnotationPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnnotationPatternIdentifiersStaticsImpl: Sized {
    fn AnnotationTypeIdProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AnnotationTypeNameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AuthorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DateTimeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn TargetProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnnotationPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAnnotationPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn AnnotationTypeIdProperty<Impl: IAnnotationPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeIdProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeNameProperty<Impl: IAnnotationPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorProperty<Impl: IAnnotationPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTimeProperty<Impl: IAnnotationPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetProperty<Impl: IAnnotationPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnnotationPatternIdentifiersStatics, BASE_OFFSET>(),
            AnnotationTypeIdProperty: AnnotationTypeIdProperty::<Impl, IMPL_OFFSET>,
            AnnotationTypeNameProperty: AnnotationTypeNameProperty::<Impl, IMPL_OFFSET>,
            AuthorProperty: AuthorProperty::<Impl, IMPL_OFFSET>,
            DateTimeProperty: DateTimeProperty::<Impl, IMPL_OFFSET>,
            TargetProperty: TargetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnnotationPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationAnnotationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<AnnotationType>;
    fn SetType(&self, value: AnnotationType) -> ::windows::core::Result<()>;
    fn Element(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetElement(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationAnnotation";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationAnnotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationAnnotationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationAnnotationVtbl {
        unsafe extern "system" fn Type<Impl: IAutomationAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IAutomationAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Element<Impl: IAutomationAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Element() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElement<Impl: IAutomationAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElement(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationAnnotation, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Element: Element::<Impl, IMPL_OFFSET>,
            SetElement: SetElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationAnnotation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationAnnotationFactoryImpl: Sized {
    fn CreateInstance(&self, r#type: AnnotationType) -> ::windows::core::Result<AutomationAnnotation>;
    fn CreateWithElementParameter(&self, r#type: AnnotationType, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<AutomationAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationAnnotationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationAnnotationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationAnnotationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationAnnotationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationAnnotationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationAnnotationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithElementParameter<Impl: IAutomationAnnotationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AnnotationType, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithElementParameter(r#type, &*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationAnnotationFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateWithElementParameter: CreateWithElementParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationAnnotationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationAnnotationStaticsImpl: Sized {
    fn TypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationAnnotationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationAnnotationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationAnnotationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationAnnotationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationAnnotationStaticsVtbl {
        unsafe extern "system" fn TypeProperty<Impl: IAutomationAnnotationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementProperty<Impl: IAutomationAnnotationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationAnnotationStatics, BASE_OFFSET>(),
            TypeProperty: TypeProperty::<Impl, IMPL_OFFSET>,
            ElementProperty: ElementProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationAnnotationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStaticsImpl: Sized {
    fn AcceleratorKeyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AutomationIdProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn BoundingRectangleProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ClassNameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ClickablePointProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ControlTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HasKeyboardFocusProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HelpTextProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsContentElementProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsControlElementProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsEnabledProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsKeyboardFocusableProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsOffscreenProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsPasswordProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsRequiredForFormProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ItemStatusProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ItemTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LabeledByProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LocalizedControlTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn NameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LiveSettingProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStaticsVtbl {
        unsafe extern "system" fn AcceleratorKeyProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceleratorKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutomationIdProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationIdProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRectangleProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingRectangleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassNameProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClickablePointProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClickablePointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlTypeProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocusProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKeyboardFocusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HelpTextProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HelpTextProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElementProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElementProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyboardFocusableProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKeyboardFocusableProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreenProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffscreenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPasswordProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPasswordProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForFormProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequiredForFormProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemStatusProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemStatusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTypeProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LabeledByProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LabeledByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedControlTypeProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedControlTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientationProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrientationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LiveSettingProperty<Impl: IAutomationElementIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LiveSettingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics, BASE_OFFSET>(),
            AcceleratorKeyProperty: AcceleratorKeyProperty::<Impl, IMPL_OFFSET>,
            AccessKeyProperty: AccessKeyProperty::<Impl, IMPL_OFFSET>,
            AutomationIdProperty: AutomationIdProperty::<Impl, IMPL_OFFSET>,
            BoundingRectangleProperty: BoundingRectangleProperty::<Impl, IMPL_OFFSET>,
            ClassNameProperty: ClassNameProperty::<Impl, IMPL_OFFSET>,
            ClickablePointProperty: ClickablePointProperty::<Impl, IMPL_OFFSET>,
            ControlTypeProperty: ControlTypeProperty::<Impl, IMPL_OFFSET>,
            HasKeyboardFocusProperty: HasKeyboardFocusProperty::<Impl, IMPL_OFFSET>,
            HelpTextProperty: HelpTextProperty::<Impl, IMPL_OFFSET>,
            IsContentElementProperty: IsContentElementProperty::<Impl, IMPL_OFFSET>,
            IsControlElementProperty: IsControlElementProperty::<Impl, IMPL_OFFSET>,
            IsEnabledProperty: IsEnabledProperty::<Impl, IMPL_OFFSET>,
            IsKeyboardFocusableProperty: IsKeyboardFocusableProperty::<Impl, IMPL_OFFSET>,
            IsOffscreenProperty: IsOffscreenProperty::<Impl, IMPL_OFFSET>,
            IsPasswordProperty: IsPasswordProperty::<Impl, IMPL_OFFSET>,
            IsRequiredForFormProperty: IsRequiredForFormProperty::<Impl, IMPL_OFFSET>,
            ItemStatusProperty: ItemStatusProperty::<Impl, IMPL_OFFSET>,
            ItemTypeProperty: ItemTypeProperty::<Impl, IMPL_OFFSET>,
            LabeledByProperty: LabeledByProperty::<Impl, IMPL_OFFSET>,
            LocalizedControlTypeProperty: LocalizedControlTypeProperty::<Impl, IMPL_OFFSET>,
            NameProperty: NameProperty::<Impl, IMPL_OFFSET>,
            OrientationProperty: OrientationProperty::<Impl, IMPL_OFFSET>,
            LiveSettingProperty: LiveSettingProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics2Impl: Sized {
    fn ControlledPeersProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics2Vtbl {
        unsafe extern "system" fn ControlledPeersProperty<Impl: IAutomationElementIdentifiersStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlledPeersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics2, BASE_OFFSET>(),
            ControlledPeersProperty: ControlledPeersProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics3Impl: Sized {
    fn PositionInSetProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SizeOfSetProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LevelProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AnnotationsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics3Vtbl {
        unsafe extern "system" fn PositionInSetProperty<Impl: IAutomationElementIdentifiersStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionInSetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeOfSetProperty<Impl: IAutomationElementIdentifiersStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeOfSetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LevelProperty<Impl: IAutomationElementIdentifiersStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationsProperty<Impl: IAutomationElementIdentifiersStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics3, BASE_OFFSET>(),
            PositionInSetProperty: PositionInSetProperty::<Impl, IMPL_OFFSET>,
            SizeOfSetProperty: SizeOfSetProperty::<Impl, IMPL_OFFSET>,
            LevelProperty: LevelProperty::<Impl, IMPL_OFFSET>,
            AnnotationsProperty: AnnotationsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics4Impl: Sized {
    fn LandmarkTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LocalizedLandmarkTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics4Vtbl {
        unsafe extern "system" fn LandmarkTypeProperty<Impl: IAutomationElementIdentifiersStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LandmarkTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedLandmarkTypeProperty<Impl: IAutomationElementIdentifiersStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedLandmarkTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics4, BASE_OFFSET>(),
            LandmarkTypeProperty: LandmarkTypeProperty::<Impl, IMPL_OFFSET>,
            LocalizedLandmarkTypeProperty: LocalizedLandmarkTypeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics5Impl: Sized {
    fn IsPeripheralProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsDataValidForFormProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FullDescriptionProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DescribedByProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FlowsToProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FlowsFromProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics5Vtbl {
        unsafe extern "system" fn IsPeripheralProperty<Impl: IAutomationElementIdentifiersStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeripheralProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForFormProperty<Impl: IAutomationElementIdentifiersStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataValidForFormProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullDescriptionProperty<Impl: IAutomationElementIdentifiersStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullDescriptionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DescribedByProperty<Impl: IAutomationElementIdentifiersStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DescribedByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowsToProperty<Impl: IAutomationElementIdentifiersStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowsToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowsFromProperty<Impl: IAutomationElementIdentifiersStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowsFromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics5, BASE_OFFSET>(),
            IsPeripheralProperty: IsPeripheralProperty::<Impl, IMPL_OFFSET>,
            IsDataValidForFormProperty: IsDataValidForFormProperty::<Impl, IMPL_OFFSET>,
            FullDescriptionProperty: FullDescriptionProperty::<Impl, IMPL_OFFSET>,
            DescribedByProperty: DescribedByProperty::<Impl, IMPL_OFFSET>,
            FlowsToProperty: FlowsToProperty::<Impl, IMPL_OFFSET>,
            FlowsFromProperty: FlowsFromProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics6Impl: Sized {
    fn CultureProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics6Vtbl {
        unsafe extern "system" fn CultureProperty<Impl: IAutomationElementIdentifiersStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CultureProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics6, BASE_OFFSET>(),
            CultureProperty: CultureProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics7Impl: Sized {
    fn HeadingLevelProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics7Vtbl {
        unsafe extern "system" fn HeadingLevelProperty<Impl: IAutomationElementIdentifiersStatics7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingLevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics7, BASE_OFFSET>(),
            HeadingLevelProperty: HeadingLevelProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics8Impl: Sized {
    fn IsDialogProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics8Vtbl {
        unsafe extern "system" fn IsDialogProperty<Impl: IAutomationElementIdentifiersStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDialogProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiersStatics8, BASE_OFFSET>(),
            IsDialogProperty: IsDialogProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiersStatics8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationProperties {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationProperties, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStaticsImpl: Sized {
    fn AcceleratorKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAcceleratorKey(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAcceleratorKey(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAccessKey(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AutomationIdProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAutomationId(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAutomationId(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HelpTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHelpText(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHelpText(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsRequiredForFormProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsRequiredForForm(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsRequiredForForm(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn ItemStatusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemStatus(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemStatus(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemType(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LabeledByProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLabeledBy(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::UIElement>;
    fn SetLabeledBy(&self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn NameProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetName(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LiveSettingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLiveSetting(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationLiveSetting>;
    fn SetLiveSetting(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationLiveSetting) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStaticsVtbl {
        unsafe extern "system" fn AcceleratorKeyProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceleratorKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcceleratorKey<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcceleratorKey(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleratorKey<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleratorKey(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKey<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessKey(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKey<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutomationIdProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationIdProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationId<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationId(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationId<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomationId(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HelpTextProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HelpTextProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpText<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpText(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpText<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpText(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRequiredForFormProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequiredForFormProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsRequiredForForm<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsRequiredForForm(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRequiredForForm<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRequiredForForm(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ItemStatusProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemStatusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatus<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemStatus(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemStatus<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemStatus(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemTypeProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemType<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemType<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LabeledByProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LabeledByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledBy<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLabeledBy(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabeledBy<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabeledBy(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NameProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LiveSettingProperty<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LiveSettingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSetting<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLiveSetting(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLiveSetting<Impl: IAutomationPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLiveSetting(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics, BASE_OFFSET>(),
            AcceleratorKeyProperty: AcceleratorKeyProperty::<Impl, IMPL_OFFSET>,
            GetAcceleratorKey: GetAcceleratorKey::<Impl, IMPL_OFFSET>,
            SetAcceleratorKey: SetAcceleratorKey::<Impl, IMPL_OFFSET>,
            AccessKeyProperty: AccessKeyProperty::<Impl, IMPL_OFFSET>,
            GetAccessKey: GetAccessKey::<Impl, IMPL_OFFSET>,
            SetAccessKey: SetAccessKey::<Impl, IMPL_OFFSET>,
            AutomationIdProperty: AutomationIdProperty::<Impl, IMPL_OFFSET>,
            GetAutomationId: GetAutomationId::<Impl, IMPL_OFFSET>,
            SetAutomationId: SetAutomationId::<Impl, IMPL_OFFSET>,
            HelpTextProperty: HelpTextProperty::<Impl, IMPL_OFFSET>,
            GetHelpText: GetHelpText::<Impl, IMPL_OFFSET>,
            SetHelpText: SetHelpText::<Impl, IMPL_OFFSET>,
            IsRequiredForFormProperty: IsRequiredForFormProperty::<Impl, IMPL_OFFSET>,
            GetIsRequiredForForm: GetIsRequiredForForm::<Impl, IMPL_OFFSET>,
            SetIsRequiredForForm: SetIsRequiredForForm::<Impl, IMPL_OFFSET>,
            ItemStatusProperty: ItemStatusProperty::<Impl, IMPL_OFFSET>,
            GetItemStatus: GetItemStatus::<Impl, IMPL_OFFSET>,
            SetItemStatus: SetItemStatus::<Impl, IMPL_OFFSET>,
            ItemTypeProperty: ItemTypeProperty::<Impl, IMPL_OFFSET>,
            GetItemType: GetItemType::<Impl, IMPL_OFFSET>,
            SetItemType: SetItemType::<Impl, IMPL_OFFSET>,
            LabeledByProperty: LabeledByProperty::<Impl, IMPL_OFFSET>,
            GetLabeledBy: GetLabeledBy::<Impl, IMPL_OFFSET>,
            SetLabeledBy: SetLabeledBy::<Impl, IMPL_OFFSET>,
            NameProperty: NameProperty::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            LiveSettingProperty: LiveSettingProperty::<Impl, IMPL_OFFSET>,
            GetLiveSetting: GetLiveSetting::<Impl, IMPL_OFFSET>,
            SetLiveSetting: SetLiveSetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics2Impl: Sized {
    fn AccessibilityViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAccessibilityView(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AccessibilityView>;
    fn SetAccessibilityView(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AccessibilityView) -> ::windows::core::Result<()>;
    fn ControlledPeersProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetControlledPeers(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics2Vtbl {
        unsafe extern "system" fn AccessibilityViewProperty<Impl: IAutomationPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessibilityViewProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessibilityView<Impl: IAutomationPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AccessibilityView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityView(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityView<Impl: IAutomationPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AccessibilityView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityView(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ControlledPeersProperty<Impl: IAutomationPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlledPeersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlledPeers<Impl: IAutomationPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlledPeers(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics2, BASE_OFFSET>(),
            AccessibilityViewProperty: AccessibilityViewProperty::<Impl, IMPL_OFFSET>,
            GetAccessibilityView: GetAccessibilityView::<Impl, IMPL_OFFSET>,
            SetAccessibilityView: SetAccessibilityView::<Impl, IMPL_OFFSET>,
            ControlledPeersProperty: ControlledPeersProperty::<Impl, IMPL_OFFSET>,
            GetControlledPeers: GetControlledPeers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics3Impl: Sized {
    fn PositionInSetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetPositionInSet(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetPositionInSet(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn SizeOfSetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetSizeOfSet(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetSizeOfSet(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn LevelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLevel(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetLevel(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn AnnotationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAnnotations(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics3Vtbl {
        unsafe extern "system" fn PositionInSetProperty<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionInSetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionInSet<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionInSet(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionInSet<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionInSet(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SizeOfSetProperty<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeOfSetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSet<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeOfSet(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeOfSet<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeOfSet(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn LevelProperty<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevel(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLevel(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AnnotationsProperty<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotations<Impl: IAutomationPropertiesStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotations(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics3, BASE_OFFSET>(),
            PositionInSetProperty: PositionInSetProperty::<Impl, IMPL_OFFSET>,
            GetPositionInSet: GetPositionInSet::<Impl, IMPL_OFFSET>,
            SetPositionInSet: SetPositionInSet::<Impl, IMPL_OFFSET>,
            SizeOfSetProperty: SizeOfSetProperty::<Impl, IMPL_OFFSET>,
            GetSizeOfSet: GetSizeOfSet::<Impl, IMPL_OFFSET>,
            SetSizeOfSet: SetSizeOfSet::<Impl, IMPL_OFFSET>,
            LevelProperty: LevelProperty::<Impl, IMPL_OFFSET>,
            GetLevel: GetLevel::<Impl, IMPL_OFFSET>,
            SetLevel: SetLevel::<Impl, IMPL_OFFSET>,
            AnnotationsProperty: AnnotationsProperty::<Impl, IMPL_OFFSET>,
            GetAnnotations: GetAnnotations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics4Impl: Sized {
    fn LandmarkTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationLandmarkType>;
    fn SetLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationLandmarkType) -> ::windows::core::Result<()>;
    fn LocalizedLandmarkTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLocalizedLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalizedLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics4Vtbl {
        unsafe extern "system" fn LandmarkTypeProperty<Impl: IAutomationPropertiesStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LandmarkTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLandmarkType<Impl: IAutomationPropertiesStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLandmarkType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLandmarkType<Impl: IAutomationPropertiesStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLandmarkType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn LocalizedLandmarkTypeProperty<Impl: IAutomationPropertiesStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedLandmarkTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkType<Impl: IAutomationPropertiesStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedLandmarkType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalizedLandmarkType<Impl: IAutomationPropertiesStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalizedLandmarkType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics4, BASE_OFFSET>(),
            LandmarkTypeProperty: LandmarkTypeProperty::<Impl, IMPL_OFFSET>,
            GetLandmarkType: GetLandmarkType::<Impl, IMPL_OFFSET>,
            SetLandmarkType: SetLandmarkType::<Impl, IMPL_OFFSET>,
            LocalizedLandmarkTypeProperty: LocalizedLandmarkTypeProperty::<Impl, IMPL_OFFSET>,
            GetLocalizedLandmarkType: GetLocalizedLandmarkType::<Impl, IMPL_OFFSET>,
            SetLocalizedLandmarkType: SetLocalizedLandmarkType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics5Impl: Sized {
    fn IsPeripheralProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsPeripheral(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsPeripheral(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn IsDataValidForFormProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDataValidForForm(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDataValidForForm(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn FullDescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFullDescription(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFullDescription(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LocalizedControlTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLocalizedControlType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalizedControlType(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DescribedByProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDescribedBy(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
    fn FlowsToProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFlowsTo(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
    fn FlowsFromProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFlowsFrom(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics5Vtbl {
        unsafe extern "system" fn IsPeripheralProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeripheralProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsPeripheral<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsPeripheral(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPeripheral<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPeripheral(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn IsDataValidForFormProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataValidForFormProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDataValidForForm<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsDataValidForForm(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDataValidForForm<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDataValidForForm(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn FullDescriptionProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullDescriptionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescription<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullDescription(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullDescription<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullDescription(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalizedControlTypeProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedControlTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlType<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedControlType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalizedControlType<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalizedControlType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DescribedByProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DescribedByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescribedBy<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescribedBy(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowsToProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowsToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsTo<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowsTo(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowsFromProperty<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowsFromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsFrom<Impl: IAutomationPropertiesStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowsFrom(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics5, BASE_OFFSET>(),
            IsPeripheralProperty: IsPeripheralProperty::<Impl, IMPL_OFFSET>,
            GetIsPeripheral: GetIsPeripheral::<Impl, IMPL_OFFSET>,
            SetIsPeripheral: SetIsPeripheral::<Impl, IMPL_OFFSET>,
            IsDataValidForFormProperty: IsDataValidForFormProperty::<Impl, IMPL_OFFSET>,
            GetIsDataValidForForm: GetIsDataValidForForm::<Impl, IMPL_OFFSET>,
            SetIsDataValidForForm: SetIsDataValidForForm::<Impl, IMPL_OFFSET>,
            FullDescriptionProperty: FullDescriptionProperty::<Impl, IMPL_OFFSET>,
            GetFullDescription: GetFullDescription::<Impl, IMPL_OFFSET>,
            SetFullDescription: SetFullDescription::<Impl, IMPL_OFFSET>,
            LocalizedControlTypeProperty: LocalizedControlTypeProperty::<Impl, IMPL_OFFSET>,
            GetLocalizedControlType: GetLocalizedControlType::<Impl, IMPL_OFFSET>,
            SetLocalizedControlType: SetLocalizedControlType::<Impl, IMPL_OFFSET>,
            DescribedByProperty: DescribedByProperty::<Impl, IMPL_OFFSET>,
            GetDescribedBy: GetDescribedBy::<Impl, IMPL_OFFSET>,
            FlowsToProperty: FlowsToProperty::<Impl, IMPL_OFFSET>,
            GetFlowsTo: GetFlowsTo::<Impl, IMPL_OFFSET>,
            FlowsFromProperty: FlowsFromProperty::<Impl, IMPL_OFFSET>,
            GetFlowsFrom: GetFlowsFrom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics6Impl: Sized {
    fn CultureProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCulture(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetCulture(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPropertiesStatics6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics6Vtbl {
        unsafe extern "system" fn CultureProperty<Impl: IAutomationPropertiesStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CultureProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCulture<Impl: IAutomationPropertiesStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCulture(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCulture<Impl: IAutomationPropertiesStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCulture(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics6, BASE_OFFSET>(),
            CultureProperty: CultureProperty::<Impl, IMPL_OFFSET>,
            GetCulture: GetCulture::<Impl, IMPL_OFFSET>,
            SetCulture: SetCulture::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics7Impl: Sized {
    fn HeadingLevelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHeadingLevel(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationHeadingLevel>;
    fn SetHeadingLevel(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationHeadingLevel) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics7Vtbl {
        unsafe extern "system" fn HeadingLevelProperty<Impl: IAutomationPropertiesStatics7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingLevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHeadingLevel<Impl: IAutomationPropertiesStatics7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeadingLevel(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeadingLevel<Impl: IAutomationPropertiesStatics7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeadingLevel(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics7, BASE_OFFSET>(),
            HeadingLevelProperty: HeadingLevelProperty::<Impl, IMPL_OFFSET>,
            GetHeadingLevel: GetHeadingLevel::<Impl, IMPL_OFFSET>,
            SetHeadingLevel: SetHeadingLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics8Impl: Sized {
    fn IsDialogProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDialog(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDialog(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPropertiesStatics8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics8Vtbl {
        unsafe extern "system" fn IsDialogProperty<Impl: IAutomationPropertiesStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDialogProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDialog<Impl: IAutomationPropertiesStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsDialog(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDialog<Impl: IAutomationPropertiesStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDialog(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics8, BASE_OFFSET>(),
            IsDialogProperty: IsDialogProperty::<Impl, IMPL_OFFSET>,
            GetIsDialog: GetIsDialog::<Impl, IMPL_OFFSET>,
            SetIsDialog: SetIsDialog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics9Impl: Sized {
    fn AutomationControlTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAutomationControlType(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<Peers::AutomationControlType>;
    fn SetAutomationControlType(&self, element: &::core::option::Option<super::UIElement>, value: Peers::AutomationControlType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics9Vtbl {
        unsafe extern "system" fn AutomationControlTypeProperty<Impl: IAutomationPropertiesStatics9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationControlTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlType<Impl: IAutomationPropertiesStatics9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationControlType(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationControlType<Impl: IAutomationPropertiesStatics9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomationControlType(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPropertiesStatics9, BASE_OFFSET>(),
            AutomationControlTypeProperty: AutomationControlTypeProperty::<Impl, IMPL_OFFSET>,
            GetAutomationControlType: GetAutomationControlType::<Impl, IMPL_OFFSET>,
            SetAutomationControlType: SetAutomationControlType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPropertiesStatics9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertyImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationProperty";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertyVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationProperty, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDockPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDockPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDockPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IDockPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDockPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDockPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDockPatternIdentifiersStaticsImpl: Sized {
    fn DockPositionProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDockPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDockPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn DockPositionProperty<Impl: IDockPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DockPositionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDockPatternIdentifiersStatics, BASE_OFFSET>(),
            DockPositionProperty: DockPositionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDockPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDragPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IDragPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDragPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragPatternIdentifiersStaticsImpl: Sized {
    fn DropEffectProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DropEffectsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn GrabbedItemsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsGrabbedProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDragPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn DropEffectProperty<Impl: IDragPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffectsProperty<Impl: IDragPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffectsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrabbedItemsProperty<Impl: IDragPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrabbedItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrabbedProperty<Impl: IDragPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrabbedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragPatternIdentifiersStatics, BASE_OFFSET>(),
            DropEffectProperty: DropEffectProperty::<Impl, IMPL_OFFSET>,
            DropEffectsProperty: DropEffectsProperty::<Impl, IMPL_OFFSET>,
            GrabbedItemsProperty: GrabbedItemsProperty::<Impl, IMPL_OFFSET>,
            IsGrabbedProperty: IsGrabbedProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDropTargetPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetPatternIdentifiersStaticsImpl: Sized {
    fn DropTargetEffectProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DropTargetEffectsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn DropTargetEffectProperty<Impl: IDropTargetPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropTargetEffectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropTargetEffectsProperty<Impl: IDropTargetPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropTargetEffectsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDropTargetPatternIdentifiersStatics, BASE_OFFSET>(),
            DropTargetEffectProperty: DropTargetEffectProperty::<Impl, IMPL_OFFSET>,
            DropTargetEffectsProperty: DropTargetEffectsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExpandCollapsePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IExpandCollapsePatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapsePatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapsePatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IExpandCollapsePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpandCollapsePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExpandCollapsePatternIdentifiersStaticsImpl: Sized {
    fn ExpandCollapseStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExpandCollapsePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IExpandCollapsePatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapsePatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapsePatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ExpandCollapseStateProperty<Impl: IExpandCollapsePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandCollapseStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExpandCollapsePatternIdentifiersStatics, BASE_OFFSET>(),
            ExpandCollapseStateProperty: ExpandCollapseStateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpandCollapsePatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IGridItemPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridItemPatternIdentifiersStaticsImpl: Sized {
    fn ColumnProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ColumnSpanProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ContainingGridProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowSpanProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridItemPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ColumnProperty<Impl: IGridItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpanProperty<Impl: IGridItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnSpanProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGridProperty<Impl: IGridItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainingGridProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowProperty<Impl: IGridItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpanProperty<Impl: IGridItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowSpanProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridItemPatternIdentifiersStatics, BASE_OFFSET>(),
            ColumnProperty: ColumnProperty::<Impl, IMPL_OFFSET>,
            ColumnSpanProperty: ColumnSpanProperty::<Impl, IMPL_OFFSET>,
            ContainingGridProperty: ContainingGridProperty::<Impl, IMPL_OFFSET>,
            RowProperty: RowProperty::<Impl, IMPL_OFFSET>,
            RowSpanProperty: RowSpanProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridItemPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IGridPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridPatternIdentifiersStaticsImpl: Sized {
    fn ColumnCountProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowCountProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ColumnCountProperty<Impl: IGridPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnCountProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCountProperty<Impl: IGridPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowCountProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridPatternIdentifiersStatics, BASE_OFFSET>(),
            ColumnCountProperty: ColumnCountProperty::<Impl, IMPL_OFFSET>,
            RowCountProperty: RowCountProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultipleViewPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultipleViewPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IMultipleViewPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMultipleViewPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleViewPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultipleViewPatternIdentifiersStaticsImpl: Sized {
    fn CurrentViewProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SupportedViewsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultipleViewPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMultipleViewPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn CurrentViewProperty<Impl: IMultipleViewPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentViewProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedViewsProperty<Impl: IMultipleViewPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedViewsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMultipleViewPatternIdentifiersStatics, BASE_OFFSET>(),
            CurrentViewProperty: CurrentViewProperty::<Impl, IMPL_OFFSET>,
            SupportedViewsProperty: SupportedViewsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleViewPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeValuePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeValuePatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValuePatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValuePatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeValuePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeValuePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeValuePatternIdentifiersStaticsImpl: Sized {
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LargeChangeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MaximumProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MinimumProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SmallChangeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeValuePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeValuePatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValuePatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValuePatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn IsReadOnlyProperty<Impl: IRangeValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnlyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChangeProperty<Impl: IRangeValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeChangeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumProperty<Impl: IRangeValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumProperty<Impl: IRangeValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChangeProperty<Impl: IRangeValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallChangeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueProperty<Impl: IRangeValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeValuePatternIdentifiersStatics, BASE_OFFSET>(),
            IsReadOnlyProperty: IsReadOnlyProperty::<Impl, IMPL_OFFSET>,
            LargeChangeProperty: LargeChangeProperty::<Impl, IMPL_OFFSET>,
            MaximumProperty: MaximumProperty::<Impl, IMPL_OFFSET>,
            MinimumProperty: MinimumProperty::<Impl, IMPL_OFFSET>,
            SmallChangeProperty: SmallChangeProperty::<Impl, IMPL_OFFSET>,
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeValuePatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IScrollPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollPatternIdentifiersStaticsImpl: Sized {
    fn HorizontallyScrollableProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HorizontalScrollPercentProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HorizontalViewSizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn NoScroll(&self) -> ::windows::core::Result<f64>;
    fn VerticallyScrollableProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn VerticalScrollPercentProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn VerticalViewSizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn HorizontallyScrollableProperty<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontallyScrollableProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalScrollPercentProperty<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalScrollPercentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSizeProperty<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalViewSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoScroll<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollableProperty<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticallyScrollableProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercentProperty<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalScrollPercentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSizeProperty<Impl: IScrollPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalViewSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollPatternIdentifiersStatics, BASE_OFFSET>(),
            HorizontallyScrollableProperty: HorizontallyScrollableProperty::<Impl, IMPL_OFFSET>,
            HorizontalScrollPercentProperty: HorizontalScrollPercentProperty::<Impl, IMPL_OFFSET>,
            HorizontalViewSizeProperty: HorizontalViewSizeProperty::<Impl, IMPL_OFFSET>,
            NoScroll: NoScroll::<Impl, IMPL_OFFSET>,
            VerticallyScrollableProperty: VerticallyScrollableProperty::<Impl, IMPL_OFFSET>,
            VerticalScrollPercentProperty: VerticalScrollPercentProperty::<Impl, IMPL_OFFSET>,
            VerticalViewSizeProperty: VerticalViewSizeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionItemPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionItemPatternIdentifiersStaticsImpl: Sized {
    fn IsSelectedProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SelectionContainerProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionItemPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn IsSelectedProperty<Impl: ISelectionItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainerProperty<Impl: ISelectionItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionContainerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionItemPatternIdentifiersStatics, BASE_OFFSET>(),
            IsSelectedProperty: IsSelectedProperty::<Impl, IMPL_OFFSET>,
            SelectionContainerProperty: SelectionContainerProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionItemPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionPatternIdentifiersStaticsImpl: Sized {
    fn CanSelectMultipleProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsSelectionRequiredProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SelectionProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn CanSelectMultipleProperty<Impl: ISelectionPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSelectMultipleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequiredProperty<Impl: ISelectionPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectionRequiredProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionProperty<Impl: ISelectionPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionPatternIdentifiersStatics, BASE_OFFSET>(),
            CanSelectMultipleProperty: CanSelectMultipleProperty::<Impl, IMPL_OFFSET>,
            IsSelectionRequiredProperty: IsSelectionRequiredProperty::<Impl, IMPL_OFFSET>,
            SelectionProperty: SelectionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpreadsheetItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ISpreadsheetItemPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpreadsheetItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpreadsheetItemPatternIdentifiersStaticsImpl: Sized {
    fn FormulaProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpreadsheetItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpreadsheetItemPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn FormulaProperty<Impl: ISpreadsheetItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormulaProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpreadsheetItemPatternIdentifiersStatics, BASE_OFFSET>(),
            FormulaProperty: FormulaProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetItemPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStylesPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStylesPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IStylesPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IStylesPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStylesPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylesPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStylesPatternIdentifiersStaticsImpl: Sized {
    fn ExtendedPropertiesProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FillColorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FillPatternColorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FillPatternStyleProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ShapeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn StyleIdProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn StyleNameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStylesPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStylesPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ExtendedPropertiesProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedPropertiesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColorProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColorProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyleProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternStyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShapeProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShapeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleIdProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleIdProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleNameProperty<Impl: IStylesPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStylesPatternIdentifiersStatics, BASE_OFFSET>(),
            ExtendedPropertiesProperty: ExtendedPropertiesProperty::<Impl, IMPL_OFFSET>,
            FillColorProperty: FillColorProperty::<Impl, IMPL_OFFSET>,
            FillPatternColorProperty: FillPatternColorProperty::<Impl, IMPL_OFFSET>,
            FillPatternStyleProperty: FillPatternStyleProperty::<Impl, IMPL_OFFSET>,
            ShapeProperty: ShapeProperty::<Impl, IMPL_OFFSET>,
            StyleIdProperty: StyleIdProperty::<Impl, IMPL_OFFSET>,
            StyleNameProperty: StyleNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylesPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITableItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITableItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITableItemPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITableItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITableItemPatternIdentifiersStaticsImpl: Sized {
    fn ColumnHeaderItemsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowHeaderItemsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITableItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITableItemPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ColumnHeaderItemsProperty<Impl: ITableItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnHeaderItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowHeaderItemsProperty<Impl: ITableItemPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowHeaderItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITableItemPatternIdentifiersStatics, BASE_OFFSET>(),
            ColumnHeaderItemsProperty: ColumnHeaderItemsProperty::<Impl, IMPL_OFFSET>,
            RowHeaderItemsProperty: RowHeaderItemsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableItemPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITablePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITablePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITablePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITablePatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITablePatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITablePatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITablePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITablePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITablePatternIdentifiersStaticsImpl: Sized {
    fn ColumnHeadersProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowHeadersProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowOrColumnMajorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITablePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITablePatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITablePatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITablePatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ColumnHeadersProperty<Impl: ITablePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnHeadersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowHeadersProperty<Impl: ITablePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowHeadersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowOrColumnMajorProperty<Impl: ITablePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowOrColumnMajorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITablePatternIdentifiersStatics, BASE_OFFSET>(),
            ColumnHeadersProperty: ColumnHeadersProperty::<Impl, IMPL_OFFSET>,
            RowHeadersProperty: RowHeadersProperty::<Impl, IMPL_OFFSET>,
            RowOrColumnMajorProperty: RowOrColumnMajorProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITablePatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITogglePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITogglePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITogglePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITogglePatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITogglePatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITogglePatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITogglePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITogglePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITogglePatternIdentifiersStaticsImpl: Sized {
    fn ToggleStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITogglePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITogglePatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITogglePatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITogglePatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn ToggleStateProperty<Impl: ITogglePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITogglePatternIdentifiersStatics, BASE_OFFSET>(),
            ToggleStateProperty: ToggleStateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITogglePatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPattern2IdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPattern2Identifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPattern2Identifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPattern2IdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPattern2IdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPattern2IdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformPattern2Identifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformPattern2Identifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPattern2IdentifiersStaticsImpl: Sized {
    fn CanZoomProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ZoomLevelProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MaxZoomProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MinZoomProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPattern2IdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPattern2IdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPattern2IdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPattern2IdentifiersStaticsVtbl {
        unsafe extern "system" fn CanZoomProperty<Impl: ITransformPattern2IdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoomProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevelProperty<Impl: ITransformPattern2IdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxZoomProperty<Impl: ITransformPattern2IdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxZoomProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinZoomProperty<Impl: ITransformPattern2IdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinZoomProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformPattern2IdentifiersStatics, BASE_OFFSET>(),
            CanZoomProperty: CanZoomProperty::<Impl, IMPL_OFFSET>,
            ZoomLevelProperty: ZoomLevelProperty::<Impl, IMPL_OFFSET>,
            MaxZoomProperty: MaxZoomProperty::<Impl, IMPL_OFFSET>,
            MinZoomProperty: MinZoomProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformPattern2IdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPatternIdentifiersStaticsImpl: Sized {
    fn CanMoveProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn CanResizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn CanRotateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn CanMoveProperty<Impl: ITransformPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMoveProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResizeProperty<Impl: ITransformPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanResizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotateProperty<Impl: ITransformPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRotateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformPatternIdentifiersStatics, BASE_OFFSET>(),
            CanMoveProperty: CanMoveProperty::<Impl, IMPL_OFFSET>,
            CanResizeProperty: CanResizeProperty::<Impl, IMPL_OFFSET>,
            CanRotateProperty: CanRotateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IValuePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IValuePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IValuePatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValuePatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValuePatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IValuePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValuePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IValuePatternIdentifiersStaticsImpl: Sized {
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IValuePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IValuePatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValuePatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValuePatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn IsReadOnlyProperty<Impl: IValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnlyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueProperty<Impl: IValuePatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IValuePatternIdentifiersStatics, BASE_OFFSET>(),
            IsReadOnlyProperty: IsReadOnlyProperty::<Impl, IMPL_OFFSET>,
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValuePatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IWindowPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowPatternIdentifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowPatternIdentifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowPatternIdentifiersVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowPatternIdentifiersStaticsImpl: Sized {
    fn CanMaximizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn CanMinimizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsModalProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsTopmostProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn WindowInteractionStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn WindowVisualStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowPatternIdentifiersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowPatternIdentifiersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowPatternIdentifiersStaticsVtbl {
        unsafe extern "system" fn CanMaximizeProperty<Impl: IWindowPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMaximizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMinimizeProperty<Impl: IWindowPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMinimizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsModalProperty<Impl: IWindowPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsModalProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmostProperty<Impl: IWindowPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTopmostProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowInteractionStateProperty<Impl: IWindowPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowInteractionStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowVisualStateProperty<Impl: IWindowPatternIdentifiersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowVisualStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowPatternIdentifiersStatics, BASE_OFFSET>(),
            CanMaximizeProperty: CanMaximizeProperty::<Impl, IMPL_OFFSET>,
            CanMinimizeProperty: CanMinimizeProperty::<Impl, IMPL_OFFSET>,
            IsModalProperty: IsModalProperty::<Impl, IMPL_OFFSET>,
            IsTopmostProperty: IsTopmostProperty::<Impl, IMPL_OFFSET>,
            WindowInteractionStateProperty: WindowInteractionStateProperty::<Impl, IMPL_OFFSET>,
            WindowVisualStateProperty: WindowVisualStateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowPatternIdentifiersStatics as ::windows::core::Interface>::IID
    }
}

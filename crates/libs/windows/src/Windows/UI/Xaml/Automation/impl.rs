#[cfg(feature = "implement_exclusive")]
pub trait IAnnotationPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnnotationPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IAnnotationPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAnnotationPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnnotationPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnnotationPatternIdentifiersStatics_Impl: Sized {
    fn AnnotationTypeIdProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn AnnotationTypeNameProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn AuthorProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn DateTimeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn TargetProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnnotationPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAnnotationPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn AnnotationTypeIdProperty<Impl: IAnnotationPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AnnotationTypeNameProperty<Impl: IAnnotationPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthorProperty<Impl: IAnnotationPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DateTimeProperty<Impl: IAnnotationPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetProperty<Impl: IAnnotationPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationAnnotation_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<AnnotationType>;
    fn SetType(&mut self, value: AnnotationType) -> ::windows::core::Result<()>;
    fn Element(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn SetElement(&mut self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationAnnotation";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationAnnotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationAnnotation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationAnnotation_Vtbl {
        unsafe extern "system" fn Type<Impl: IAutomationAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnnotationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IAutomationAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Element<Impl: IAutomationAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetElement<Impl: IAutomationAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationAnnotationFactory_Impl: Sized {
    fn CreateInstance(&mut self, r#type: AnnotationType) -> ::windows::core::Result<AutomationAnnotation>;
    fn CreateWithElementParameter(&mut self, r#type: AnnotationType, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<AutomationAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationAnnotationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationAnnotationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationAnnotationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationAnnotationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationAnnotationFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationAnnotationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithElementParameter<Impl: IAutomationAnnotationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AnnotationType, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationAnnotationStatics_Impl: Sized {
    fn TypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationAnnotationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationAnnotationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationAnnotationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationAnnotationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationAnnotationStatics_Vtbl {
        unsafe extern "system" fn TypeProperty<Impl: IAutomationAnnotationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ElementProperty<Impl: IAutomationAnnotationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElementIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElementIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics_Impl: Sized {
    fn AcceleratorKeyProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn AccessKeyProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn AutomationIdProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn BoundingRectangleProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ClassNameProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ClickablePointProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ControlTypeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn HasKeyboardFocusProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn HelpTextProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsContentElementProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsControlElementProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsEnabledProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsKeyboardFocusableProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsOffscreenProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsPasswordProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsRequiredForFormProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ItemStatusProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ItemTypeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn LabeledByProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn LocalizedControlTypeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn NameProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn OrientationProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn LiveSettingProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics_Vtbl {
        unsafe extern "system" fn AcceleratorKeyProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccessKeyProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutomationIdProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BoundingRectangleProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClassNameProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClickablePointProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControlTypeProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKeyboardFocusProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HelpTextProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsContentElementProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsControlElementProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabledProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsKeyboardFocusableProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOffscreenProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPasswordProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRequiredForFormProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemStatusProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemTypeProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LabeledByProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalizedControlTypeProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NameProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OrientationProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LiveSettingProperty<Impl: IAutomationElementIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics2_Impl: Sized {
    fn ControlledPeersProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics2_Vtbl {
        unsafe extern "system" fn ControlledPeersProperty<Impl: IAutomationElementIdentifiersStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics3_Impl: Sized {
    fn PositionInSetProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn SizeOfSetProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn LevelProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn AnnotationsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics3_Vtbl {
        unsafe extern "system" fn PositionInSetProperty<Impl: IAutomationElementIdentifiersStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SizeOfSetProperty<Impl: IAutomationElementIdentifiersStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LevelProperty<Impl: IAutomationElementIdentifiersStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AnnotationsProperty<Impl: IAutomationElementIdentifiersStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics4_Impl: Sized {
    fn LandmarkTypeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn LocalizedLandmarkTypeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics4_Vtbl {
        unsafe extern "system" fn LandmarkTypeProperty<Impl: IAutomationElementIdentifiersStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalizedLandmarkTypeProperty<Impl: IAutomationElementIdentifiersStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics5_Impl: Sized {
    fn IsPeripheralProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsDataValidForFormProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn FullDescriptionProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn DescribedByProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn FlowsToProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn FlowsFromProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics5_Vtbl {
        unsafe extern "system" fn IsPeripheralProperty<Impl: IAutomationElementIdentifiersStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDataValidForFormProperty<Impl: IAutomationElementIdentifiersStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FullDescriptionProperty<Impl: IAutomationElementIdentifiersStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DescribedByProperty<Impl: IAutomationElementIdentifiersStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FlowsToProperty<Impl: IAutomationElementIdentifiersStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FlowsFromProperty<Impl: IAutomationElementIdentifiersStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics6_Impl: Sized {
    fn CultureProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics6_Vtbl {
        unsafe extern "system" fn CultureProperty<Impl: IAutomationElementIdentifiersStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics7_Impl: Sized {
    fn HeadingLevelProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics7_Vtbl {
        unsafe extern "system" fn HeadingLevelProperty<Impl: IAutomationElementIdentifiersStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationElementIdentifiersStatics8_Impl: Sized {
    fn IsDialogProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElementIdentifiersStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementIdentifiersStatics8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementIdentifiersStatics8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElementIdentifiersStatics8_Vtbl {
        unsafe extern "system" fn IsDialogProperty<Impl: IAutomationElementIdentifiersStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationProperties_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationProperties {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationProperties_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationProperties, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IAutomationPropertiesStatics_Impl: Sized {
    fn AcceleratorKeyProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAcceleratorKey(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAcceleratorKey(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AccessKeyProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAccessKey(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AutomationIdProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAutomationId(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAutomationId(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HelpTextProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHelpText(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHelpText(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsRequiredForFormProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsRequiredForForm(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsRequiredForForm(&mut self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn ItemStatusProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemStatus(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemStatus(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemTypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemType(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemType(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LabeledByProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLabeledBy(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::UIElement>;
    fn SetLabeledBy(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn NameProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetName(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LiveSettingProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLiveSetting(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationLiveSetting>;
    fn SetLiveSetting(&mut self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationLiveSetting) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics_Vtbl {
        unsafe extern "system" fn AcceleratorKeyProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAcceleratorKey<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAcceleratorKey<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleratorKey(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAccessKey<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccessKey<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutomationIdProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAutomationId<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutomationId<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomationId(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HelpTextProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetHelpText<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHelpText<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpText(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRequiredForFormProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsRequiredForForm<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRequiredForForm<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRequiredForForm(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ItemStatusProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemStatus<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemStatus<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemStatus(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemTypeProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemType<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemType<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LabeledByProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLabeledBy<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLabeledBy<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabeledBy(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NameProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetName<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LiveSettingProperty<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLiveSetting<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLiveSetting) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLiveSetting<Impl: IAutomationPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationLiveSetting) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics2_Impl: Sized {
    fn AccessibilityViewProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAccessibilityView(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AccessibilityView>;
    fn SetAccessibilityView(&mut self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AccessibilityView) -> ::windows::core::Result<()>;
    fn ControlledPeersProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetControlledPeers(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics2_Vtbl {
        unsafe extern "system" fn AccessibilityViewProperty<Impl: IAutomationPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAccessibilityView<Impl: IAutomationPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AccessibilityView) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccessibilityView<Impl: IAutomationPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AccessibilityView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityView(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ControlledPeersProperty<Impl: IAutomationPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetControlledPeers<Impl: IAutomationPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics3_Impl: Sized {
    fn PositionInSetProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetPositionInSet(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetPositionInSet(&mut self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn SizeOfSetProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetSizeOfSet(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetSizeOfSet(&mut self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn LevelProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLevel(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetLevel(&mut self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn AnnotationsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAnnotations(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics3_Vtbl {
        unsafe extern "system" fn PositionInSetProperty<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPositionInSet<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionInSet<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionInSet(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SizeOfSetProperty<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSizeOfSet<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSizeOfSet<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeOfSet(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn LevelProperty<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLevel<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLevel<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLevel(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AnnotationsProperty<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAnnotations<Impl: IAutomationPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics4_Impl: Sized {
    fn LandmarkTypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLandmarkType(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationLandmarkType>;
    fn SetLandmarkType(&mut self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationLandmarkType) -> ::windows::core::Result<()>;
    fn LocalizedLandmarkTypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLocalizedLandmarkType(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalizedLandmarkType(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics4_Vtbl {
        unsafe extern "system" fn LandmarkTypeProperty<Impl: IAutomationPropertiesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLandmarkType<Impl: IAutomationPropertiesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLandmarkType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLandmarkType<Impl: IAutomationPropertiesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLandmarkType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn LocalizedLandmarkTypeProperty<Impl: IAutomationPropertiesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocalizedLandmarkType<Impl: IAutomationPropertiesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocalizedLandmarkType<Impl: IAutomationPropertiesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics5_Impl: Sized {
    fn IsPeripheralProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsPeripheral(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsPeripheral(&mut self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn IsDataValidForFormProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDataValidForForm(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDataValidForForm(&mut self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn FullDescriptionProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFullDescription(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFullDescription(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LocalizedControlTypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLocalizedControlType(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalizedControlType(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DescribedByProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDescribedBy(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
    fn FlowsToProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFlowsTo(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
    fn FlowsFromProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFlowsFrom(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics5_Vtbl {
        unsafe extern "system" fn IsPeripheralProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsPeripheral<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPeripheral<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPeripheral(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn IsDataValidForFormProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsDataValidForForm<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDataValidForForm<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDataValidForForm(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn FullDescriptionProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFullDescription<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFullDescription<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullDescription(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalizedControlTypeProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocalizedControlType<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocalizedControlType<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalizedControlType(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DescribedByProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDescribedBy<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FlowsToProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFlowsTo<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FlowsFromProperty<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFlowsFrom<Impl: IAutomationPropertiesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics6_Impl: Sized {
    fn CultureProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCulture(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetCulture(&mut self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPropertiesStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics6_Vtbl {
        unsafe extern "system" fn CultureProperty<Impl: IAutomationPropertiesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCulture<Impl: IAutomationPropertiesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCulture<Impl: IAutomationPropertiesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics7_Impl: Sized {
    fn HeadingLevelProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHeadingLevel(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationHeadingLevel>;
    fn SetHeadingLevel(&mut self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationHeadingLevel) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics7_Vtbl {
        unsafe extern "system" fn HeadingLevelProperty<Impl: IAutomationPropertiesStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetHeadingLevel<Impl: IAutomationPropertiesStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeadingLevel<Impl: IAutomationPropertiesStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics8_Impl: Sized {
    fn IsDialogProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDialog(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDialog(&mut self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPropertiesStatics8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics8_Vtbl {
        unsafe extern "system" fn IsDialogProperty<Impl: IAutomationPropertiesStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsDialog<Impl: IAutomationPropertiesStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDialog<Impl: IAutomationPropertiesStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
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
pub trait IAutomationPropertiesStatics9_Impl: Sized {
    fn AutomationControlTypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAutomationControlType(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<Peers::AutomationControlType>;
    fn SetAutomationControlType(&mut self, element: &::core::option::Option<super::UIElement>, value: Peers::AutomationControlType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPropertiesStatics9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9";
}
#[cfg(all(feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IAutomationPropertiesStatics9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPropertiesStatics9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPropertiesStatics9_Vtbl {
        unsafe extern "system" fn AutomationControlTypeProperty<Impl: IAutomationPropertiesStatics9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAutomationControlType<Impl: IAutomationPropertiesStatics9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationControlType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutomationControlType<Impl: IAutomationPropertiesStatics9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationControlType) -> ::windows::core::HRESULT {
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
pub trait IAutomationProperty_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IAutomationProperty";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationProperty_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationProperty, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDockPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDockPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDockPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IDockPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDockPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDockPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDockPatternIdentifiersStatics_Impl: Sized {
    fn DockPositionProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDockPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDockPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn DockPositionProperty<Impl: IDockPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDragPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDragPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IDragPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDragPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragPatternIdentifiersStatics_Impl: Sized {
    fn DropEffectProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn DropEffectsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn GrabbedItemsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsGrabbedProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDragPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn DropEffectProperty<Impl: IDragPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DropEffectsProperty<Impl: IDragPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GrabbedItemsProperty<Impl: IDragPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsGrabbedProperty<Impl: IDragPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDropTargetPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDropTargetPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetPatternIdentifiersStatics_Impl: Sized {
    fn DropTargetEffectProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn DropTargetEffectsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn DropTargetEffectProperty<Impl: IDropTargetPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DropTargetEffectsProperty<Impl: IDropTargetPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IExpandCollapsePatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IExpandCollapsePatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapsePatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapsePatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IExpandCollapsePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpandCollapsePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExpandCollapsePatternIdentifiersStatics_Impl: Sized {
    fn ExpandCollapseStateProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExpandCollapsePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IExpandCollapsePatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapsePatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapsePatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ExpandCollapseStateProperty<Impl: IExpandCollapsePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGridItemPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IGridItemPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridItemPatternIdentifiersStatics_Impl: Sized {
    fn ColumnProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ColumnSpanProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ContainingGridProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn RowProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn RowSpanProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridItemPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ColumnProperty<Impl: IGridItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColumnSpanProperty<Impl: IGridItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContainingGridProperty<Impl: IGridItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RowProperty<Impl: IGridItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RowSpanProperty<Impl: IGridItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGridPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IGridPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridPatternIdentifiersStatics_Impl: Sized {
    fn ColumnCountProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn RowCountProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ColumnCountProperty<Impl: IGridPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RowCountProperty<Impl: IGridPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IMultipleViewPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultipleViewPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IMultipleViewPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMultipleViewPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleViewPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultipleViewPatternIdentifiersStatics_Impl: Sized {
    fn CurrentViewProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn SupportedViewsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultipleViewPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMultipleViewPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn CurrentViewProperty<Impl: IMultipleViewPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedViewsProperty<Impl: IMultipleViewPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRangeValuePatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeValuePatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValuePatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValuePatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeValuePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeValuePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeValuePatternIdentifiersStatics_Impl: Sized {
    fn IsReadOnlyProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn LargeChangeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn MaximumProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn MinimumProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn SmallChangeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ValueProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeValuePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeValuePatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValuePatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValuePatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn IsReadOnlyProperty<Impl: IRangeValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LargeChangeProperty<Impl: IRangeValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaximumProperty<Impl: IRangeValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumProperty<Impl: IRangeValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SmallChangeProperty<Impl: IRangeValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueProperty<Impl: IRangeValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IScrollPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IScrollPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollPatternIdentifiersStatics_Impl: Sized {
    fn HorizontallyScrollableProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn HorizontalScrollPercentProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn HorizontalViewSizeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn NoScroll(&mut self) -> ::windows::core::Result<f64>;
    fn VerticallyScrollableProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn VerticalScrollPercentProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn VerticalViewSizeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn HorizontallyScrollableProperty<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HorizontalScrollPercentProperty<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HorizontalViewSizeProperty<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NoScroll<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticallyScrollableProperty<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalScrollPercentProperty<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalViewSizeProperty<Impl: IScrollPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISelectionItemPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionItemPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionItemPatternIdentifiersStatics_Impl: Sized {
    fn IsSelectedProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn SelectionContainerProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionItemPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn IsSelectedProperty<Impl: ISelectionItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionContainerProperty<Impl: ISelectionItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISelectionPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionPatternIdentifiersStatics_Impl: Sized {
    fn CanSelectMultipleProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsSelectionRequiredProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn SelectionProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectionPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectionPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn CanSelectMultipleProperty<Impl: ISelectionPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSelectionRequiredProperty<Impl: ISelectionPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionProperty<Impl: ISelectionPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISpreadsheetItemPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ISpreadsheetItemPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpreadsheetItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpreadsheetItemPatternIdentifiersStatics_Impl: Sized {
    fn FormulaProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpreadsheetItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpreadsheetItemPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn FormulaProperty<Impl: ISpreadsheetItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IStylesPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStylesPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IStylesPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IStylesPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStylesPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylesPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStylesPatternIdentifiersStatics_Impl: Sized {
    fn ExtendedPropertiesProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn FillColorProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn FillPatternColorProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn FillPatternStyleProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ShapeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn StyleIdProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn StyleNameProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStylesPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStylesPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ExtendedPropertiesProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FillColorProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FillPatternColorProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FillPatternStyleProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShapeProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StyleIdProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StyleNameProperty<Impl: IStylesPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITableItemPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITableItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITableItemPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITableItemPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableItemPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITableItemPatternIdentifiersStatics_Impl: Sized {
    fn ColumnHeaderItemsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn RowHeaderItemsProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITableItemPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITableItemPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ColumnHeaderItemsProperty<Impl: ITableItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RowHeaderItemsProperty<Impl: ITableItemPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITablePatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITablePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITablePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITablePatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITablePatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITablePatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITablePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITablePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITablePatternIdentifiersStatics_Impl: Sized {
    fn ColumnHeadersProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn RowHeadersProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn RowOrColumnMajorProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITablePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITablePatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITablePatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITablePatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ColumnHeadersProperty<Impl: ITablePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RowHeadersProperty<Impl: ITablePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RowOrColumnMajorProperty<Impl: ITablePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITogglePatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITogglePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITogglePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITogglePatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITogglePatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITogglePatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITogglePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITogglePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITogglePatternIdentifiersStatics_Impl: Sized {
    fn ToggleStateProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITogglePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITogglePatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITogglePatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITogglePatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn ToggleStateProperty<Impl: ITogglePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITransformPattern2Identifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPattern2Identifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPattern2Identifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPattern2Identifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPattern2Identifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPattern2Identifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformPattern2Identifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformPattern2Identifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPattern2IdentifiersStatics_Impl: Sized {
    fn CanZoomProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ZoomLevelProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn MaxZoomProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn MinZoomProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPattern2IdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPattern2IdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPattern2IdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPattern2IdentifiersStatics_Vtbl {
        unsafe extern "system" fn CanZoomProperty<Impl: ITransformPattern2IdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZoomLevelProperty<Impl: ITransformPattern2IdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxZoomProperty<Impl: ITransformPattern2IdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinZoomProperty<Impl: ITransformPattern2IdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITransformPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPatternIdentifiersStatics_Impl: Sized {
    fn CanMoveProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn CanResizeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn CanRotateProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn CanMoveProperty<Impl: ITransformPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanResizeProperty<Impl: ITransformPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanRotateProperty<Impl: ITransformPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IValuePatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IValuePatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IValuePatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValuePatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValuePatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IValuePatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValuePatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IValuePatternIdentifiersStatics_Impl: Sized {
    fn IsReadOnlyProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn ValueProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IValuePatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IValuePatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValuePatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValuePatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn IsReadOnlyProperty<Impl: IValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueProperty<Impl: IValuePatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWindowPatternIdentifiers_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IWindowPatternIdentifiers";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowPatternIdentifiers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowPatternIdentifiers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowPatternIdentifiers_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowPatternIdentifiers, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowPatternIdentifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowPatternIdentifiersStatics_Impl: Sized {
    fn CanMaximizeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn CanMinimizeProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsModalProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn IsTopmostProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn WindowInteractionStateProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
    fn WindowVisualStateProperty(&mut self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowPatternIdentifiersStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowPatternIdentifiersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowPatternIdentifiersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowPatternIdentifiersStatics_Vtbl {
        unsafe extern "system" fn CanMaximizeProperty<Impl: IWindowPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanMinimizeProperty<Impl: IWindowPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsModalProperty<Impl: IWindowPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTopmostProperty<Impl: IWindowPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WindowInteractionStateProperty<Impl: IWindowPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WindowVisualStateProperty<Impl: IWindowPatternIdentifiersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
